use ::Client;
use ::models::repository::Repository;

use time::{self, Tm, Timespec};

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ContentMetadata {
    #[serde(rename="resourceURI")]
    pub resource_uri: String,
    #[serde(rename="relativePath")]
    pub relative_path: String,
    pub text: String,
    pub leaf: bool,
    #[serde(rename="lastModified", with="::deserializers::datetime")]
    pub last_modified: Tm,
    #[serde(rename="sizeOnDisk")]
    pub size_on_disk: i64,
}

impl From<Repository> for ContentMetadata {
    fn from(repository: Repository) -> Self {
        Self {
            resource_uri: format!("service/local/repositories/{}/content/", repository.id),
            relative_path: String::from("/"),
            text: String::from(""),
            leaf: false,
            last_modified: time::at_utc(Timespec::new(0, 0)),
            size_on_disk: -1
        }
    }
}

impl Client {
    pub fn children<'a, T: Into<&'a ContentMetadata>>(&self, content_metadata: T) -> Result<Vec<ContentMetadata>, String> {
        let content_metadata: &ContentMetadata = content_metadata.into();
        if content_metadata.leaf {
            Ok(Vec::new())
        } else {
            let children_uri = content_metadata.resource_uri.as_str();
            self.get::<Vec<ContentMetadata>>(children_uri)
        }
    }

    pub fn with_descendants<T: Into<ContentMetadata>>(&self, content_metadata: T) -> Result<Vec<ContentMetadata>, String> {
        let content_metadata = content_metadata.into();
        match self.children(&content_metadata) {
            Ok(children) => {
                let mut descendants = if children.is_empty() {
                    children
                } else {
                    children.iter().flat_map(|child| self.with_descendants(child.clone()).unwrap()).collect::<Vec<ContentMetadata>>()
                };

                descendants.insert(0, content_metadata);
                Ok(descendants)
            },
            Err(x) => Err(x)
        }
    }
}
