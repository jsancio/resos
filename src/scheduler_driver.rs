use hyper;
use hyper::client::Response;
use hyper::error::Result;
use hyper::header::{ContentType, Headers};
use hyper::status::StatusCode;
use hyper::uri::RequestUri::AbsolutePath;
use proto::{ExecutorID, Filters, FrameworkID, FrameworkInfo, MasterInfo, OfferID, Request, SlaveID, Status, TaskID, TaskInfo};
use proto_internal::{RegisterFrameworkMessage, FrameworkRegisteredMessage};
use protobuf::{Message, parse_from_bytes};
use scheduler;
use utils;
use std::cell::RefCell;
use std::io::Read;
use std::net::SocketAddr;
use std::sync::Mutex;
use std::sync::mpsc::{channel, Sender};
use std::thread;

/// Abstract interface for connecting a scheduler to Mesos. This
/// interface is used both to manage the scheduler's lifecycle (start
/// it, stop it, or wait for it to finish) and to interact with Mesos
/// (e.g., launch tasks, kill tasks, etc.).
pub trait SchedulerDriver {

    /// @param offerId The offer ID.
    /// @param tasks   The collection of tasks to be launched.
    /// @param filters The filters to set for any remaining resources.
    ///
    /// @return            The state of the driver after the call.
    fn launch_tasks(&self,
        offer_id: &OfferID,
        tasks: &Vec<TaskInfo>,
        filters: &Filters) -> Status;

    /// Starts the scheduler driver. This needs to be called before any
    /// other driver calls are made.
    ///
    /// @return            The state of the driver after the call.
    fn start(&self) -> Status;

    /// Stops the scheduler driver. If the 'failover' flag is set to
    /// false then it is expected that this framework will never
    /// reconnect to Mesos. So Mesos will unregister the framework
    /// and shutdown all its tasks and executors. If 'failover' is true,
    /// all executors and tasks will remain running (for some framework
    /// specific failover timeout) allowing the scheduler to reconnect
    /// (possibly in the same process, or from a different process, for
    /// example, on a different machine).
    ///
    /// @param failover    Whether framework failover is expected.
    ///
    /// @return            The state of the driver after the call.
    fn stop(&self, failover: bool) -> Status;

    /// Aborts the driver so that no more callbacks can be made to the
    /// scheduler. The semantics of abort and stop have deliberately been
    /// separated so that code can detect an aborted driver (i.e., via
    /// the return status of {@link #join}, see below), and instantiate
    /// and start another driver if desired (from within the same
    /// process).
    ///
    /// @return The state of the driver after the call.
    fn abort(&self) -> Status;

    /// Waits for the driver to be stopped or aborted, possibly
    /// _blocking_ the current thread indefinitely. The return status of
    /// this function can be used to determine if the driver was aborted
    /// (see mesos.proto for a description of Status).
    ///
    /// @return The state of the driver after the call.
    fn join(&self) -> Status;

    /// Starts and immediately joins (i.e., blocks on) the driver.
    ///
    /// @return The state of the driver after the call.
    fn run(&self) -> Status;

    /// Requests resources from Mesos (see mesos.proto for a description
    /// of Request and how, for example, to request resources
    /// from specific slaves). Any resources available are offered to the
    /// framework via {@link Scheduler#resourceOffers} callback,
    /// asynchronously.
    ///
    /// @param requests    The resource requests.
    ///
    /// @return            The state of the driver after the call.
    fn request_resources(&self,
        requests_data: &Request) -> Status;

    /// Declines an offer in its entirety and applies the specified
    /// filters on the resources (see mesos.proto for a description of
    /// Filters). Note that this can be done at any time, it is not
    /// necessary to do this within the {@link Scheduler#resourceOffers}
    /// callback.
    ///
    /// @param offerId The ID of the offer to be declined.
    /// @param filters The filters to set for any remaining resources.
    ///
    /// @return        The state of the driver after the call.
    fn decline_offer(&self,
        offer_id: &OfferID,
        filters: &Filters) -> Status;

    /// Kills the specified task. Note that attempting to kill a task is
    /// currently not reliable. If, for example, a scheduler fails over
    /// while it was attempting to kill a task it will need to retry in
    /// the future Likewise, if unregistered / disconnected, the request
    /// will be dropped (these semantics may be changed in the future).
    ///
    /// @param taskId  The ID of the task to be killed.
    ///
    /// @return        The state of the driver after the call.
    fn kill_task(&self, task_id: &TaskID) -> Status;

    /// Removes all filters, previously set by the framework (via {@link
    /// #launchTasks}). This enables the framework to receive offers
    /// from those filtered slaves.
    ///
    /// @return    The state of the driver after the call.
    fn revive_offers(&self) -> Status;

    /// Sends a message from the framework to one of its executors. These
    /// messages are best effort; do not expect a framework message to be
    /// retransmitted in any reliable fashion.
    ///
    /// @param executorId  The ID of the executor to send the message to.
    /// @param slaveId     The ID of the slave that is running the executor.
    /// @param data        The message.
    ///
    /// @return            The state of the driver after the call.
    fn send_framework_message(&self,
        executor_id: &ExecutorID,
        slave_id: &SlaveID,
        data: &Vec<u8>) -> Status;

    // fn destroy(driver: *mut libc::c_void, scheduler: *mut libc::c_void);
}

// HTTP CLIENT/SERVER WORK
header! {
    (LibprocessFrom, "Libprocess-From") => [String]
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
    fn new(id: &str, address: &SocketAddr) -> UPID {
        UPID{id: id.to_string(), address: address.clone()}
    }
}

