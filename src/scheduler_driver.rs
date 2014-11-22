mod scheduler {

    extern crate libc;
    use native;

    /// Abstract interface for connecting a scheduler to Mesos. This
    /// interface is used both to manage the scheduler's lifecycle (start
    /// it, stop it, or wait for it to finish) and to interact with Mesos
    /// (e.g., launch tasks, kill tasks, etc.).
    trait SchedulerDriver {

        /// @param offerId The offer ID.
        /// @param tasks   The collection of tasks to be launched.
        /// @param filters The filters to set for any remaining resources.
        ///
        /// @return        The state of the driver after the call.
        fn launch_tasks(
            offerId: *mut native::ProtobufObj,
            tasks: *mut native::ProtobufObj,
            filters: *mut native::ProtobufObj) -> libc::c_int;

        /// Starts the scheduler driver. This needs to be called before any
        /// other driver calls are made.
        ///
        /// @return    The state of the driver after the call.
        fn start() -> libc::c_int;

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
        fn stop(failover: libc::c_int) -> libc::c_int;

        /// Aborts the driver so that no more callbacks can be made to the
        /// scheduler. The semantics of abort and stop have deliberately been
        /// separated so that code can detect an aborted driver (i.e., via
        /// the return status of {@link #join}, see below), and instantiate
        /// and start another driver if desired (from within the same
        /// process).
        ///
        /// @return The state of the driver after the call.
        fn abort() -> libc::c_int;

        /// Waits for the driver to be stopped or aborted, possibly
        /// _blocking_ the current thread indefinitely. The return status of
        /// this function can be used to determine if the driver was aborted
        /// (see mesos.proto for a description of Status).
        ///
        /// @return The state of the driver after the call.
        fn join() -> libc::c_int;

        /// Starts and immediately joins (i.e., blocks on) the driver.
        ///
        /// @return The state of the driver after the call.
        fn run() -> libc::c_int;

        /// Requests resources from Mesos (see mesos.proto for a description
        /// of Request and how, for example, to request resources
        /// from specific slaves). Any resources available are offered to the
        /// framework via {@link Scheduler#resourceOffers} callback,
        /// asynchronously.
        ///
        /// @param requests    The resource requests.
        ///
        /// @return            The state of the driver after the call.
        fn request_resources(
            requestsData: *mut native::ProtobufObj) -> libc::c_int;

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
        fn decline_offer(
            offerId: *mut native::ProtobufObj,
            filters: *mut native::ProtobufObj) -> libc::c_int;

        /// Kills the specified task. Note that attempting to kill a task is
        /// currently not reliable. If, for example, a scheduler fails over
        /// while it was attempting to kill a task it will need to retry in
        /// the future Likewise, if unregistered / disconnected, the request
        /// will be dropped (these semantics may be changed in the future).
        ///
        /// @param taskId  The ID of the task to be killed.
        ///
        /// @return        The state of the driver after the call.
        fn kill_task(taskId: *mut native::ProtobufObj) -> libc::c_int;

        /// Removes all filters, previously set by the framework (via {@link
        /// #launchTasks}). This enables the framework to receive offers
        /// from those filtered slaves.
        ///
        /// @return    The state of the driver after the call.
        fn revive_offers() -> libc::c_int;

        /// Sends a message from the framework to one of its executors. These
        /// messages are best effort; do not expect a framework message to be
        /// retransmitted in any reliable fashion.
        ///
        /// @param executorId  The ID of the executor to send the message to.
        /// @param slaveId     The ID of the slave that is running the executor.
        /// @param data        The message.
        ///
        /// @return            The state of the driver after the call.
        fn send_framework_message(
            executor: *mut native::ProtobufObj,
            slaveId: *mut native::ProtobufObj,
            data: *const libc::c_char) -> libc::c_int;

        fn init(
            callbacks: *mut native::SchedulerCallbacks,
            payload: *mut libc::c_void,
            framework: *mut native::ProtobufObj,
            master: *const libc::c_char) -> native::SchedulerPtrPair;

        fn destroy(scheduler: *mut libc::c_void);
    }

/*
    /// Scheduler Driver implemented by calling out to native library.
    impl MesosSchedulerDriver for SchedulerDriver {

        // TODO(CD): How does one do instance members in rust?

        // let mut scheduler: libc::c_void;
        // let mut underlyingDriver: libc::c_void;

        fn launch_tasks(
            offerId: *mut native::ProtobufObj,
            tasks: *mut native::ProtobufObj,
            filters: *mut native::ProtobufObj) -> libc::c_int {
        }

        fn start() -> libc::c_int {
        }

        fn stop(failover: libc::c_int) -> libc::c_int {
        }

        fn abort() -> libc::c_int {
        }

        fn join() -> libc::c_int {
        }

        fn run() -> libc::c_int {
        }

        fn request_resources(
            requestsData: *mut native::ProtobufObj) -> libc::c_int {
        }

        fn decline_offer(
            offerId: *mut native::ProtobufObj,
            filters: *mut native::ProtobufObj) -> libc::c_int {
        }

        fn kill_task(taskId: *mut native::ProtobufObj) -> libc::c_int {
        }

        fn revive_offers() -> libc::c_int {
        }

        fn send_framework_message(
            executor: *mut native::ProtobufObj,
            slaveId: *mut native::ProtobufObj,
            data: *const libc::c_char) -> libc::c_int {
        }

        fn init(
            callbacks: *mut native::SchedulerCallbacks,
            payload: *mut libc::c_void,
            framework: *mut native::ProtobufObj,
            master: *const libc::c_char) -> native::SchedulerPtrPair {
        }

        fn destroy(scheduler: *mut libc::c_void) {
        }
    }
*/

}

