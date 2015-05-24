extern crate mesos;

use mesos::proto::{ExecutorID, FrameworkID, FrameworkInfo, MasterInfo, Offer, OfferID, SlaveID, TaskStatus};
use mesos::scheduler::Scheduler;
use mesos::scheduler_driver::{SchedulerDriver, MesosSchedulerDriver};

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

fn main() {

    let scheduler = MyScheduler;

    let mut framework = FrameworkInfo::new();
    framework.set_name("rust-example".to_string());
    framework.set_user("bonifaido".to_string());

    let master = "localhost:5050";

    let driver = MesosSchedulerDriver::new(&scheduler, &framework, master);

    driver.start();
}