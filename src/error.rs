use std::io;
use hyper;
use serde_json;
use time;
use std::result;

pub type Result<T> = result::Result<T, NexusError>;

quick_error! {
    #[derive(Debug)]
    pub enum NexusError {
        Client(err: String)
        Server(err: String)
        Io(err: io::Error) {
            from()
        }
        UrlParse(err: hyper::error::ParseError) {
            from()
        }
        Http(err: hyper::Error) {
            from()
        }
        Json(err: serde_json::Error) {
            from()
        }
        TimeParse(err: time::ParseError) {
            from()
        }
    }
}
