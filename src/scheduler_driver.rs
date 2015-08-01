use libprocess::{LibProcess, UPID};
use master_detector::MasterDetector;
use proto::{ExecutorID, Filters, FrameworkInfo, OfferID, Request, SlaveID, Status, TaskID, TaskInfo};
use proto::internal::{FrameworkRegisteredMessage, RegisterFrameworkMessage, UnregisterFrameworkMessage};
use protobuf;
use scheduler::Scheduler;
use std::collections::HashMap;
use std::sync::{Arc, Barrier, Mutex};

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
    fn request_resources(&self, requests_data: &Request) -> Status;

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
//#[derive(Clone)]
pub struct MesosSchedulerDriver<S> {
    scheduler: Arc<S>,
    libprocess: Arc<Mutex<LibProcess>>,
    master_detector: Arc<Mutex<MasterDetector>>,
    framework: Arc<Mutex<FrameworkInfo>>,
    status: Arc<Mutex<Status>>,
    join: Arc<Barrier> // TODO we need a CountDownLatch maybe
}

// TODO why do I need to do this??
impl <S> Clone for MesosSchedulerDriver<S> {
    fn clone(&self) -> MesosSchedulerDriver<S> {
        MesosSchedulerDriver{
            scheduler: self.scheduler.clone(),
            libprocess: self.libprocess.clone(),
            master_detector: self.master_detector.clone(),
            framework: self.framework.clone(),
            status: self.status.clone(),
            join: self.join.clone()
        }
    }
}

impl <S: Scheduler + Send + Sync + 'static> MesosSchedulerDriver<S> {

    pub fn new(scheduler: S,
               framework: FrameworkInfo,
               master: &str) -> MesosSchedulerDriver<S> {

        let libprocess = LibProcess::new(framework.get_name());

        let master_detector = MasterDetector::new(master).unwrap();

        let driver = MesosSchedulerDriver{
            scheduler: Arc::new(scheduler),
            libprocess: Arc::new(Mutex::new(libprocess)),
            master_detector: Arc::new(Mutex::new(master_detector)),
            framework: Arc::new(Mutex::new(framework)),
            status: Arc::new(Mutex::new(Status::DRIVER_NOT_STARTED)),
            join: Arc::new(Barrier::new(2))
        };

        let mut handlers = HashMap::new();
        handlers.insert("mesos.internal.FrameworkRegisteredMessage".to_string(), Self::registered);

        driver.libprocess.lock().unwrap().start(driver.clone(), handlers);

        driver
    }

    fn registered(sender: &UPID, msg: &FrameworkRegisteredMessage, driver: &MesosSchedulerDriver<S>) {
        debug!("FrameworkRegisteredMessage {:?}", msg);
        driver.scheduler.registered(driver, msg.get_framework_id(), msg.get_master_info());
    }
}

impl <S> SchedulerDriver for MesosSchedulerDriver<S> {

    fn start(&self) -> Status {
        let mut status = self.status.lock().unwrap();
        if *status == Status::DRIVER_NOT_STARTED {
            let mut libprocess = self.libprocess.lock().unwrap();
            let mut message = RegisterFrameworkMessage::new();
            message.set_framework(self.framework.lock().unwrap().clone());
            let mut master_detector = self.master_detector.lock().unwrap();
            master_detector.start();
            let master = master_detector.master().expect("No master found by detector");
            debug!("Registering with master {}", master);
            match libprocess.send(&master, &message) {
                Ok(_) => *status = Status::DRIVER_RUNNING,
                Err(_) => error!("Failed to start driver")
            }
        }
        *status
    }

    fn stop(&self, failover: bool) -> Status {
        let mut status = self.status.lock().unwrap();
        if *status == Status::DRIVER_RUNNING {
            let mut libprocess = self.libprocess.lock().unwrap();
            if failover {
                let mut message = UnregisterFrameworkMessage::new();
                message.set_framework_id(self.framework.lock().unwrap().take_id());
                let master_detector = self.master_detector.lock().unwrap();
                let master = master_detector.master().unwrap();
                debug!("Unregistering with master {}", master);
                match libprocess.send(&master, &message) {
                    Ok(_) => *status = Status::DRIVER_STOPPED,
                    Err(_) => error!("Failed to stop driver")
                }
            }
            libprocess.close();
        }
        *status
    }

    fn abort(&self) -> Status {
        let mut status = self.status.lock().unwrap();
        *status = Status::DRIVER_ABORTED;
        *status
    }

    fn join(&self) -> Status {
        self.join.wait();
        Status::DRIVER_RUNNING
    }

    fn run(&self) -> Status {
        match self.start() {
            Status::DRIVER_RUNNING => self.join(),
            status => status
        }
    }

    fn request_resources(&self, request_data: &Request) -> Status {
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

// TODO delete me
impl <S: Scheduler + Send + Sync> MesosSchedulerDriver<S> {

    fn handle(&self, name: &str, sender: &UPID, data: &Vec<u8>) {
        match name {
            "mesos.internal.FrameworkRegisteredMessage" => {
                match protobuf::parse_from_bytes::<FrameworkRegisteredMessage>(data) {
                    Ok(msg) => {
                        debug!("FrameworkRegisteredMessage {:?}", msg);
                        self.scheduler.registered(self, msg.get_framework_id(), msg.get_master_info());
                    },
                    Err(_) => {
                        error!("Failed to parse protobuf message from master");
                    }
                }
            },
            msg => {
                warn!("Unhandled {:?}", msg);
            }
        }
    }
}
