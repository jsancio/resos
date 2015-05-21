#![allow(dead_code)]
#![allow(non_snake_case)]

extern crate libc;

#[link(name="mesos")]
extern "C" {

    //////////////////////////////////////////////////////////////////
    // Scheduler driver methods
    //////////////////////////////////////////////////////////////////

    /// SchedulerDriverStatus scheduler_launchTasks() [int]
    ///   SchedulerDriverPtr driver [void *]
    ///   ProtobufObj * offerId [ProtobufObj *]
    ///   ProtobufObj * tasks [ProtobufObj *]
    ///   ProtobufObj * filters [ProtobufObj *]
    pub fn scheduler_launchTasks(
        driver: *mut libc::c_void,
        offerId: *mut ProtobufObj,
        tasks: *mut ProtobufObj,
        filters: *mut ProtobufObj) -> libc::c_int;

    /// SchedulerDriverStatus scheduler_start() [int]
    ///   SchedulerDriverPtr driver [void *]
    pub fn scheduler_start(driver: *mut libc::c_void) -> libc::c_int;

    /// SchedulerDriverStatus scheduler_stop() [int]
    ///   SchedulerDriverPtr driver [void *]
    ///   int failover
    pub fn scheduler_stop(
        driver: *mut libc::c_void,
        failover: libc::c_int) -> libc::c_int;

    /// SchedulerDriverStatus scheduler_abort() [int]
    ///   SchedulerDriverPtr driver [void *]
    pub fn scheduler_abort(driver: *mut libc::c_void) -> libc::c_int;

    /// SchedulerDriverStatus scheduler_join() [int]
    ///   SchedulerDriverPtr driver [void *]
    pub fn scheduler_join(driver: *mut libc::c_void) -> libc::c_int;

    /// SchedulerDriverStatus scheduler_run() [int]
    ///   SchedulerDriverPtr driver [void *]
    pub fn scheduler_run(driver: *mut libc::c_void) -> libc::c_int;

    /// SchedulerDriverStatus scheduler_requestResources() [int]
    ///   SchedulerDriverPtr driver [void *]
    ///   ProtobufObj * requestsData [ProtobufObj *]
    pub fn scheduler_requestResources(
        driver: *mut libc::c_void,
        requestsData: *mut ProtobufObj) -> libc::c_int;

    /// SchedulerDriverStatus scheduler_declineOffer() [int]
    ///   SchedulerDriverPtr driver [void *]
    ///   ProtobufObj * offerId [ProtobufObj *]
    ///   ProtobufObj * filters [ProtobufObj *]
    pub fn scheduler_declineOffer(
        driver: *mut libc::c_void,
        offerId: *mut ProtobufObj,
        filters: *mut ProtobufObj) -> libc::c_int;

    /// SchedulerDriverStatus scheduler_killTask() [int]
    ///   SchedulerDriverPtr driver [void *]
    ///   ProtobufObj * taskId [ProtobufObj *]
    pub fn scheduler_killTask(
        driver: *mut libc::c_void,
        taskId: *mut ProtobufObj) -> libc::c_int;

    /// SchedulerDriverStatus scheduler_reviveOffers() [int]
    ///   SchedulerDriverPtr driver [void *]
    pub fn scheduler_reviveOffers(driver: *mut libc::c_void) -> libc::c_int;

    /// SchedulerDriverStatus scheduler_sendFrameworkMessage() [int]
    ///   SchedulerDriverPtr driver [void *]
    ///   ProtobufObj * executor [ProtobufObj *]
    ///   ProtobufObj * slaveId [ProtobufObj *]
    ///   const char * data
    pub fn scheduler_sendFrameworkMessage(
        driver: *mut libc::c_void,
        executor: *mut ProtobufObj,
        slaveId: *mut ProtobufObj,
        data: *const libc::c_char) -> libc::c_int;

    /// SchedulerPtrPair scheduler_init() [SchedulerPtrPair]
    ///   SchedulerCallbacks * callbacks [SchedulerCallbacks *]
    ///   void * payload
    ///   ProtobufObj * framework [ProtobufObj *]
    ///   const char * master
    pub fn scheduler_init(
        callbacks: *mut SchedulerCallbacks,
        payload: *mut libc::c_void,
        framework: *mut ProtobufObj,
        master: *const libc::c_char) -> SchedulerPtrPair;

    /// void scheduler_destroy()
    ///   void * driver
    ///   void * scheduler
    pub fn scheduler_destroy(
        driver: *mut libc::c_void,
        scheduler: *mut libc::c_void);


    //////////////////////////////////////////////////////////////////
    // Executor driver methods
    //////////////////////////////////////////////////////////////////

    /// ExecutorDriverStatus executor_start() [int]
    ///   ExecutorDriverPtr driver [void *]
    pub fn executor_start(driver: *mut libc::c_void) -> libc::c_int;

    /// ExecutorDriverStatus executor_stop() [int]
    ///   ExecutorDriverPtr driver [void *]
    pub fn executor_stop(driver: *mut libc::c_void) -> libc::c_int;

    /// ExecutorDriverStatus executor_abort() [int]
    ///   ExecutorDriverPtr driver [void *]
    pub fn executor_abort(driver: *mut libc::c_void) -> libc::c_int;

    /// ExecutorDriverStatus executor_join() [int]
    ///   ExecutorDriverPtr driver [void *]
    pub fn executor_join(driver: *mut libc::c_void) -> libc::c_int;

    /// ExecutorDriverStatus executor_run() [int]
    ///   ExecutorDriverPtr driver [void *]
    pub fn executor_run(driver: *mut libc::c_void) -> libc::c_int;

    /// ExecutorDriverStatus executor_sendStatusUpdate() [int]
    ///   ExecutorDriverPtr driver [void *]
    ///   ProtobufObj * status [ProtobufObj *]
    pub fn executor_sendStatusUpdate(
        driver: *mut libc::c_void,
        status: *mut ProtobufObj) -> libc::c_int;

    /// ExecutorDriverStatus executor_sendFrameworkMessage() [int]
    ///   ExecutorDriverPtr driver [void *]
    ///   const char * data
    pub fn executor_sendFrameworkMessage(
        driver: *mut libc::c_void,
        data: *const libc::c_char) -> libc::c_int;

    /// ExecutorPtrPair executor_init() [ExecutorPtrPair]
    ///   ExecutorCallbacks * callbacks [ExecutorCallbacks *]
    ///   void * payload
    pub fn executor_init(
        callbacks: *mut ExecutorCallbacks,
        payload: *mut libc::c_void) -> ExecutorPtrPair;

    /// void executor_destroy()
    ///   void * driver
    ///   void * executor
    pub fn executor_destroy(
        driver: *mut libc::c_void,
        executor: *mut libc::c_void);

}

