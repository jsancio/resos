use hyper;
use http_api;
use http_api::{HttpApi, HttpHandler};
use master_detector::MasterDetector;
use proto::mesos::{AgentID,
                   ExecutorID,
                   Filters,
                   FrameworkInfo,
                   OfferID,
                   Offer_Operation,
                   Request,
                   Status,
                   TaskID,
                   TaskInfo,
                   TaskState,
                   TaskStatus};
use proto::scheduler::{Call,
                       Call_Type,
                       Call_Subscribe,
                       Call_Accept,
                       Call_Acknowledge,
                       Event,
                       Event_Type,
                       Event_Subscribed,
                       Event_Offers,
                       Event_Rescind,
                       Event_Update,
                       Event_Message,
                       Event_Failure,
                       Event_Error};
use protobuf::{Message, RepeatedField};
use scheduler::Scheduler;
use std::sync::{Arc, Mutex};
use std::thread;
use time::now_utc;
use uuid::Uuid;
use zookeeper::ZkError;

/// Abstract interface for connecting a scheduler to Mesos. This
/// interface is used both to manage the scheduler's lifecycle (start
/// it, stop it, or wait for it to finish) and to interact with Mesos
/// (e.g., launch tasks, kill tasks, etc.).
pub trait SchedulerDriver {

    /// Launches the given set of tasks.
    ///
    /// @param offerIds      The offer IDs.
    /// @param operations    The collection of tasks to be launched.
    /// @param filters       The filters to set for any remaining resources.
    ///
    /// @return            The state of the driver after the call.
    fn accept(&self,
              offer_ids: &Vec<OfferID>,
              operations: &Vec<Offer_Operation>,
              filters: &Filters) -> Result<Status>;

    /// Allows the framework to query the status for non-terminal tasks.
    ///
    /// @param tasks   The collection of tasks to be launched.
    ///
    /// @return            The state of the driver after the call.
    fn reconcile_tasks(&self, statuses: &Vec<TaskStatus>) -> Result<Status>;

    /// Starts the scheduler driver. This needs to be called before any
    /// other driver calls are made.
    ///
    /// @return            The state of the driver after the call.
    fn start(&self) -> Result<Status>;

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
    fn stop(&self, failover: bool) -> Result<Status>;

    /// Aborts the driver so that no more callbacks can be made to the
    /// scheduler. The semantics of abort and stop have deliberately been
    /// separated so that code can detect an aborted driver (i.e., via
    /// the return status of {@link #join}, see below), and instantiate
    /// and start another driver if desired (from within the same
    /// process).
    ///
    /// @return The state of the driver after the call.
    fn abort(&self) -> Result<Status>;

    /// Waits for the driver to be stopped or aborted, possibly
    /// _blocking_ the current thread indefinitely. The return status of
    /// this function can be used to determine if the driver was aborted
    /// (see mesos.proto for a description of Status).
    ///
    /// @return The state of the driver after the call.
    fn join(&self) -> Result<Status>;

    /// Starts and immediately joins (i.e., blocks on) the driver.
    ///
    /// @return The state of the driver after the call.
    fn run(&self) -> Result<Status>;

    /// Requests resources from Mesos (see mesos.proto for a description
    /// of Request and how, for example, to request resources
    /// from specific slaves). Any resources available are offered to the
    /// framework via {@link Scheduler#resourceOffers} callback,
    /// asynchronously.
    ///
    /// @param requests    The resource requests.
    ///
    /// @return            The state of the driver after the call.
    fn request_resources(&self, requests_data: &Vec<Request>) -> Result<Status>;

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
                     filters: &Filters) -> Result<Status>;

    /// Kills the specified task. Note that attempting to kill a task is
    /// currently not reliable. If, for example, a scheduler fails over
    /// while it was attempting to kill a task it will need to retry in
    /// the future Likewise, if unregistered / disconnected, the request
    /// will be dropped (these semantics may be changed in the future).
    ///
    /// @param taskId  The ID of the task to be killed.
    ///
    /// @return        The state of the driver after the call.
    fn kill_task(&self, task_id: &TaskID) -> Result<Status>;

    /// Removes all filters, previously set by the framework (via {@link
    /// #launchTasks}). This enables the framework to receive offers
    /// from those filtered slaves.
    ///
    /// @return    The state of the driver after the call.
    fn revive_offers(&self) -> Result<Status>;

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
                              slave_id: &AgentID,
                              data: &[u8]) -> Result<Status>;

    // fn destroy(driver: *mut libc::c_void, scheduler: *mut libc::c_void);
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

