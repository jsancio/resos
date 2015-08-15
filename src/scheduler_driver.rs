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
                      FrameworkToExecutorMessage,
                      FrameworkRegisteredMessage,
                      FrameworkReregisteredMessage,
                      KillTaskMessage,
                      LaunchTasksMessage,
                      RegisterFrameworkMessage,
                      UnregisterFrameworkMessage,
                      ReconcileTasksMessage,
                      RescindResourceOfferMessage,
                      ResourceRequestMessage,
                      ResourceOffersMessage,
                      ReviveOffersMessage,
                      StatusUpdate,
                      StatusUpdateMessage};
use protobuf::RepeatedField;
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
    fn reconcile_tasks(&self, statuses: &Vec<TaskStatus>) -> Status;

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
    fn request_resources(&self, requests_data: &Vec<Request>) -> Status;

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
        map
    }

    fn registered(_sender: &UPID, msg: FrameworkRegisteredMessage, driver: &Self) {
        {
            let mut internal = driver.internal.lock().unwrap();
            internal.framework.set_id(msg.get_framework_id().clone());
        }
        info!("Framework registered with id {}", msg.get_framework_id().get_value());
        driver.scheduler.registered(driver, msg.get_framework_id(), msg.get_master_info());
    }

    fn reregistered(_sender: &UPID, msg: FrameworkReregisteredMessage, driver: &Self) {
        driver.scheduler.reregistered(driver, msg.get_master_info());
    }

    fn offer_rescinded(_sender: &UPID, msg: RescindResourceOfferMessage, driver: &Self) {
        driver.scheduler.offer_rescinded(driver, msg.get_offer_id());
    }

    fn resource_offers(_sender: &UPID, msg: ResourceOffersMessage, driver: &Self) {
        driver.scheduler.resource_offers(driver, &msg.get_offers().to_vec());
    }

    fn status_update(_sender: &UPID, msg: StatusUpdateMessage, driver: &Self) {
        driver.scheduler.status_update(driver, msg.get_update().get_status());
        if msg.has_pid() {
            // TODO send ACK if needed
        }
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

            Self::status_update(&internal.libprocess.pid, msg, self);
        }
    }
}

