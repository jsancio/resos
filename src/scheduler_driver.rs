extern crate libc;

use mesos;
use native;
use scheduler;

/// Abstract interface for connecting a scheduler to Mesos. This
/// interface is used both to manage the scheduler's lifecycle (start
/// it, stop it, or wait for it to finish) and to interact with Mesos
/// (e.g., launch tasks, kill tasks, etc.).
trait SchedulerDriver {
    /// @param offerId The offer ID.
    /// @param tasks   The collection of tasks to be launched.
    /// @param filters The filters to set for any remaining resources.
    ///
    /// @return            The state of the driver after the call.
    fn launch_tasks(
        offerId: &mesos::OfferID,
        tasks: &Vec<mesos::TaskInfo>,
        filters: &mesos::Filters) -> mesos::Status;

    /// Starts the scheduler driver. This needs to be called before any
    /// other driver calls are made.
    ///
    /// @return            The state of the driver after the call.
    fn start() -> mesos::Status;

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
    fn stop(failover: bool) -> mesos::Status;

    /// Aborts the driver so that no more callbacks can be made to the
    /// scheduler. The semantics of abort and stop have deliberately been
    /// separated so that code can detect an aborted driver (i.e., via
    /// the return status of {@link #join}, see below), and instantiate
    /// and start another driver if desired (from within the same
    /// process).
    ///
    /// @return The state of the driver after the call.
    fn abort() -> mesos::Status;

    /// Waits for the driver to be stopped or aborted, possibly
    /// _blocking_ the current thread indefinitely. The return status of
    /// this function can be used to determine if the driver was aborted
    /// (see mesos.proto for a description of Status).
    ///
    /// @return The state of the driver after the call.
    fn join() -> mesos::Status;

    /// Starts and immediately joins (i.e., blocks on) the driver.
    ///
    /// @return The state of the driver after the call.
    fn run() -> mesos::Status;

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
        requestsData: &mesos::Request) -> mesos::Status;

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
        offerId: &mesos::OfferID,
        filters: &mesos::Filters) -> mesos::Status;

    /// Kills the specified task. Note that attempting to kill a task is
    /// currently not reliable. If, for example, a scheduler fails over
    /// while it was attempting to kill a task it will need to retry in
    /// the future Likewise, if unregistered / disconnected, the request
    /// will be dropped (these semantics may be changed in the future).
    ///
    /// @param taskId  The ID of the task to be killed.
    ///
    /// @return        The state of the driver after the call.
    fn kill_task(taskId: &mesos::TaskID) -> mesos::Status;

    /// Removes all filters, previously set by the framework (via {@link
    /// #launchTasks}). This enables the framework to receive offers
    /// from those filtered slaves.
    ///
    /// @return    The state of the driver after the call.
    fn revive_offers() -> mesos::Status;

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
        executorId: &mesos::ExecutorID,
        slaveId: &mesos::SlaveID,
        data: &Vec<u8>) -> mesos::Status;

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
struct MesosSchedulerDriver; // {
    // scheduler: scheduler::Scheduler
// }

// impl SchedulerDriver for MesosSchedulerDriver {
//      // TODO(CD): Call native::scheduler_init upon construction
//      // TODO(CD): Implement the trait by marshalling the protobuf objects
//      //           and calling the native implementations.
// }

