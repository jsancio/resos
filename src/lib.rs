#![feature(core,libc)]
extern crate protobuf; // depend on rust-protobuf runtime

pub mod proto;     // protobuf messages
pub mod scheduler;
pub mod scheduler_driver;
mod executor;
mod executor_driver;
mod utils;