/*
   struct
   void * data
   size_t size
*/
#[repr(C)]
pub struct ProtobufObj {
    pub data: *mut libc::c_void,
    pub size: libc::size_t,
}

/*
   struct
   scheduler_registeredCallBack_t registeredCallBack [void (*)(void *, ProtobufObj *, ProtobufObj *)]
   scheduler_reregisteredCallBack_t reregisteredCallBack [void (*)(void *, ProtobufObj *)]
   scheduler_resourceOffersCallBack_t resourceOffersCallBack [void (*)(void *, ProtobufObj *, size_t)]
   scheduler_statusUpdateCallBack_t statusUpdateCallBack [void (*)(void *, ProtobufObj *)]
   scheduler_disconnectedCallBack_t disconnectedCallBack [void (*)(void *)]
   scheduler_offerRescindedCallBack_t offerRescindedCallBack [void (*)(void *, ProtobufObj *)]
   scheduler_frameworkMessageCallBack_t frameworkMessageCallBack [void (*)(void *, ProtobufObj *, ProtobufObj *, ProtobufObj *)]
   scheduler_slaveLostCallBack_t slaveLostCallBack [void (*)(void *, ProtobufObj *)]
   scheduler_executorLostCallBack_t executorLostCallBack [void (*)(void *, ProtobufObj *, ProtobufObj *, int)]
   scheduler_errorCallBack_t errorCallBack [void (*)(void *, ProtobufObj *)]
*/
#[repr(C)]
pub struct SchedulerCallbacks {
    registeredCallBack: extern fn(
        *mut libc::c_void,
        *mut ProtobufObj,
        *mut ProtobufObj),

    reregisteredCallBack: extern fn(
        *mut libc::c_void,
        *mut ProtobufObj),

    resourceOffersCallBack: extern fn(
        *mut libc::c_void,
        *mut ProtobufObj,
        libc::size_t),

    statusUpdateCallBack: extern fn(*mut libc::c_void, *mut ProtobufObj),

    disconnectedCallBack: extern fn(*mut libc::c_void),

    offerRescindedCallBack: extern fn(*mut libc::c_void, *mut ProtobufObj),

    frameworkMessageCallBack: extern fn(
        *mut libc::c_void,
        *mut ProtobufObj,
        *mut ProtobufObj,
        *mut ProtobufObj),

    slaveLostCallBack: extern fn(*mut libc::c_void, *mut ProtobufObj),

    executorLostCallBack: extern fn(
        *mut libc::c_void,
        *mut ProtobufObj,
        *mut ProtobufObj,
        libc::c_int),

    errorCallBack: extern fn(*mut libc::c_void, *mut ProtobufObj),
}

/*
   struct
   void * scheduler
   void * driver
*/
#[repr(C)]
pub struct SchedulerPtrPair {
    scheduler: libc::c_void,
    driver: libc::c_void,
}

/*
   struct
   executor_registeredCallBack_t registeredCallBack [void (*)(void *, ProtobufObj *, ProtobufObj *, ProtobufObj *)]
   executor_reregisteredCallBack_t reregisteredCallBack [void (*)(void *, ProtobufObj *)]
   executor_disconnectedCallBack_t disconnectedCallBack [void (*)(void *)]
   executor_launchTaskCallBack_t launchTaskCallBack [void (*)(void *, ProtobufObj *)]
   executor_killTaskCallBack_t killTaskCallBack [void (*)(void *, ProtobufObj *)]
   executor_frameworkMessageCallBack_t frameworkMessageCallBack [void (*)(void *, ProtobufObj *)]
   executor_shutdownCallBack_t shutdownCallBack [void (*)(void *)]
   executor_errorCallBack_t errorCallBack [void (*)(void *, ProtobufObj *)]
*/
#[repr(C)]
pub struct ExecutorCallbacks {

    registeredCallBack: extern fn(
        *mut libc::c_void,
        *mut ProtobufObj,
        *mut ProtobufObj,
        *mut ProtobufObj),

    reregisteredCallBack: extern fn(*mut libc::c_void, *mut ProtobufObj),

    disconnectedCallBack: extern fn(*mut libc::c_void),

    launchTaskCallBack: extern fn(*mut libc::c_void, *mut ProtobufObj),

    killTaskCallBack: extern fn(*mut libc::c_void, *mut ProtobufObj),

    frameworkMessageCallBack: extern fn(*mut libc::c_void, *mut ProtobufObj),

    shutdownCallBack: extern fn(*mut libc::c_void),

    errorCallBack: extern fn(*mut libc::c_void, *mut ProtobufObj),
}

/*
   struct
   void * executor
   void * driver
*/
#[repr(C)]
pub struct ExecutorPtrPair {
    executor: libc::c_void,
    driver: libc::c_void,
}

