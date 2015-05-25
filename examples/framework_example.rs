extern crate hyper;
extern crate mesos;
extern crate protobuf;

use hyper::Client;
use hyper::client::Response;
use hyper::error::Result;
use hyper::header::{ContentType, Headers};
use hyper::mime::Mime;
use hyper::server;
use hyper::server::Server;
use protobuf::{Message, parse_from_reader};
use mesos::proto::{ExecutorID, FrameworkID, FrameworkInfo, MasterInfo, Offer, OfferID, SlaveID, TaskStatus};
use mesos::proto_internal::{RegisterFrameworkMessage, FrameworkRegisteredMessage};
use mesos::scheduler::Scheduler;
use mesos::scheduler_driver::{SchedulerDriver, MesosSchedulerDriver};
use mesos::utils;

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

// header! {
//     (LibprocessFrom, "Libprocess-From") => [&str]
// }

pub fn request<M: Message>(master_uri: &str, message: &M) -> Result<Response> {
    let mut uri = master_uri.to_string();
    uri.push_str("/mesos.internal.");
    uri.push_str(message.descriptor().name());
    let mut client = Client::new();
    let mut headers = Headers::new();
    let proto_mime: Mime = "application/x-protobuf".parse().unwrap();
    headers.set(ContentType(proto_mime));
    headers.set_raw("Libprocess-From", vec![b"rustclient@0.0.0.0:4567".to_vec()]);
    let data = utils::serialize(message).unwrap();
    let res = client.post(uri.trim())
          .headers(headers)
          .body(&data[..])
          .send();
    res
}

fn register_framework(master_uri: &str, framework: FrameworkInfo) -> Result<Response> {
    let mut register_framework = RegisterFrameworkMessage::new();
    register_framework.set_framework(framework);
    request(master_uri, &register_framework)
}

fn server(address: &str) -> server::Listening {
    Server::http(|mut req: server::Request, res: server::Response| {
        println!("{:?} - {:?}\n{:?}", req.method, req.uri, req.headers);
        let framework_registered: FrameworkRegisteredMessage = parse_from_reader(&mut req).unwrap();
        println!("FrameworkRegisteredMessage {:?}", framework_registered);
    }).listen(address).unwrap()
}

fn main() {

    let scheduler = MyScheduler;

    let mut framework = FrameworkInfo::new();

    framework.set_name("rust-example".to_string());
    framework.set_user("bonifaido".to_string());

    let master = "http://localhost:5050/master";

    let server = server("0.0.0.0:4567");

    let resp = register_framework(master, framework);
    println!("{:?}", resp);

    //let driver = MesosSchedulerDriver::new(&scheduler, &framework, master);
    //driver.start();
}