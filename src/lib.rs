//#![deny(unused_mut)]
#![feature(io)]
#[macro_use]
extern crate hyper;
extern crate protobuf;
extern crate proto; // mesos protobuf messages, bad name?
extern crate zookeeper;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate rustc_serialize;
extern crate time;
extern crate uuid;

mod master_detector;
pub mod scheduler;
pub mod scheduler_driver;
mod executor;
mod executor_driver;
#[allow(non_upper_case_globals)]
mod http_api;
