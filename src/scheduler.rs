use proto::{ExecutorID, FrameworkID, MasterInfo, Offer, OfferID, SlaveID, TaskStatus};
use scheduler_driver::SchedulerDriver;

pub trait Scheduler {
    // Invoked when the scheduler becomes "disconnected" from the master (e.g., the master fails and another is taking over).
    fn disconnected(&self, driver: &SchedulerDriver);

    // Invoked when there is an unrecoverable error in the scheduler or driver.
    fn error(&self, driver: &SchedulerDriver, message: &str);

    // Invoked when an executor has exited/terminated.
    fn executor_lost(&self, driver: &SchedulerDriver, executor_id: &ExecutorID, slave_id: &SlaveID, status: isize);

    // Invoked when an executor sends a message.
    fn framework_message(&self, driver: &SchedulerDriver, executor_id: &ExecutorID, slave_id: &SlaveID, data: &[u8]);

    // Invoked when an offer is no longer valid (e.g., the slave was lost or another framework used resources in the offer).
    fn offer_rescinded(&self, driver: &SchedulerDriver, offer_id: &OfferID);

    // Invoked when the scheduler successfully registers with a Mesos master.
    fn registered(&self, driver: &SchedulerDriver, framework_id: &FrameworkID, master_info: &MasterInfo);

    // Invoked when the scheduler re-registers with a newly elected Mesos master.
    fn reregistered(&self, driver: &SchedulerDriver, master_info: &MasterInfo);

    // Invoked when resources have been offered to this framework.
    fn resource_offers(&self, driver: &SchedulerDriver, offers: &Vec<Offer>);

    // Invoked when a slave has been determined unreachable (e.g., machine failure, network partition).
    fn slave_lost(&self, driver: &SchedulerDriver, slave_id: &SlaveID);

    // Invoked when the status of a task has changed (e.g., a slave is lost and so the task is lost, a task finishes and an executor sends a status update saying so, etc).
    fn status_update(&self, driver: &SchedulerDriver, status: &TaskStatus);
}
