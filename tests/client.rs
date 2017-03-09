extern crate nexus_rs;
extern crate mockito;

mod fixtures;

use nexus_rs::Client;

#[test]
fn invalid_url() {
    assert!(Client::new("INVALID_URL").is_err());
}

#[test]
fn unresponsive_server() {
    assert!(Client::new(mockito::SERVER_URL).is_err());
}

#[test]
fn all_repositories() {
    fixtures::test_repository::mock_repository_for(|| {
        let client = Client::new(mockito::SERVER_URL).expect("Client is not Ok");
        let all_repositories = client.all_repositories().expect("All Repositories is not Ok");

        assert_eq!(all_repositories,
                   vec![fixtures::test_repository::repository_summary()]);
    });
}

#[test]
fn repository() {
    fixtures::test_repository::mock_repository_for(|| {
        let client = Client::new(mockito::SERVER_URL).expect("Client is not Ok");
        let repository = client.repository("test-repository").expect("Repository is not Ok");

        assert_eq!(repository, fixtures::test_repository::repository());
    });
}
