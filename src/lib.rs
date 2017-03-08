#![crate_name = "nexus_rs"]

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate hyper;

extern crate time;

pub mod deserializers;
pub mod models;
pub mod client;

pub use client::Client;
