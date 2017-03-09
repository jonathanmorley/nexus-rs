use error;
use client::{Client, parse_response};
use models::repository::Repository;

use std::iter;
use std::path::PathBuf;
use time::{self, Tm, Timespec};

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ContentMetadata {
    #[serde(rename="resourceURI")]
    pub resource_uri: String,
    #[serde(rename="relativePath")]
    pub relative_path: PathBuf,
    pub text: String,
    pub leaf: bool,
    #[serde(rename="lastModified", with="::deserializers::datetime")]
    pub last_modified: Tm,
    #[serde(rename="sizeOnDisk")]
    pub size_on_disk: i64,
}

impl From<Repository> for ContentMetadata {
    fn from(repository: Repository) -> Self {
        ContentMetadata {
            resource_uri: format!("service/local/repositories/{}/content/", repository.id),
            relative_path: PathBuf::from("/"),
            text: String::from(""),
            leaf: false,
            last_modified: time::at_utc(Timespec::new(0, 0)),
            size_on_disk: -1,
        }
    }
}

impl Client {
    pub fn children<'a, T: Into<&'a ContentMetadata>>(&self,
                                                      content_metadata: T)
                                                      -> error::Result<Vec<ContentMetadata>> {
        let content_metadata = content_metadata.into();
        if content_metadata.leaf {
            Ok(Vec::new())
        } else {
            let res = self.fetch(&content_metadata.resource_uri)?;
            parse_response::<Vec<ContentMetadata>>(res)
        }
    }

    pub fn with_descendants<T: Into<ContentMetadata>>(&self,
                                                      content_metadata: T)
                                                      -> error::Result<Vec<ContentMetadata>> {
        let content_metadata = content_metadata.into();
        let children = self.children(&content_metadata);

        Ok(iter::once(content_metadata)
            .chain(children?
                    .into_iter()
                    .flat_map(|child| self.with_descendants(child).unwrap()))
            .collect::<Vec<ContentMetadata>>())
    }
}
