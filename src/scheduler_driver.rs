use libprocess::{LibProcess, UPID, HandlerMap};
use master_detector::MasterDetector;
use proto::{ExecutorID,
            Filters,
            FrameworkInfo,
            OfferID,
            Request,
            SlaveID,
            Status,
            TaskID,
            TaskInfo,
            TaskState,
            TaskStatus};
use proto::internal::{DeactivateFrameworkMessage,
                      ExecutorToFrameworkMessage,
                      ExitedExecutorMessage,
                      FrameworkErrorMessage,
                      FrameworkToExecutorMessage,
                      FrameworkRegisteredMessage,
                      FrameworkReregisteredMessage,
                      KillTaskMessage,
                      LaunchTasksMessage,
                      LostSlaveMessage,
                      RegisterFrameworkMessage,
                      UnregisterFrameworkMessage,
                      ReconcileTasksMessage,
                      RescindResourceOfferMessage,
                      ResourceRequestMessage,
                      ResourceOffersMessage,
                      ReviveOffersMessage,
                      StatusUpdate,
                      StatusUpdateMessage,
                      StatusUpdateAcknowledgementMessage};
use protobuf::{Message, RepeatedField};
use scheduler::Scheduler;
use std::sync::{Arc, Mutex};
use std::thread;
use time::now_utc;
use uuid::Uuid;

/// Abstract interface for connecting a scheduler to Mesos. This
/// interface is used both to manage the scheduler's lifecycle (start
/// it, stop it, or wait for it to finish) and to interact with Mesos
/// (e.g., launch tasks, kill tasks, etc.).
pub trait SchedulerDriver {

    /// Launches the given set of tasks.
    ///
    /// @param offerId The offer ID.
    /// @param tasks   The collection of tasks to be launched.
    /// @param filters The filters to set for any remaining resources.
    ///
    /// @return            The state of the driver after the call.
    fn launch_tasks(&self,
        offer_id: &Vec<OfferID>,
        tasks: &Vec<TaskInfo>,
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
                              slave_id: &SlaveID,
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
    libprocess: LibProcess,
    master_detector: MasterDetector,
    framework: FrameworkInfo,
    status: Status
}

impl DriverInternal {
    fn send_master(&mut self, msg: &Message) -> Result<()> {
        match self.master_detector.master() {
            Some(master) => {
                if let Err(e) = self.libprocess.send(&master, msg) {
                    error!("Failed to send {} to {:?} error: {:?}", msg.descriptor().name(), master, e);
                    Err(Error::LibProcess)
                } else {
                    Ok(())
                }
            },
            None => {
                error!("Failed to send {}, master is not available", msg.descriptor().name());
                Err(Error::Zk)
            }
        }
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
    Zk,
    LibProcess
}

pub type Result<T> = ::std::result::Result<T, Error>;

impl <S: Scheduler + Send + Sync + 'static> MesosSchedulerDriver<S> {

    pub fn new(scheduler: S,
               framework: FrameworkInfo,
               master: &str) -> Result<Self> {

        let libprocess = try!(LibProcess::new(framework.get_name()).map_err(|_| Error::LibProcess));

        assert!(master.starts_with("zk://"), "Only zk masters are allowed");

        let master_detector = try!(MasterDetector::new(&master[5..]).map_err(|_| Error::Zk));

        let internal = DriverInternal{
            libprocess: libprocess,
            master_detector: master_detector,
            framework: framework,
            status: Status::DRIVER_NOT_STARTED
        };

        let driver = MesosSchedulerDriver{
            scheduler: Arc::new(scheduler),
            internal: Arc::new(Mutex::new(internal))
        };

        let handlers = Self::handler_map(driver.clone());

        driver.internal.lock().unwrap().libprocess.start(handlers);

        Ok(driver)
    }

    fn handler_map(context: Self) -> HandlerMap<Self> {
        let mut map = HandlerMap::new(context);
        map.on(Self::registered);
        map.on(Self::reregistered);
        map.on(Self::offer_rescinded);
        map.on(Self::resource_offers);
        map.on(Self::error);
        map.on(Self::executor_lost);
        map.on(Self::slave_lost);
        map.on(Self::status_update);
        map.on(Self::framework_message);
        map
    }

    fn registered(driver: &Self, _from: &UPID, msg: FrameworkRegisteredMessage) {
        {
            let mut internal = driver.internal.lock().unwrap();
            internal.framework.set_id(msg.get_framework_id().clone());
        }
        info!("Framework registered with id {}", msg.get_framework_id().get_value());
        driver.scheduler.registered(driver, msg.get_framework_id(), msg.get_master_info());
    }

