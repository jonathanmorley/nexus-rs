extern crate nexus;
extern crate mockito;

use nexus::Client;
use nexus::models::content::ContentMetadata;
use nexus::models::repository::Repository;
use mockito::mock;

use std::str::FromStr;

fn test_repository() -> Repository {
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
        default_local_storage_url: String::from("file:/sonatype-work/storage/test-repository/"),
        remote_storage: None,
        file_type_validation: None,
        artifact_max_age: None,
        metadata_max_age: None,
        item_max_age: None,
        auto_block_active: None,
    }
}

fn test_repository_string() -> String {
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

fn test_content_metadata(path: &str, leaf: bool) -> ContentMetadata {
    ContentMetadata {
        resource_uri: String::from(format!("{}/service/local/repositories/test-repository/content/{}", mockito::SERVER_URL, path)),
        relative_path: String::from(path),
        text: String::from(path.split("/").last().unwrap()),
        leaf: leaf,
        last_modified: String::from("1970-01-01 00:00:00.0 UTC"),
        size_on_disk: if leaf { 1 } else { -1 },
    }
}

fn test_content_metadata_string(path: &str, leaf: bool) -> String {
    let resource_uri = format!("{}/service/local/repositories/test-repository/content/{}", mockito::SERVER_URL, path);
    format!(r#"{{"data": [{{
        "resourceURI": "{}",
        "relativePath": "{}",
        "text": "{}",
        "leaf": {},
        "lastModified": "1970-01-01 00:00:00.0 UTC",
        "sizeOnDisk": {}
    }}]}}"#, resource_uri, path, path.split("/").last().unwrap(), leaf, if leaf { 1 } else { -1 })
}

fn test_all_content_metadata() -> Vec<ContentMetadata> {
    vec![
        test_content_metadata("a", false),
        test_content_metadata("a/b", false),
        test_content_metadata("a/b/c", true),
    ]
}

#[test]
fn repository_from_id() {
    mock("GET", "/service/local/repositories/test-repository")
        .with_body(test_repository_string().as_str()).create_for(|| {
            let client = Client::from_str(mockito::SERVER_URL);
            assert!(client.is_ok());
            let client = client.unwrap();

            let repository = client.repository("test-repository");
            assert!(repository.is_ok());
            let repository = repository.unwrap();

            assert_eq!(repository.item, test_repository());
        });
}


#[test]
fn content_metadata_children_at() {
    mock("GET", "/service/local/repositories/test-repository")
        .with_body(test_repository_string().as_str()).create_for(|| {
            mock("GET", "/service/local/repositories/test-repository/content/a")
                .with_body(test_content_metadata_string("a/b", false).as_str()).create_for(|| {
                    let client = Client::from_str(mockito::SERVER_URL);
                    assert!(client.is_ok());
                    let client = client.unwrap();

                    let repository = client.repository("test-repository");
                    assert!(repository.is_ok());
                    let repository = repository.unwrap();

                    let content_metadata_children = repository.content_metadata_children_at("a")
                        .iter().map(|cm| cm.item.to_owned()).collect::<Vec<ContentMetadata>>();
                    assert_eq!(content_metadata_children, vec![test_content_metadata("a/b", false)]);
                });
        });
}

#[test]
fn all_content_metadata() {
    mock("GET", "/service/local/repositories/test-repository")
        .with_body(test_repository_string().as_str()).create_for(|| {
            mock("GET", "/service/local/repositories/test-repository/content/")
                .with_body(test_content_metadata_string("a", false).as_str()).create_for(|| {
                    mock("GET", "/service/local/repositories/test-repository/content/a")
                        .with_body(test_content_metadata_string("a/b", false).as_str()).create_for(|| {
                            mock("GET", "/service/local/repositories/test-repository/content/a/b")
                                .with_body(test_content_metadata_string("a/b/c", true).as_str()).create_for(|| {
                                    let client = Client::from_str(mockito::SERVER_URL);
                                    assert!(client.is_ok());
                                    let client = client.unwrap();

                                    let repository = client.repository("test-repository");
                                    assert!(repository.is_ok());
                                    let repository = repository.unwrap();

                                    let all_content_metadata = repository.all_content_metadata()
                                        .iter().map(|cm| cm.item.to_owned()).collect::<Vec<ContentMetadata>>();
                                    assert_eq!(all_content_metadata, test_all_content_metadata());
                                });
                        });
                });
        });
}
