#[macro_use]
extern crate hyper;
extern crate env_logger;
extern crate mesos;
extern crate protobuf;

use hyper::Client;
use hyper::client::Response;
use hyper::error::Result;
use hyper::header::{ContentType, Headers};
use hyper::server;
use hyper::server::Server;
use hyper::status::StatusCode;
use hyper::uri::RequestUri::AbsolutePath;
use protobuf::{Message, parse_from_reader};
use mesos::proto::{ExecutorID, FrameworkID, FrameworkInfo, MasterInfo, Offer, OfferID, SlaveID, TaskStatus};
use mesos::proto_internal::{RegisterFrameworkMessage, FrameworkRegisteredMessage};
use mesos::scheduler::Scheduler;
use mesos::scheduler_driver::{SchedulerDriver, MesosSchedulerDriver};
use mesos::utils;
use std::net::SocketAddr;

struct MyScheduler;
impl Scheduler for MyScheduler {
    fn disconnected(&self, driver: &SchedulerDriver) {}

    // Invoked when there is an unrecoverable error in the scheduler or driver.
    fn error(&self, driver: &SchedulerDriver, message: &str) {}

    // Invoked when an executor has exited/terminated.
    fn executor_lost(&self, driver: &SchedulerDriver, executorId: &ExecutorID, slaveId: &SlaveID, status: isize) {}

    // Invoked when an executor sends a message.
    fn framework_message(&self, driver: &SchedulerDriver, executorId: &ExecutorID, slaveId: &SlaveID, data: &[u8]) {}

    // Invoked when an offer is no longer valid (e.g., the slave was lost or another framework used resources in the offer).
    fn offer_rescinded(&self, driver: &SchedulerDriver, offerId: &OfferID) {}

    // Invoked when the scheduler successfully registers with a Mesos master.
    fn registered(&self, driver: &SchedulerDriver, frameworkId: &FrameworkID, masterInfo: &MasterInfo) {}

    // Invoked when the scheduler re-registers with a newly elected Mesos master.
    fn reregistered(&self, driver: &SchedulerDriver, masterInfo: &MasterInfo) {}

    // Invoked when resources have been offered to this framework.
    fn resource_offers(&self, driver: &SchedulerDriver, offers: &Vec<Offer>) {}

    // Invoked when a slave has been determined unreachable (e.g., machine failure, network partition).
    fn slave_lost(&self, driver: &SchedulerDriver, slaveId: &SlaveID) {}

    // Invoked when the status of a task has changed (e.g., a slave is lost and so the task is lost, a task finishes and an executor sends a status update saying so, etc).
    fn status_update(&self, driver: &SchedulerDriver, status: &TaskStatus) {}
}

// HTTP CLIENT/SERVER WORK

header! {
    (LibprocessFrom, "Libprocess-From") => [String]
}

pub fn request<M: Message>(sender: &UPID, master_uri: &str, message: &M) -> Result<Response> {
    let mut uri = master_uri.to_string();
    uri.push_str("/mesos.internal.");
    uri.push_str(message.descriptor().name());
    let mut client = Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType("application/x-protobuf".parse().unwrap()));
    headers.set(LibprocessFrom(sender.to_string()));
    let data = utils::serialize(message).unwrap();
    let res = client.post(uri.trim())
          .headers(headers)
          .body(&data[..])
          .send();
    res
}

fn register_framework(me: &UPID, master_uri: &str, framework: FrameworkInfo) -> Result<Response> {
    let mut register_framework = RegisterFrameworkMessage::new();
    register_framework.set_framework(framework);
    request(me, master_uri, &register_framework)
}

fn server(me: &UPID) -> server::Listening {
    let id_end = me.id.len() + 1;
    Server::http(move |mut req: server::Request, mut resp: server::Response| {
        match req.uri.clone() { // TODO I hate borrowing rules
            // slice the id from the path
            AbsolutePath(ref path) => match &path[id_end..] {
                "/mesos.internal.FrameworkRegisteredMessage" => {
                   let message: FrameworkRegisteredMessage = parse_from_reader(&mut req).unwrap();
                    println!("FrameworkRegisteredMessage {:?}", message);
                    *resp.status_mut() = StatusCode::Accepted;
                },
                message => {
                    println!("Unhandled {:?}", message);
                }
            },
            _ => {}
        }
    }).listen(me.address).unwrap()
}

pub struct UPID {
    id: String,
    address: SocketAddr
}

impl ToString for UPID {
    fn to_string(&self) -> String {
        format!("{}@{}", self.id, self.address)
    }
}

impl UPID {
    fn new(id: &str, address: SocketAddr) -> UPID {
        UPID{id: id.to_string(), address: address}
    }
}

fn main() {
    env_logger::init().unwrap();

    let scheduler = MyScheduler;

    let mut framework = FrameworkInfo::new();

    framework.set_name("rust-example".to_string());
    framework.set_user("bonifaido".to_string());
    framework.set_failover_timeout(60.0);

    let master = "http://localhost:5050/master";

    let driver = MesosSchedulerDriver::new(scheduler, &framework, master);
    driver.start();

    let me = UPID::new("rustclient", "127.0.0.1:4567".parse().unwrap());
    let server = server(&me);
    let resp = register_framework(&me, master, framework);

    println!("{:?}", resp);
}