    fn reregistered(driver: &Self, _from: &UPID, msg: FrameworkReregisteredMessage) {
        driver.scheduler.reregistered(driver, msg.get_master_info());
    }

    fn offer_rescinded(driver: &Self, _from: &UPID, msg: RescindResourceOfferMessage) {
        driver.scheduler.offer_rescinded(driver, msg.get_offer_id());
    }

    fn resource_offers(driver: &Self, _from: &UPID, msg: ResourceOffersMessage) {
        driver.scheduler.resource_offers(driver, &msg.get_offers().to_vec());
    }

    fn status_update(driver: &Self, from: &UPID, msg: StatusUpdateMessage) {
        driver.scheduler.status_update(driver, msg.get_update().get_status());
        let mut internal = driver.internal.lock().unwrap();
        if *from != internal.libprocess.pid {
            let mut ack = StatusUpdateAcknowledgementMessage::new();
            ack.set_framework_id(internal.framework.get_id().clone());
            ack.set_slave_id(msg.get_update().get_slave_id().clone());
            ack.set_task_id(msg.get_update().get_status().get_task_id().clone());
            ack.set_uuid(msg.get_update().get_uuid().to_vec());
            let _ = internal.send_master(&ack);
        }
    }

    fn framework_message(driver: &Self, _from: &UPID, msg: ExecutorToFrameworkMessage) {
        driver.scheduler.framework_message(driver, &msg.get_executor_id(), &msg.get_slave_id(), &msg.get_data());
    }

    fn error(driver: &Self, _from: &UPID, msg: FrameworkErrorMessage) {
        driver.scheduler.error(driver, msg.get_message());
    }

    fn executor_lost(driver: &Self, _from: &UPID, msg: ExitedExecutorMessage) {
        driver.scheduler.executor_lost(driver, &msg.get_executor_id(), &msg.get_slave_id(), msg.get_status());
    }

    fn slave_lost(driver: &Self, _from: &UPID, msg: LostSlaveMessage) {
        driver.scheduler.slave_lost(driver, &msg.get_slave_id());
    }

    fn loose_tasks(&self, internal: &DriverInternal, tasks: &Vec<TaskInfo>, message: &str) {
        for task in tasks {
            let mut msg = StatusUpdateMessage::new();
            let mut update = StatusUpdate::new();
            update.set_framework_id(internal.framework.get_id().clone());
            update.set_slave_id(task.get_slave_id().clone());
            update.set_executor_id(task.get_executor().get_executor_id().clone());
            let mut status = TaskStatus::new();
            status.set_task_id(task.get_task_id().clone());
            status.set_state(TaskState::TASK_LOST);
            status.set_message(message.to_string());
            update.set_status(status);
            update.set_timestamp(now_utc().to_timespec().sec as f64);
            update.set_uuid(Uuid::new_v4().as_bytes().to_vec());
            msg.set_update(update);

            Self::status_update(self, &internal.libprocess.pid, msg);
        }
    }
}

impl <S: Scheduler + Send + Sync + 'static> SchedulerDriver for MesosSchedulerDriver<S> {

    fn start(&self) -> Result<Status> {
        let mut internal = self.internal.lock().unwrap();
        if internal.status == Status::DRIVER_NOT_STARTED {

            internal.master_detector.start();
            let master = internal.master_detector.master().expect("No master found by MasterDetector");
            debug!("Registering with master {}", master);

            let mut msg = RegisterFrameworkMessage::new();
            msg.set_framework(internal.framework.clone());

            try!(internal.send_master(&msg));
            internal.status = Status::DRIVER_RUNNING;
        }
        Ok(internal.status)
    }

    fn stop(&self, failover: bool) -> Result<Status> {
        let mut internal = self.internal.lock().unwrap();
        if internal.status == Status::DRIVER_RUNNING {
            if failover {
                let mut msg = UnregisterFrameworkMessage::new();
                msg.set_framework_id(internal.framework.get_id().clone());
                try!(internal.send_master(&msg));
            }
            try!(internal.libprocess.close().map_err(|_| Error::LibProcess));
            internal.status = Status::DRIVER_STOPPED;
        }
        Ok(internal.status)
    }

