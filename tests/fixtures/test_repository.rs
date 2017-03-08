extern crate nexus_rs;
extern crate mockito;
extern crate time;

use fixtures::client::mock_nexus_for;
use nexus_rs::models::content::ContentMetadata;
use nexus_rs::models::repository::{RepositorySummary, Repository};
use mockito::mock;
use self::time::Timespec;

fn repository_summary_string() -> String {
    format!(r#"{{"data": [{{
        "resourceURI": "{}/service/local/repositories/test-repository",
        "contentResourceURI": "{}/content/sites/test-repository",
        "id": "test-repository",
        "name": "Test Repository",
        "repoType": "hosted",
        "provider": "site",
        "providerRole": "org.sonatype.nexus.proxy.repository.WebSiteRepository",
        "format": "site",
        "userManaged": true,
        "exposed": true,
        "effectiveLocalStorageUrl": "file:/sonatype-work/storage/test-repository/"
    }}]}}"#, mockito::SERVER_URL, mockito::SERVER_URL)
}

fn repository_string() -> String {
    format!(r#"{{"data": {{
        "contentResourceURI": "{}/content/sites/test-repository",
        "id": "test-repository",
        "name": "Test Repository",
        "provider": "site",
        "providerRole": "org.sonatype.nexus.proxy.repository.WebSiteRepository",
        "format": "site",
        "repoType": "hosted",
        "exposed": true,
        "writePolicy": "ALLOW_WRITE",
        "browseable": true,
        "indexable": false,
        "notFoundCacheTTL": 1440,
        "repoPolicy": "MIXED",
        "checksumPolicy": "IGNORE",
        "downloadRemoteIndexes": false,
        "defaultLocalStorageUrl": "file:/sonatype-work/storage/test-repository/"
    }}}}"#, mockito::SERVER_URL)
}

fn content_metadata_string(path: &str, leaf: bool) -> String {
    let resource_uri = format!("{}/service/local/repositories/test-repository/content/{}", mockito::SERVER_URL, path);
    format!(r#"{{"data": [{{
        "resourceURI": "{}",
        "relativePath": "{}",
        "text": "{}",
        "leaf": {},
        "lastModified": "1970-01-01 00:00:00.0 UTC",
        "sizeOnDisk": {}
    }}]}}"#, resource_uri, path, path.split('/').last().unwrap(), leaf, if leaf { 1 } else { -1 })
}

#[allow(dead_code)]
pub fn repository_summary() -> RepositorySummary {
    RepositorySummary {
        resource_uri: String::from(format!("{}/service/local/repositories/test-repository", mockito::SERVER_URL)),
        content_resource_uri: Some(String::from(format!("{}/content/sites/test-repository", mockito::SERVER_URL))),
        id: String::from("test-repository"),
        name: String::from("Test Repository"),
        repo_type: String::from("hosted"),
        repo_policy: None,
        provider: String::from("site"),
        provider_role: String::from("org.sonatype.nexus.proxy.repository.WebSiteRepository"),
        format: String::from("site"),
        user_managed: true,
        exposed: true,
        local_storage_url: String::from("file:/sonatype-work/storage/test-repository/"),
        remote_uri: None,
    }
}

#[allow(dead_code)]
pub fn repository() -> Repository {
    Repository {
        content_resource_uri: Some(String::from(format!("{}/content/sites/test-repository", mockito::SERVER_URL))),
        id: String::from("test-repository"),
        name: String::from("Test Repository"),
        provider: String::from("site"),
        provider_role: String::from("org.sonatype.nexus.proxy.repository.WebSiteRepository"),
        format: String::from("site"),
        repo_type: String::from("hosted"),
        exposed: true,
        write_policy: String::from("ALLOW_WRITE"),
        browseable: true,
        indexable: false,
        not_found_cache_ttl: 1440,
        repo_policy: Some(String::from("MIXED")),
        checksum_policy: Some(String::from("IGNORE")),
        download_remote_indexes: false,
        local_storage_url: String::from("file:/sonatype-work/storage/test-repository/"),
        remote_storage: None,
        file_type_validation: None,
        artifact_max_age: None,
        metadata_max_age: None,
        item_max_age: None,
        auto_block_active: None,
    }
}

pub fn content_metadata(path: &str, leaf: bool) -> ContentMetadata {
    ContentMetadata {
        resource_uri: String::from(format!("{}/service/local/repositories/test-repository/content/{}", mockito::SERVER_URL, path)),
        relative_path: String::from(path),
        text: String::from(path.split('/').last().unwrap()),
        leaf: leaf,
        last_modified: time::at_utc(Timespec::new(0, 0)),
        size_on_disk: if leaf { 1 } else { -1 },
    }
}

#[allow(dead_code)]
pub fn all_content_metadata() -> Vec<ContentMetadata> {
    vec![
        ContentMetadata {
            resource_uri: String::from("service/local/repositories/test-repository/content/"),
            relative_path: String::from("/"),
            text: String::from(""),
            leaf: false,
            last_modified: time::at_utc(Timespec::new(0, 0)),
            size_on_disk: -1,
        },
        content_metadata("a", false),
        content_metadata("a/b", false),
        content_metadata("a/b/c", true),
    ]
}

pub fn mock_repository_for<F: Fn() -> ()>(environment: F) {
    mock_nexus_for(|| {
        mock("GET", "/service/local/all_repositories")
        .with_body(repository_summary_string().as_str()).create_for(|| {
            mock("GET", "/service/local/repositories/test-repository")
            .with_body(repository_string().as_str()).create_for(|| {
                mock("GET", "/service/local/repositories/test-repository/content/")
                .with_body(content_metadata_string("a", false).as_str()).create_for(|| {
                    mock("GET", "/service/local/repositories/test-repository/content/a")
                    .with_body(content_metadata_string("a/b", false).as_str()).create_for(|| {
                        mock("GET", "/service/local/repositories/test-repository/content/a/b")
                        .with_body(content_metadata_string("a/b/c", true).as_str()).create_for(|| {
                            mock("GET", "/service/local/repositories/test-repository/content/a/b/c")
                            .with_body("Test Content").create_for(|| {
                                environment();
                            });
                        });
                    });
                });
            });
        });
    });
}
