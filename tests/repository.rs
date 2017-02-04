extern crate nexus;
extern crate mockito;

mod fixtures;

use nexus::Client;
use nexus::models::content::ContentMetadata;

use std::str::FromStr;

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
