use hyper::Url;
use serde::{self, Deserialize, Deserializer};

pub fn deserialize<D: Deserializer>(deserializer: D) -> Result<Url, D::Error> {
    let s = String::deserialize(deserializer)?;
    Url::parse(&s).map_err(serde::de::Error::custom)
}
