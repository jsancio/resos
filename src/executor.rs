use proto::{AgentInfo, ExecutorInfo, FrameworkInfo, TaskID, TaskInfo};
use executor_driver::ExecutorDriver;

pub trait Executor {
    // Invoked when the executor becomes "disconnected" from the slave (e.g., the slave is being restarted due to an upgrade).
    fn disconnected(&self, driver: &ExecutorDriver);

    // Invoked when a fatal error has occurred with the executor and/or executor driver.
    fn error(&self, driver: &ExecutorDriver, message: &str);

    // Invoked when a framework message has arrived for this executor.
    fn framework_message(&self, driver: &ExecutorDriver, data: &[u8]);

    // Invoked when a task running within this executor has been killed (via SchedulerDriver.killTask(TaskID)).
    fn kill_task(&self, driver: &ExecutorDriver, task_id: &TaskID);

    // Invoked when a task has been launched on this executor (initiated via SchedulerDriver.launchTasks(java.util.Collection<OfferID>, java.util.Collection<TaskInfo>, Filters).
    fn launch_task(&self, driver: &ExecutorDriver, task: &TaskInfo);

    // Invoked once the executor driver has been able to successfully connect with Mesos.
    fn registered(&self, driver: &ExecutorDriver, executor_info: &ExecutorInfo, framework_info: &FrameworkInfo, agent_info: &AgentInfo);

    // Invoked when the executor re-registers with a restarted slave.
    fn reregistered(&self, driver: &ExecutorDriver, agent_info: &AgentInfo);

    // Invoked when the executor should terminate all of it's currently running tasks.
    fn shutdown(&self, driver: &ExecutorDriver);
}