fn server(tx: Sender<(String, Vec<u8>)>) -> hyper::server::Listening {
    let gtx = Mutex::new(tx); // TODO lock needed because of Sync contstraint on Handler
    hyper::Server::http(move |req: hyper::server::Request,
                              mut resp: hyper::server::Response| {
        let (_, _, _, uri, _, mut body) = req.deconstruct();
        match uri {
            AbsolutePath(path) => {
                let mut data = Vec::new();
                body.read_to_end(&mut data).unwrap();
                gtx.lock().unwrap().send((path, data)).unwrap();
                *resp.status_mut() = StatusCode::Accepted;
            },
            _ => {}
        }
    }).listen("0.0.0.0:0").unwrap()
}

pub fn request<M: Message>(client: &mut hyper::Client,
                           sender: &UPID,
                           master_uri: &str,
                           message: &M) -> Result<hyper::client::Response> {
    let mut uri = master_uri.to_string();
    uri.push_str("/mesos.internal.");
    uri.push_str(message.descriptor().name());
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

fn register_framework(client: &mut hyper::Client,
                      me: &UPID,
                      master_uri: &str,
                      framework: &FrameworkInfo) -> Result<Response> {
    let mut register_framework = RegisterFrameworkMessage::new();
    register_framework.set_framework(framework.clone());
    request(client, me, master_uri, &register_framework)
}

/// Concrete implementation of a SchedulerDriver that connects a
/// Scheduler with a Mesos master. The MesosSchedulerDriver is
/// thread-safe.
///
/// Note that scheduler failover is supported in Mesos. After a
/// scheduler is registered with Mesos it may failover (to a new
/// process on the same machine or across multiple machines) by
/// creating a new driver with the ID given to it in
/// Scheduler::registered.
///
/// The driver is responsible for invoking the Scheduler callbacks as
/// it communicates with the Mesos master.
///
/// Note that blocking on the MesosSchedulerDriver
/// doesn't affect the scheduler callbacks in anyway because
/// they are handled by a different thread.
///
/// Note that the driver uses GLOG to do its own logging. GLOG flags can
/// be set via environment variables, prefixing the flag name with
/// "GLOG_", e.g., "GLOG_v=1". For Mesos specific logging flags see
/// <mesos>/src/logging/flags.hpp. Mesos flags can also be set via environment
/// variables, prefixing the flag name with "MESOS_", e.g.,
/// "MESOS_QUIET=1".
pub struct MesosSchedulerDriver {
    //scheduler: Box<scheduler::Scheduler>,
    http_client: RefCell<hyper::Client>,
    http_server: RefCell<hyper::server::Listening>,
    framework: FrameworkInfo,
    master: String,
    pid: UPID,
//    join: Option<thread::JoinHandle<()>>
}

impl MesosSchedulerDriver {
    pub fn new<S: scheduler::Scheduler + Send + Sync + 'static>(scheduler: S,
                                                  framework: FrameworkInfo,
                                                  master: &str) -> MesosSchedulerDriver {
        let (tx, rx) = channel();
        let http_server = server(tx);
        let pid = UPID::new(framework.get_name(), &http_server.socket);
        let id_end = pid.id.len() + 1;

        let driver = MesosSchedulerDriver{
            //scheduler: Box::new(scheduler),
            http_client: RefCell::new(hyper::Client::new()),
            http_server: RefCell::new(http_server),
            framework: framework,
            master: master.to_string(),
            pid: pid,
//            join: join
        };

        let _join = thread::spawn(move || {
            loop {
                let (path, data) = rx.recv().unwrap();
                // slice the id from the path
                match &path[id_end..] {
                    "/mesos.internal.FrameworkRegisteredMessage" => {
                       let message: FrameworkRegisteredMessage = parse_from_bytes(&data).unwrap();
                        println!("FrameworkRegisteredMessage {:?}", message);
                        //scheduler.registered(&driver, message.get_framework_id(), message.get_master_info());
                    },
                    message => {
                        println!("Unhandled {:?}", message);
                    }
                }
            }
        });

        driver
    }
}

impl SchedulerDriver for MesosSchedulerDriver {

    fn start(&self) -> Status {
        let mut client = self.http_client.borrow_mut();
        let resp = register_framework(&mut *client, &self.pid, &self.master, &self.framework);
        println!("{:?}", resp);
        Status::DRIVER_RUNNING
    }

    fn stop(&self, failover: bool) -> Status {
        self.http_server.borrow_mut().close();
        Status::DRIVER_STOPPED
    }

    fn abort(&self) -> Status {
        Status::DRIVER_ABORTED
    }

    fn join(&self) -> Status {
        Status::DRIVER_RUNNING
    }

    fn run(&self) -> Status {
        Status::DRIVER_RUNNING
    }

    fn request_resources(&self,
        request_data: &Request) -> Status {
        Status::DRIVER_RUNNING
    }

    fn decline_offer(&self,
        offer_id: &OfferID,
        filters: &Filters) -> Status {
        Status::DRIVER_RUNNING
    }

    fn launch_tasks(&self,
        offer_id: &OfferID,
        tasks: &Vec<TaskInfo>,
        filters: &Filters) -> Status {
        Status::DRIVER_RUNNING
    }

    fn kill_task(&self, task_id: &TaskID) -> Status {
        Status::DRIVER_RUNNING
    }

    fn revive_offers(&self) -> Status {
        Status::DRIVER_RUNNING
    }

    fn send_framework_message(&self,
        executor_id: &ExecutorID,
        slave_id: &SlaveID,
        data: &Vec<u8>) -> Status {
        Status::DRIVER_RUNNING        
    }
}
