#![deny(unused_mut)]
#![feature(duration)]
#[macro_use]
extern crate hyper;
extern crate protobuf;
extern crate proto; // mesos protobuf messages, bad name?
extern crate zookeeper;
#[macro_use]
extern crate log;

mod master_detector;
pub mod scheduler;
pub mod scheduler_driver;
mod executor;
mod executor_driver;
mod libprocess;