    fn abort(&self) -> Result<Status> {
        let mut internal = self.internal.lock().unwrap();
        if internal.status == Status::DRIVER_RUNNING {
            let mut msg = DeactivateFrameworkMessage::new();
            msg.set_framework_id(internal.framework.get_id().clone());
            try!(internal.send_master(&msg));
            internal.status = Status::DRIVER_ABORTED;
        }
        Ok(internal.status)
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

    fn request_resources(&self, requests: &Vec<Request>) -> Result<Status> {
        let mut internal = self.internal.lock().unwrap();
        if internal.status == Status::DRIVER_RUNNING {
            let mut msg = ResourceRequestMessage::new();
            msg.set_framework_id(internal.framework.get_id().clone());
            msg.set_requests(RepeatedField::from_vec(requests.clone()));
            try!(internal.send_master(&msg));
        }
        Ok(internal.status)
    }

    fn decline_offer(&self,
                     offer_id: &OfferID,
                     filters: &Filters) -> Result<Status> {
        let mut internal = self.internal.lock().unwrap();
        if internal.status == Status::DRIVER_RUNNING {

            // Launching an empty task list will decline the offer
            let mut msg = LaunchTasksMessage::new();
            msg.set_framework_id(internal.framework.get_id().clone());
            msg.mut_offer_ids().push(offer_id.clone());
            msg.set_filters(filters.clone());
            try!(internal.send_master(&msg));
        }
        Ok(internal.status)
    }

    fn launch_tasks(&self,
                    offer_ids: &Vec<OfferID>,
                    tasks: &Vec<TaskInfo>,
                    filters: &Filters) -> Result<Status> {
        let mut internal = self.internal.lock().unwrap();
        if internal.status == Status::DRIVER_RUNNING {

            let mut msg = LaunchTasksMessage::new();
            msg.set_framework_id(internal.framework.get_id().clone());
            msg.set_offer_ids(RepeatedField::from_vec(offer_ids.clone()));
            msg.set_tasks(RepeatedField::from_vec(tasks.clone()));
            msg.set_filters(filters.clone());

            // Set TaskInfo.executor.framework_id, if it's missing.
            for task in msg.mut_tasks().iter_mut() {
                if task.has_executor() && !task.get_executor().has_framework_id() {
                    task.mut_executor().set_framework_id(internal.framework.get_id().clone());
                }
            }

            if let Err(e) = internal.send_master(&msg) {
                self.loose_tasks(&internal, tasks, &format!("Unable to launch tasks: {:?}", e));
                return Err(e)
            }
        }

        Ok(internal.status)
    }

    fn reconcile_tasks(&self, statuses: &Vec<TaskStatus>) -> Result<Status> {
        let mut internal = self.internal.lock().unwrap();
        if internal.status == Status::DRIVER_RUNNING {
            let mut msg = ReconcileTasksMessage::new();
            msg.set_framework_id(internal.framework.get_id().clone());
            msg.set_statuses(RepeatedField::from_vec(statuses.clone()));
            try!(internal.send_master(&msg));
        }
        Ok(internal.status)
    }

    fn kill_task(&self, task_id: &TaskID) -> Result<Status> {
        let mut internal = self.internal.lock().unwrap();
        if internal.status == Status::DRIVER_RUNNING {
            let mut msg = KillTaskMessage::new();
            msg.set_framework_id(internal.framework.get_id().clone());
            msg.set_task_id(task_id.clone());
            try!(internal.send_master(&msg));
        }
        Ok(internal.status)
    }

    fn revive_offers(&self) -> Result<Status> {
        let mut internal = self.internal.lock().unwrap();
        if internal.status == Status::DRIVER_RUNNING {
            let mut msg = ReviveOffersMessage::new();
            msg.set_framework_id(internal.framework.get_id().clone());
            try!(internal.send_master(&msg));
        }
        Ok(internal.status)
    }

    fn send_framework_message(&self,
                              executor_id: &ExecutorID,
                              slave_id: &SlaveID,
                              data: &[u8]) -> Result<Status> {
        let mut internal = self.internal.lock().unwrap();
        if internal.status == Status::DRIVER_RUNNING {
            let mut msg = FrameworkToExecutorMessage::new();
            msg.set_slave_id(slave_id.clone());
            msg.set_framework_id(internal.framework.get_id().clone());
            msg.set_executor_id(executor_id.clone());
            msg.set_data(data.to_vec());
            try!(internal.send_master(&msg)); // TODO check what the Go driver does here
        }
        Ok(internal.status)
    }
}
