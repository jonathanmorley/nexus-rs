use models::container::DataContainer;
use models::repository::{RepositorySummary, Repository};

use hyper::{Client as HyperClient, Url};
use hyper::client::{Response, IntoUrl};
use hyper::error::ParseError;
use hyper::header::{Accept, Authorization, Basic, Headers, qitem};
use hyper::mime::{Mime, TopLevel, SubLevel};
use serde::Deserialize;
use serde_json;
use std::io::{Bytes, Read};

#[derive(Debug)]
pub struct Client {
    client: HyperClient,
    headers: Headers,
    base_url: Url,
}

impl Client {
    pub fn new<U: IntoUrl>(base_url: U) -> Result<Self, String> {
        let base_url = match base_url.into_url() {
            Ok(url) => url,
            Err(x) => return Err(x.to_string())
        };

        let mut headers = Headers::new();

        headers.set(Authorization(Basic {
            username: base_url.username().to_string(),
            password: base_url.password().map(|x| x.to_string()),
        }));

        headers.set(Accept(vec![qitem(Mime(TopLevel::Application, SubLevel::Json, vec![]))]));

        let hyper_client = HyperClient::new();

        match hyper_client.get(base_url.as_str()).send() {
            Err(x) => Err(x.to_string()),
            Ok(x) => {
                if x.status == ::hyper::Ok {
                    Ok(Client {
                        client: hyper_client,
                        headers: headers,
                        base_url: base_url,
                    })
                } else {
                    Err(String::from("Non-success error code"))
                }
            }
        }
    }

    pub fn get_raw(&self, url: &str) -> Result<Response, String> {
        let url = self.parse_url(url).unwrap();

        println!("{:?}", url);

        let req = self.client.get(url.clone()).headers(self.headers.clone());
        match req.send() {
            Ok(res) => Ok(res),
            Err(x) => { Err(format!("Error fetching {}, ({})", &url.to_string(), x.to_string())) }
        }
    }

    pub fn get<T: Deserialize>(&self, url: &str) -> Result<T, String> {
        match self.get_raw(url) {
            Ok(res) => {
                let bytes = res.bytes();
                match serde_json::from_iter::<Bytes<Response>, DataContainer<T>>(bytes) {
                    Ok(data_container) => Ok(data_container.data),
                    Err(x) => { Err(format!("Error parsing {} ({})", url, x.to_string())) }
                }
            },
            Err(x) => Err(x)
        }
    }

    pub fn all_repositories(&self) -> Result<Vec<RepositorySummary>, String> {
        self.get::<Vec<RepositorySummary>>("service/local/all_repositories")
    }

    pub fn repository(&self, id: &str) -> Result<Repository, String> {
        let path = format!("service/local/repositories/{}", id);
        self.get::<Repository>(path.as_str())
    }

    pub fn repository_from_summary(&self, summary: RepositorySummary) -> Result<Repository, String> {
        self.repository(summary.id.as_str())
    }

    fn parse_url(&self, url: &str) -> Result<Url, ParseError> {
        match url.into_url() {
            Ok(url) => Ok(url),
            Err(err) => match err {
                ParseError::RelativeUrlWithoutBase => self.base_url.join(url),
                _ => Err(err)
            }
        }
    }
}
