#![experimental]

extern crate protobuf; // depend on rust-protobuf runtime

mod mesos;     // protobuf messages
mod native;    // scheduler and executor native hooks
mod scheduler;
mod scheduler_driver;
mod executor;
mod executor_driver;
mod utils;
