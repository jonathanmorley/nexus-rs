#![crate_name = "nexus_rs"]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate quick_error;

extern crate serde;
extern crate serde_json;

extern crate hyper;

extern crate time;

pub mod deserializers;
pub mod error;
pub mod models;
pub mod client;

pub use client::Client;
