extern crate nexus_rs;
extern crate mockito;

mod fixtures;

use nexus_rs::Client;

use std::io::Read;

#[test]
fn content_metadata_at() {
    fixtures::test_repository::mock_repository_for(|| {
        let client = Client::new(mockito::SERVER_URL).expect("Client is not Ok");
        let repository = client.repository("test-repository").expect("Repository is not Ok");
        let content_metadata = client.content_metadata_at(&repository, "a")
            .expect("Content Metadata is not Ok");

        assert_eq!(content_metadata,
                   vec![fixtures::test_repository::content_metadata("a/b", false)]);
    });
}

#[test]
fn all_content_metadata() {
    fixtures::test_repository::mock_repository_for(|| {
        let client = Client::new(mockito::SERVER_URL).expect("Client is not Ok");
        let repository = client.repository("test-repository").expect("Repository is not Ok");
        let all_content_metadata = client.with_descendants(repository)
            .expect("All content metadata is not Ok");

        assert_eq!(all_content_metadata,
                   fixtures::test_repository::all_content_metadata());
    });
}

#[test]
fn content_at() {
    fixtures::test_repository::mock_repository_for(|| {
        let client = Client::new(mockito::SERVER_URL).expect("Client is not Ok");
        let repository = client.repository("test-repository").expect("Repository is not Ok");
        let mut content_at = client.content_at(&repository, "a/b/c").expect("Content is not Ok");

        let mut buffer = String::new();
        let read_result = content_at.read_to_string(&mut buffer);
        assert!(read_result.is_ok());

        assert_eq!(buffer, "Test Content");
    });
}
