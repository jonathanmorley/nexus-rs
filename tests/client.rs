extern crate nexus_rs;
extern crate mockito;

mod fixtures;

use nexus_rs::Client;
use nexus_rs::models::repository::RepositorySummary;

use std::str::FromStr;

#[test]
fn invalid_url() {
    let client = Client::from_str("INVALID_URL");
    assert_eq!(client.err(), Some(String::from("Invalid URL")));
}

#[test]
fn unresponsive_server() {
    let client = Client::from_str(mockito::SERVER_URL);
    assert!(client.is_err());
}

#[test]
fn all_repositories() {
    fixtures::test_repository::mock_repository_for(|| {
        let client = Client::from_str(mockito::SERVER_URL);
        assert!(client.is_ok());
        let client = client.unwrap();

        let all_repositories = client.all_repositories();
        assert!(all_repositories.is_ok());
        let all_repositories = all_repositories.unwrap().iter().map({|repo|
            repo.item.to_owned()
        }).collect::<Vec<RepositorySummary>>();

        assert_eq!(all_repositories, vec![fixtures::test_repository::repository_summary()]);
    });
}
