mod scheduler {

  extern crate libc;
  use native;

  // The type of Scheduler Driver implementations.
  trait SchedulerDriver {

    fn launch_tasks(
      offerId: *mut native::ProtobufObj,
      tasks: *mut native::ProtobufObj,
      filters: *mut native::ProtobufObj) -> libc::c_int;

    fn start() -> libc::c_int;

    fn stop(failover: libc::c_int) -> libc::c_int;

    fn abort() -> libc::c_int;

    fn join() -> libc::c_int;

    fn run() -> libc::c_int;

    fn request_resources(
      requestsData: *mut native::ProtobufObj) -> libc::c_int;

    fn decline_offer(
      offerId: *mut native::ProtobufObj,
      filters: *mut native::ProtobufObj) -> libc::c_int;

    fn kill_task(taskId: *mut native::ProtobufObj) -> libc::c_int;

    fn revive_offers() -> libc::c_int;

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


  // Scheduler Driver implemented by calling out to native library.
/*
  impl MesosSchedulerDriver for SchedulerDriver {
    // TODO(CD)
  }
*/

}

