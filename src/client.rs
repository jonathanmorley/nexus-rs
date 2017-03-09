use error;

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

pub fn parse_response<T: Deserialize>(response: Response) -> error::Result<T> {
    let bytes = response.bytes();
    match serde_json::from_iter::<Bytes<Response>, DataContainer<T>>(bytes) {
        Ok(data_container) => Ok(data_container.data),
        Err(x) => Err(x.into()),
    }
}

impl Client {
    pub fn new<U: IntoUrl>(base_url: U) -> error::Result<Self> {
        let base_url = base_url.into_url()?;

        let mut headers = Headers::new();

        headers.set(Authorization(Basic {
            username: base_url.username().to_string(),
            password: base_url.password().map(|x| x.to_string()),
        }));

        headers.set(Accept(vec![qitem(Mime(TopLevel::Application, SubLevel::Json, vec![]))]));

        let hyper_client = HyperClient::new();
        let res = hyper_client.get(base_url.as_str()).send()?;
        if res.status == ::hyper::Ok {
            Ok(Client {
                client: hyper_client,
                headers: headers,
                base_url: base_url,
            })
        } else {
            Err(error::NexusError::Server(String::from("Non-success error code")))
        }
    }

    pub fn fetch(&self, url: &str) -> error::Result<Response> {
        self.client
            .get(self.parse_url(url)?)
            .headers(self.headers.clone())
            .send()
            .map_err(|err| err.into())
    }

    pub fn all_repositories(&self) -> error::Result<Vec<RepositorySummary>> {
        parse_response::<Vec<RepositorySummary>>(self.fetch("service/local/all_repositories")?)
    }

    pub fn repository(&self, id: &str) -> error::Result<Repository> {
        parse_response::<Repository>(self.fetch(&format!("service/local/repositories/{}", id))?)
    }

    fn parse_url(&self, url: &str) -> Result<Url, ParseError> {
        match url.into_url() {
            Ok(url) => Ok(url),
            Err(err) => {
                match err {
                    ParseError::RelativeUrlWithoutBase => self.base_url.join(url),
                    _ => Err(err),
                }
            }
        }
    }
}
