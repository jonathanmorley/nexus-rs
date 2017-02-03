extern crate nexus;
extern crate mockito;

use nexus::Client;
use mockito::mock;

use std::str::FromStr;

#[test]
fn invalid_url() {
    let client = Client::from_str("INVALID_URL");
    assert_eq!(client.err(), Some(String::from("Invalid URL")));
}

#[test]
fn unresponsive_server() {
    let client = Client::from_str(mockito::SERVER_URL);
    assert_eq!(client.err(), Some(String::from("Connection refused (os error 61)")));
}

#[test]
fn all_repositories() {
    mock("GET", "/service/local/all_repositories")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body("{\"data\": []}")
        .create_for(|| {
            let client = Client::from_str(mockito::SERVER_URL);
            assert!(client.is_ok());
            let client = client.unwrap();
            assert_eq!(client.all_repositories().len(), 0);
        });
}