struct DriverInternal {
    http_api: HttpApi,
    master_detector: MasterDetector,
    framework: FrameworkInfo,
    status: Status
}

impl DriverInternal {
    fn send_master(&mut self, msg: &Message) -> Result<()> {
        let master = try!(self.master_detector.master().map_err(Error::Zk));
        try!(self.http_api.send(&master, msg).map_err(Error::HttpApi));
        Ok(())
    }
}

//#[derive(Clone)]
pub struct MesosSchedulerDriver<S> {
    scheduler: Arc<S>,
    internal: Arc<Mutex<DriverInternal>>
}

// TODO why do I need to do this??
impl <S> Clone for MesosSchedulerDriver<S> {
    fn clone(&self) -> Self {
        MesosSchedulerDriver{
            scheduler: self.scheduler.clone(),
            internal: self.internal.clone()
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Zk(ZkError),
    HttpApi(http_api::Error)
}

pub type Result<T> = ::std::result::Result<T, Error>;

impl <S: Scheduler + Sync + Send + 'static> MesosSchedulerDriver<S> {

    pub fn new(scheduler: S,
               framework: FrameworkInfo,
               master: &str) -> Result<Self> {

        let http_api = try!(HttpApi::new("/api/v1/scheduler").map_err(Error::HttpApi));

        assert!(master.starts_with("zk://"), "Only zk masters are allowed");

        let master_detector = try!(MasterDetector::new(&master[5..]).map_err(Error::Zk));

        let internal = DriverInternal{
            http_api: http_api,
            master_detector: master_detector,
            framework: framework,
            status: Status::DRIVER_NOT_STARTED
        };

        let driver = MesosSchedulerDriver{
            scheduler: Arc::new(scheduler),
            internal: Arc::new(Mutex::new(internal))
        };

        Ok(driver)
    }

    fn registered(&self, msg: &Event_Subscribed) {
        {
            let mut internal = self.internal.lock().unwrap();
            internal.framework.set_id(msg.get_framework_id().clone());
        }
        info!("Framework registered with id {}", msg.get_framework_id().get_value());
        self.scheduler.registered(self, msg.get_framework_id());
    }

    fn resource_offers(&self, msg: &Event_Offers) {
        self.scheduler.resource_offers(self, &msg.get_offers().to_vec());
    }

    fn offer_rescinded(&self, msg: &Event_Rescind) {
        self.scheduler.offer_rescinded(self, msg.get_offer_id());
    }

    fn status_update(&self, msg: &Event_Update) {
        let status = msg.get_status();
        self.scheduler.status_update(self, status);
        if status.has_uuid() {
            let mut internal = self.internal.lock().unwrap();
            let mut call = Call::new();
            call.set_field_type(Call_Type::ACKNOWLEDGE);
            call.set_framework_id(internal.framework.get_id().clone());
            let mut ack = Call_Acknowledge::new();
            ack.set_agent_id(status.get_agent_id().clone());
            ack.set_task_id(status.get_task_id().clone());
            ack.set_uuid(status.get_uuid().to_vec());
            call.set_acknowledge(ack);
            let _ = internal.send_master(&call);
        }
    }

    fn framework_message(&self, msg: &Event_Message) {
        self.scheduler.framework_message(self, msg.get_agent_id(), msg.get_executor_id(), msg.get_data());
    }

    fn failure(&self, msg: &Event_Failure) {
        if msg.has_executor_id() {
            self.scheduler.executor_lost(self, msg.get_agent_id(), msg.get_executor_id(), msg.get_status());
        } else {
            self.scheduler.agent_lost(self, msg.get_agent_id());
        }
    }

    fn error(&self, msg: &Event_Error) {
        self.scheduler.error(self, msg.get_message());
    }

    fn heartbeat(&self) {
        info!("Heartbeat")
    }    

    // fn reregistered(driver: &Self, _from: &UPID, msg: FrameworkReregisteredMessage) {
    //     driver.scheduler.reregistered(driver, msg.get_master_info());
    // }

    // fn loose_tasks(&self, internal: &DriverInternal, tasks: &Vec<TaskInfo>, message: &str) {
    //     for task in tasks {
    //         let mut msg = StatusUpdateMessage::new();
    //         let mut update = StatusUpdate::new();
    //         update.set_framework_id(internal.framework.get_id().clone());
    //         update.set_slave_id(task.get_slave_id().clone());
    //         update.set_executor_id(task.get_executor().get_executor_id().clone());
    //         let mut status = TaskStatus::new();
    //         status.set_task_id(task.get_task_id().clone());
    //         status.set_state(TaskState::TASK_LOST);
    //         status.set_message(message.to_string());
    //         update.set_status(status);
    //         update.set_timestamp(now_utc().to_timespec().sec as f64);
    //         update.set_uuid(Uuid::new_v4().as_bytes().to_vec());
    //         msg.set_update(update);

    //         Self::status_update(self, &internal.libprocess.pid, msg);
    //     }
    // }
}

impl <S: Scheduler + Sync + Send + 'static> HttpHandler<Event> for MesosSchedulerDriver<S> {
    fn on_event(&self, event: Event) {
        info!("{:?}", event.get_field_type());

        match event.get_field_type() {
            Event_Type::SUBSCRIBED => self.registered(event.get_subscribed()),
            Event_Type::OFFERS => self.resource_offers(event.get_offers()),
            Event_Type::RESCIND => self.offer_rescinded(event.get_rescind()),
            Event_Type::UPDATE => self.status_update(event.get_update()),
            Event_Type::MESSAGE => self.framework_message(event.get_message()),
            Event_Type::FAILURE => self.failure(event.get_failure()),
            Event_Type::ERROR => self.error(event.get_error()),
            Event_Type::HEARTBEAT => self.heartbeat()
        }
    }

