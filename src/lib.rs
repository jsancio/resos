#![deny(unused_mut)]
#[macro_use]
extern crate hyper;
extern crate protobuf;

// mesos protobuf messages, bad name?
//
// We want to reexport the messages as part of this crate.
// Otherwise one cannot use the provided interfaces.
// `pub extern crate` does not work. See https://github.com/rust-lang/rust/issues/21757
// This is why import crate for a different name here such that we can use pub use below.
extern crate proto as proto_crate; 

extern crate zookeeper;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate rustc_serialize;

pub use proto_crate as proto;

mod master_detector;
pub mod scheduler;
pub mod scheduler_driver;
mod executor;
mod executor_driver;
#[allow(non_upper_case_globals)]
mod http_api;
