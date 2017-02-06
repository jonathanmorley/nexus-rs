use ::{Client, Response};

use models::content::ContentMetadata;
use hyper::client::Response as HyperResponse;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RepositorySummary {
    #[serde(rename="resourceURI")]
    pub resource_uri: String,
    #[serde(rename="contentResourceURI")]
    pub content_resource_uri: Option<String>,
    pub id: String,
    pub name: String,
    #[serde(rename="repoType")]
    pub repo_type: String,
    #[serde(rename="repoPolicy")]
    pub repo_policy: Option<String>,
    pub provider: String,
    #[serde(rename="providerRole")]
    pub provider_role: String,
    pub format: String,
    #[serde(rename="userManaged")]
    pub user_managed: bool,
    pub exposed: bool,
    #[serde(rename="effectiveLocalStorageUrl")]
    pub effective_local_storage_url: String,
    #[serde(rename="remoteUri")]
    pub remote_uri: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Repository {
    #[serde(rename="contentResourceURI")]
    pub content_resource_uri: Option<String>,
    pub id: String,
    pub name: String,
    pub provider: String,
    #[serde(rename="providerRole")]
    pub provider_role: String,
    pub format: String,
    #[serde(rename="repoType")]
    pub repo_type: String,
    pub exposed: bool,
    #[serde(rename="writePolicy")]
    pub write_policy: String,
    pub browseable: bool,
    pub indexable: bool,
    #[serde(rename="notFoundCacheTTL")]
    pub not_found_cache_ttl: i32,
    #[serde(rename="repoPolicy")]
    pub repo_policy: Option<String>,
    #[serde(rename="checksumPolicy")]
    pub checksum_policy: Option<String>,
    #[serde(rename="downloadRemoteIndexes")]
    pub download_remote_indexes: bool,
    #[serde(rename="defaultLocalStorageUrl")]
    pub default_local_storage_url: String,
    #[serde(rename="remoteStorage")]
    pub remote_storage: Option<RemoteStorage>,
    #[serde(rename="fileTypeValidation")]
    pub file_type_validation: Option<bool>,
    #[serde(rename="artifactMaxAge")]
    pub artifact_max_age: Option<i32>,
    #[serde(rename="metadataMaxAge")]
    pub metadata_max_age: Option<i32>,
    #[serde(rename="itemMaxAge")]
    pub item_max_age: Option<i32>,
    #[serde(rename="autoBlockActive")]
    pub auto_block_active: Option<bool>,
}

impl Repository {
    pub fn from_id<'a>(client: &'a Client, id: &str) -> Result<Response<'a, Self>, String> {
        let path = format!("service/local/repositories/{}", id);
        client.get_relative::<Repository>(path.as_str())
    }

    pub fn from_summary(client: &Client, summary: RepositorySummary) -> Result<Response<Self>, String> {
        Repository::from_id(client, summary.id.as_str())
    }
}

impl<'a> Response<'a, Repository> {
    pub fn content_metadata_children_at<'b>(&'b self, path: &str) -> Result<Vec<Response<'b, ContentMetadata>>, String> {
        let path = format!("service/local/repositories/{}/content/{}", self.item.id, path);
        self.client.get_relative::<Vec<ContentMetadata>>(path.as_str()).map(|x| x.into())
    }

    pub fn all_content_metadata(&self) -> Result<Vec<Response<ContentMetadata>>, String> {
        match self.content_metadata_children_at("") {
            Ok(root_children) => Ok(root_children.into_iter().flat_map(|c| c.with_descendants().unwrap()).collect()),
            Err(x) => Err(x)
        }
    }

    pub fn content_at<'b>(&'b self, path: &str) -> Result<Response<'a, HyperResponse>, String> {
        let path = format!("service/local/repositories/{}/content/{}", self.item.id, path);
        self.client.get_relative_raw(path.as_str())
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RemoteStorage {
    #[serde(rename="remoteStorageUrl")]
    pub remote_storage_url: String,
}
