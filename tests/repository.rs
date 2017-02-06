extern crate nexus_rs;
extern crate mockito;

mod fixtures;

use nexus_rs::Client;
use nexus_rs::models::content::ContentMetadata;

use std::str::FromStr;
use std::io::Read;

#[test]
fn repository_from_id() {
    fixtures::test_repository::mock_repository_for(|| {
        let client = Client::from_str(mockito::SERVER_URL);
        assert!(client.is_ok());
        let client = client.unwrap();

        let repository = client.repository("test-repository");
        assert!(repository.is_ok());
        let repository = repository.unwrap();

        assert_eq!(repository.item, fixtures::test_repository::repository());
    });
}


#[test]
fn content_metadata_children_at() {
    fixtures::test_repository::mock_repository_for(|| {
        let client = Client::from_str(mockito::SERVER_URL);
        assert!(client.is_ok());
        let client = client.unwrap();

        let repository = client.repository("test-repository");
        assert!(repository.is_ok());
        let repository = repository.unwrap();

        let content_metadata_children = repository.content_metadata_children_at("a");
        assert!(content_metadata_children.is_ok());
        let content_metadata_children = content_metadata_children.unwrap().iter().map({|cm|
            cm.item.to_owned()
        }).collect::<Vec<ContentMetadata>>();

        assert_eq!(content_metadata_children, vec![fixtures::test_repository::content_metadata("a/b", false)]);
    });
}

#[test]
fn all_content_metadata() {
    fixtures::test_repository::mock_repository_for(|| {
        let client = Client::from_str(mockito::SERVER_URL);
        assert!(client.is_ok());
        let client = client.unwrap();

        let repository = client.repository("test-repository");
        assert!(repository.is_ok());
        let repository = repository.unwrap();

        let all_content_metadata = repository.all_content_metadata();

        assert!(all_content_metadata.is_ok());
        let all_content_metadata = all_content_metadata.unwrap().iter().map({|cm|
            cm.item.to_owned()
        }).collect::<Vec<ContentMetadata>>();

        assert_eq!(all_content_metadata, fixtures::test_repository::all_content_metadata());
    });
}

#[test]
fn content_at() {
    fixtures::test_repository::mock_repository_for(|| {
        let client = Client::from_str(mockito::SERVER_URL);
        assert!(client.is_ok());
        let client = client.unwrap();

        let repository = client.repository("test-repository");
        assert!(repository.is_ok());
        let repository = repository.unwrap();

        let content_at = repository.content_at("a/b/c");
        assert!(content_at.is_ok());

        let mut buffer = String::new();
        let read_result = content_at.unwrap().item.read_to_string(&mut buffer);
        assert!(read_result.is_ok());

        assert_eq!(buffer, "Test Content");
    });
}