impl <S: Scheduler + Send + Sync + 'static> SchedulerDriver for MesosSchedulerDriver<S> {

    fn start(&self) -> Status {
        let mut internal = self.internal.lock().unwrap();
        if internal.status == Status::DRIVER_NOT_STARTED {

            internal.master_detector.start();
            let master = internal.master_detector.master().expect("No master found by MasterDetector");
            debug!("Registering with master {}", master);

            let mut msg = RegisterFrameworkMessage::new();
            msg.set_framework(internal.framework.clone());

            match internal.libprocess.send(&master, &msg) {
                Ok(_) => internal.status = Status::DRIVER_RUNNING,
                Err(e) => error!("Failed to start driver {:?}", e)
            }
        }
        internal.status
    }

    fn stop(&self, failover: bool) -> Status {
        let mut internal = self.internal.lock().unwrap();
        if internal.status == Status::DRIVER_RUNNING {
            if failover {
                let mut msg = UnregisterFrameworkMessage::new();
                msg.set_framework_id(internal.framework.get_id().clone());
                let master = internal.master_detector.master().unwrap();
                match internal.libprocess.send(&master, &msg) {
                    Ok(_) => internal.status = Status::DRIVER_STOPPED,
                    Err(e) => error!("Failed to stop driver {:?}", e)
                }
            }
            internal.libprocess.close().unwrap();
        }
        internal.status
    }

    fn abort(&self) -> Status {
        let mut internal = self.internal.lock().unwrap();
        if internal.status == Status::DRIVER_RUNNING {
            let mut msg = DeactivateFrameworkMessage::new();
            msg.set_framework_id(internal.framework.get_id().clone());
            let master = internal.master_detector.master().unwrap();
            if let Err(e) = internal.libprocess.send(&master, &msg) {
                error!("Failed to send deactivate framework message {:?}", e)
            }
            internal.status = Status::DRIVER_ABORTED;
        }
        internal.status
    }

    fn join(&self) -> Status {
        thread::sleep_ms(999999999);
        Status::DRIVER_RUNNING
    }

    fn run(&self) -> Status {
        match self.start() {
            Status::DRIVER_RUNNING => self.join(),
            status => status
        }
    }

    fn request_resources(&self, requests: &Vec<Request>) -> Status {
        let mut internal = self.internal.lock().unwrap();
        if internal.status == Status::DRIVER_RUNNING {
            let mut msg = ResourceRequestMessage::new();
            msg.set_framework_id(internal.framework.get_id().clone());
            msg.set_requests(RepeatedField::from_vec(requests.clone()));
            let master = internal.master_detector.master().unwrap();
            if let Err(e) = internal.libprocess.send(&master, &msg) {
                error!("Failed to send resorce request message {:?}", e)
            }
        }
        internal.status
    }

    fn decline_offer(&self,
                     offer_id: &OfferID,
                     filters: &Filters) -> Status {
        let mut internal = self.internal.lock().unwrap();
        if internal.status == Status::DRIVER_RUNNING {

            // Launching an empty task list will decline the offer
            let mut msg = LaunchTasksMessage::new();
            msg.set_framework_id(internal.framework.get_id().clone());
            msg.mut_offer_ids().push(offer_id.clone());
            msg.set_filters(filters.clone());
            let master = internal.master_detector.master().unwrap();
            if let Err(e) = internal.libprocess.send(&master, &msg) {
                error!("Failed to send decline offer message {:?}", e)
            }
        }
        internal.status
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
                if !task.get_executor().has_framework_id() {
                    task.mut_executor().set_framework_id(internal.framework.get_id().clone());
                }
            }

            match internal.master_detector.master() {
                Some(master) => {
                    if let Err(e) = internal.libprocess.send(&master, &msg) {
                        error!("Failed to send launch tasks message {:?}", e);
                        self.loose_tasks(&internal, tasks, &format!("Unable to launch tasks: {:?}", e));
                    }
                }
                None => {
                    self.loose_tasks(&internal, tasks, "Master is disconnected");
                }
            }
        }
        Ok(internal.status)
    }

    fn reconcile_tasks(&self, statuses: &Vec<TaskStatus>) -> Status {
        let mut internal = self.internal.lock().unwrap();
        if internal.status == Status::DRIVER_RUNNING {
            let mut msg = ReconcileTasksMessage::new();
            msg.set_framework_id(internal.framework.get_id().clone());
            msg.set_statuses(RepeatedField::from_vec(statuses.clone()));
            let master = internal.master_detector.master().unwrap();
            if let Err(e) = internal.libprocess.send(&master, &msg) {
                error!("Failed to send kill task message {:?}", e)
            }
        }
        internal.status
    }

    fn kill_task(&self, task_id: &TaskID) -> Status {
        let mut internal = self.internal.lock().unwrap();
        if internal.status == Status::DRIVER_RUNNING {
            let mut msg = KillTaskMessage::new();
            msg.set_framework_id(internal.framework.get_id().clone());
            msg.set_task_id(task_id.clone());
            let master = internal.master_detector.master().unwrap();
            if let Err(e) = internal.libprocess.send(&master, &msg) {
                error!("Failed to send kill task message {:?}", e)
            }
        }
        internal.status
    }

    fn revive_offers(&self) -> Status {
        let mut internal = self.internal.lock().unwrap();
        if internal.status == Status::DRIVER_RUNNING {
            let mut msg = ReviveOffersMessage::new();
            msg.set_framework_id(internal.framework.get_id().clone());
            let master = internal.master_detector.master().unwrap();
            if let Err(e) = internal.libprocess.send(&master, &msg) {
                error!("Failed to send revive offers message {:?}", e)
            }
        }
        internal.status
    }

    fn send_framework_message(&self,
                              executor_id: &ExecutorID,
                              slave_id: &SlaveID,
                              data: &Vec<u8>) -> Status {
        let mut internal = self.internal.lock().unwrap();
        if internal.status == Status::DRIVER_RUNNING {
            let mut msg = FrameworkToExecutorMessage::new();
            msg.set_slave_id(slave_id.clone());
            msg.set_framework_id(internal.framework.get_id().clone());
            msg.set_executor_id(executor_id.clone());
            msg.set_data(data.clone());
            let master = internal.master_detector.master().unwrap();
            if let Err(e) = internal.libprocess.send(&master, &msg) {
                error!("Failed to send framework message {:?}", e)
            }
        }
        internal.status
    }
}