    fn on_error(&self, error: http_api::Error) {
        error!("{:?}", error);
    }
}

impl <S: Scheduler + Sync + Send + 'static> SchedulerDriver for MesosSchedulerDriver<S> {

    fn start(&self) -> Result<Status> {
        let mut internal = self.internal.lock().unwrap();
        if internal.status == Status::DRIVER_NOT_STARTED {

            internal.master_detector.start();
            let master = try!(internal.master_detector.master().map_err(Error::Zk));
            debug!("Registering with master {}", master);

            let mut call = Call::new();
            call.set_field_type(Call_Type::SUBSCRIBE);
            let mut subscribe = Call_Subscribe::new();
            subscribe.set_framework_info(internal.framework.clone());
            subscribe.set_force(true);
            call.set_subscribe(subscribe);
            let master = internal.master_detector.master().unwrap();
            try!(internal.http_api.subscribe(&master, call, self.clone()).map_err(Error::HttpApi));
            internal.status = Status::DRIVER_RUNNING;
        }
        Ok(internal.status)
    }

    fn stop(&self, failover: bool) -> Result<Status> {
        let mut internal = self.internal.lock().unwrap();
        if internal.status == Status::DRIVER_RUNNING {
            let mut call = Call::new();
            call.set_field_type(Call_Type::TEARDOWN);
            call.set_framework_id(internal.framework.get_id().clone());
            try!(internal.send_master(&call));
            try!(internal.http_api.join().map_err(Error::HttpApi));
            internal.status = Status::DRIVER_STOPPED;
        }
        Ok(internal.status)
    }

    fn abort(&self) -> Result<Status> {
        panic!("Unimplemented")
    }

    fn join(&self) -> Result<Status> {
        thread::sleep_ms(999999999); // TODO
        Ok(Status::DRIVER_RUNNING)
    }

    fn run(&self) -> Result<Status> {
        match self.start() {
            Ok(Status::DRIVER_RUNNING) => self.join(),
            status => status
        }
    }

    fn accept(&self,
              offer_ids: &Vec<OfferID>,
              operations: &Vec<Offer_Operation>,
              filters: &Filters) -> Result<Status> {
        let mut internal = self.internal.lock().unwrap();
        if internal.status == Status::DRIVER_RUNNING {

            let mut call = Call::new();
            call.set_field_type(Call_Type::ACCEPT);
            call.set_framework_id(internal.framework.get_id().clone());
            let mut accept = Call_Accept::new();
            accept.set_offer_ids(RepeatedField::from_vec(offer_ids.clone()));
            accept.set_operations(RepeatedField::from_vec(operations.clone()));
            accept.set_filters(filters.clone());
            call.set_accept(accept);

            try!(internal.send_master(&call));

            // if let Err(e) =  {
            //     self.loose_tasks(&internal, tasks, &format!("Unable to launch tasks: {:?}", e));
            //     return Err(e)
            // }
        }
        Ok(internal.status)
    }

    fn request_resources(&self, requests: &Vec<Request>) -> Result<Status> {
        let mut internal = self.internal.lock().unwrap();
        // if internal.status == Status::DRIVER_RUNNING {
        //     let mut msg = ResourceRequestMessage::new();
        //     msg.set_framework_id(internal.framework.get_id().clone());
        //     msg.set_requests(RepeatedField::from_vec(requests.clone()));
        //     try!(internal.send_master(&msg));
        // }
        Ok(internal.status)
    }

    fn decline_offer(&self,
                     offer_id: &OfferID,
                     filters: &Filters) -> Result<Status> {
        let mut internal = self.internal.lock().unwrap();
        // if internal.status == Status::DRIVER_RUNNING {
        //     // Launching an empty task list will decline the offer
        //     let mut msg = LaunchTasksMessage::new();
        //     msg.set_framework_id(internal.framework.get_id().clone());
        //     msg.mut_offer_ids().push(offer_id.clone());
        //     msg.set_filters(filters.clone());
        //     try!(internal.send_master(&msg));
        // }
        Ok(internal.status)
    }

    fn reconcile_tasks(&self, statuses: &Vec<TaskStatus>) -> Result<Status> {
        let mut internal = self.internal.lock().unwrap();
        // if internal.status == Status::DRIVER_RUNNING {
        //     let mut msg = ReconcileTasksMessage::new();
        //     msg.set_framework_id(internal.framework.get_id().clone());
        //     msg.set_statuses(RepeatedField::from_vec(statuses.clone()));
        //     try!(internal.send_master(&msg));
        // }
        Ok(internal.status)
    }

    fn kill_task(&self, task_id: &TaskID) -> Result<Status> {
        let mut internal = self.internal.lock().unwrap();
        // if internal.status == Status::DRIVER_RUNNING {
        //     let mut msg = KillTaskMessage::new();
        //     msg.set_framework_id(internal.framework.get_id().clone());
        //     msg.set_task_id(task_id.clone());
        //     try!(internal.send_master(&msg));
        // }
        Ok(internal.status)
    }

    fn revive_offers(&self) -> Result<Status> {
        let mut internal = self.internal.lock().unwrap();
        // if internal.status == Status::DRIVER_RUNNING {
        //     let mut msg = ReviveOffersMessage::new();
        //     msg.set_framework_id(internal.framework.get_id().clone());
        //     try!(internal.send_master(&msg));
        // }
        Ok(internal.status)
    }

    fn send_framework_message(&self,
                              executor_id: &ExecutorID,
                              slave_id: &AgentID,
                              data: &[u8]) -> Result<Status> {
        let mut internal = self.internal.lock().unwrap();
        // if internal.status == Status::DRIVER_RUNNING {
        //     let mut msg = FrameworkToExecutorMessage::new();
        //     msg.set_slave_id(slave_id.clone());
        //     msg.set_framework_id(internal.framework.get_id().clone());
        //     msg.set_executor_id(executor_id.clone());
        //     msg.set_data(data.to_vec());
        //     try!(internal.send_master(&msg)); // TODO check what the Go driver does here
        // }
        Ok(internal.status)
    }
}
