extern crate mockito;

use mockito::mock;

pub fn mock_nexus_for<F: Fn() -> ()>(environment: F) {
    mock("GET", "/").create_for(|| { environment(); });
}
