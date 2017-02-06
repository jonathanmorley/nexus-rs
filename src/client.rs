use ::Response;
use models::container::DataContainer;
use models::repository::{RepositorySummary, Repository};

use hyper::{Client as HyperClient, Url};
use hyper::client::Response as HyperResponse;
use hyper::header::{Accept, Authorization, Basic, Headers, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel};
use serde::Deserialize;
use serde_json;
use std::io::{Bytes, Read};
use std::str::FromStr;

#[derive(Debug)]
pub struct Client {
    client: HyperClient,
    headers: Headers,
    base_url: Url,
}

impl FromStr for Client {
    type Err=String;
    fn from_str(url: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(url);

        match url {
            Ok(url) => Client::from_url(url),
            Err(_) => Err("Invalid URL".to_string())
        }
    }
}

impl Client {
    pub fn from_url(url: Url) -> Result<Self, String> {
        let mut headers = Headers::new();

        headers.set(Authorization(Basic {
            username: url.username().to_string(),
            password: url.password().map(|x| x.to_string()),
        }));

        headers.set(Accept(vec![qitem(Mime(TopLevel::Application, SubLevel::Json, vec![]))]));

        let hyper_client = HyperClient::new();

        match hyper_client.get(url.as_str()).send() {
            Err(x) => Err(x.to_string()),
            Ok(x) => {
                if x.status == ::hyper::Ok {
                    Ok(Client {
                        client: hyper_client,
                        headers: headers,
                        base_url: url,
                    })
                } else {
                    Err(String::from("Non-success error code"))
                }
            }
        }
    }

    pub fn get_absolute_raw<'a>(&'a self, url: &str) -> Result<Response<'a, HyperResponse>, String> {
        let url = Url::parse(url).unwrap();
        let req = self.client.get(url.clone()).headers(self.headers.clone());
        match req.send() {
            Ok(res) => Ok(Response { client: self, item: res }),
            Err(x) => { Err(format!("Error fetching {}, ({})", &url.to_string(), x.to_string())) }
        }
    }

    pub fn get_absolute<'a, T: Deserialize>(&'a self, url: &str) -> Result<Response<'a, T>, String> {
        match self.get_absolute_raw(url) {
            Ok(res) => match serde_json::from_iter::<Bytes<HyperResponse>, DataContainer<T>>(res.item.bytes()) {
                Ok(data_container) => Ok(Response { client: self, item: data_container.data }),
                Err(x) => { Err(format!("Error parsing {}, ({})", &url.to_string(), x.to_string())) }
            },
            Err(x) => Err(x)
        }
    }

    pub fn get_relative_raw<'a>(&'a self, path: &str) -> Result<Response<'a, HyperResponse>, String> {
        self.get_absolute_raw(self.base_url.join(path).unwrap().as_str())
    }

    pub fn get_relative<'a, T: Deserialize>(&'a self, path: &str) -> Result<Response<'a, T>, String> {
        self.get_absolute::<T>(self.base_url.join(path).unwrap().as_str())
    }

    pub fn all_repositories(&self) -> Result<Vec<Response<RepositorySummary>>, String> {
        self.get_relative::<Vec<RepositorySummary>>("service/local/all_repositories").map(|x| x.into())
    }

    pub fn repository(&self, id: &str) -> Result<Response<Repository>, String> {
        Repository::from_id(self, id)
    }

    pub fn with_item<T>(&self, item: T) -> Response<T> {
        Response {
            client: self,
            item: item,
        }
    }
}
