extern crate nexus_rs;
extern crate mockito;
extern crate hyper;

#[macro_use]
extern crate matches;

mod fixtures;

use nexus_rs::Client;
use nexus_rs::error::NexusError::*;
use hyper::error::ParseError::*;

#[test]
fn invalid_url() {
    let client = Client::new("INVALID_URL");
    println!("{:?}", client);
    matches!(client.err(), Some(UrlParse(RelativeUrlWithoutBase)));
}

#[test]
fn unresponsive_server() {
    let client = Client::new(mockito::SERVER_URL);
    assert!(client.is_err());
}

#[test]
fn all_repositories() {
    fixtures::test_repository::mock_repository_for(|| {
        let client = Client::new(mockito::SERVER_URL);
        assert!(client.is_ok());
        let client = client.unwrap();

        let all_repositories = client.all_repositories();
        assert!(all_repositories.is_ok());
        let all_repositories = all_repositories.unwrap();

        assert_eq!(all_repositories, vec![fixtures::test_repository::repository_summary()]);
    });
}
