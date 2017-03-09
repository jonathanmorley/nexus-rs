use error;
use client::{Client, parse_response};

use models::content::ContentMetadata;
use hyper::Url;
use hyper::client::Response;

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
    #[serde(rename="effectiveLocalStorageUrl", with="::deserializers::url")]
    pub local_storage_url: Url,
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
    #[serde(rename="defaultLocalStorageUrl", with="::deserializers::url")]
    pub local_storage_url: Url,
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

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RemoteStorage {
    #[serde(rename="remoteStorageUrl")]
    pub remote_storage_url: String,
}

impl From<Repository> for RepositorySummary {
    fn from(repository: Repository) -> Self {
        RepositorySummary {
            resource_uri: format!("service/local/repositories/{}", repository.id),
            content_resource_uri: repository.content_resource_uri,
            id: repository.id,
            name: repository.name,
            repo_type: repository.repo_type,
            repo_policy: repository.repo_policy,
            provider: repository.provider,
            provider_role: repository.provider_role,
            format: repository.format,
            user_managed: true,
            exposed: repository.exposed,
            local_storage_url: repository.local_storage_url,
            remote_uri: repository.remote_storage.map(|rs| rs.remote_storage_url),
        }
    }
}

impl Client {
    pub fn content_metadata_at(&self,
                               repository: &Repository,
                               path: &str)
                               -> error::Result<Vec<ContentMetadata>> {
        parse_response::<Vec<ContentMetadata>>(self.content_at(repository, path)?)
    }

    pub fn content_at(&self, repository: &Repository, path: &str) -> error::Result<Response> {
        self.fetch(&format!("service/local/repositories/{}/content/{}",
                            repository.id,
                            path))
    }
}
