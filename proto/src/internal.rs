// This file is generated. Do not edit

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;
use super::proto::FrameworkID;
use super::proto::OfferID;
use super::proto::SlaveID;
use super::proto::TaskID;
use super::proto::ExecutorID;
use super::proto::ContainerID;
use super::proto::FrameworkInfo;
use super::proto::HealthCheck;
use super::proto::CommandInfo;
use super::proto::ExecutorInfo;
use super::proto::MasterInfo;
use super::proto::SlaveInfo;
use super::proto::Value;
use super::proto::Attribute;
use super::proto::Resource;
use super::proto::ResourceStatistics;
use super::proto::ResourceUsage;
use super::proto::PerfStatistics;
use super::proto::Request;
use super::proto::Offer;
use super::proto::TaskInfo;
use super::proto::TaskStatus;
use super::proto::Filters;
use super::proto::Environment;
use super::proto::Parameter;
use super::proto::Parameters;
use super::proto::Credential;
use super::proto::Credentials;
use super::proto::ACL;
use super::proto::ACLs;
use super::proto::RateLimit;
use super::proto::RateLimits;
use super::proto::Volume;
use super::proto::ContainerInfo;
use super::proto::Labels;
use super::proto::Label;
use super::proto::Port;
use super::proto::Ports;
use super::proto::DiscoveryInfo;
use super::proto::Status;
use super::proto::TaskState;

#[derive(Clone,Default)]
pub struct Task {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    task_id: ::protobuf::SingularPtrField<TaskID>,
    framework_id: ::protobuf::SingularPtrField<FrameworkID>,
    executor_id: ::protobuf::SingularPtrField<ExecutorID>,
    slave_id: ::protobuf::SingularPtrField<SlaveID>,
    state: ::std::option::Option<TaskState>,
    resources: ::protobuf::RepeatedField<Resource>,
    statuses: ::protobuf::RepeatedField<TaskStatus>,
    status_update_state: ::std::option::Option<TaskState>,
    status_update_uuid: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    labels: ::protobuf::SingularPtrField<Labels>,
    discovery: ::protobuf::SingularPtrField<DiscoveryInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Task {
    pub fn new() -> Task {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Task {
        static mut instance: ::protobuf::lazy::Lazy<Task> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Task,
        };
        unsafe {
            instance.get(|| {
                Task {
                    name: ::protobuf::SingularField::none(),
                    task_id: ::protobuf::SingularPtrField::none(),
                    framework_id: ::protobuf::SingularPtrField::none(),
                    executor_id: ::protobuf::SingularPtrField::none(),
                    slave_id: ::protobuf::SingularPtrField::none(),
                    state: ::std::option::Option::None,
                    resources: ::protobuf::RepeatedField::new(),
                    statuses: ::protobuf::RepeatedField::new(),
                    status_update_state: ::std::option::Option::None,
                    status_update_uuid: ::protobuf::SingularField::none(),
                    labels: ::protobuf::SingularPtrField::none(),
                    discovery: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name<'a>(&'a self) -> &'a str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required .mesos.TaskID task_id = 2;

    pub fn clear_task_id(&mut self) {
        self.task_id.clear();
    }

    pub fn has_task_id(&self) -> bool {
        self.task_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_task_id(&mut self, v: TaskID) {
        self.task_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_task_id<'a>(&'a mut self) -> &'a mut TaskID {
        if self.task_id.is_none() {
            self.task_id.set_default();
        };
        self.task_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_task_id(&mut self) -> TaskID {
        self.task_id.take().unwrap_or_else(|| TaskID::new())
    }

    pub fn get_task_id<'a>(&'a self) -> &'a TaskID {
        self.task_id.as_ref().unwrap_or_else(|| TaskID::default_instance())
    }

    // required .mesos.FrameworkID framework_id = 3;

    pub fn clear_framework_id(&mut self) {
        self.framework_id.clear();
    }

    pub fn has_framework_id(&self) -> bool {
        self.framework_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framework_id(&mut self, v: FrameworkID) {
        self.framework_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_framework_id<'a>(&'a mut self) -> &'a mut FrameworkID {
        if self.framework_id.is_none() {
            self.framework_id.set_default();
        };
        self.framework_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_framework_id(&mut self) -> FrameworkID {
        self.framework_id.take().unwrap_or_else(|| FrameworkID::new())
    }

    pub fn get_framework_id<'a>(&'a self) -> &'a FrameworkID {
        self.framework_id.as_ref().unwrap_or_else(|| FrameworkID::default_instance())
    }

    // optional .mesos.ExecutorID executor_id = 4;

    pub fn clear_executor_id(&mut self) {
        self.executor_id.clear();
    }

    pub fn has_executor_id(&self) -> bool {
        self.executor_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_executor_id(&mut self, v: ExecutorID) {
        self.executor_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_executor_id<'a>(&'a mut self) -> &'a mut ExecutorID {
        if self.executor_id.is_none() {
            self.executor_id.set_default();
        };
        self.executor_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_executor_id(&mut self) -> ExecutorID {
        self.executor_id.take().unwrap_or_else(|| ExecutorID::new())
    }

    pub fn get_executor_id<'a>(&'a self) -> &'a ExecutorID {
        self.executor_id.as_ref().unwrap_or_else(|| ExecutorID::default_instance())
    }

    // required .mesos.SlaveID slave_id = 5;

    pub fn clear_slave_id(&mut self) {
        self.slave_id.clear();
    }

    pub fn has_slave_id(&self) -> bool {
        self.slave_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slave_id(&mut self, v: SlaveID) {
        self.slave_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_slave_id<'a>(&'a mut self) -> &'a mut SlaveID {
        if self.slave_id.is_none() {
            self.slave_id.set_default();
        };
        self.slave_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_slave_id(&mut self) -> SlaveID {
        self.slave_id.take().unwrap_or_else(|| SlaveID::new())
    }

    pub fn get_slave_id<'a>(&'a self) -> &'a SlaveID {
        self.slave_id.as_ref().unwrap_or_else(|| SlaveID::default_instance())
    }

    // required .mesos.TaskState state = 6;

    pub fn clear_state(&mut self) {
        self.state = ::std::option::Option::None;
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: TaskState) {
        self.state = ::std::option::Option::Some(v);
    }

    pub fn get_state<'a>(&self) -> TaskState {
        self.state.unwrap_or(TaskState::TASK_STAGING)
    }

    // repeated .mesos.Resource resources = 7;

    pub fn clear_resources(&mut self) {
        self.resources.clear();
    }

    // Param is passed by value, moved
    pub fn set_resources(&mut self, v: ::protobuf::RepeatedField<Resource>) {
        self.resources = v;
    }

    // Mutable pointer to the field.
    pub fn mut_resources<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Resource> {
        &mut self.resources
    }

    // Take field
    pub fn take_resources(&mut self) -> ::protobuf::RepeatedField<Resource> {
        ::std::mem::replace(&mut self.resources, ::protobuf::RepeatedField::new())
    }

    pub fn get_resources<'a>(&'a self) -> &'a [Resource] {
        &self.resources
    }

    // repeated .mesos.TaskStatus statuses = 8;

    pub fn clear_statuses(&mut self) {
        self.statuses.clear();
    }

    // Param is passed by value, moved
    pub fn set_statuses(&mut self, v: ::protobuf::RepeatedField<TaskStatus>) {
        self.statuses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_statuses<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<TaskStatus> {
        &mut self.statuses
    }

    // Take field
    pub fn take_statuses(&mut self) -> ::protobuf::RepeatedField<TaskStatus> {
        ::std::mem::replace(&mut self.statuses, ::protobuf::RepeatedField::new())
    }

    pub fn get_statuses<'a>(&'a self) -> &'a [TaskStatus] {
        &self.statuses
    }

    // optional .mesos.TaskState status_update_state = 9;

    pub fn clear_status_update_state(&mut self) {
        self.status_update_state = ::std::option::Option::None;
    }

    pub fn has_status_update_state(&self) -> bool {
        self.status_update_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status_update_state(&mut self, v: TaskState) {
        self.status_update_state = ::std::option::Option::Some(v);
    }

    pub fn get_status_update_state<'a>(&self) -> TaskState {
        self.status_update_state.unwrap_or(TaskState::TASK_STAGING)
    }

    // optional bytes status_update_uuid = 10;

    pub fn clear_status_update_uuid(&mut self) {
        self.status_update_uuid.clear();
    }

    pub fn has_status_update_uuid(&self) -> bool {
        self.status_update_uuid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status_update_uuid(&mut self, v: ::std::vec::Vec<u8>) {
        self.status_update_uuid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status_update_uuid<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.status_update_uuid.is_none() {
            self.status_update_uuid.set_default();
        };
        self.status_update_uuid.as_mut().unwrap()
    }

    // Take field
    pub fn take_status_update_uuid(&mut self) -> ::std::vec::Vec<u8> {
        self.status_update_uuid.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_status_update_uuid<'a>(&'a self) -> &'a [u8] {
        match self.status_update_uuid.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional .mesos.Labels labels = 11;

    pub fn clear_labels(&mut self) {
        self.labels.clear();
    }

    pub fn has_labels(&self) -> bool {
        self.labels.is_some()
    }

    // Param is passed by value, moved
    pub fn set_labels(&mut self, v: Labels) {
        self.labels = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_labels<'a>(&'a mut self) -> &'a mut Labels {
        if self.labels.is_none() {
            self.labels.set_default();
        };
        self.labels.as_mut().unwrap()
    }

    // Take field
    pub fn take_labels(&mut self) -> Labels {
        self.labels.take().unwrap_or_else(|| Labels::new())
    }

    pub fn get_labels<'a>(&'a self) -> &'a Labels {
        self.labels.as_ref().unwrap_or_else(|| Labels::default_instance())
    }

    // optional .mesos.DiscoveryInfo discovery = 12;

    pub fn clear_discovery(&mut self) {
        self.discovery.clear();
    }

    pub fn has_discovery(&self) -> bool {
        self.discovery.is_some()
    }

    // Param is passed by value, moved
    pub fn set_discovery(&mut self, v: DiscoveryInfo) {
        self.discovery = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_discovery<'a>(&'a mut self) -> &'a mut DiscoveryInfo {
        if self.discovery.is_none() {
            self.discovery.set_default();
        };
        self.discovery.as_mut().unwrap()
    }

    // Take field
    pub fn take_discovery(&mut self) -> DiscoveryInfo {
        self.discovery.take().unwrap_or_else(|| DiscoveryInfo::new())
    }

    pub fn get_discovery<'a>(&'a self) -> &'a DiscoveryInfo {
        self.discovery.as_ref().unwrap_or_else(|| DiscoveryInfo::default_instance())
    }
}

impl ::protobuf::Message for Task {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        if self.task_id.is_none() {
            return false;
        };
        if self.framework_id.is_none() {
            return false;
        };
        if self.slave_id.is_none() {
            return false;
        };
        if self.state.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.name.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.task_id.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.framework_id.set_default();
                    try!(is.merge_message(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.executor_id.set_default();
                    try!(is.merge_message(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.slave_id.set_default();
                    try!(is.merge_message(tmp))
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.state = ::std::option::Option::Some(tmp);
                },
                7 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.resources));
                },
                8 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.statuses));
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.status_update_state = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.status_update_uuid.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.labels.set_default();
                    try!(is.merge_message(tmp))
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.discovery.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.task_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.framework_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.executor_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.slave_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.state.iter() {
            my_size += ::protobuf::rt::enum_size(6, *value);
        };
        for value in self.resources.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.statuses.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.status_update_state.iter() {
            my_size += ::protobuf::rt::enum_size(9, *value);
        };
        for value in self.status_update_uuid.iter() {
            my_size += ::protobuf::rt::bytes_size(10, &value);
        };
        for value in self.labels.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.discovery.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.task_id.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.framework_id.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.executor_id.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.slave_id.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.state {
            try!(os.write_enum(6, v as i32));
        };
        for v in self.resources.iter() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.statuses.iter() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.status_update_state {
            try!(os.write_enum(9, v as i32));
        };
        if let Some(v) = self.status_update_uuid.as_ref() {
            try!(os.write_bytes(10, &v));
        };
        if let Some(v) = self.labels.as_ref() {
            try!(os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.discovery.as_ref() {
            try!(os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Task>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Task {
    fn new() -> Task {
        Task::new()
    }

    fn descriptor_static(_: ::std::option::Option<Task>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    Task::has_name,
                    Task::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "task_id",
                    Task::has_task_id,
                    Task::get_task_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "framework_id",
                    Task::has_framework_id,
                    Task::get_framework_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "executor_id",
                    Task::has_executor_id,
                    Task::get_executor_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "slave_id",
                    Task::has_slave_id,
                    Task::get_slave_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "state",
                    Task::has_state,
                    Task::get_state,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "resources",
                    Task::get_resources,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "statuses",
                    Task::get_statuses,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "status_update_state",
                    Task::has_status_update_state,
                    Task::get_status_update_state,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "status_update_uuid",
                    Task::has_status_update_uuid,
                    Task::get_status_update_uuid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "labels",
                    Task::has_labels,
                    Task::get_labels,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "discovery",
                    Task::has_discovery,
                    Task::get_discovery,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Task>(
                    "Task",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Task {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_task_id();
        self.clear_framework_id();
        self.clear_executor_id();
        self.clear_slave_id();
        self.clear_state();
        self.clear_resources();
        self.clear_statuses();
        self.clear_status_update_state();
        self.clear_status_update_uuid();
        self.clear_labels();
        self.clear_discovery();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Task {
    fn eq(&self, other: &Task) -> bool {
        self.name == other.name &&
        self.task_id == other.task_id &&
        self.framework_id == other.framework_id &&
        self.executor_id == other.executor_id &&
        self.slave_id == other.slave_id &&
        self.state == other.state &&
        self.resources == other.resources &&
        self.statuses == other.statuses &&
        self.status_update_state == other.status_update_state &&
        self.status_update_uuid == other.status_update_uuid &&
        self.labels == other.labels &&
        self.discovery == other.discovery &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Task {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StatusUpdate {
    // message fields
    framework_id: ::protobuf::SingularPtrField<FrameworkID>,
    executor_id: ::protobuf::SingularPtrField<ExecutorID>,
    slave_id: ::protobuf::SingularPtrField<SlaveID>,
    status: ::protobuf::SingularPtrField<TaskStatus>,
    timestamp: ::std::option::Option<f64>,
    uuid: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    latest_state: ::std::option::Option<TaskState>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl StatusUpdate {
    pub fn new() -> StatusUpdate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StatusUpdate {
        static mut instance: ::protobuf::lazy::Lazy<StatusUpdate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StatusUpdate,
        };
        unsafe {
            instance.get(|| {
                StatusUpdate {
                    framework_id: ::protobuf::SingularPtrField::none(),
                    executor_id: ::protobuf::SingularPtrField::none(),
                    slave_id: ::protobuf::SingularPtrField::none(),
                    status: ::protobuf::SingularPtrField::none(),
                    timestamp: ::std::option::Option::None,
                    uuid: ::protobuf::SingularField::none(),
                    latest_state: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .mesos.FrameworkID framework_id = 1;

    pub fn clear_framework_id(&mut self) {
        self.framework_id.clear();
    }

    pub fn has_framework_id(&self) -> bool {
        self.framework_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framework_id(&mut self, v: FrameworkID) {
        self.framework_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_framework_id<'a>(&'a mut self) -> &'a mut FrameworkID {
        if self.framework_id.is_none() {
            self.framework_id.set_default();
        };
        self.framework_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_framework_id(&mut self) -> FrameworkID {
        self.framework_id.take().unwrap_or_else(|| FrameworkID::new())
    }

    pub fn get_framework_id<'a>(&'a self) -> &'a FrameworkID {
        self.framework_id.as_ref().unwrap_or_else(|| FrameworkID::default_instance())
    }

    // optional .mesos.ExecutorID executor_id = 2;

    pub fn clear_executor_id(&mut self) {
        self.executor_id.clear();
    }

    pub fn has_executor_id(&self) -> bool {
        self.executor_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_executor_id(&mut self, v: ExecutorID) {
        self.executor_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_executor_id<'a>(&'a mut self) -> &'a mut ExecutorID {
        if self.executor_id.is_none() {
            self.executor_id.set_default();
        };
        self.executor_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_executor_id(&mut self) -> ExecutorID {
        self.executor_id.take().unwrap_or_else(|| ExecutorID::new())
    }

    pub fn get_executor_id<'a>(&'a self) -> &'a ExecutorID {
        self.executor_id.as_ref().unwrap_or_else(|| ExecutorID::default_instance())
    }

    // optional .mesos.SlaveID slave_id = 3;

    pub fn clear_slave_id(&mut self) {
        self.slave_id.clear();
    }

    pub fn has_slave_id(&self) -> bool {
        self.slave_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slave_id(&mut self, v: SlaveID) {
        self.slave_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_slave_id<'a>(&'a mut self) -> &'a mut SlaveID {
        if self.slave_id.is_none() {
            self.slave_id.set_default();
        };
        self.slave_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_slave_id(&mut self) -> SlaveID {
        self.slave_id.take().unwrap_or_else(|| SlaveID::new())
    }

    pub fn get_slave_id<'a>(&'a self) -> &'a SlaveID {
        self.slave_id.as_ref().unwrap_or_else(|| SlaveID::default_instance())
    }

    // required .mesos.TaskStatus status = 4;

    pub fn clear_status(&mut self) {
        self.status.clear();
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: TaskStatus) {
        self.status = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status<'a>(&'a mut self) -> &'a mut TaskStatus {
        if self.status.is_none() {
            self.status.set_default();
        };
        self.status.as_mut().unwrap()
    }

    // Take field
    pub fn take_status(&mut self) -> TaskStatus {
        self.status.take().unwrap_or_else(|| TaskStatus::new())
    }

    pub fn get_status<'a>(&'a self) -> &'a TaskStatus {
        self.status.as_ref().unwrap_or_else(|| TaskStatus::default_instance())
    }

    // required double timestamp = 5;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = ::std::option::Option::None;
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: f64) {
        self.timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp<'a>(&self) -> f64 {
        self.timestamp.unwrap_or(0.)
    }

    // required bytes uuid = 6;

    pub fn clear_uuid(&mut self) {
        self.uuid.clear();
    }

    pub fn has_uuid(&self) -> bool {
        self.uuid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uuid(&mut self, v: ::std::vec::Vec<u8>) {
        self.uuid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uuid<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.uuid.is_none() {
            self.uuid.set_default();
        };
        self.uuid.as_mut().unwrap()
    }

    // Take field
    pub fn take_uuid(&mut self) -> ::std::vec::Vec<u8> {
        self.uuid.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_uuid<'a>(&'a self) -> &'a [u8] {
        match self.uuid.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional .mesos.TaskState latest_state = 7;

    pub fn clear_latest_state(&mut self) {
        self.latest_state = ::std::option::Option::None;
    }

    pub fn has_latest_state(&self) -> bool {
        self.latest_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_latest_state(&mut self, v: TaskState) {
        self.latest_state = ::std::option::Option::Some(v);
    }

    pub fn get_latest_state<'a>(&self) -> TaskState {
        self.latest_state.unwrap_or(TaskState::TASK_STAGING)
    }
}

impl ::protobuf::Message for StatusUpdate {
    fn is_initialized(&self) -> bool {
        if self.framework_id.is_none() {
            return false;
        };
        if self.status.is_none() {
            return false;
        };
        if self.timestamp.is_none() {
            return false;
        };
        if self.uuid.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.framework_id.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.executor_id.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.slave_id.set_default();
                    try!(is.merge_message(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.status.set_default();
                    try!(is.merge_message(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.timestamp = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.uuid.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.latest_state = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.framework_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.executor_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.slave_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.status.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.timestamp.is_some() {
            my_size += 9;
        };
        for value in self.uuid.iter() {
            my_size += ::protobuf::rt::bytes_size(6, &value);
        };
        for value in self.latest_state.iter() {
            my_size += ::protobuf::rt::enum_size(7, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.framework_id.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.executor_id.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.slave_id.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.status.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.timestamp {
            try!(os.write_double(5, v));
        };
        if let Some(v) = self.uuid.as_ref() {
            try!(os.write_bytes(6, &v));
        };
        if let Some(v) = self.latest_state {
            try!(os.write_enum(7, v as i32));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<StatusUpdate>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StatusUpdate {
    fn new() -> StatusUpdate {
        StatusUpdate::new()
    }

    fn descriptor_static(_: ::std::option::Option<StatusUpdate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "framework_id",
                    StatusUpdate::has_framework_id,
                    StatusUpdate::get_framework_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "executor_id",
                    StatusUpdate::has_executor_id,
                    StatusUpdate::get_executor_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "slave_id",
                    StatusUpdate::has_slave_id,
                    StatusUpdate::get_slave_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "status",
                    StatusUpdate::has_status,
                    StatusUpdate::get_status,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_f64_accessor(
                    "timestamp",
                    StatusUpdate::has_timestamp,
                    StatusUpdate::get_timestamp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "uuid",
                    StatusUpdate::has_uuid,
                    StatusUpdate::get_uuid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "latest_state",
                    StatusUpdate::has_latest_state,
                    StatusUpdate::get_latest_state,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StatusUpdate>(
                    "StatusUpdate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StatusUpdate {
    fn clear(&mut self) {
        self.clear_framework_id();
        self.clear_executor_id();
        self.clear_slave_id();
        self.clear_status();
        self.clear_timestamp();
        self.clear_uuid();
        self.clear_latest_state();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StatusUpdate {
    fn eq(&self, other: &StatusUpdate) -> bool {
        self.framework_id == other.framework_id &&
        self.executor_id == other.executor_id &&
        self.slave_id == other.slave_id &&
        self.status == other.status &&
        self.timestamp == other.timestamp &&
        self.uuid == other.uuid &&
        self.latest_state == other.latest_state &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StatusUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StatusUpdateRecord {
    // message fields
    field_type: ::std::option::Option<StatusUpdateRecord_Type>,
    update: ::protobuf::SingularPtrField<StatusUpdate>,
    uuid: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl StatusUpdateRecord {
    pub fn new() -> StatusUpdateRecord {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StatusUpdateRecord {
        static mut instance: ::protobuf::lazy::Lazy<StatusUpdateRecord> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StatusUpdateRecord,
        };
        unsafe {
            instance.get(|| {
                StatusUpdateRecord {
                    field_type: ::std::option::Option::None,
                    update: ::protobuf::SingularPtrField::none(),
                    uuid: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .mesos.internal.StatusUpdateRecord.Type type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: StatusUpdateRecord_Type) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type<'a>(&self) -> StatusUpdateRecord_Type {
        self.field_type.unwrap_or(StatusUpdateRecord_Type::UPDATE)
    }

    // optional .mesos.internal.StatusUpdate update = 2;

    pub fn clear_update(&mut self) {
        self.update.clear();
    }

    pub fn has_update(&self) -> bool {
        self.update.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update(&mut self, v: StatusUpdate) {
        self.update = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_update<'a>(&'a mut self) -> &'a mut StatusUpdate {
        if self.update.is_none() {
            self.update.set_default();
        };
        self.update.as_mut().unwrap()
    }

    // Take field
    pub fn take_update(&mut self) -> StatusUpdate {
        self.update.take().unwrap_or_else(|| StatusUpdate::new())
    }

    pub fn get_update<'a>(&'a self) -> &'a StatusUpdate {
        self.update.as_ref().unwrap_or_else(|| StatusUpdate::default_instance())
    }

    // optional bytes uuid = 3;

    pub fn clear_uuid(&mut self) {
        self.uuid.clear();
    }

    pub fn has_uuid(&self) -> bool {
        self.uuid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uuid(&mut self, v: ::std::vec::Vec<u8>) {
        self.uuid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uuid<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.uuid.is_none() {
            self.uuid.set_default();
        };
        self.uuid.as_mut().unwrap()
    }

    // Take field
    pub fn take_uuid(&mut self) -> ::std::vec::Vec<u8> {
        self.uuid.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_uuid<'a>(&'a self) -> &'a [u8] {
        match self.uuid.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for StatusUpdateRecord {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.update.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.uuid.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.field_type.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.update.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.uuid.iter() {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            try!(os.write_enum(1, v as i32));
        };
        if let Some(v) = self.update.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.uuid.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<StatusUpdateRecord>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StatusUpdateRecord {
    fn new() -> StatusUpdateRecord {
        StatusUpdateRecord::new()
    }

    fn descriptor_static(_: ::std::option::Option<StatusUpdateRecord>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "field_type",
                    StatusUpdateRecord::has_field_type,
                    StatusUpdateRecord::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "update",
                    StatusUpdateRecord::has_update,
                    StatusUpdateRecord::get_update,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "uuid",
                    StatusUpdateRecord::has_uuid,
                    StatusUpdateRecord::get_uuid,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StatusUpdateRecord>(
                    "StatusUpdateRecord",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StatusUpdateRecord {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_update();
        self.clear_uuid();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StatusUpdateRecord {
    fn eq(&self, other: &StatusUpdateRecord) -> bool {
        self.field_type == other.field_type &&
        self.update == other.update &&
        self.uuid == other.uuid &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StatusUpdateRecord {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub enum StatusUpdateRecord_Type {
    UPDATE = 0,
    ACK = 1,
}

impl ::protobuf::ProtobufEnum for StatusUpdateRecord_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<StatusUpdateRecord_Type> {
        match value {
            0 => ::std::option::Option::Some(StatusUpdateRecord_Type::UPDATE),
            1 => ::std::option::Option::Some(StatusUpdateRecord_Type::ACK),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<StatusUpdateRecord_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("StatusUpdateRecord_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for StatusUpdateRecord_Type {
}

#[derive(Clone,Default)]
pub struct SubmitSchedulerRequest {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl SubmitSchedulerRequest {
    pub fn new() -> SubmitSchedulerRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SubmitSchedulerRequest {
        static mut instance: ::protobuf::lazy::Lazy<SubmitSchedulerRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SubmitSchedulerRequest,
        };
        unsafe {
            instance.get(|| {
                SubmitSchedulerRequest {
                    name: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name<'a>(&'a self) -> &'a str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for SubmitSchedulerRequest {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.name.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SubmitSchedulerRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SubmitSchedulerRequest {
    fn new() -> SubmitSchedulerRequest {
        SubmitSchedulerRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<SubmitSchedulerRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    SubmitSchedulerRequest::has_name,
                    SubmitSchedulerRequest::get_name,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SubmitSchedulerRequest>(
                    "SubmitSchedulerRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SubmitSchedulerRequest {
    fn clear(&mut self) {
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SubmitSchedulerRequest {
    fn eq(&self, other: &SubmitSchedulerRequest) -> bool {
        self.name == other.name &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SubmitSchedulerRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SubmitSchedulerResponse {
    // message fields
    okay: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl SubmitSchedulerResponse {
    pub fn new() -> SubmitSchedulerResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SubmitSchedulerResponse {
        static mut instance: ::protobuf::lazy::Lazy<SubmitSchedulerResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SubmitSchedulerResponse,
        };
        unsafe {
            instance.get(|| {
                SubmitSchedulerResponse {
                    okay: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bool okay = 1;

    pub fn clear_okay(&mut self) {
        self.okay = ::std::option::Option::None;
    }

    pub fn has_okay(&self) -> bool {
        self.okay.is_some()
    }

    // Param is passed by value, moved
    pub fn set_okay(&mut self, v: bool) {
        self.okay = ::std::option::Option::Some(v);
    }

    pub fn get_okay<'a>(&self) -> bool {
        self.okay.unwrap_or(false)
    }
}

impl ::protobuf::Message for SubmitSchedulerResponse {
    fn is_initialized(&self) -> bool {
        if self.okay.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.okay = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.okay.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.okay {
            try!(os.write_bool(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SubmitSchedulerResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SubmitSchedulerResponse {
    fn new() -> SubmitSchedulerResponse {
        SubmitSchedulerResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<SubmitSchedulerResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "okay",
                    SubmitSchedulerResponse::has_okay,
                    SubmitSchedulerResponse::get_okay,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SubmitSchedulerResponse>(
                    "SubmitSchedulerResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SubmitSchedulerResponse {
    fn clear(&mut self) {
        self.clear_okay();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SubmitSchedulerResponse {
    fn eq(&self, other: &SubmitSchedulerResponse) -> bool {
        self.okay == other.okay &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SubmitSchedulerResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ExecutorToFrameworkMessage {
    // message fields
    slave_id: ::protobuf::SingularPtrField<SlaveID>,
    framework_id: ::protobuf::SingularPtrField<FrameworkID>,
    executor_id: ::protobuf::SingularPtrField<ExecutorID>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ExecutorToFrameworkMessage {
    pub fn new() -> ExecutorToFrameworkMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExecutorToFrameworkMessage {
        static mut instance: ::protobuf::lazy::Lazy<ExecutorToFrameworkMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExecutorToFrameworkMessage,
        };
        unsafe {
            instance.get(|| {
                ExecutorToFrameworkMessage {
                    slave_id: ::protobuf::SingularPtrField::none(),
                    framework_id: ::protobuf::SingularPtrField::none(),
                    executor_id: ::protobuf::SingularPtrField::none(),
                    data: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .mesos.SlaveID slave_id = 1;

    pub fn clear_slave_id(&mut self) {
        self.slave_id.clear();
    }

    pub fn has_slave_id(&self) -> bool {
        self.slave_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slave_id(&mut self, v: SlaveID) {
        self.slave_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_slave_id<'a>(&'a mut self) -> &'a mut SlaveID {
        if self.slave_id.is_none() {
            self.slave_id.set_default();
        };
        self.slave_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_slave_id(&mut self) -> SlaveID {
        self.slave_id.take().unwrap_or_else(|| SlaveID::new())
    }

    pub fn get_slave_id<'a>(&'a self) -> &'a SlaveID {
        self.slave_id.as_ref().unwrap_or_else(|| SlaveID::default_instance())
    }

    // required .mesos.FrameworkID framework_id = 2;

    pub fn clear_framework_id(&mut self) {
        self.framework_id.clear();
    }

    pub fn has_framework_id(&self) -> bool {
        self.framework_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framework_id(&mut self, v: FrameworkID) {
        self.framework_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_framework_id<'a>(&'a mut self) -> &'a mut FrameworkID {
        if self.framework_id.is_none() {
            self.framework_id.set_default();
        };
        self.framework_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_framework_id(&mut self) -> FrameworkID {
        self.framework_id.take().unwrap_or_else(|| FrameworkID::new())
    }

    pub fn get_framework_id<'a>(&'a self) -> &'a FrameworkID {
        self.framework_id.as_ref().unwrap_or_else(|| FrameworkID::default_instance())
    }

    // required .mesos.ExecutorID executor_id = 3;

    pub fn clear_executor_id(&mut self) {
        self.executor_id.clear();
    }

    pub fn has_executor_id(&self) -> bool {
        self.executor_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_executor_id(&mut self, v: ExecutorID) {
        self.executor_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_executor_id<'a>(&'a mut self) -> &'a mut ExecutorID {
        if self.executor_id.is_none() {
            self.executor_id.set_default();
        };
        self.executor_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_executor_id(&mut self) -> ExecutorID {
        self.executor_id.take().unwrap_or_else(|| ExecutorID::new())
    }

    pub fn get_executor_id<'a>(&'a self) -> &'a ExecutorID {
        self.executor_id.as_ref().unwrap_or_else(|| ExecutorID::default_instance())
    }

    // required bytes data = 4;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        };
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_data<'a>(&'a self) -> &'a [u8] {
        match self.data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for ExecutorToFrameworkMessage {
    fn is_initialized(&self) -> bool {
        if self.slave_id.is_none() {
            return false;
        };
        if self.framework_id.is_none() {
            return false;
        };
        if self.executor_id.is_none() {
            return false;
        };
        if self.data.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.slave_id.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.framework_id.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.executor_id.set_default();
                    try!(is.merge_message(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.data.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.slave_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.framework_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.executor_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.data.iter() {
            my_size += ::protobuf::rt::bytes_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.slave_id.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.framework_id.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.executor_id.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.data.as_ref() {
            try!(os.write_bytes(4, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ExecutorToFrameworkMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ExecutorToFrameworkMessage {
    fn new() -> ExecutorToFrameworkMessage {
        ExecutorToFrameworkMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExecutorToFrameworkMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "slave_id",
                    ExecutorToFrameworkMessage::has_slave_id,
                    ExecutorToFrameworkMessage::get_slave_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "framework_id",
                    ExecutorToFrameworkMessage::has_framework_id,
                    ExecutorToFrameworkMessage::get_framework_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "executor_id",
                    ExecutorToFrameworkMessage::has_executor_id,
                    ExecutorToFrameworkMessage::get_executor_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "data",
                    ExecutorToFrameworkMessage::has_data,
                    ExecutorToFrameworkMessage::get_data,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExecutorToFrameworkMessage>(
                    "ExecutorToFrameworkMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExecutorToFrameworkMessage {
    fn clear(&mut self) {
        self.clear_slave_id();
        self.clear_framework_id();
        self.clear_executor_id();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ExecutorToFrameworkMessage {
    fn eq(&self, other: &ExecutorToFrameworkMessage) -> bool {
        self.slave_id == other.slave_id &&
        self.framework_id == other.framework_id &&
        self.executor_id == other.executor_id &&
        self.data == other.data &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ExecutorToFrameworkMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct FrameworkToExecutorMessage {
    // message fields
    slave_id: ::protobuf::SingularPtrField<SlaveID>,
    framework_id: ::protobuf::SingularPtrField<FrameworkID>,
    executor_id: ::protobuf::SingularPtrField<ExecutorID>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl FrameworkToExecutorMessage {
    pub fn new() -> FrameworkToExecutorMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FrameworkToExecutorMessage {
        static mut instance: ::protobuf::lazy::Lazy<FrameworkToExecutorMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FrameworkToExecutorMessage,
        };
        unsafe {
            instance.get(|| {
                FrameworkToExecutorMessage {
                    slave_id: ::protobuf::SingularPtrField::none(),
                    framework_id: ::protobuf::SingularPtrField::none(),
                    executor_id: ::protobuf::SingularPtrField::none(),
                    data: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .mesos.SlaveID slave_id = 1;

    pub fn clear_slave_id(&mut self) {
        self.slave_id.clear();
    }

    pub fn has_slave_id(&self) -> bool {
        self.slave_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slave_id(&mut self, v: SlaveID) {
        self.slave_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_slave_id<'a>(&'a mut self) -> &'a mut SlaveID {
        if self.slave_id.is_none() {
            self.slave_id.set_default();
        };
        self.slave_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_slave_id(&mut self) -> SlaveID {
        self.slave_id.take().unwrap_or_else(|| SlaveID::new())
    }

    pub fn get_slave_id<'a>(&'a self) -> &'a SlaveID {
        self.slave_id.as_ref().unwrap_or_else(|| SlaveID::default_instance())
    }

    // required .mesos.FrameworkID framework_id = 2;

    pub fn clear_framework_id(&mut self) {
        self.framework_id.clear();
    }

    pub fn has_framework_id(&self) -> bool {
        self.framework_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framework_id(&mut self, v: FrameworkID) {
        self.framework_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_framework_id<'a>(&'a mut self) -> &'a mut FrameworkID {
        if self.framework_id.is_none() {
            self.framework_id.set_default();
        };
        self.framework_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_framework_id(&mut self) -> FrameworkID {
        self.framework_id.take().unwrap_or_else(|| FrameworkID::new())
    }

    pub fn get_framework_id<'a>(&'a self) -> &'a FrameworkID {
        self.framework_id.as_ref().unwrap_or_else(|| FrameworkID::default_instance())
    }

    // required .mesos.ExecutorID executor_id = 3;

    pub fn clear_executor_id(&mut self) {
        self.executor_id.clear();
    }

    pub fn has_executor_id(&self) -> bool {
        self.executor_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_executor_id(&mut self, v: ExecutorID) {
        self.executor_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_executor_id<'a>(&'a mut self) -> &'a mut ExecutorID {
        if self.executor_id.is_none() {
            self.executor_id.set_default();
        };
        self.executor_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_executor_id(&mut self) -> ExecutorID {
        self.executor_id.take().unwrap_or_else(|| ExecutorID::new())
    }

    pub fn get_executor_id<'a>(&'a self) -> &'a ExecutorID {
        self.executor_id.as_ref().unwrap_or_else(|| ExecutorID::default_instance())
    }

    // required bytes data = 4;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        };
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_data<'a>(&'a self) -> &'a [u8] {
        match self.data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for FrameworkToExecutorMessage {
    fn is_initialized(&self) -> bool {
        if self.slave_id.is_none() {
            return false;
        };
        if self.framework_id.is_none() {
            return false;
        };
        if self.executor_id.is_none() {
            return false;
        };
        if self.data.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.slave_id.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.framework_id.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.executor_id.set_default();
                    try!(is.merge_message(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.data.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.slave_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.framework_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.executor_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.data.iter() {
            my_size += ::protobuf::rt::bytes_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.slave_id.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.framework_id.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.executor_id.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.data.as_ref() {
            try!(os.write_bytes(4, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<FrameworkToExecutorMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FrameworkToExecutorMessage {
    fn new() -> FrameworkToExecutorMessage {
        FrameworkToExecutorMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<FrameworkToExecutorMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "slave_id",
                    FrameworkToExecutorMessage::has_slave_id,
                    FrameworkToExecutorMessage::get_slave_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "framework_id",
                    FrameworkToExecutorMessage::has_framework_id,
                    FrameworkToExecutorMessage::get_framework_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "executor_id",
                    FrameworkToExecutorMessage::has_executor_id,
                    FrameworkToExecutorMessage::get_executor_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "data",
                    FrameworkToExecutorMessage::has_data,
                    FrameworkToExecutorMessage::get_data,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FrameworkToExecutorMessage>(
                    "FrameworkToExecutorMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FrameworkToExecutorMessage {
    fn clear(&mut self) {
        self.clear_slave_id();
        self.clear_framework_id();
        self.clear_executor_id();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FrameworkToExecutorMessage {
    fn eq(&self, other: &FrameworkToExecutorMessage) -> bool {
        self.slave_id == other.slave_id &&
        self.framework_id == other.framework_id &&
        self.executor_id == other.executor_id &&
        self.data == other.data &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FrameworkToExecutorMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RegisterFrameworkMessage {
    // message fields
    framework: ::protobuf::SingularPtrField<FrameworkInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl RegisterFrameworkMessage {
    pub fn new() -> RegisterFrameworkMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RegisterFrameworkMessage {
        static mut instance: ::protobuf::lazy::Lazy<RegisterFrameworkMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RegisterFrameworkMessage,
        };
        unsafe {
            instance.get(|| {
                RegisterFrameworkMessage {
                    framework: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .mesos.FrameworkInfo framework = 1;

    pub fn clear_framework(&mut self) {
        self.framework.clear();
    }

    pub fn has_framework(&self) -> bool {
        self.framework.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framework(&mut self, v: FrameworkInfo) {
        self.framework = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_framework<'a>(&'a mut self) -> &'a mut FrameworkInfo {
        if self.framework.is_none() {
            self.framework.set_default();
        };
        self.framework.as_mut().unwrap()
    }

    // Take field
    pub fn take_framework(&mut self) -> FrameworkInfo {
        self.framework.take().unwrap_or_else(|| FrameworkInfo::new())
    }

    pub fn get_framework<'a>(&'a self) -> &'a FrameworkInfo {
        self.framework.as_ref().unwrap_or_else(|| FrameworkInfo::default_instance())
    }
}

impl ::protobuf::Message for RegisterFrameworkMessage {
    fn is_initialized(&self) -> bool {
        if self.framework.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.framework.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.framework.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.framework.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<RegisterFrameworkMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RegisterFrameworkMessage {
    fn new() -> RegisterFrameworkMessage {
        RegisterFrameworkMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<RegisterFrameworkMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "framework",
                    RegisterFrameworkMessage::has_framework,
                    RegisterFrameworkMessage::get_framework,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RegisterFrameworkMessage>(
                    "RegisterFrameworkMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RegisterFrameworkMessage {
    fn clear(&mut self) {
        self.clear_framework();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RegisterFrameworkMessage {
    fn eq(&self, other: &RegisterFrameworkMessage) -> bool {
        self.framework == other.framework &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RegisterFrameworkMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ReregisterFrameworkMessage {
    // message fields
    framework: ::protobuf::SingularPtrField<FrameworkInfo>,
    failover: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ReregisterFrameworkMessage {
    pub fn new() -> ReregisterFrameworkMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReregisterFrameworkMessage {
        static mut instance: ::protobuf::lazy::Lazy<ReregisterFrameworkMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReregisterFrameworkMessage,
        };
        unsafe {
            instance.get(|| {
                ReregisterFrameworkMessage {
                    framework: ::protobuf::SingularPtrField::none(),
                    failover: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .mesos.FrameworkInfo framework = 2;

    pub fn clear_framework(&mut self) {
        self.framework.clear();
    }

    pub fn has_framework(&self) -> bool {
        self.framework.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framework(&mut self, v: FrameworkInfo) {
        self.framework = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_framework<'a>(&'a mut self) -> &'a mut FrameworkInfo {
        if self.framework.is_none() {
            self.framework.set_default();
        };
        self.framework.as_mut().unwrap()
    }

    // Take field
    pub fn take_framework(&mut self) -> FrameworkInfo {
        self.framework.take().unwrap_or_else(|| FrameworkInfo::new())
    }

    pub fn get_framework<'a>(&'a self) -> &'a FrameworkInfo {
        self.framework.as_ref().unwrap_or_else(|| FrameworkInfo::default_instance())
    }

    // required bool failover = 3;

    pub fn clear_failover(&mut self) {
        self.failover = ::std::option::Option::None;
    }

    pub fn has_failover(&self) -> bool {
        self.failover.is_some()
    }

    // Param is passed by value, moved
    pub fn set_failover(&mut self, v: bool) {
        self.failover = ::std::option::Option::Some(v);
    }

    pub fn get_failover<'a>(&self) -> bool {
        self.failover.unwrap_or(false)
    }
}

impl ::protobuf::Message for ReregisterFrameworkMessage {
    fn is_initialized(&self) -> bool {
        if self.framework.is_none() {
            return false;
        };
        if self.failover.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.framework.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.failover = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.framework.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.failover.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.framework.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.failover {
            try!(os.write_bool(3, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ReregisterFrameworkMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReregisterFrameworkMessage {
    fn new() -> ReregisterFrameworkMessage {
        ReregisterFrameworkMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReregisterFrameworkMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "framework",
                    ReregisterFrameworkMessage::has_framework,
                    ReregisterFrameworkMessage::get_framework,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "failover",
                    ReregisterFrameworkMessage::has_failover,
                    ReregisterFrameworkMessage::get_failover,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReregisterFrameworkMessage>(
                    "ReregisterFrameworkMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReregisterFrameworkMessage {
    fn clear(&mut self) {
        self.clear_framework();
        self.clear_failover();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ReregisterFrameworkMessage {
    fn eq(&self, other: &ReregisterFrameworkMessage) -> bool {
        self.framework == other.framework &&
        self.failover == other.failover &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ReregisterFrameworkMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct FrameworkRegisteredMessage {
    // message fields
    framework_id: ::protobuf::SingularPtrField<FrameworkID>,
    master_info: ::protobuf::SingularPtrField<MasterInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl FrameworkRegisteredMessage {
    pub fn new() -> FrameworkRegisteredMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FrameworkRegisteredMessage {
        static mut instance: ::protobuf::lazy::Lazy<FrameworkRegisteredMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FrameworkRegisteredMessage,
        };
        unsafe {
            instance.get(|| {
                FrameworkRegisteredMessage {
                    framework_id: ::protobuf::SingularPtrField::none(),
                    master_info: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .mesos.FrameworkID framework_id = 1;

    pub fn clear_framework_id(&mut self) {
        self.framework_id.clear();
    }

    pub fn has_framework_id(&self) -> bool {
        self.framework_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framework_id(&mut self, v: FrameworkID) {
        self.framework_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_framework_id<'a>(&'a mut self) -> &'a mut FrameworkID {
        if self.framework_id.is_none() {
            self.framework_id.set_default();
        };
        self.framework_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_framework_id(&mut self) -> FrameworkID {
        self.framework_id.take().unwrap_or_else(|| FrameworkID::new())
    }

    pub fn get_framework_id<'a>(&'a self) -> &'a FrameworkID {
        self.framework_id.as_ref().unwrap_or_else(|| FrameworkID::default_instance())
    }

    // required .mesos.MasterInfo master_info = 2;

    pub fn clear_master_info(&mut self) {
        self.master_info.clear();
    }

    pub fn has_master_info(&self) -> bool {
        self.master_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_master_info(&mut self, v: MasterInfo) {
        self.master_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_master_info<'a>(&'a mut self) -> &'a mut MasterInfo {
        if self.master_info.is_none() {
            self.master_info.set_default();
        };
        self.master_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_master_info(&mut self) -> MasterInfo {
        self.master_info.take().unwrap_or_else(|| MasterInfo::new())
    }

    pub fn get_master_info<'a>(&'a self) -> &'a MasterInfo {
        self.master_info.as_ref().unwrap_or_else(|| MasterInfo::default_instance())
    }
}

impl ::protobuf::Message for FrameworkRegisteredMessage {
    fn is_initialized(&self) -> bool {
        if self.framework_id.is_none() {
            return false;
        };
        if self.master_info.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.framework_id.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.master_info.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.framework_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.master_info.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.framework_id.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.master_info.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<FrameworkRegisteredMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FrameworkRegisteredMessage {
    fn new() -> FrameworkRegisteredMessage {
        FrameworkRegisteredMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<FrameworkRegisteredMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "framework_id",
                    FrameworkRegisteredMessage::has_framework_id,
                    FrameworkRegisteredMessage::get_framework_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "master_info",
                    FrameworkRegisteredMessage::has_master_info,
                    FrameworkRegisteredMessage::get_master_info,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FrameworkRegisteredMessage>(
                    "FrameworkRegisteredMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FrameworkRegisteredMessage {
    fn clear(&mut self) {
        self.clear_framework_id();
        self.clear_master_info();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FrameworkRegisteredMessage {
    fn eq(&self, other: &FrameworkRegisteredMessage) -> bool {
        self.framework_id == other.framework_id &&
        self.master_info == other.master_info &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FrameworkRegisteredMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct FrameworkReregisteredMessage {
    // message fields
    framework_id: ::protobuf::SingularPtrField<FrameworkID>,
    master_info: ::protobuf::SingularPtrField<MasterInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl FrameworkReregisteredMessage {
    pub fn new() -> FrameworkReregisteredMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FrameworkReregisteredMessage {
        static mut instance: ::protobuf::lazy::Lazy<FrameworkReregisteredMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FrameworkReregisteredMessage,
        };
        unsafe {
            instance.get(|| {
                FrameworkReregisteredMessage {
                    framework_id: ::protobuf::SingularPtrField::none(),
                    master_info: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .mesos.FrameworkID framework_id = 1;

    pub fn clear_framework_id(&mut self) {
        self.framework_id.clear();
    }

    pub fn has_framework_id(&self) -> bool {
        self.framework_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framework_id(&mut self, v: FrameworkID) {
        self.framework_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_framework_id<'a>(&'a mut self) -> &'a mut FrameworkID {
        if self.framework_id.is_none() {
            self.framework_id.set_default();
        };
        self.framework_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_framework_id(&mut self) -> FrameworkID {
        self.framework_id.take().unwrap_or_else(|| FrameworkID::new())
    }

    pub fn get_framework_id<'a>(&'a self) -> &'a FrameworkID {
        self.framework_id.as_ref().unwrap_or_else(|| FrameworkID::default_instance())
    }

    // required .mesos.MasterInfo master_info = 2;

    pub fn clear_master_info(&mut self) {
        self.master_info.clear();
    }

    pub fn has_master_info(&self) -> bool {
        self.master_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_master_info(&mut self, v: MasterInfo) {
        self.master_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_master_info<'a>(&'a mut self) -> &'a mut MasterInfo {
        if self.master_info.is_none() {
            self.master_info.set_default();
        };
        self.master_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_master_info(&mut self) -> MasterInfo {
        self.master_info.take().unwrap_or_else(|| MasterInfo::new())
    }

    pub fn get_master_info<'a>(&'a self) -> &'a MasterInfo {
        self.master_info.as_ref().unwrap_or_else(|| MasterInfo::default_instance())
    }
}

impl ::protobuf::Message for FrameworkReregisteredMessage {
    fn is_initialized(&self) -> bool {
        if self.framework_id.is_none() {
            return false;
        };
        if self.master_info.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.framework_id.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.master_info.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.framework_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.master_info.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.framework_id.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.master_info.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<FrameworkReregisteredMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FrameworkReregisteredMessage {
    fn new() -> FrameworkReregisteredMessage {
        FrameworkReregisteredMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<FrameworkReregisteredMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "framework_id",
                    FrameworkReregisteredMessage::has_framework_id,
                    FrameworkReregisteredMessage::get_framework_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "master_info",
                    FrameworkReregisteredMessage::has_master_info,
                    FrameworkReregisteredMessage::get_master_info,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FrameworkReregisteredMessage>(
                    "FrameworkReregisteredMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FrameworkReregisteredMessage {
    fn clear(&mut self) {
        self.clear_framework_id();
        self.clear_master_info();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FrameworkReregisteredMessage {
    fn eq(&self, other: &FrameworkReregisteredMessage) -> bool {
        self.framework_id == other.framework_id &&
        self.master_info == other.master_info &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FrameworkReregisteredMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct UnregisterFrameworkMessage {
    // message fields
    framework_id: ::protobuf::SingularPtrField<FrameworkID>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl UnregisterFrameworkMessage {
    pub fn new() -> UnregisterFrameworkMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UnregisterFrameworkMessage {
        static mut instance: ::protobuf::lazy::Lazy<UnregisterFrameworkMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UnregisterFrameworkMessage,
        };
        unsafe {
            instance.get(|| {
                UnregisterFrameworkMessage {
                    framework_id: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .mesos.FrameworkID framework_id = 1;

    pub fn clear_framework_id(&mut self) {
        self.framework_id.clear();
    }

    pub fn has_framework_id(&self) -> bool {
        self.framework_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framework_id(&mut self, v: FrameworkID) {
        self.framework_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_framework_id<'a>(&'a mut self) -> &'a mut FrameworkID {
        if self.framework_id.is_none() {
            self.framework_id.set_default();
        };
        self.framework_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_framework_id(&mut self) -> FrameworkID {
        self.framework_id.take().unwrap_or_else(|| FrameworkID::new())
    }

    pub fn get_framework_id<'a>(&'a self) -> &'a FrameworkID {
        self.framework_id.as_ref().unwrap_or_else(|| FrameworkID::default_instance())
    }
}

impl ::protobuf::Message for UnregisterFrameworkMessage {
    fn is_initialized(&self) -> bool {
        if self.framework_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.framework_id.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.framework_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.framework_id.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<UnregisterFrameworkMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UnregisterFrameworkMessage {
    fn new() -> UnregisterFrameworkMessage {
        UnregisterFrameworkMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<UnregisterFrameworkMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "framework_id",
                    UnregisterFrameworkMessage::has_framework_id,
                    UnregisterFrameworkMessage::get_framework_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UnregisterFrameworkMessage>(
                    "UnregisterFrameworkMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UnregisterFrameworkMessage {
    fn clear(&mut self) {
        self.clear_framework_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for UnregisterFrameworkMessage {
    fn eq(&self, other: &UnregisterFrameworkMessage) -> bool {
        self.framework_id == other.framework_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for UnregisterFrameworkMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DeactivateFrameworkMessage {
    // message fields
    framework_id: ::protobuf::SingularPtrField<FrameworkID>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl DeactivateFrameworkMessage {
    pub fn new() -> DeactivateFrameworkMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeactivateFrameworkMessage {
        static mut instance: ::protobuf::lazy::Lazy<DeactivateFrameworkMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeactivateFrameworkMessage,
        };
        unsafe {
            instance.get(|| {
                DeactivateFrameworkMessage {
                    framework_id: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .mesos.FrameworkID framework_id = 1;

    pub fn clear_framework_id(&mut self) {
        self.framework_id.clear();
    }

    pub fn has_framework_id(&self) -> bool {
        self.framework_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framework_id(&mut self, v: FrameworkID) {
        self.framework_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_framework_id<'a>(&'a mut self) -> &'a mut FrameworkID {
        if self.framework_id.is_none() {
            self.framework_id.set_default();
        };
        self.framework_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_framework_id(&mut self) -> FrameworkID {
        self.framework_id.take().unwrap_or_else(|| FrameworkID::new())
    }

    pub fn get_framework_id<'a>(&'a self) -> &'a FrameworkID {
        self.framework_id.as_ref().unwrap_or_else(|| FrameworkID::default_instance())
    }
}

impl ::protobuf::Message for DeactivateFrameworkMessage {
    fn is_initialized(&self) -> bool {
        if self.framework_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.framework_id.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.framework_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.framework_id.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<DeactivateFrameworkMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DeactivateFrameworkMessage {
    fn new() -> DeactivateFrameworkMessage {
        DeactivateFrameworkMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeactivateFrameworkMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "framework_id",
                    DeactivateFrameworkMessage::has_framework_id,
                    DeactivateFrameworkMessage::get_framework_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeactivateFrameworkMessage>(
                    "DeactivateFrameworkMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeactivateFrameworkMessage {
    fn clear(&mut self) {
        self.clear_framework_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DeactivateFrameworkMessage {
    fn eq(&self, other: &DeactivateFrameworkMessage) -> bool {
        self.framework_id == other.framework_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DeactivateFrameworkMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ResourceRequestMessage {
    // message fields
    framework_id: ::protobuf::SingularPtrField<FrameworkID>,
    requests: ::protobuf::RepeatedField<Request>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ResourceRequestMessage {
    pub fn new() -> ResourceRequestMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResourceRequestMessage {
        static mut instance: ::protobuf::lazy::Lazy<ResourceRequestMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResourceRequestMessage,
        };
        unsafe {
            instance.get(|| {
                ResourceRequestMessage {
                    framework_id: ::protobuf::SingularPtrField::none(),
                    requests: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .mesos.FrameworkID framework_id = 1;

    pub fn clear_framework_id(&mut self) {
        self.framework_id.clear();
    }

    pub fn has_framework_id(&self) -> bool {
        self.framework_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framework_id(&mut self, v: FrameworkID) {
        self.framework_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_framework_id<'a>(&'a mut self) -> &'a mut FrameworkID {
        if self.framework_id.is_none() {
            self.framework_id.set_default();
        };
        self.framework_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_framework_id(&mut self) -> FrameworkID {
        self.framework_id.take().unwrap_or_else(|| FrameworkID::new())
    }

    pub fn get_framework_id<'a>(&'a self) -> &'a FrameworkID {
        self.framework_id.as_ref().unwrap_or_else(|| FrameworkID::default_instance())
    }

    // repeated .mesos.Request requests = 2;

    pub fn clear_requests(&mut self) {
        self.requests.clear();
    }

    // Param is passed by value, moved
    pub fn set_requests(&mut self, v: ::protobuf::RepeatedField<Request>) {
        self.requests = v;
    }

    // Mutable pointer to the field.
    pub fn mut_requests<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Request> {
        &mut self.requests
    }

    // Take field
    pub fn take_requests(&mut self) -> ::protobuf::RepeatedField<Request> {
        ::std::mem::replace(&mut self.requests, ::protobuf::RepeatedField::new())
    }

    pub fn get_requests<'a>(&'a self) -> &'a [Request] {
        &self.requests
    }
}

impl ::protobuf::Message for ResourceRequestMessage {
    fn is_initialized(&self) -> bool {
        if self.framework_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.framework_id.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.requests));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.framework_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.requests.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.framework_id.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.requests.iter() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ResourceRequestMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ResourceRequestMessage {
    fn new() -> ResourceRequestMessage {
        ResourceRequestMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResourceRequestMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "framework_id",
                    ResourceRequestMessage::has_framework_id,
                    ResourceRequestMessage::get_framework_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "requests",
                    ResourceRequestMessage::get_requests,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResourceRequestMessage>(
                    "ResourceRequestMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResourceRequestMessage {
    fn clear(&mut self) {
        self.clear_framework_id();
        self.clear_requests();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ResourceRequestMessage {
    fn eq(&self, other: &ResourceRequestMessage) -> bool {
        self.framework_id == other.framework_id &&
        self.requests == other.requests &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ResourceRequestMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ResourceOffersMessage {
    // message fields
    offers: ::protobuf::RepeatedField<Offer>,
    pids: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ResourceOffersMessage {
    pub fn new() -> ResourceOffersMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResourceOffersMessage {
        static mut instance: ::protobuf::lazy::Lazy<ResourceOffersMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResourceOffersMessage,
        };
        unsafe {
            instance.get(|| {
                ResourceOffersMessage {
                    offers: ::protobuf::RepeatedField::new(),
                    pids: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .mesos.Offer offers = 1;

    pub fn clear_offers(&mut self) {
        self.offers.clear();
    }

    // Param is passed by value, moved
    pub fn set_offers(&mut self, v: ::protobuf::RepeatedField<Offer>) {
        self.offers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_offers<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Offer> {
        &mut self.offers
    }

    // Take field
    pub fn take_offers(&mut self) -> ::protobuf::RepeatedField<Offer> {
        ::std::mem::replace(&mut self.offers, ::protobuf::RepeatedField::new())
    }

    pub fn get_offers<'a>(&'a self) -> &'a [Offer] {
        &self.offers
    }

    // repeated string pids = 2;

    pub fn clear_pids(&mut self) {
        self.pids.clear();
    }

    // Param is passed by value, moved
    pub fn set_pids(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.pids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_pids<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.pids
    }

    // Take field
    pub fn take_pids(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.pids, ::protobuf::RepeatedField::new())
    }

    pub fn get_pids<'a>(&'a self) -> &'a [::std::string::String] {
        &self.pids
    }
}

impl ::protobuf::Message for ResourceOffersMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.offers));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.pids));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.offers.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.pids.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.offers.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.pids.iter() {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ResourceOffersMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ResourceOffersMessage {
    fn new() -> ResourceOffersMessage {
        ResourceOffersMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResourceOffersMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "offers",
                    ResourceOffersMessage::get_offers,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "pids",
                    ResourceOffersMessage::get_pids,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResourceOffersMessage>(
                    "ResourceOffersMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResourceOffersMessage {
    fn clear(&mut self) {
        self.clear_offers();
        self.clear_pids();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ResourceOffersMessage {
    fn eq(&self, other: &ResourceOffersMessage) -> bool {
        self.offers == other.offers &&
        self.pids == other.pids &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ResourceOffersMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct LaunchTasksMessage {
    // message fields
    framework_id: ::protobuf::SingularPtrField<FrameworkID>,
    tasks: ::protobuf::RepeatedField<TaskInfo>,
    filters: ::protobuf::SingularPtrField<Filters>,
    offer_ids: ::protobuf::RepeatedField<OfferID>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl LaunchTasksMessage {
    pub fn new() -> LaunchTasksMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LaunchTasksMessage {
        static mut instance: ::protobuf::lazy::Lazy<LaunchTasksMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LaunchTasksMessage,
        };
        unsafe {
            instance.get(|| {
                LaunchTasksMessage {
                    framework_id: ::protobuf::SingularPtrField::none(),
                    tasks: ::protobuf::RepeatedField::new(),
                    filters: ::protobuf::SingularPtrField::none(),
                    offer_ids: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .mesos.FrameworkID framework_id = 1;

    pub fn clear_framework_id(&mut self) {
        self.framework_id.clear();
    }

    pub fn has_framework_id(&self) -> bool {
        self.framework_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framework_id(&mut self, v: FrameworkID) {
        self.framework_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_framework_id<'a>(&'a mut self) -> &'a mut FrameworkID {
        if self.framework_id.is_none() {
            self.framework_id.set_default();
        };
        self.framework_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_framework_id(&mut self) -> FrameworkID {
        self.framework_id.take().unwrap_or_else(|| FrameworkID::new())
    }

    pub fn get_framework_id<'a>(&'a self) -> &'a FrameworkID {
        self.framework_id.as_ref().unwrap_or_else(|| FrameworkID::default_instance())
    }

    // repeated .mesos.TaskInfo tasks = 3;

    pub fn clear_tasks(&mut self) {
        self.tasks.clear();
    }

    // Param is passed by value, moved
    pub fn set_tasks(&mut self, v: ::protobuf::RepeatedField<TaskInfo>) {
        self.tasks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tasks<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<TaskInfo> {
        &mut self.tasks
    }

    // Take field
    pub fn take_tasks(&mut self) -> ::protobuf::RepeatedField<TaskInfo> {
        ::std::mem::replace(&mut self.tasks, ::protobuf::RepeatedField::new())
    }

    pub fn get_tasks<'a>(&'a self) -> &'a [TaskInfo] {
        &self.tasks
    }

    // required .mesos.Filters filters = 5;

    pub fn clear_filters(&mut self) {
        self.filters.clear();
    }

    pub fn has_filters(&self) -> bool {
        self.filters.is_some()
    }

    // Param is passed by value, moved
    pub fn set_filters(&mut self, v: Filters) {
        self.filters = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_filters<'a>(&'a mut self) -> &'a mut Filters {
        if self.filters.is_none() {
            self.filters.set_default();
        };
        self.filters.as_mut().unwrap()
    }

    // Take field
    pub fn take_filters(&mut self) -> Filters {
        self.filters.take().unwrap_or_else(|| Filters::new())
    }

    pub fn get_filters<'a>(&'a self) -> &'a Filters {
        self.filters.as_ref().unwrap_or_else(|| Filters::default_instance())
    }

    // repeated .mesos.OfferID offer_ids = 6;

    pub fn clear_offer_ids(&mut self) {
        self.offer_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_offer_ids(&mut self, v: ::protobuf::RepeatedField<OfferID>) {
        self.offer_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_offer_ids<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<OfferID> {
        &mut self.offer_ids
    }

    // Take field
    pub fn take_offer_ids(&mut self) -> ::protobuf::RepeatedField<OfferID> {
        ::std::mem::replace(&mut self.offer_ids, ::protobuf::RepeatedField::new())
    }

    pub fn get_offer_ids<'a>(&'a self) -> &'a [OfferID] {
        &self.offer_ids
    }
}

impl ::protobuf::Message for LaunchTasksMessage {
    fn is_initialized(&self) -> bool {
        if self.framework_id.is_none() {
            return false;
        };
        if self.filters.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.framework_id.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tasks));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.filters.set_default();
                    try!(is.merge_message(tmp))
                },
                6 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.offer_ids));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.framework_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.tasks.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.filters.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.offer_ids.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.framework_id.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.tasks.iter() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.filters.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.offer_ids.iter() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<LaunchTasksMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LaunchTasksMessage {
    fn new() -> LaunchTasksMessage {
        LaunchTasksMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<LaunchTasksMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "framework_id",
                    LaunchTasksMessage::has_framework_id,
                    LaunchTasksMessage::get_framework_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "tasks",
                    LaunchTasksMessage::get_tasks,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "filters",
                    LaunchTasksMessage::has_filters,
                    LaunchTasksMessage::get_filters,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "offer_ids",
                    LaunchTasksMessage::get_offer_ids,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LaunchTasksMessage>(
                    "LaunchTasksMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LaunchTasksMessage {
    fn clear(&mut self) {
        self.clear_framework_id();
        self.clear_tasks();
        self.clear_filters();
        self.clear_offer_ids();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for LaunchTasksMessage {
    fn eq(&self, other: &LaunchTasksMessage) -> bool {
        self.framework_id == other.framework_id &&
        self.tasks == other.tasks &&
        self.filters == other.filters &&
        self.offer_ids == other.offer_ids &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for LaunchTasksMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RescindResourceOfferMessage {
    // message fields
    offer_id: ::protobuf::SingularPtrField<OfferID>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl RescindResourceOfferMessage {
    pub fn new() -> RescindResourceOfferMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RescindResourceOfferMessage {
        static mut instance: ::protobuf::lazy::Lazy<RescindResourceOfferMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RescindResourceOfferMessage,
        };
        unsafe {
            instance.get(|| {
                RescindResourceOfferMessage {
                    offer_id: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .mesos.OfferID offer_id = 1;

    pub fn clear_offer_id(&mut self) {
        self.offer_id.clear();
    }

    pub fn has_offer_id(&self) -> bool {
        self.offer_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_offer_id(&mut self, v: OfferID) {
        self.offer_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_offer_id<'a>(&'a mut self) -> &'a mut OfferID {
        if self.offer_id.is_none() {
            self.offer_id.set_default();
        };
        self.offer_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_offer_id(&mut self) -> OfferID {
        self.offer_id.take().unwrap_or_else(|| OfferID::new())
    }

    pub fn get_offer_id<'a>(&'a self) -> &'a OfferID {
        self.offer_id.as_ref().unwrap_or_else(|| OfferID::default_instance())
    }
}

impl ::protobuf::Message for RescindResourceOfferMessage {
    fn is_initialized(&self) -> bool {
        if self.offer_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.offer_id.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.offer_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.offer_id.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<RescindResourceOfferMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RescindResourceOfferMessage {
    fn new() -> RescindResourceOfferMessage {
        RescindResourceOfferMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<RescindResourceOfferMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "offer_id",
                    RescindResourceOfferMessage::has_offer_id,
                    RescindResourceOfferMessage::get_offer_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RescindResourceOfferMessage>(
                    "RescindResourceOfferMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RescindResourceOfferMessage {
    fn clear(&mut self) {
        self.clear_offer_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RescindResourceOfferMessage {
    fn eq(&self, other: &RescindResourceOfferMessage) -> bool {
        self.offer_id == other.offer_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RescindResourceOfferMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ReviveOffersMessage {
    // message fields
    framework_id: ::protobuf::SingularPtrField<FrameworkID>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ReviveOffersMessage {
    pub fn new() -> ReviveOffersMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReviveOffersMessage {
        static mut instance: ::protobuf::lazy::Lazy<ReviveOffersMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReviveOffersMessage,
        };
        unsafe {
            instance.get(|| {
                ReviveOffersMessage {
                    framework_id: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .mesos.FrameworkID framework_id = 1;

    pub fn clear_framework_id(&mut self) {
        self.framework_id.clear();
    }

    pub fn has_framework_id(&self) -> bool {
        self.framework_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framework_id(&mut self, v: FrameworkID) {
        self.framework_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_framework_id<'a>(&'a mut self) -> &'a mut FrameworkID {
        if self.framework_id.is_none() {
            self.framework_id.set_default();
        };
        self.framework_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_framework_id(&mut self) -> FrameworkID {
        self.framework_id.take().unwrap_or_else(|| FrameworkID::new())
    }

    pub fn get_framework_id<'a>(&'a self) -> &'a FrameworkID {
        self.framework_id.as_ref().unwrap_or_else(|| FrameworkID::default_instance())
    }
}

impl ::protobuf::Message for ReviveOffersMessage {
    fn is_initialized(&self) -> bool {
        if self.framework_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.framework_id.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.framework_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.framework_id.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ReviveOffersMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReviveOffersMessage {
    fn new() -> ReviveOffersMessage {
        ReviveOffersMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReviveOffersMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "framework_id",
                    ReviveOffersMessage::has_framework_id,
                    ReviveOffersMessage::get_framework_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReviveOffersMessage>(
                    "ReviveOffersMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReviveOffersMessage {
    fn clear(&mut self) {
        self.clear_framework_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ReviveOffersMessage {
    fn eq(&self, other: &ReviveOffersMessage) -> bool {
        self.framework_id == other.framework_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ReviveOffersMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RunTaskMessage {
    // message fields
    framework_id: ::protobuf::SingularPtrField<FrameworkID>,
    framework: ::protobuf::SingularPtrField<FrameworkInfo>,
    pid: ::protobuf::SingularField<::std::string::String>,
    task: ::protobuf::SingularPtrField<TaskInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl RunTaskMessage {
    pub fn new() -> RunTaskMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RunTaskMessage {
        static mut instance: ::protobuf::lazy::Lazy<RunTaskMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RunTaskMessage,
        };
        unsafe {
            instance.get(|| {
                RunTaskMessage {
                    framework_id: ::protobuf::SingularPtrField::none(),
                    framework: ::protobuf::SingularPtrField::none(),
                    pid: ::protobuf::SingularField::none(),
                    task: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .mesos.FrameworkID framework_id = 1;

    pub fn clear_framework_id(&mut self) {
        self.framework_id.clear();
    }

    pub fn has_framework_id(&self) -> bool {
        self.framework_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framework_id(&mut self, v: FrameworkID) {
        self.framework_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_framework_id<'a>(&'a mut self) -> &'a mut FrameworkID {
        if self.framework_id.is_none() {
            self.framework_id.set_default();
        };
        self.framework_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_framework_id(&mut self) -> FrameworkID {
        self.framework_id.take().unwrap_or_else(|| FrameworkID::new())
    }

    pub fn get_framework_id<'a>(&'a self) -> &'a FrameworkID {
        self.framework_id.as_ref().unwrap_or_else(|| FrameworkID::default_instance())
    }

    // required .mesos.FrameworkInfo framework = 2;

    pub fn clear_framework(&mut self) {
        self.framework.clear();
    }

    pub fn has_framework(&self) -> bool {
        self.framework.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framework(&mut self, v: FrameworkInfo) {
        self.framework = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_framework<'a>(&'a mut self) -> &'a mut FrameworkInfo {
        if self.framework.is_none() {
            self.framework.set_default();
        };
        self.framework.as_mut().unwrap()
    }

    // Take field
    pub fn take_framework(&mut self) -> FrameworkInfo {
        self.framework.take().unwrap_or_else(|| FrameworkInfo::new())
    }

    pub fn get_framework<'a>(&'a self) -> &'a FrameworkInfo {
        self.framework.as_ref().unwrap_or_else(|| FrameworkInfo::default_instance())
    }

    // required string pid = 3;

    pub fn clear_pid(&mut self) {
        self.pid.clear();
    }

    pub fn has_pid(&self) -> bool {
        self.pid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pid(&mut self, v: ::std::string::String) {
        self.pid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pid<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.pid.is_none() {
            self.pid.set_default();
        };
        self.pid.as_mut().unwrap()
    }

    // Take field
    pub fn take_pid(&mut self) -> ::std::string::String {
        self.pid.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_pid<'a>(&'a self) -> &'a str {
        match self.pid.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required .mesos.TaskInfo task = 4;

    pub fn clear_task(&mut self) {
        self.task.clear();
    }

    pub fn has_task(&self) -> bool {
        self.task.is_some()
    }

    // Param is passed by value, moved
    pub fn set_task(&mut self, v: TaskInfo) {
        self.task = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_task<'a>(&'a mut self) -> &'a mut TaskInfo {
        if self.task.is_none() {
            self.task.set_default();
        };
        self.task.as_mut().unwrap()
    }

    // Take field
    pub fn take_task(&mut self) -> TaskInfo {
        self.task.take().unwrap_or_else(|| TaskInfo::new())
    }

    pub fn get_task<'a>(&'a self) -> &'a TaskInfo {
        self.task.as_ref().unwrap_or_else(|| TaskInfo::default_instance())
    }
}

impl ::protobuf::Message for RunTaskMessage {
    fn is_initialized(&self) -> bool {
        if self.framework.is_none() {
            return false;
        };
        if self.pid.is_none() {
            return false;
        };
        if self.task.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.framework_id.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.framework.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.pid.set_default();
                    try!(is.read_string_into(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.task.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.framework_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.framework.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.pid.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in self.task.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.framework_id.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.framework.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.pid.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.task.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<RunTaskMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RunTaskMessage {
    fn new() -> RunTaskMessage {
        RunTaskMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<RunTaskMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "framework_id",
                    RunTaskMessage::has_framework_id,
                    RunTaskMessage::get_framework_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "framework",
                    RunTaskMessage::has_framework,
                    RunTaskMessage::get_framework,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "pid",
                    RunTaskMessage::has_pid,
                    RunTaskMessage::get_pid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "task",
                    RunTaskMessage::has_task,
                    RunTaskMessage::get_task,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RunTaskMessage>(
                    "RunTaskMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RunTaskMessage {
    fn clear(&mut self) {
        self.clear_framework_id();
        self.clear_framework();
        self.clear_pid();
        self.clear_task();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RunTaskMessage {
    fn eq(&self, other: &RunTaskMessage) -> bool {
        self.framework_id == other.framework_id &&
        self.framework == other.framework &&
        self.pid == other.pid &&
        self.task == other.task &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RunTaskMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct KillTaskMessage {
    // message fields
    framework_id: ::protobuf::SingularPtrField<FrameworkID>,
    task_id: ::protobuf::SingularPtrField<TaskID>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl KillTaskMessage {
    pub fn new() -> KillTaskMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static KillTaskMessage {
        static mut instance: ::protobuf::lazy::Lazy<KillTaskMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const KillTaskMessage,
        };
        unsafe {
            instance.get(|| {
                KillTaskMessage {
                    framework_id: ::protobuf::SingularPtrField::none(),
                    task_id: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .mesos.FrameworkID framework_id = 1;

    pub fn clear_framework_id(&mut self) {
        self.framework_id.clear();
    }

    pub fn has_framework_id(&self) -> bool {
        self.framework_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framework_id(&mut self, v: FrameworkID) {
        self.framework_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_framework_id<'a>(&'a mut self) -> &'a mut FrameworkID {
        if self.framework_id.is_none() {
            self.framework_id.set_default();
        };
        self.framework_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_framework_id(&mut self) -> FrameworkID {
        self.framework_id.take().unwrap_or_else(|| FrameworkID::new())
    }

    pub fn get_framework_id<'a>(&'a self) -> &'a FrameworkID {
        self.framework_id.as_ref().unwrap_or_else(|| FrameworkID::default_instance())
    }

    // required .mesos.TaskID task_id = 2;

    pub fn clear_task_id(&mut self) {
        self.task_id.clear();
    }

    pub fn has_task_id(&self) -> bool {
        self.task_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_task_id(&mut self, v: TaskID) {
        self.task_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_task_id<'a>(&'a mut self) -> &'a mut TaskID {
        if self.task_id.is_none() {
            self.task_id.set_default();
        };
        self.task_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_task_id(&mut self) -> TaskID {
        self.task_id.take().unwrap_or_else(|| TaskID::new())
    }

    pub fn get_task_id<'a>(&'a self) -> &'a TaskID {
        self.task_id.as_ref().unwrap_or_else(|| TaskID::default_instance())
    }
}

impl ::protobuf::Message for KillTaskMessage {
    fn is_initialized(&self) -> bool {
        if self.framework_id.is_none() {
            return false;
        };
        if self.task_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.framework_id.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.task_id.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.framework_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.task_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.framework_id.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.task_id.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<KillTaskMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for KillTaskMessage {
    fn new() -> KillTaskMessage {
        KillTaskMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<KillTaskMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "framework_id",
                    KillTaskMessage::has_framework_id,
                    KillTaskMessage::get_framework_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "task_id",
                    KillTaskMessage::has_task_id,
                    KillTaskMessage::get_task_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<KillTaskMessage>(
                    "KillTaskMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for KillTaskMessage {
    fn clear(&mut self) {
        self.clear_framework_id();
        self.clear_task_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for KillTaskMessage {
    fn eq(&self, other: &KillTaskMessage) -> bool {
        self.framework_id == other.framework_id &&
        self.task_id == other.task_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for KillTaskMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StatusUpdateMessage {
    // message fields
    update: ::protobuf::SingularPtrField<StatusUpdate>,
    pid: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl StatusUpdateMessage {
    pub fn new() -> StatusUpdateMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StatusUpdateMessage {
        static mut instance: ::protobuf::lazy::Lazy<StatusUpdateMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StatusUpdateMessage,
        };
        unsafe {
            instance.get(|| {
                StatusUpdateMessage {
                    update: ::protobuf::SingularPtrField::none(),
                    pid: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .mesos.internal.StatusUpdate update = 1;

    pub fn clear_update(&mut self) {
        self.update.clear();
    }

    pub fn has_update(&self) -> bool {
        self.update.is_some()
    }

    // Param is passed by value, moved
    pub fn set_update(&mut self, v: StatusUpdate) {
        self.update = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_update<'a>(&'a mut self) -> &'a mut StatusUpdate {
        if self.update.is_none() {
            self.update.set_default();
        };
        self.update.as_mut().unwrap()
    }

    // Take field
    pub fn take_update(&mut self) -> StatusUpdate {
        self.update.take().unwrap_or_else(|| StatusUpdate::new())
    }

    pub fn get_update<'a>(&'a self) -> &'a StatusUpdate {
        self.update.as_ref().unwrap_or_else(|| StatusUpdate::default_instance())
    }

    // optional string pid = 2;

    pub fn clear_pid(&mut self) {
        self.pid.clear();
    }

    pub fn has_pid(&self) -> bool {
        self.pid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pid(&mut self, v: ::std::string::String) {
        self.pid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pid<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.pid.is_none() {
            self.pid.set_default();
        };
        self.pid.as_mut().unwrap()
    }

    // Take field
    pub fn take_pid(&mut self) -> ::std::string::String {
        self.pid.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_pid<'a>(&'a self) -> &'a str {
        match self.pid.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for StatusUpdateMessage {
    fn is_initialized(&self) -> bool {
        if self.update.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.update.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.pid.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.update.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.pid.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.update.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.pid.as_ref() {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<StatusUpdateMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StatusUpdateMessage {
    fn new() -> StatusUpdateMessage {
        StatusUpdateMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<StatusUpdateMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "update",
                    StatusUpdateMessage::has_update,
                    StatusUpdateMessage::get_update,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "pid",
                    StatusUpdateMessage::has_pid,
                    StatusUpdateMessage::get_pid,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StatusUpdateMessage>(
                    "StatusUpdateMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StatusUpdateMessage {
    fn clear(&mut self) {
        self.clear_update();
        self.clear_pid();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StatusUpdateMessage {
    fn eq(&self, other: &StatusUpdateMessage) -> bool {
        self.update == other.update &&
        self.pid == other.pid &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StatusUpdateMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StatusUpdateAcknowledgementMessage {
    // message fields
    slave_id: ::protobuf::SingularPtrField<SlaveID>,
    framework_id: ::protobuf::SingularPtrField<FrameworkID>,
    task_id: ::protobuf::SingularPtrField<TaskID>,
    uuid: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl StatusUpdateAcknowledgementMessage {
    pub fn new() -> StatusUpdateAcknowledgementMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StatusUpdateAcknowledgementMessage {
        static mut instance: ::protobuf::lazy::Lazy<StatusUpdateAcknowledgementMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StatusUpdateAcknowledgementMessage,
        };
        unsafe {
            instance.get(|| {
                StatusUpdateAcknowledgementMessage {
                    slave_id: ::protobuf::SingularPtrField::none(),
                    framework_id: ::protobuf::SingularPtrField::none(),
                    task_id: ::protobuf::SingularPtrField::none(),
                    uuid: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .mesos.SlaveID slave_id = 1;

    pub fn clear_slave_id(&mut self) {
        self.slave_id.clear();
    }

    pub fn has_slave_id(&self) -> bool {
        self.slave_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slave_id(&mut self, v: SlaveID) {
        self.slave_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_slave_id<'a>(&'a mut self) -> &'a mut SlaveID {
        if self.slave_id.is_none() {
            self.slave_id.set_default();
        };
        self.slave_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_slave_id(&mut self) -> SlaveID {
        self.slave_id.take().unwrap_or_else(|| SlaveID::new())
    }

    pub fn get_slave_id<'a>(&'a self) -> &'a SlaveID {
        self.slave_id.as_ref().unwrap_or_else(|| SlaveID::default_instance())
    }

    // required .mesos.FrameworkID framework_id = 2;

    pub fn clear_framework_id(&mut self) {
        self.framework_id.clear();
    }

    pub fn has_framework_id(&self) -> bool {
        self.framework_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framework_id(&mut self, v: FrameworkID) {
        self.framework_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_framework_id<'a>(&'a mut self) -> &'a mut FrameworkID {
        if self.framework_id.is_none() {
            self.framework_id.set_default();
        };
        self.framework_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_framework_id(&mut self) -> FrameworkID {
        self.framework_id.take().unwrap_or_else(|| FrameworkID::new())
    }

    pub fn get_framework_id<'a>(&'a self) -> &'a FrameworkID {
        self.framework_id.as_ref().unwrap_or_else(|| FrameworkID::default_instance())
    }

    // required .mesos.TaskID task_id = 3;

    pub fn clear_task_id(&mut self) {
        self.task_id.clear();
    }

    pub fn has_task_id(&self) -> bool {
        self.task_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_task_id(&mut self, v: TaskID) {
        self.task_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_task_id<'a>(&'a mut self) -> &'a mut TaskID {
        if self.task_id.is_none() {
            self.task_id.set_default();
        };
        self.task_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_task_id(&mut self) -> TaskID {
        self.task_id.take().unwrap_or_else(|| TaskID::new())
    }

    pub fn get_task_id<'a>(&'a self) -> &'a TaskID {
        self.task_id.as_ref().unwrap_or_else(|| TaskID::default_instance())
    }

    // required bytes uuid = 4;

    pub fn clear_uuid(&mut self) {
        self.uuid.clear();
    }

    pub fn has_uuid(&self) -> bool {
        self.uuid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uuid(&mut self, v: ::std::vec::Vec<u8>) {
        self.uuid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uuid<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.uuid.is_none() {
            self.uuid.set_default();
        };
        self.uuid.as_mut().unwrap()
    }

    // Take field
    pub fn take_uuid(&mut self) -> ::std::vec::Vec<u8> {
        self.uuid.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_uuid<'a>(&'a self) -> &'a [u8] {
        match self.uuid.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for StatusUpdateAcknowledgementMessage {
    fn is_initialized(&self) -> bool {
        if self.slave_id.is_none() {
            return false;
        };
        if self.framework_id.is_none() {
            return false;
        };
        if self.task_id.is_none() {
            return false;
        };
        if self.uuid.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.slave_id.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.framework_id.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.task_id.set_default();
                    try!(is.merge_message(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.uuid.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.slave_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.framework_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.task_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.uuid.iter() {
            my_size += ::protobuf::rt::bytes_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.slave_id.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.framework_id.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.task_id.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.uuid.as_ref() {
            try!(os.write_bytes(4, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<StatusUpdateAcknowledgementMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StatusUpdateAcknowledgementMessage {
    fn new() -> StatusUpdateAcknowledgementMessage {
        StatusUpdateAcknowledgementMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<StatusUpdateAcknowledgementMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "slave_id",
                    StatusUpdateAcknowledgementMessage::has_slave_id,
                    StatusUpdateAcknowledgementMessage::get_slave_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "framework_id",
                    StatusUpdateAcknowledgementMessage::has_framework_id,
                    StatusUpdateAcknowledgementMessage::get_framework_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "task_id",
                    StatusUpdateAcknowledgementMessage::has_task_id,
                    StatusUpdateAcknowledgementMessage::get_task_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "uuid",
                    StatusUpdateAcknowledgementMessage::has_uuid,
                    StatusUpdateAcknowledgementMessage::get_uuid,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StatusUpdateAcknowledgementMessage>(
                    "StatusUpdateAcknowledgementMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StatusUpdateAcknowledgementMessage {
    fn clear(&mut self) {
        self.clear_slave_id();
        self.clear_framework_id();
        self.clear_task_id();
        self.clear_uuid();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StatusUpdateAcknowledgementMessage {
    fn eq(&self, other: &StatusUpdateAcknowledgementMessage) -> bool {
        self.slave_id == other.slave_id &&
        self.framework_id == other.framework_id &&
        self.task_id == other.task_id &&
        self.uuid == other.uuid &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StatusUpdateAcknowledgementMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct LostSlaveMessage {
    // message fields
    slave_id: ::protobuf::SingularPtrField<SlaveID>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl LostSlaveMessage {
    pub fn new() -> LostSlaveMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LostSlaveMessage {
        static mut instance: ::protobuf::lazy::Lazy<LostSlaveMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LostSlaveMessage,
        };
        unsafe {
            instance.get(|| {
                LostSlaveMessage {
                    slave_id: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .mesos.SlaveID slave_id = 1;

    pub fn clear_slave_id(&mut self) {
        self.slave_id.clear();
    }

    pub fn has_slave_id(&self) -> bool {
        self.slave_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slave_id(&mut self, v: SlaveID) {
        self.slave_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_slave_id<'a>(&'a mut self) -> &'a mut SlaveID {
        if self.slave_id.is_none() {
            self.slave_id.set_default();
        };
        self.slave_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_slave_id(&mut self) -> SlaveID {
        self.slave_id.take().unwrap_or_else(|| SlaveID::new())
    }

    pub fn get_slave_id<'a>(&'a self) -> &'a SlaveID {
        self.slave_id.as_ref().unwrap_or_else(|| SlaveID::default_instance())
    }
}

impl ::protobuf::Message for LostSlaveMessage {
    fn is_initialized(&self) -> bool {
        if self.slave_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.slave_id.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.slave_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.slave_id.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<LostSlaveMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LostSlaveMessage {
    fn new() -> LostSlaveMessage {
        LostSlaveMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<LostSlaveMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "slave_id",
                    LostSlaveMessage::has_slave_id,
                    LostSlaveMessage::get_slave_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LostSlaveMessage>(
                    "LostSlaveMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LostSlaveMessage {
    fn clear(&mut self) {
        self.clear_slave_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for LostSlaveMessage {
    fn eq(&self, other: &LostSlaveMessage) -> bool {
        self.slave_id == other.slave_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for LostSlaveMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ReconcileTasksMessage {
    // message fields
    framework_id: ::protobuf::SingularPtrField<FrameworkID>,
    statuses: ::protobuf::RepeatedField<TaskStatus>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ReconcileTasksMessage {
    pub fn new() -> ReconcileTasksMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReconcileTasksMessage {
        static mut instance: ::protobuf::lazy::Lazy<ReconcileTasksMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReconcileTasksMessage,
        };
        unsafe {
            instance.get(|| {
                ReconcileTasksMessage {
                    framework_id: ::protobuf::SingularPtrField::none(),
                    statuses: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .mesos.FrameworkID framework_id = 1;

    pub fn clear_framework_id(&mut self) {
        self.framework_id.clear();
    }

    pub fn has_framework_id(&self) -> bool {
        self.framework_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framework_id(&mut self, v: FrameworkID) {
        self.framework_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_framework_id<'a>(&'a mut self) -> &'a mut FrameworkID {
        if self.framework_id.is_none() {
            self.framework_id.set_default();
        };
        self.framework_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_framework_id(&mut self) -> FrameworkID {
        self.framework_id.take().unwrap_or_else(|| FrameworkID::new())
    }

    pub fn get_framework_id<'a>(&'a self) -> &'a FrameworkID {
        self.framework_id.as_ref().unwrap_or_else(|| FrameworkID::default_instance())
    }

    // repeated .mesos.TaskStatus statuses = 2;

    pub fn clear_statuses(&mut self) {
        self.statuses.clear();
    }

    // Param is passed by value, moved
    pub fn set_statuses(&mut self, v: ::protobuf::RepeatedField<TaskStatus>) {
        self.statuses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_statuses<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<TaskStatus> {
        &mut self.statuses
    }

    // Take field
    pub fn take_statuses(&mut self) -> ::protobuf::RepeatedField<TaskStatus> {
        ::std::mem::replace(&mut self.statuses, ::protobuf::RepeatedField::new())
    }

    pub fn get_statuses<'a>(&'a self) -> &'a [TaskStatus] {
        &self.statuses
    }
}

impl ::protobuf::Message for ReconcileTasksMessage {
    fn is_initialized(&self) -> bool {
        if self.framework_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.framework_id.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.statuses));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.framework_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.statuses.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.framework_id.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.statuses.iter() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ReconcileTasksMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReconcileTasksMessage {
    fn new() -> ReconcileTasksMessage {
        ReconcileTasksMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReconcileTasksMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "framework_id",
                    ReconcileTasksMessage::has_framework_id,
                    ReconcileTasksMessage::get_framework_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "statuses",
                    ReconcileTasksMessage::get_statuses,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReconcileTasksMessage>(
                    "ReconcileTasksMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReconcileTasksMessage {
    fn clear(&mut self) {
        self.clear_framework_id();
        self.clear_statuses();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ReconcileTasksMessage {
    fn eq(&self, other: &ReconcileTasksMessage) -> bool {
        self.framework_id == other.framework_id &&
        self.statuses == other.statuses &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ReconcileTasksMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct FrameworkErrorMessage {
    // message fields
    message: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl FrameworkErrorMessage {
    pub fn new() -> FrameworkErrorMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FrameworkErrorMessage {
        static mut instance: ::protobuf::lazy::Lazy<FrameworkErrorMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FrameworkErrorMessage,
        };
        unsafe {
            instance.get(|| {
                FrameworkErrorMessage {
                    message: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string message = 2;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.message.is_none() {
            self.message.set_default();
        };
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        self.message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_message<'a>(&'a self) -> &'a str {
        match self.message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for FrameworkErrorMessage {
    fn is_initialized(&self) -> bool {
        if self.message.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.message.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.message.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.message.as_ref() {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<FrameworkErrorMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FrameworkErrorMessage {
    fn new() -> FrameworkErrorMessage {
        FrameworkErrorMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<FrameworkErrorMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "message",
                    FrameworkErrorMessage::has_message,
                    FrameworkErrorMessage::get_message,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FrameworkErrorMessage>(
                    "FrameworkErrorMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FrameworkErrorMessage {
    fn clear(&mut self) {
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FrameworkErrorMessage {
    fn eq(&self, other: &FrameworkErrorMessage) -> bool {
        self.message == other.message &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FrameworkErrorMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RegisterSlaveMessage {
    // message fields
    slave: ::protobuf::SingularPtrField<SlaveInfo>,
    checkpointed_resources: ::protobuf::RepeatedField<Resource>,
    version: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl RegisterSlaveMessage {
    pub fn new() -> RegisterSlaveMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RegisterSlaveMessage {
        static mut instance: ::protobuf::lazy::Lazy<RegisterSlaveMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RegisterSlaveMessage,
        };
        unsafe {
            instance.get(|| {
                RegisterSlaveMessage {
                    slave: ::protobuf::SingularPtrField::none(),
                    checkpointed_resources: ::protobuf::RepeatedField::new(),
                    version: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .mesos.SlaveInfo slave = 1;

    pub fn clear_slave(&mut self) {
        self.slave.clear();
    }

    pub fn has_slave(&self) -> bool {
        self.slave.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slave(&mut self, v: SlaveInfo) {
        self.slave = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_slave<'a>(&'a mut self) -> &'a mut SlaveInfo {
        if self.slave.is_none() {
            self.slave.set_default();
        };
        self.slave.as_mut().unwrap()
    }

    // Take field
    pub fn take_slave(&mut self) -> SlaveInfo {
        self.slave.take().unwrap_or_else(|| SlaveInfo::new())
    }

    pub fn get_slave<'a>(&'a self) -> &'a SlaveInfo {
        self.slave.as_ref().unwrap_or_else(|| SlaveInfo::default_instance())
    }

    // repeated .mesos.Resource checkpointed_resources = 3;

    pub fn clear_checkpointed_resources(&mut self) {
        self.checkpointed_resources.clear();
    }

    // Param is passed by value, moved
    pub fn set_checkpointed_resources(&mut self, v: ::protobuf::RepeatedField<Resource>) {
        self.checkpointed_resources = v;
    }

    // Mutable pointer to the field.
    pub fn mut_checkpointed_resources<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Resource> {
        &mut self.checkpointed_resources
    }

    // Take field
    pub fn take_checkpointed_resources(&mut self) -> ::protobuf::RepeatedField<Resource> {
        ::std::mem::replace(&mut self.checkpointed_resources, ::protobuf::RepeatedField::new())
    }

    pub fn get_checkpointed_resources<'a>(&'a self) -> &'a [Resource] {
        &self.checkpointed_resources
    }

    // optional string version = 2;

    pub fn clear_version(&mut self) {
        self.version.clear();
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: ::std::string::String) {
        self.version = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.version.is_none() {
            self.version.set_default();
        };
        self.version.as_mut().unwrap()
    }

    // Take field
    pub fn take_version(&mut self) -> ::std::string::String {
        self.version.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_version<'a>(&'a self) -> &'a str {
        match self.version.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for RegisterSlaveMessage {
    fn is_initialized(&self) -> bool {
        if self.slave.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.slave.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.checkpointed_resources));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.version.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.slave.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.checkpointed_resources.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.version.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.slave.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.checkpointed_resources.iter() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.version.as_ref() {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<RegisterSlaveMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RegisterSlaveMessage {
    fn new() -> RegisterSlaveMessage {
        RegisterSlaveMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<RegisterSlaveMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "slave",
                    RegisterSlaveMessage::has_slave,
                    RegisterSlaveMessage::get_slave,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "checkpointed_resources",
                    RegisterSlaveMessage::get_checkpointed_resources,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "version",
                    RegisterSlaveMessage::has_version,
                    RegisterSlaveMessage::get_version,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RegisterSlaveMessage>(
                    "RegisterSlaveMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RegisterSlaveMessage {
    fn clear(&mut self) {
        self.clear_slave();
        self.clear_checkpointed_resources();
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RegisterSlaveMessage {
    fn eq(&self, other: &RegisterSlaveMessage) -> bool {
        self.slave == other.slave &&
        self.checkpointed_resources == other.checkpointed_resources &&
        self.version == other.version &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RegisterSlaveMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ReregisterSlaveMessage {
    // message fields
    slave: ::protobuf::SingularPtrField<SlaveInfo>,
    checkpointed_resources: ::protobuf::RepeatedField<Resource>,
    executor_infos: ::protobuf::RepeatedField<ExecutorInfo>,
    tasks: ::protobuf::RepeatedField<Task>,
    completed_frameworks: ::protobuf::RepeatedField<Archive_Framework>,
    version: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ReregisterSlaveMessage {
    pub fn new() -> ReregisterSlaveMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReregisterSlaveMessage {
        static mut instance: ::protobuf::lazy::Lazy<ReregisterSlaveMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReregisterSlaveMessage,
        };
        unsafe {
            instance.get(|| {
                ReregisterSlaveMessage {
                    slave: ::protobuf::SingularPtrField::none(),
                    checkpointed_resources: ::protobuf::RepeatedField::new(),
                    executor_infos: ::protobuf::RepeatedField::new(),
                    tasks: ::protobuf::RepeatedField::new(),
                    completed_frameworks: ::protobuf::RepeatedField::new(),
                    version: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .mesos.SlaveInfo slave = 2;

    pub fn clear_slave(&mut self) {
        self.slave.clear();
    }

    pub fn has_slave(&self) -> bool {
        self.slave.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slave(&mut self, v: SlaveInfo) {
        self.slave = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_slave<'a>(&'a mut self) -> &'a mut SlaveInfo {
        if self.slave.is_none() {
            self.slave.set_default();
        };
        self.slave.as_mut().unwrap()
    }

    // Take field
    pub fn take_slave(&mut self) -> SlaveInfo {
        self.slave.take().unwrap_or_else(|| SlaveInfo::new())
    }

    pub fn get_slave<'a>(&'a self) -> &'a SlaveInfo {
        self.slave.as_ref().unwrap_or_else(|| SlaveInfo::default_instance())
    }

    // repeated .mesos.Resource checkpointed_resources = 7;

    pub fn clear_checkpointed_resources(&mut self) {
        self.checkpointed_resources.clear();
    }

    // Param is passed by value, moved
    pub fn set_checkpointed_resources(&mut self, v: ::protobuf::RepeatedField<Resource>) {
        self.checkpointed_resources = v;
    }

    // Mutable pointer to the field.
    pub fn mut_checkpointed_resources<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Resource> {
        &mut self.checkpointed_resources
    }

    // Take field
    pub fn take_checkpointed_resources(&mut self) -> ::protobuf::RepeatedField<Resource> {
        ::std::mem::replace(&mut self.checkpointed_resources, ::protobuf::RepeatedField::new())
    }

    pub fn get_checkpointed_resources<'a>(&'a self) -> &'a [Resource] {
        &self.checkpointed_resources
    }

    // repeated .mesos.ExecutorInfo executor_infos = 4;

    pub fn clear_executor_infos(&mut self) {
        self.executor_infos.clear();
    }

    // Param is passed by value, moved
    pub fn set_executor_infos(&mut self, v: ::protobuf::RepeatedField<ExecutorInfo>) {
        self.executor_infos = v;
    }

    // Mutable pointer to the field.
    pub fn mut_executor_infos<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<ExecutorInfo> {
        &mut self.executor_infos
    }

    // Take field
    pub fn take_executor_infos(&mut self) -> ::protobuf::RepeatedField<ExecutorInfo> {
        ::std::mem::replace(&mut self.executor_infos, ::protobuf::RepeatedField::new())
    }

    pub fn get_executor_infos<'a>(&'a self) -> &'a [ExecutorInfo] {
        &self.executor_infos
    }

    // repeated .mesos.internal.Task tasks = 3;

    pub fn clear_tasks(&mut self) {
        self.tasks.clear();
    }

    // Param is passed by value, moved
    pub fn set_tasks(&mut self, v: ::protobuf::RepeatedField<Task>) {
        self.tasks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tasks<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Task> {
        &mut self.tasks
    }

    // Take field
    pub fn take_tasks(&mut self) -> ::protobuf::RepeatedField<Task> {
        ::std::mem::replace(&mut self.tasks, ::protobuf::RepeatedField::new())
    }

    pub fn get_tasks<'a>(&'a self) -> &'a [Task] {
        &self.tasks
    }

    // repeated .mesos.internal.Archive.Framework completed_frameworks = 5;

    pub fn clear_completed_frameworks(&mut self) {
        self.completed_frameworks.clear();
    }

    // Param is passed by value, moved
    pub fn set_completed_frameworks(&mut self, v: ::protobuf::RepeatedField<Archive_Framework>) {
        self.completed_frameworks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_completed_frameworks<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Archive_Framework> {
        &mut self.completed_frameworks
    }

    // Take field
    pub fn take_completed_frameworks(&mut self) -> ::protobuf::RepeatedField<Archive_Framework> {
        ::std::mem::replace(&mut self.completed_frameworks, ::protobuf::RepeatedField::new())
    }

    pub fn get_completed_frameworks<'a>(&'a self) -> &'a [Archive_Framework] {
        &self.completed_frameworks
    }

    // optional string version = 6;

    pub fn clear_version(&mut self) {
        self.version.clear();
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: ::std::string::String) {
        self.version = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.version.is_none() {
            self.version.set_default();
        };
        self.version.as_mut().unwrap()
    }

    // Take field
    pub fn take_version(&mut self) -> ::std::string::String {
        self.version.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_version<'a>(&'a self) -> &'a str {
        match self.version.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for ReregisterSlaveMessage {
    fn is_initialized(&self) -> bool {
        if self.slave.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.slave.set_default();
                    try!(is.merge_message(tmp))
                },
                7 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.checkpointed_resources));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.executor_infos));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tasks));
                },
                5 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.completed_frameworks));
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.version.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.slave.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.checkpointed_resources.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.executor_infos.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.tasks.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.completed_frameworks.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.version.iter() {
            my_size += ::protobuf::rt::string_size(6, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.slave.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.checkpointed_resources.iter() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.executor_infos.iter() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.tasks.iter() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.completed_frameworks.iter() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.version.as_ref() {
            try!(os.write_string(6, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ReregisterSlaveMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReregisterSlaveMessage {
    fn new() -> ReregisterSlaveMessage {
        ReregisterSlaveMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReregisterSlaveMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "slave",
                    ReregisterSlaveMessage::has_slave,
                    ReregisterSlaveMessage::get_slave,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "checkpointed_resources",
                    ReregisterSlaveMessage::get_checkpointed_resources,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "executor_infos",
                    ReregisterSlaveMessage::get_executor_infos,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "tasks",
                    ReregisterSlaveMessage::get_tasks,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "completed_frameworks",
                    ReregisterSlaveMessage::get_completed_frameworks,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "version",
                    ReregisterSlaveMessage::has_version,
                    ReregisterSlaveMessage::get_version,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReregisterSlaveMessage>(
                    "ReregisterSlaveMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReregisterSlaveMessage {
    fn clear(&mut self) {
        self.clear_slave();
        self.clear_checkpointed_resources();
        self.clear_executor_infos();
        self.clear_tasks();
        self.clear_completed_frameworks();
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ReregisterSlaveMessage {
    fn eq(&self, other: &ReregisterSlaveMessage) -> bool {
        self.slave == other.slave &&
        self.checkpointed_resources == other.checkpointed_resources &&
        self.executor_infos == other.executor_infos &&
        self.tasks == other.tasks &&
        self.completed_frameworks == other.completed_frameworks &&
        self.version == other.version &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ReregisterSlaveMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SlaveRegisteredMessage {
    // message fields
    slave_id: ::protobuf::SingularPtrField<SlaveID>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl SlaveRegisteredMessage {
    pub fn new() -> SlaveRegisteredMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SlaveRegisteredMessage {
        static mut instance: ::protobuf::lazy::Lazy<SlaveRegisteredMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SlaveRegisteredMessage,
        };
        unsafe {
            instance.get(|| {
                SlaveRegisteredMessage {
                    slave_id: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .mesos.SlaveID slave_id = 1;

    pub fn clear_slave_id(&mut self) {
        self.slave_id.clear();
    }

    pub fn has_slave_id(&self) -> bool {
        self.slave_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slave_id(&mut self, v: SlaveID) {
        self.slave_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_slave_id<'a>(&'a mut self) -> &'a mut SlaveID {
        if self.slave_id.is_none() {
            self.slave_id.set_default();
        };
        self.slave_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_slave_id(&mut self) -> SlaveID {
        self.slave_id.take().unwrap_or_else(|| SlaveID::new())
    }

    pub fn get_slave_id<'a>(&'a self) -> &'a SlaveID {
        self.slave_id.as_ref().unwrap_or_else(|| SlaveID::default_instance())
    }
}

impl ::protobuf::Message for SlaveRegisteredMessage {
    fn is_initialized(&self) -> bool {
        if self.slave_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.slave_id.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.slave_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.slave_id.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SlaveRegisteredMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SlaveRegisteredMessage {
    fn new() -> SlaveRegisteredMessage {
        SlaveRegisteredMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<SlaveRegisteredMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "slave_id",
                    SlaveRegisteredMessage::has_slave_id,
                    SlaveRegisteredMessage::get_slave_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SlaveRegisteredMessage>(
                    "SlaveRegisteredMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SlaveRegisteredMessage {
    fn clear(&mut self) {
        self.clear_slave_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SlaveRegisteredMessage {
    fn eq(&self, other: &SlaveRegisteredMessage) -> bool {
        self.slave_id == other.slave_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SlaveRegisteredMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SlaveReregisteredMessage {
    // message fields
    slave_id: ::protobuf::SingularPtrField<SlaveID>,
    reconciliations: ::protobuf::RepeatedField<ReconcileTasksMessage>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl SlaveReregisteredMessage {
    pub fn new() -> SlaveReregisteredMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SlaveReregisteredMessage {
        static mut instance: ::protobuf::lazy::Lazy<SlaveReregisteredMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SlaveReregisteredMessage,
        };
        unsafe {
            instance.get(|| {
                SlaveReregisteredMessage {
                    slave_id: ::protobuf::SingularPtrField::none(),
                    reconciliations: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .mesos.SlaveID slave_id = 1;

    pub fn clear_slave_id(&mut self) {
        self.slave_id.clear();
    }

    pub fn has_slave_id(&self) -> bool {
        self.slave_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slave_id(&mut self, v: SlaveID) {
        self.slave_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_slave_id<'a>(&'a mut self) -> &'a mut SlaveID {
        if self.slave_id.is_none() {
            self.slave_id.set_default();
        };
        self.slave_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_slave_id(&mut self) -> SlaveID {
        self.slave_id.take().unwrap_or_else(|| SlaveID::new())
    }

    pub fn get_slave_id<'a>(&'a self) -> &'a SlaveID {
        self.slave_id.as_ref().unwrap_or_else(|| SlaveID::default_instance())
    }

    // repeated .mesos.internal.ReconcileTasksMessage reconciliations = 2;

    pub fn clear_reconciliations(&mut self) {
        self.reconciliations.clear();
    }

    // Param is passed by value, moved
    pub fn set_reconciliations(&mut self, v: ::protobuf::RepeatedField<ReconcileTasksMessage>) {
        self.reconciliations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_reconciliations<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<ReconcileTasksMessage> {
        &mut self.reconciliations
    }

    // Take field
    pub fn take_reconciliations(&mut self) -> ::protobuf::RepeatedField<ReconcileTasksMessage> {
        ::std::mem::replace(&mut self.reconciliations, ::protobuf::RepeatedField::new())
    }

    pub fn get_reconciliations<'a>(&'a self) -> &'a [ReconcileTasksMessage] {
        &self.reconciliations
    }
}

impl ::protobuf::Message for SlaveReregisteredMessage {
    fn is_initialized(&self) -> bool {
        if self.slave_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.slave_id.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.reconciliations));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.slave_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.reconciliations.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.slave_id.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.reconciliations.iter() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SlaveReregisteredMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SlaveReregisteredMessage {
    fn new() -> SlaveReregisteredMessage {
        SlaveReregisteredMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<SlaveReregisteredMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "slave_id",
                    SlaveReregisteredMessage::has_slave_id,
                    SlaveReregisteredMessage::get_slave_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "reconciliations",
                    SlaveReregisteredMessage::get_reconciliations,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SlaveReregisteredMessage>(
                    "SlaveReregisteredMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SlaveReregisteredMessage {
    fn clear(&mut self) {
        self.clear_slave_id();
        self.clear_reconciliations();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SlaveReregisteredMessage {
    fn eq(&self, other: &SlaveReregisteredMessage) -> bool {
        self.slave_id == other.slave_id &&
        self.reconciliations == other.reconciliations &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SlaveReregisteredMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct UnregisterSlaveMessage {
    // message fields
    slave_id: ::protobuf::SingularPtrField<SlaveID>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl UnregisterSlaveMessage {
    pub fn new() -> UnregisterSlaveMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UnregisterSlaveMessage {
        static mut instance: ::protobuf::lazy::Lazy<UnregisterSlaveMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UnregisterSlaveMessage,
        };
        unsafe {
            instance.get(|| {
                UnregisterSlaveMessage {
                    slave_id: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .mesos.SlaveID slave_id = 1;

    pub fn clear_slave_id(&mut self) {
        self.slave_id.clear();
    }

    pub fn has_slave_id(&self) -> bool {
        self.slave_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slave_id(&mut self, v: SlaveID) {
        self.slave_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_slave_id<'a>(&'a mut self) -> &'a mut SlaveID {
        if self.slave_id.is_none() {
            self.slave_id.set_default();
        };
        self.slave_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_slave_id(&mut self) -> SlaveID {
        self.slave_id.take().unwrap_or_else(|| SlaveID::new())
    }

    pub fn get_slave_id<'a>(&'a self) -> &'a SlaveID {
        self.slave_id.as_ref().unwrap_or_else(|| SlaveID::default_instance())
    }
}

impl ::protobuf::Message for UnregisterSlaveMessage {
    fn is_initialized(&self) -> bool {
        if self.slave_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.slave_id.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.slave_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.slave_id.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<UnregisterSlaveMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UnregisterSlaveMessage {
    fn new() -> UnregisterSlaveMessage {
        UnregisterSlaveMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<UnregisterSlaveMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "slave_id",
                    UnregisterSlaveMessage::has_slave_id,
                    UnregisterSlaveMessage::get_slave_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UnregisterSlaveMessage>(
                    "UnregisterSlaveMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UnregisterSlaveMessage {
    fn clear(&mut self) {
        self.clear_slave_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for UnregisterSlaveMessage {
    fn eq(&self, other: &UnregisterSlaveMessage) -> bool {
        self.slave_id == other.slave_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for UnregisterSlaveMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PingSlaveMessage {
    // message fields
    connected: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl PingSlaveMessage {
    pub fn new() -> PingSlaveMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PingSlaveMessage {
        static mut instance: ::protobuf::lazy::Lazy<PingSlaveMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PingSlaveMessage,
        };
        unsafe {
            instance.get(|| {
                PingSlaveMessage {
                    connected: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bool connected = 1;

    pub fn clear_connected(&mut self) {
        self.connected = ::std::option::Option::None;
    }

    pub fn has_connected(&self) -> bool {
        self.connected.is_some()
    }

    // Param is passed by value, moved
    pub fn set_connected(&mut self, v: bool) {
        self.connected = ::std::option::Option::Some(v);
    }

    pub fn get_connected<'a>(&self) -> bool {
        self.connected.unwrap_or(false)
    }
}

impl ::protobuf::Message for PingSlaveMessage {
    fn is_initialized(&self) -> bool {
        if self.connected.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.connected = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.connected.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.connected {
            try!(os.write_bool(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PingSlaveMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PingSlaveMessage {
    fn new() -> PingSlaveMessage {
        PingSlaveMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<PingSlaveMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "connected",
                    PingSlaveMessage::has_connected,
                    PingSlaveMessage::get_connected,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PingSlaveMessage>(
                    "PingSlaveMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PingSlaveMessage {
    fn clear(&mut self) {
        self.clear_connected();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PingSlaveMessage {
    fn eq(&self, other: &PingSlaveMessage) -> bool {
        self.connected == other.connected &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PingSlaveMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PongSlaveMessage {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl PongSlaveMessage {
    pub fn new() -> PongSlaveMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PongSlaveMessage {
        static mut instance: ::protobuf::lazy::Lazy<PongSlaveMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PongSlaveMessage,
        };
        unsafe {
            instance.get(|| {
                PongSlaveMessage {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for PongSlaveMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PongSlaveMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PongSlaveMessage {
    fn new() -> PongSlaveMessage {
        PongSlaveMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<PongSlaveMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<PongSlaveMessage>(
                    "PongSlaveMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PongSlaveMessage {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PongSlaveMessage {
    fn eq(&self, other: &PongSlaveMessage) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PongSlaveMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ShutdownFrameworkMessage {
    // message fields
    framework_id: ::protobuf::SingularPtrField<FrameworkID>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ShutdownFrameworkMessage {
    pub fn new() -> ShutdownFrameworkMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ShutdownFrameworkMessage {
        static mut instance: ::protobuf::lazy::Lazy<ShutdownFrameworkMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ShutdownFrameworkMessage,
        };
        unsafe {
            instance.get(|| {
                ShutdownFrameworkMessage {
                    framework_id: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .mesos.FrameworkID framework_id = 1;

    pub fn clear_framework_id(&mut self) {
        self.framework_id.clear();
    }

    pub fn has_framework_id(&self) -> bool {
        self.framework_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framework_id(&mut self, v: FrameworkID) {
        self.framework_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_framework_id<'a>(&'a mut self) -> &'a mut FrameworkID {
        if self.framework_id.is_none() {
            self.framework_id.set_default();
        };
        self.framework_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_framework_id(&mut self) -> FrameworkID {
        self.framework_id.take().unwrap_or_else(|| FrameworkID::new())
    }

    pub fn get_framework_id<'a>(&'a self) -> &'a FrameworkID {
        self.framework_id.as_ref().unwrap_or_else(|| FrameworkID::default_instance())
    }
}

impl ::protobuf::Message for ShutdownFrameworkMessage {
    fn is_initialized(&self) -> bool {
        if self.framework_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.framework_id.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.framework_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.framework_id.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ShutdownFrameworkMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ShutdownFrameworkMessage {
    fn new() -> ShutdownFrameworkMessage {
        ShutdownFrameworkMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<ShutdownFrameworkMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "framework_id",
                    ShutdownFrameworkMessage::has_framework_id,
                    ShutdownFrameworkMessage::get_framework_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ShutdownFrameworkMessage>(
                    "ShutdownFrameworkMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ShutdownFrameworkMessage {
    fn clear(&mut self) {
        self.clear_framework_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ShutdownFrameworkMessage {
    fn eq(&self, other: &ShutdownFrameworkMessage) -> bool {
        self.framework_id == other.framework_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ShutdownFrameworkMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ShutdownExecutorMessage {
    // message fields
    executor_id: ::protobuf::SingularPtrField<ExecutorID>,
    framework_id: ::protobuf::SingularPtrField<FrameworkID>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ShutdownExecutorMessage {
    pub fn new() -> ShutdownExecutorMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ShutdownExecutorMessage {
        static mut instance: ::protobuf::lazy::Lazy<ShutdownExecutorMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ShutdownExecutorMessage,
        };
        unsafe {
            instance.get(|| {
                ShutdownExecutorMessage {
                    executor_id: ::protobuf::SingularPtrField::none(),
                    framework_id: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .mesos.ExecutorID executor_id = 1;

    pub fn clear_executor_id(&mut self) {
        self.executor_id.clear();
    }

    pub fn has_executor_id(&self) -> bool {
        self.executor_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_executor_id(&mut self, v: ExecutorID) {
        self.executor_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_executor_id<'a>(&'a mut self) -> &'a mut ExecutorID {
        if self.executor_id.is_none() {
            self.executor_id.set_default();
        };
        self.executor_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_executor_id(&mut self) -> ExecutorID {
        self.executor_id.take().unwrap_or_else(|| ExecutorID::new())
    }

    pub fn get_executor_id<'a>(&'a self) -> &'a ExecutorID {
        self.executor_id.as_ref().unwrap_or_else(|| ExecutorID::default_instance())
    }

    // optional .mesos.FrameworkID framework_id = 2;

    pub fn clear_framework_id(&mut self) {
        self.framework_id.clear();
    }

    pub fn has_framework_id(&self) -> bool {
        self.framework_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framework_id(&mut self, v: FrameworkID) {
        self.framework_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_framework_id<'a>(&'a mut self) -> &'a mut FrameworkID {
        if self.framework_id.is_none() {
            self.framework_id.set_default();
        };
        self.framework_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_framework_id(&mut self) -> FrameworkID {
        self.framework_id.take().unwrap_or_else(|| FrameworkID::new())
    }

    pub fn get_framework_id<'a>(&'a self) -> &'a FrameworkID {
        self.framework_id.as_ref().unwrap_or_else(|| FrameworkID::default_instance())
    }
}

impl ::protobuf::Message for ShutdownExecutorMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.executor_id.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.framework_id.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.executor_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.framework_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.executor_id.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.framework_id.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ShutdownExecutorMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ShutdownExecutorMessage {
    fn new() -> ShutdownExecutorMessage {
        ShutdownExecutorMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<ShutdownExecutorMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "executor_id",
                    ShutdownExecutorMessage::has_executor_id,
                    ShutdownExecutorMessage::get_executor_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "framework_id",
                    ShutdownExecutorMessage::has_framework_id,
                    ShutdownExecutorMessage::get_framework_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ShutdownExecutorMessage>(
                    "ShutdownExecutorMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ShutdownExecutorMessage {
    fn clear(&mut self) {
        self.clear_executor_id();
        self.clear_framework_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ShutdownExecutorMessage {
    fn eq(&self, other: &ShutdownExecutorMessage) -> bool {
        self.executor_id == other.executor_id &&
        self.framework_id == other.framework_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ShutdownExecutorMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct UpdateFrameworkMessage {
    // message fields
    framework_id: ::protobuf::SingularPtrField<FrameworkID>,
    pid: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl UpdateFrameworkMessage {
    pub fn new() -> UpdateFrameworkMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UpdateFrameworkMessage {
        static mut instance: ::protobuf::lazy::Lazy<UpdateFrameworkMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UpdateFrameworkMessage,
        };
        unsafe {
            instance.get(|| {
                UpdateFrameworkMessage {
                    framework_id: ::protobuf::SingularPtrField::none(),
                    pid: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .mesos.FrameworkID framework_id = 1;

    pub fn clear_framework_id(&mut self) {
        self.framework_id.clear();
    }

    pub fn has_framework_id(&self) -> bool {
        self.framework_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framework_id(&mut self, v: FrameworkID) {
        self.framework_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_framework_id<'a>(&'a mut self) -> &'a mut FrameworkID {
        if self.framework_id.is_none() {
            self.framework_id.set_default();
        };
        self.framework_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_framework_id(&mut self) -> FrameworkID {
        self.framework_id.take().unwrap_or_else(|| FrameworkID::new())
    }

    pub fn get_framework_id<'a>(&'a self) -> &'a FrameworkID {
        self.framework_id.as_ref().unwrap_or_else(|| FrameworkID::default_instance())
    }

    // required string pid = 2;

    pub fn clear_pid(&mut self) {
        self.pid.clear();
    }

    pub fn has_pid(&self) -> bool {
        self.pid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pid(&mut self, v: ::std::string::String) {
        self.pid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pid<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.pid.is_none() {
            self.pid.set_default();
        };
        self.pid.as_mut().unwrap()
    }

    // Take field
    pub fn take_pid(&mut self) -> ::std::string::String {
        self.pid.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_pid<'a>(&'a self) -> &'a str {
        match self.pid.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for UpdateFrameworkMessage {
    fn is_initialized(&self) -> bool {
        if self.framework_id.is_none() {
            return false;
        };
        if self.pid.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.framework_id.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.pid.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.framework_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.pid.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.framework_id.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.pid.as_ref() {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<UpdateFrameworkMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UpdateFrameworkMessage {
    fn new() -> UpdateFrameworkMessage {
        UpdateFrameworkMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<UpdateFrameworkMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "framework_id",
                    UpdateFrameworkMessage::has_framework_id,
                    UpdateFrameworkMessage::get_framework_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "pid",
                    UpdateFrameworkMessage::has_pid,
                    UpdateFrameworkMessage::get_pid,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UpdateFrameworkMessage>(
                    "UpdateFrameworkMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UpdateFrameworkMessage {
    fn clear(&mut self) {
        self.clear_framework_id();
        self.clear_pid();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for UpdateFrameworkMessage {
    fn eq(&self, other: &UpdateFrameworkMessage) -> bool {
        self.framework_id == other.framework_id &&
        self.pid == other.pid &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for UpdateFrameworkMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CheckpointResourcesMessage {
    // message fields
    resources: ::protobuf::RepeatedField<Resource>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CheckpointResourcesMessage {
    pub fn new() -> CheckpointResourcesMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CheckpointResourcesMessage {
        static mut instance: ::protobuf::lazy::Lazy<CheckpointResourcesMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CheckpointResourcesMessage,
        };
        unsafe {
            instance.get(|| {
                CheckpointResourcesMessage {
                    resources: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .mesos.Resource resources = 1;

    pub fn clear_resources(&mut self) {
        self.resources.clear();
    }

    // Param is passed by value, moved
    pub fn set_resources(&mut self, v: ::protobuf::RepeatedField<Resource>) {
        self.resources = v;
    }

    // Mutable pointer to the field.
    pub fn mut_resources<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Resource> {
        &mut self.resources
    }

    // Take field
    pub fn take_resources(&mut self) -> ::protobuf::RepeatedField<Resource> {
        ::std::mem::replace(&mut self.resources, ::protobuf::RepeatedField::new())
    }

    pub fn get_resources<'a>(&'a self) -> &'a [Resource] {
        &self.resources
    }
}

impl ::protobuf::Message for CheckpointResourcesMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.resources));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.resources.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.resources.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<CheckpointResourcesMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CheckpointResourcesMessage {
    fn new() -> CheckpointResourcesMessage {
        CheckpointResourcesMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<CheckpointResourcesMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "resources",
                    CheckpointResourcesMessage::get_resources,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CheckpointResourcesMessage>(
                    "CheckpointResourcesMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CheckpointResourcesMessage {
    fn clear(&mut self) {
        self.clear_resources();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CheckpointResourcesMessage {
    fn eq(&self, other: &CheckpointResourcesMessage) -> bool {
        self.resources == other.resources &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CheckpointResourcesMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct OversubscribeResourcesMessage {
    // message fields
    slave_id: ::protobuf::SingularPtrField<SlaveID>,
    resources: ::protobuf::RepeatedField<Resource>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl OversubscribeResourcesMessage {
    pub fn new() -> OversubscribeResourcesMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OversubscribeResourcesMessage {
        static mut instance: ::protobuf::lazy::Lazy<OversubscribeResourcesMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OversubscribeResourcesMessage,
        };
        unsafe {
            instance.get(|| {
                OversubscribeResourcesMessage {
                    slave_id: ::protobuf::SingularPtrField::none(),
                    resources: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .mesos.SlaveID slave_id = 1;

    pub fn clear_slave_id(&mut self) {
        self.slave_id.clear();
    }

    pub fn has_slave_id(&self) -> bool {
        self.slave_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slave_id(&mut self, v: SlaveID) {
        self.slave_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_slave_id<'a>(&'a mut self) -> &'a mut SlaveID {
        if self.slave_id.is_none() {
            self.slave_id.set_default();
        };
        self.slave_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_slave_id(&mut self) -> SlaveID {
        self.slave_id.take().unwrap_or_else(|| SlaveID::new())
    }

    pub fn get_slave_id<'a>(&'a self) -> &'a SlaveID {
        self.slave_id.as_ref().unwrap_or_else(|| SlaveID::default_instance())
    }

    // repeated .mesos.Resource resources = 2;

    pub fn clear_resources(&mut self) {
        self.resources.clear();
    }

    // Param is passed by value, moved
    pub fn set_resources(&mut self, v: ::protobuf::RepeatedField<Resource>) {
        self.resources = v;
    }

    // Mutable pointer to the field.
    pub fn mut_resources<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Resource> {
        &mut self.resources
    }

    // Take field
    pub fn take_resources(&mut self) -> ::protobuf::RepeatedField<Resource> {
        ::std::mem::replace(&mut self.resources, ::protobuf::RepeatedField::new())
    }

    pub fn get_resources<'a>(&'a self) -> &'a [Resource] {
        &self.resources
    }
}

impl ::protobuf::Message for OversubscribeResourcesMessage {
    fn is_initialized(&self) -> bool {
        if self.slave_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.slave_id.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.resources));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.slave_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.resources.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.slave_id.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.resources.iter() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<OversubscribeResourcesMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OversubscribeResourcesMessage {
    fn new() -> OversubscribeResourcesMessage {
        OversubscribeResourcesMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<OversubscribeResourcesMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "slave_id",
                    OversubscribeResourcesMessage::has_slave_id,
                    OversubscribeResourcesMessage::get_slave_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "resources",
                    OversubscribeResourcesMessage::get_resources,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OversubscribeResourcesMessage>(
                    "OversubscribeResourcesMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OversubscribeResourcesMessage {
    fn clear(&mut self) {
        self.clear_slave_id();
        self.clear_resources();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for OversubscribeResourcesMessage {
    fn eq(&self, other: &OversubscribeResourcesMessage) -> bool {
        self.slave_id == other.slave_id &&
        self.resources == other.resources &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for OversubscribeResourcesMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RegisterExecutorMessage {
    // message fields
    framework_id: ::protobuf::SingularPtrField<FrameworkID>,
    executor_id: ::protobuf::SingularPtrField<ExecutorID>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl RegisterExecutorMessage {
    pub fn new() -> RegisterExecutorMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RegisterExecutorMessage {
        static mut instance: ::protobuf::lazy::Lazy<RegisterExecutorMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RegisterExecutorMessage,
        };
        unsafe {
            instance.get(|| {
                RegisterExecutorMessage {
                    framework_id: ::protobuf::SingularPtrField::none(),
                    executor_id: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .mesos.FrameworkID framework_id = 1;

    pub fn clear_framework_id(&mut self) {
        self.framework_id.clear();
    }

    pub fn has_framework_id(&self) -> bool {
        self.framework_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framework_id(&mut self, v: FrameworkID) {
        self.framework_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_framework_id<'a>(&'a mut self) -> &'a mut FrameworkID {
        if self.framework_id.is_none() {
            self.framework_id.set_default();
        };
        self.framework_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_framework_id(&mut self) -> FrameworkID {
        self.framework_id.take().unwrap_or_else(|| FrameworkID::new())
    }

    pub fn get_framework_id<'a>(&'a self) -> &'a FrameworkID {
        self.framework_id.as_ref().unwrap_or_else(|| FrameworkID::default_instance())
    }

    // required .mesos.ExecutorID executor_id = 2;

    pub fn clear_executor_id(&mut self) {
        self.executor_id.clear();
    }

    pub fn has_executor_id(&self) -> bool {
        self.executor_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_executor_id(&mut self, v: ExecutorID) {
        self.executor_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_executor_id<'a>(&'a mut self) -> &'a mut ExecutorID {
        if self.executor_id.is_none() {
            self.executor_id.set_default();
        };
        self.executor_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_executor_id(&mut self) -> ExecutorID {
        self.executor_id.take().unwrap_or_else(|| ExecutorID::new())
    }

    pub fn get_executor_id<'a>(&'a self) -> &'a ExecutorID {
        self.executor_id.as_ref().unwrap_or_else(|| ExecutorID::default_instance())
    }
}

impl ::protobuf::Message for RegisterExecutorMessage {
    fn is_initialized(&self) -> bool {
        if self.framework_id.is_none() {
            return false;
        };
        if self.executor_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.framework_id.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.executor_id.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.framework_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.executor_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.framework_id.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.executor_id.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<RegisterExecutorMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RegisterExecutorMessage {
    fn new() -> RegisterExecutorMessage {
        RegisterExecutorMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<RegisterExecutorMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "framework_id",
                    RegisterExecutorMessage::has_framework_id,
                    RegisterExecutorMessage::get_framework_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "executor_id",
                    RegisterExecutorMessage::has_executor_id,
                    RegisterExecutorMessage::get_executor_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RegisterExecutorMessage>(
                    "RegisterExecutorMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RegisterExecutorMessage {
    fn clear(&mut self) {
        self.clear_framework_id();
        self.clear_executor_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RegisterExecutorMessage {
    fn eq(&self, other: &RegisterExecutorMessage) -> bool {
        self.framework_id == other.framework_id &&
        self.executor_id == other.executor_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RegisterExecutorMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ExecutorRegisteredMessage {
    // message fields
    executor_info: ::protobuf::SingularPtrField<ExecutorInfo>,
    framework_id: ::protobuf::SingularPtrField<FrameworkID>,
    framework_info: ::protobuf::SingularPtrField<FrameworkInfo>,
    slave_id: ::protobuf::SingularPtrField<SlaveID>,
    slave_info: ::protobuf::SingularPtrField<SlaveInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ExecutorRegisteredMessage {
    pub fn new() -> ExecutorRegisteredMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExecutorRegisteredMessage {
        static mut instance: ::protobuf::lazy::Lazy<ExecutorRegisteredMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExecutorRegisteredMessage,
        };
        unsafe {
            instance.get(|| {
                ExecutorRegisteredMessage {
                    executor_info: ::protobuf::SingularPtrField::none(),
                    framework_id: ::protobuf::SingularPtrField::none(),
                    framework_info: ::protobuf::SingularPtrField::none(),
                    slave_id: ::protobuf::SingularPtrField::none(),
                    slave_info: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .mesos.ExecutorInfo executor_info = 2;

    pub fn clear_executor_info(&mut self) {
        self.executor_info.clear();
    }

    pub fn has_executor_info(&self) -> bool {
        self.executor_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_executor_info(&mut self, v: ExecutorInfo) {
        self.executor_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_executor_info<'a>(&'a mut self) -> &'a mut ExecutorInfo {
        if self.executor_info.is_none() {
            self.executor_info.set_default();
        };
        self.executor_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_executor_info(&mut self) -> ExecutorInfo {
        self.executor_info.take().unwrap_or_else(|| ExecutorInfo::new())
    }

    pub fn get_executor_info<'a>(&'a self) -> &'a ExecutorInfo {
        self.executor_info.as_ref().unwrap_or_else(|| ExecutorInfo::default_instance())
    }

    // required .mesos.FrameworkID framework_id = 3;

    pub fn clear_framework_id(&mut self) {
        self.framework_id.clear();
    }

    pub fn has_framework_id(&self) -> bool {
        self.framework_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framework_id(&mut self, v: FrameworkID) {
        self.framework_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_framework_id<'a>(&'a mut self) -> &'a mut FrameworkID {
        if self.framework_id.is_none() {
            self.framework_id.set_default();
        };
        self.framework_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_framework_id(&mut self) -> FrameworkID {
        self.framework_id.take().unwrap_or_else(|| FrameworkID::new())
    }

    pub fn get_framework_id<'a>(&'a self) -> &'a FrameworkID {
        self.framework_id.as_ref().unwrap_or_else(|| FrameworkID::default_instance())
    }

    // required .mesos.FrameworkInfo framework_info = 4;

    pub fn clear_framework_info(&mut self) {
        self.framework_info.clear();
    }

    pub fn has_framework_info(&self) -> bool {
        self.framework_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framework_info(&mut self, v: FrameworkInfo) {
        self.framework_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_framework_info<'a>(&'a mut self) -> &'a mut FrameworkInfo {
        if self.framework_info.is_none() {
            self.framework_info.set_default();
        };
        self.framework_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_framework_info(&mut self) -> FrameworkInfo {
        self.framework_info.take().unwrap_or_else(|| FrameworkInfo::new())
    }

    pub fn get_framework_info<'a>(&'a self) -> &'a FrameworkInfo {
        self.framework_info.as_ref().unwrap_or_else(|| FrameworkInfo::default_instance())
    }

    // required .mesos.SlaveID slave_id = 5;

    pub fn clear_slave_id(&mut self) {
        self.slave_id.clear();
    }

    pub fn has_slave_id(&self) -> bool {
        self.slave_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slave_id(&mut self, v: SlaveID) {
        self.slave_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_slave_id<'a>(&'a mut self) -> &'a mut SlaveID {
        if self.slave_id.is_none() {
            self.slave_id.set_default();
        };
        self.slave_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_slave_id(&mut self) -> SlaveID {
        self.slave_id.take().unwrap_or_else(|| SlaveID::new())
    }

    pub fn get_slave_id<'a>(&'a self) -> &'a SlaveID {
        self.slave_id.as_ref().unwrap_or_else(|| SlaveID::default_instance())
    }

    // required .mesos.SlaveInfo slave_info = 6;

    pub fn clear_slave_info(&mut self) {
        self.slave_info.clear();
    }

    pub fn has_slave_info(&self) -> bool {
        self.slave_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slave_info(&mut self, v: SlaveInfo) {
        self.slave_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_slave_info<'a>(&'a mut self) -> &'a mut SlaveInfo {
        if self.slave_info.is_none() {
            self.slave_info.set_default();
        };
        self.slave_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_slave_info(&mut self) -> SlaveInfo {
        self.slave_info.take().unwrap_or_else(|| SlaveInfo::new())
    }

    pub fn get_slave_info<'a>(&'a self) -> &'a SlaveInfo {
        self.slave_info.as_ref().unwrap_or_else(|| SlaveInfo::default_instance())
    }
}

impl ::protobuf::Message for ExecutorRegisteredMessage {
    fn is_initialized(&self) -> bool {
        if self.executor_info.is_none() {
            return false;
        };
        if self.framework_id.is_none() {
            return false;
        };
        if self.framework_info.is_none() {
            return false;
        };
        if self.slave_id.is_none() {
            return false;
        };
        if self.slave_info.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.executor_info.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.framework_id.set_default();
                    try!(is.merge_message(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.framework_info.set_default();
                    try!(is.merge_message(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.slave_id.set_default();
                    try!(is.merge_message(tmp))
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.slave_info.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.executor_info.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.framework_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.framework_info.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.slave_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.slave_info.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.executor_info.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.framework_id.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.framework_info.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.slave_id.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.slave_info.as_ref() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ExecutorRegisteredMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ExecutorRegisteredMessage {
    fn new() -> ExecutorRegisteredMessage {
        ExecutorRegisteredMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExecutorRegisteredMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "executor_info",
                    ExecutorRegisteredMessage::has_executor_info,
                    ExecutorRegisteredMessage::get_executor_info,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "framework_id",
                    ExecutorRegisteredMessage::has_framework_id,
                    ExecutorRegisteredMessage::get_framework_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "framework_info",
                    ExecutorRegisteredMessage::has_framework_info,
                    ExecutorRegisteredMessage::get_framework_info,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "slave_id",
                    ExecutorRegisteredMessage::has_slave_id,
                    ExecutorRegisteredMessage::get_slave_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "slave_info",
                    ExecutorRegisteredMessage::has_slave_info,
                    ExecutorRegisteredMessage::get_slave_info,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExecutorRegisteredMessage>(
                    "ExecutorRegisteredMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExecutorRegisteredMessage {
    fn clear(&mut self) {
        self.clear_executor_info();
        self.clear_framework_id();
        self.clear_framework_info();
        self.clear_slave_id();
        self.clear_slave_info();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ExecutorRegisteredMessage {
    fn eq(&self, other: &ExecutorRegisteredMessage) -> bool {
        self.executor_info == other.executor_info &&
        self.framework_id == other.framework_id &&
        self.framework_info == other.framework_info &&
        self.slave_id == other.slave_id &&
        self.slave_info == other.slave_info &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ExecutorRegisteredMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ExecutorReregisteredMessage {
    // message fields
    slave_id: ::protobuf::SingularPtrField<SlaveID>,
    slave_info: ::protobuf::SingularPtrField<SlaveInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ExecutorReregisteredMessage {
    pub fn new() -> ExecutorReregisteredMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExecutorReregisteredMessage {
        static mut instance: ::protobuf::lazy::Lazy<ExecutorReregisteredMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExecutorReregisteredMessage,
        };
        unsafe {
            instance.get(|| {
                ExecutorReregisteredMessage {
                    slave_id: ::protobuf::SingularPtrField::none(),
                    slave_info: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .mesos.SlaveID slave_id = 1;

    pub fn clear_slave_id(&mut self) {
        self.slave_id.clear();
    }

    pub fn has_slave_id(&self) -> bool {
        self.slave_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slave_id(&mut self, v: SlaveID) {
        self.slave_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_slave_id<'a>(&'a mut self) -> &'a mut SlaveID {
        if self.slave_id.is_none() {
            self.slave_id.set_default();
        };
        self.slave_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_slave_id(&mut self) -> SlaveID {
        self.slave_id.take().unwrap_or_else(|| SlaveID::new())
    }

    pub fn get_slave_id<'a>(&'a self) -> &'a SlaveID {
        self.slave_id.as_ref().unwrap_or_else(|| SlaveID::default_instance())
    }

    // required .mesos.SlaveInfo slave_info = 2;

    pub fn clear_slave_info(&mut self) {
        self.slave_info.clear();
    }

    pub fn has_slave_info(&self) -> bool {
        self.slave_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slave_info(&mut self, v: SlaveInfo) {
        self.slave_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_slave_info<'a>(&'a mut self) -> &'a mut SlaveInfo {
        if self.slave_info.is_none() {
            self.slave_info.set_default();
        };
        self.slave_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_slave_info(&mut self) -> SlaveInfo {
        self.slave_info.take().unwrap_or_else(|| SlaveInfo::new())
    }

    pub fn get_slave_info<'a>(&'a self) -> &'a SlaveInfo {
        self.slave_info.as_ref().unwrap_or_else(|| SlaveInfo::default_instance())
    }
}

impl ::protobuf::Message for ExecutorReregisteredMessage {
    fn is_initialized(&self) -> bool {
        if self.slave_id.is_none() {
            return false;
        };
        if self.slave_info.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.slave_id.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.slave_info.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.slave_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.slave_info.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.slave_id.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.slave_info.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ExecutorReregisteredMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ExecutorReregisteredMessage {
    fn new() -> ExecutorReregisteredMessage {
        ExecutorReregisteredMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExecutorReregisteredMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "slave_id",
                    ExecutorReregisteredMessage::has_slave_id,
                    ExecutorReregisteredMessage::get_slave_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "slave_info",
                    ExecutorReregisteredMessage::has_slave_info,
                    ExecutorReregisteredMessage::get_slave_info,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExecutorReregisteredMessage>(
                    "ExecutorReregisteredMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExecutorReregisteredMessage {
    fn clear(&mut self) {
        self.clear_slave_id();
        self.clear_slave_info();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ExecutorReregisteredMessage {
    fn eq(&self, other: &ExecutorReregisteredMessage) -> bool {
        self.slave_id == other.slave_id &&
        self.slave_info == other.slave_info &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ExecutorReregisteredMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ExitedExecutorMessage {
    // message fields
    slave_id: ::protobuf::SingularPtrField<SlaveID>,
    framework_id: ::protobuf::SingularPtrField<FrameworkID>,
    executor_id: ::protobuf::SingularPtrField<ExecutorID>,
    status: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ExitedExecutorMessage {
    pub fn new() -> ExitedExecutorMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExitedExecutorMessage {
        static mut instance: ::protobuf::lazy::Lazy<ExitedExecutorMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExitedExecutorMessage,
        };
        unsafe {
            instance.get(|| {
                ExitedExecutorMessage {
                    slave_id: ::protobuf::SingularPtrField::none(),
                    framework_id: ::protobuf::SingularPtrField::none(),
                    executor_id: ::protobuf::SingularPtrField::none(),
                    status: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .mesos.SlaveID slave_id = 1;

    pub fn clear_slave_id(&mut self) {
        self.slave_id.clear();
    }

    pub fn has_slave_id(&self) -> bool {
        self.slave_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slave_id(&mut self, v: SlaveID) {
        self.slave_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_slave_id<'a>(&'a mut self) -> &'a mut SlaveID {
        if self.slave_id.is_none() {
            self.slave_id.set_default();
        };
        self.slave_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_slave_id(&mut self) -> SlaveID {
        self.slave_id.take().unwrap_or_else(|| SlaveID::new())
    }

    pub fn get_slave_id<'a>(&'a self) -> &'a SlaveID {
        self.slave_id.as_ref().unwrap_or_else(|| SlaveID::default_instance())
    }

    // required .mesos.FrameworkID framework_id = 2;

    pub fn clear_framework_id(&mut self) {
        self.framework_id.clear();
    }

    pub fn has_framework_id(&self) -> bool {
        self.framework_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framework_id(&mut self, v: FrameworkID) {
        self.framework_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_framework_id<'a>(&'a mut self) -> &'a mut FrameworkID {
        if self.framework_id.is_none() {
            self.framework_id.set_default();
        };
        self.framework_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_framework_id(&mut self) -> FrameworkID {
        self.framework_id.take().unwrap_or_else(|| FrameworkID::new())
    }

    pub fn get_framework_id<'a>(&'a self) -> &'a FrameworkID {
        self.framework_id.as_ref().unwrap_or_else(|| FrameworkID::default_instance())
    }

    // required .mesos.ExecutorID executor_id = 3;

    pub fn clear_executor_id(&mut self) {
        self.executor_id.clear();
    }

    pub fn has_executor_id(&self) -> bool {
        self.executor_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_executor_id(&mut self, v: ExecutorID) {
        self.executor_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_executor_id<'a>(&'a mut self) -> &'a mut ExecutorID {
        if self.executor_id.is_none() {
            self.executor_id.set_default();
        };
        self.executor_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_executor_id(&mut self) -> ExecutorID {
        self.executor_id.take().unwrap_or_else(|| ExecutorID::new())
    }

    pub fn get_executor_id<'a>(&'a self) -> &'a ExecutorID {
        self.executor_id.as_ref().unwrap_or_else(|| ExecutorID::default_instance())
    }

    // required int32 status = 4;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: i32) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status<'a>(&self) -> i32 {
        self.status.unwrap_or(0)
    }
}

impl ::protobuf::Message for ExitedExecutorMessage {
    fn is_initialized(&self) -> bool {
        if self.slave_id.is_none() {
            return false;
        };
        if self.framework_id.is_none() {
            return false;
        };
        if self.executor_id.is_none() {
            return false;
        };
        if self.status.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.slave_id.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.framework_id.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.executor_id.set_default();
                    try!(is.merge_message(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.status = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.slave_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.framework_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.executor_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.status.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.slave_id.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.framework_id.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.executor_id.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.status {
            try!(os.write_int32(4, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ExitedExecutorMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ExitedExecutorMessage {
    fn new() -> ExitedExecutorMessage {
        ExitedExecutorMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExitedExecutorMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "slave_id",
                    ExitedExecutorMessage::has_slave_id,
                    ExitedExecutorMessage::get_slave_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "framework_id",
                    ExitedExecutorMessage::has_framework_id,
                    ExitedExecutorMessage::get_framework_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "executor_id",
                    ExitedExecutorMessage::has_executor_id,
                    ExitedExecutorMessage::get_executor_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "status",
                    ExitedExecutorMessage::has_status,
                    ExitedExecutorMessage::get_status,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExitedExecutorMessage>(
                    "ExitedExecutorMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExitedExecutorMessage {
    fn clear(&mut self) {
        self.clear_slave_id();
        self.clear_framework_id();
        self.clear_executor_id();
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ExitedExecutorMessage {
    fn eq(&self, other: &ExitedExecutorMessage) -> bool {
        self.slave_id == other.slave_id &&
        self.framework_id == other.framework_id &&
        self.executor_id == other.executor_id &&
        self.status == other.status &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ExitedExecutorMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ReconnectExecutorMessage {
    // message fields
    slave_id: ::protobuf::SingularPtrField<SlaveID>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ReconnectExecutorMessage {
    pub fn new() -> ReconnectExecutorMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReconnectExecutorMessage {
        static mut instance: ::protobuf::lazy::Lazy<ReconnectExecutorMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReconnectExecutorMessage,
        };
        unsafe {
            instance.get(|| {
                ReconnectExecutorMessage {
                    slave_id: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .mesos.SlaveID slave_id = 1;

    pub fn clear_slave_id(&mut self) {
        self.slave_id.clear();
    }

    pub fn has_slave_id(&self) -> bool {
        self.slave_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slave_id(&mut self, v: SlaveID) {
        self.slave_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_slave_id<'a>(&'a mut self) -> &'a mut SlaveID {
        if self.slave_id.is_none() {
            self.slave_id.set_default();
        };
        self.slave_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_slave_id(&mut self) -> SlaveID {
        self.slave_id.take().unwrap_or_else(|| SlaveID::new())
    }

    pub fn get_slave_id<'a>(&'a self) -> &'a SlaveID {
        self.slave_id.as_ref().unwrap_or_else(|| SlaveID::default_instance())
    }
}

impl ::protobuf::Message for ReconnectExecutorMessage {
    fn is_initialized(&self) -> bool {
        if self.slave_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.slave_id.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.slave_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.slave_id.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ReconnectExecutorMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReconnectExecutorMessage {
    fn new() -> ReconnectExecutorMessage {
        ReconnectExecutorMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReconnectExecutorMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "slave_id",
                    ReconnectExecutorMessage::has_slave_id,
                    ReconnectExecutorMessage::get_slave_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReconnectExecutorMessage>(
                    "ReconnectExecutorMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReconnectExecutorMessage {
    fn clear(&mut self) {
        self.clear_slave_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ReconnectExecutorMessage {
    fn eq(&self, other: &ReconnectExecutorMessage) -> bool {
        self.slave_id == other.slave_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ReconnectExecutorMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ReregisterExecutorMessage {
    // message fields
    executor_id: ::protobuf::SingularPtrField<ExecutorID>,
    framework_id: ::protobuf::SingularPtrField<FrameworkID>,
    tasks: ::protobuf::RepeatedField<TaskInfo>,
    updates: ::protobuf::RepeatedField<StatusUpdate>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ReregisterExecutorMessage {
    pub fn new() -> ReregisterExecutorMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReregisterExecutorMessage {
        static mut instance: ::protobuf::lazy::Lazy<ReregisterExecutorMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReregisterExecutorMessage,
        };
        unsafe {
            instance.get(|| {
                ReregisterExecutorMessage {
                    executor_id: ::protobuf::SingularPtrField::none(),
                    framework_id: ::protobuf::SingularPtrField::none(),
                    tasks: ::protobuf::RepeatedField::new(),
                    updates: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .mesos.ExecutorID executor_id = 1;

    pub fn clear_executor_id(&mut self) {
        self.executor_id.clear();
    }

    pub fn has_executor_id(&self) -> bool {
        self.executor_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_executor_id(&mut self, v: ExecutorID) {
        self.executor_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_executor_id<'a>(&'a mut self) -> &'a mut ExecutorID {
        if self.executor_id.is_none() {
            self.executor_id.set_default();
        };
        self.executor_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_executor_id(&mut self) -> ExecutorID {
        self.executor_id.take().unwrap_or_else(|| ExecutorID::new())
    }

    pub fn get_executor_id<'a>(&'a self) -> &'a ExecutorID {
        self.executor_id.as_ref().unwrap_or_else(|| ExecutorID::default_instance())
    }

    // required .mesos.FrameworkID framework_id = 2;

    pub fn clear_framework_id(&mut self) {
        self.framework_id.clear();
    }

    pub fn has_framework_id(&self) -> bool {
        self.framework_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framework_id(&mut self, v: FrameworkID) {
        self.framework_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_framework_id<'a>(&'a mut self) -> &'a mut FrameworkID {
        if self.framework_id.is_none() {
            self.framework_id.set_default();
        };
        self.framework_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_framework_id(&mut self) -> FrameworkID {
        self.framework_id.take().unwrap_or_else(|| FrameworkID::new())
    }

    pub fn get_framework_id<'a>(&'a self) -> &'a FrameworkID {
        self.framework_id.as_ref().unwrap_or_else(|| FrameworkID::default_instance())
    }

    // repeated .mesos.TaskInfo tasks = 3;

    pub fn clear_tasks(&mut self) {
        self.tasks.clear();
    }

    // Param is passed by value, moved
    pub fn set_tasks(&mut self, v: ::protobuf::RepeatedField<TaskInfo>) {
        self.tasks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tasks<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<TaskInfo> {
        &mut self.tasks
    }

    // Take field
    pub fn take_tasks(&mut self) -> ::protobuf::RepeatedField<TaskInfo> {
        ::std::mem::replace(&mut self.tasks, ::protobuf::RepeatedField::new())
    }

    pub fn get_tasks<'a>(&'a self) -> &'a [TaskInfo] {
        &self.tasks
    }

    // repeated .mesos.internal.StatusUpdate updates = 4;

    pub fn clear_updates(&mut self) {
        self.updates.clear();
    }

    // Param is passed by value, moved
    pub fn set_updates(&mut self, v: ::protobuf::RepeatedField<StatusUpdate>) {
        self.updates = v;
    }

    // Mutable pointer to the field.
    pub fn mut_updates<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<StatusUpdate> {
        &mut self.updates
    }

    // Take field
    pub fn take_updates(&mut self) -> ::protobuf::RepeatedField<StatusUpdate> {
        ::std::mem::replace(&mut self.updates, ::protobuf::RepeatedField::new())
    }

    pub fn get_updates<'a>(&'a self) -> &'a [StatusUpdate] {
        &self.updates
    }
}

impl ::protobuf::Message for ReregisterExecutorMessage {
    fn is_initialized(&self) -> bool {
        if self.executor_id.is_none() {
            return false;
        };
        if self.framework_id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.executor_id.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.framework_id.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tasks));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.updates));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.executor_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.framework_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.tasks.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.updates.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.executor_id.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.framework_id.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.tasks.iter() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.updates.iter() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ReregisterExecutorMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReregisterExecutorMessage {
    fn new() -> ReregisterExecutorMessage {
        ReregisterExecutorMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReregisterExecutorMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "executor_id",
                    ReregisterExecutorMessage::has_executor_id,
                    ReregisterExecutorMessage::get_executor_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "framework_id",
                    ReregisterExecutorMessage::has_framework_id,
                    ReregisterExecutorMessage::get_framework_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "tasks",
                    ReregisterExecutorMessage::get_tasks,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "updates",
                    ReregisterExecutorMessage::get_updates,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReregisterExecutorMessage>(
                    "ReregisterExecutorMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReregisterExecutorMessage {
    fn clear(&mut self) {
        self.clear_executor_id();
        self.clear_framework_id();
        self.clear_tasks();
        self.clear_updates();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ReregisterExecutorMessage {
    fn eq(&self, other: &ReregisterExecutorMessage) -> bool {
        self.executor_id == other.executor_id &&
        self.framework_id == other.framework_id &&
        self.tasks == other.tasks &&
        self.updates == other.updates &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ReregisterExecutorMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ShutdownMessage {
    // message fields
    message: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ShutdownMessage {
    pub fn new() -> ShutdownMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ShutdownMessage {
        static mut instance: ::protobuf::lazy::Lazy<ShutdownMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ShutdownMessage,
        };
        unsafe {
            instance.get(|| {
                ShutdownMessage {
                    message: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string message = 1;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.message.is_none() {
            self.message.set_default();
        };
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        self.message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_message<'a>(&'a self) -> &'a str {
        match self.message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for ShutdownMessage {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.message.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.message.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.message.as_ref() {
            try!(os.write_string(1, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ShutdownMessage>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ShutdownMessage {
    fn new() -> ShutdownMessage {
        ShutdownMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<ShutdownMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "message",
                    ShutdownMessage::has_message,
                    ShutdownMessage::get_message,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ShutdownMessage>(
                    "ShutdownMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ShutdownMessage {
    fn clear(&mut self) {
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ShutdownMessage {
    fn eq(&self, other: &ShutdownMessage) -> bool {
        self.message == other.message &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ShutdownMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Archive {
    // message fields
    frameworks: ::protobuf::RepeatedField<Archive_Framework>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Archive {
    pub fn new() -> Archive {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Archive {
        static mut instance: ::protobuf::lazy::Lazy<Archive> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Archive,
        };
        unsafe {
            instance.get(|| {
                Archive {
                    frameworks: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .mesos.internal.Archive.Framework frameworks = 1;

    pub fn clear_frameworks(&mut self) {
        self.frameworks.clear();
    }

    // Param is passed by value, moved
    pub fn set_frameworks(&mut self, v: ::protobuf::RepeatedField<Archive_Framework>) {
        self.frameworks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_frameworks<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Archive_Framework> {
        &mut self.frameworks
    }

    // Take field
    pub fn take_frameworks(&mut self) -> ::protobuf::RepeatedField<Archive_Framework> {
        ::std::mem::replace(&mut self.frameworks, ::protobuf::RepeatedField::new())
    }

    pub fn get_frameworks<'a>(&'a self) -> &'a [Archive_Framework] {
        &self.frameworks
    }
}

impl ::protobuf::Message for Archive {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.frameworks));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.frameworks.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.frameworks.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Archive>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Archive {
    fn new() -> Archive {
        Archive::new()
    }

    fn descriptor_static(_: ::std::option::Option<Archive>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "frameworks",
                    Archive::get_frameworks,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Archive>(
                    "Archive",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Archive {
    fn clear(&mut self) {
        self.clear_frameworks();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Archive {
    fn eq(&self, other: &Archive) -> bool {
        self.frameworks == other.frameworks &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Archive {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Archive_Framework {
    // message fields
    framework_info: ::protobuf::SingularPtrField<FrameworkInfo>,
    pid: ::protobuf::SingularField<::std::string::String>,
    tasks: ::protobuf::RepeatedField<Task>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Archive_Framework {
    pub fn new() -> Archive_Framework {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Archive_Framework {
        static mut instance: ::protobuf::lazy::Lazy<Archive_Framework> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Archive_Framework,
        };
        unsafe {
            instance.get(|| {
                Archive_Framework {
                    framework_info: ::protobuf::SingularPtrField::none(),
                    pid: ::protobuf::SingularField::none(),
                    tasks: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .mesos.FrameworkInfo framework_info = 1;

    pub fn clear_framework_info(&mut self) {
        self.framework_info.clear();
    }

    pub fn has_framework_info(&self) -> bool {
        self.framework_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framework_info(&mut self, v: FrameworkInfo) {
        self.framework_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_framework_info<'a>(&'a mut self) -> &'a mut FrameworkInfo {
        if self.framework_info.is_none() {
            self.framework_info.set_default();
        };
        self.framework_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_framework_info(&mut self) -> FrameworkInfo {
        self.framework_info.take().unwrap_or_else(|| FrameworkInfo::new())
    }

    pub fn get_framework_info<'a>(&'a self) -> &'a FrameworkInfo {
        self.framework_info.as_ref().unwrap_or_else(|| FrameworkInfo::default_instance())
    }

    // optional string pid = 2;

    pub fn clear_pid(&mut self) {
        self.pid.clear();
    }

    pub fn has_pid(&self) -> bool {
        self.pid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pid(&mut self, v: ::std::string::String) {
        self.pid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pid<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.pid.is_none() {
            self.pid.set_default();
        };
        self.pid.as_mut().unwrap()
    }

    // Take field
    pub fn take_pid(&mut self) -> ::std::string::String {
        self.pid.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_pid<'a>(&'a self) -> &'a str {
        match self.pid.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // repeated .mesos.internal.Task tasks = 3;

    pub fn clear_tasks(&mut self) {
        self.tasks.clear();
    }

    // Param is passed by value, moved
    pub fn set_tasks(&mut self, v: ::protobuf::RepeatedField<Task>) {
        self.tasks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tasks<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Task> {
        &mut self.tasks
    }

    // Take field
    pub fn take_tasks(&mut self) -> ::protobuf::RepeatedField<Task> {
        ::std::mem::replace(&mut self.tasks, ::protobuf::RepeatedField::new())
    }

    pub fn get_tasks<'a>(&'a self) -> &'a [Task] {
        &self.tasks
    }
}

impl ::protobuf::Message for Archive_Framework {
    fn is_initialized(&self) -> bool {
        if self.framework_info.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.framework_info.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.pid.set_default();
                    try!(is.read_string_into(tmp))
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tasks));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.framework_info.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.pid.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.tasks.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.framework_info.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.pid.as_ref() {
            try!(os.write_string(2, &v));
        };
        for v in self.tasks.iter() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Archive_Framework>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Archive_Framework {
    fn new() -> Archive_Framework {
        Archive_Framework::new()
    }

    fn descriptor_static(_: ::std::option::Option<Archive_Framework>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "framework_info",
                    Archive_Framework::has_framework_info,
                    Archive_Framework::get_framework_info,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "pid",
                    Archive_Framework::has_pid,
                    Archive_Framework::get_pid,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "tasks",
                    Archive_Framework::get_tasks,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Archive_Framework>(
                    "Archive_Framework",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Archive_Framework {
    fn clear(&mut self) {
        self.clear_framework_info();
        self.clear_pid();
        self.clear_tasks();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Archive_Framework {
    fn eq(&self, other: &Archive_Framework) -> bool {
        self.framework_info == other.framework_info &&
        self.pid == other.pid &&
        self.tasks == other.tasks &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Archive_Framework {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TaskHealthStatus {
    // message fields
    task_id: ::protobuf::SingularPtrField<TaskID>,
    healthy: ::std::option::Option<bool>,
    kill_task: ::std::option::Option<bool>,
    consecutive_failures: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl TaskHealthStatus {
    pub fn new() -> TaskHealthStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TaskHealthStatus {
        static mut instance: ::protobuf::lazy::Lazy<TaskHealthStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TaskHealthStatus,
        };
        unsafe {
            instance.get(|| {
                TaskHealthStatus {
                    task_id: ::protobuf::SingularPtrField::none(),
                    healthy: ::std::option::Option::None,
                    kill_task: ::std::option::Option::None,
                    consecutive_failures: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .mesos.TaskID task_id = 1;

    pub fn clear_task_id(&mut self) {
        self.task_id.clear();
    }

    pub fn has_task_id(&self) -> bool {
        self.task_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_task_id(&mut self, v: TaskID) {
        self.task_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_task_id<'a>(&'a mut self) -> &'a mut TaskID {
        if self.task_id.is_none() {
            self.task_id.set_default();
        };
        self.task_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_task_id(&mut self) -> TaskID {
        self.task_id.take().unwrap_or_else(|| TaskID::new())
    }

    pub fn get_task_id<'a>(&'a self) -> &'a TaskID {
        self.task_id.as_ref().unwrap_or_else(|| TaskID::default_instance())
    }

    // required bool healthy = 2;

    pub fn clear_healthy(&mut self) {
        self.healthy = ::std::option::Option::None;
    }

    pub fn has_healthy(&self) -> bool {
        self.healthy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_healthy(&mut self, v: bool) {
        self.healthy = ::std::option::Option::Some(v);
    }

    pub fn get_healthy<'a>(&self) -> bool {
        self.healthy.unwrap_or(false)
    }

    // optional bool kill_task = 3;

    pub fn clear_kill_task(&mut self) {
        self.kill_task = ::std::option::Option::None;
    }

    pub fn has_kill_task(&self) -> bool {
        self.kill_task.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kill_task(&mut self, v: bool) {
        self.kill_task = ::std::option::Option::Some(v);
    }

    pub fn get_kill_task<'a>(&self) -> bool {
        self.kill_task.unwrap_or(false)
    }

    // optional int32 consecutive_failures = 4;

    pub fn clear_consecutive_failures(&mut self) {
        self.consecutive_failures = ::std::option::Option::None;
    }

    pub fn has_consecutive_failures(&self) -> bool {
        self.consecutive_failures.is_some()
    }

    // Param is passed by value, moved
    pub fn set_consecutive_failures(&mut self, v: i32) {
        self.consecutive_failures = ::std::option::Option::Some(v);
    }

    pub fn get_consecutive_failures<'a>(&self) -> i32 {
        self.consecutive_failures.unwrap_or(0)
    }
}

impl ::protobuf::Message for TaskHealthStatus {
    fn is_initialized(&self) -> bool {
        if self.task_id.is_none() {
            return false;
        };
        if self.healthy.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.task_id.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.healthy = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.kill_task = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.consecutive_failures = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.task_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.healthy.is_some() {
            my_size += 2;
        };
        if self.kill_task.is_some() {
            my_size += 2;
        };
        for value in self.consecutive_failures.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.task_id.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.healthy {
            try!(os.write_bool(2, v));
        };
        if let Some(v) = self.kill_task {
            try!(os.write_bool(3, v));
        };
        if let Some(v) = self.consecutive_failures {
            try!(os.write_int32(4, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TaskHealthStatus>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TaskHealthStatus {
    fn new() -> TaskHealthStatus {
        TaskHealthStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<TaskHealthStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "task_id",
                    TaskHealthStatus::has_task_id,
                    TaskHealthStatus::get_task_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "healthy",
                    TaskHealthStatus::has_healthy,
                    TaskHealthStatus::get_healthy,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "kill_task",
                    TaskHealthStatus::has_kill_task,
                    TaskHealthStatus::get_kill_task,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "consecutive_failures",
                    TaskHealthStatus::has_consecutive_failures,
                    TaskHealthStatus::get_consecutive_failures,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TaskHealthStatus>(
                    "TaskHealthStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TaskHealthStatus {
    fn clear(&mut self) {
        self.clear_task_id();
        self.clear_healthy();
        self.clear_kill_task();
        self.clear_consecutive_failures();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TaskHealthStatus {
    fn eq(&self, other: &TaskHealthStatus) -> bool {
        self.task_id == other.task_id &&
        self.healthy == other.healthy &&
        self.kill_task == other.kill_task &&
        self.consecutive_failures == other.consecutive_failures &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TaskHealthStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct HookExecuted {
    // message fields
    module: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl HookExecuted {
    pub fn new() -> HookExecuted {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HookExecuted {
        static mut instance: ::protobuf::lazy::Lazy<HookExecuted> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HookExecuted,
        };
        unsafe {
            instance.get(|| {
                HookExecuted {
                    module: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string module = 1;

    pub fn clear_module(&mut self) {
        self.module.clear();
    }

    pub fn has_module(&self) -> bool {
        self.module.is_some()
    }

    // Param is passed by value, moved
    pub fn set_module(&mut self, v: ::std::string::String) {
        self.module = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_module<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.module.is_none() {
            self.module.set_default();
        };
        self.module.as_mut().unwrap()
    }

    // Take field
    pub fn take_module(&mut self) -> ::std::string::String {
        self.module.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_module<'a>(&'a self) -> &'a str {
        match self.module.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for HookExecuted {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.module.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.module.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.module.as_ref() {
            try!(os.write_string(1, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<HookExecuted>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for HookExecuted {
    fn new() -> HookExecuted {
        HookExecuted::new()
    }

    fn descriptor_static(_: ::std::option::Option<HookExecuted>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "module",
                    HookExecuted::has_module,
                    HookExecuted::get_module,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HookExecuted>(
                    "HookExecuted",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HookExecuted {
    fn clear(&mut self) {
        self.clear_module();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for HookExecuted {
    fn eq(&self, other: &HookExecuted) -> bool {
        self.module == other.module &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for HookExecuted {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x12, 0x0e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c,
    0x1a, 0x0b, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xa5, 0x03,
    0x0a, 0x04, 0x54, 0x61, 0x73, 0x6b, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x09, 0x12, 0x1e, 0x0a, 0x07, 0x74, 0x61, 0x73, 0x6b, 0x5f, 0x69, 0x64, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x54, 0x61,
    0x73, 0x6b, 0x49, 0x44, 0x12, 0x28, 0x0a, 0x0c, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72,
    0x6b, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x6d, 0x65, 0x73,
    0x6f, 0x73, 0x2e, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x49, 0x44, 0x12, 0x26,
    0x0a, 0x0b, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x04, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x45, 0x78, 0x65, 0x63,
    0x75, 0x74, 0x6f, 0x72, 0x49, 0x44, 0x12, 0x20, 0x0a, 0x08, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x5f,
    0x69, 0x64, 0x18, 0x05, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73,
    0x2e, 0x53, 0x6c, 0x61, 0x76, 0x65, 0x49, 0x44, 0x12, 0x1f, 0x0a, 0x05, 0x73, 0x74, 0x61, 0x74,
    0x65, 0x18, 0x06, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x10, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e,
    0x54, 0x61, 0x73, 0x6b, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x22, 0x0a, 0x09, 0x72, 0x65, 0x73,
    0x6f, 0x75, 0x72, 0x63, 0x65, 0x73, 0x18, 0x07, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x6d,
    0x65, 0x73, 0x6f, 0x73, 0x2e, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x12, 0x23, 0x0a,
    0x08, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x65, 0x73, 0x18, 0x08, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x11, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x54, 0x61, 0x73, 0x6b, 0x53, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x12, 0x2d, 0x0a, 0x13, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x5f, 0x75, 0x70, 0x64,
    0x61, 0x74, 0x65, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x10, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x54, 0x61, 0x73, 0x6b, 0x53, 0x74, 0x61, 0x74,
    0x65, 0x12, 0x1a, 0x0a, 0x12, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x5f, 0x75, 0x70, 0x64, 0x61,
    0x74, 0x65, 0x5f, 0x75, 0x75, 0x69, 0x64, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x1d, 0x0a,
    0x06, 0x6c, 0x61, 0x62, 0x65, 0x6c, 0x73, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e,
    0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x4c, 0x61, 0x62, 0x65, 0x6c, 0x73, 0x12, 0x27, 0x0a, 0x09,
    0x64, 0x69, 0x73, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x14, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x44, 0x69, 0x73, 0x63, 0x6f, 0x76, 0x65, 0x72,
    0x79, 0x49, 0x6e, 0x66, 0x6f, 0x22, 0xee, 0x01, 0x0a, 0x0c, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x12, 0x28, 0x0a, 0x0c, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77,
    0x6f, 0x72, 0x6b, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x6d,
    0x65, 0x73, 0x6f, 0x73, 0x2e, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x49, 0x44,
    0x12, 0x26, 0x0a, 0x0b, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x5f, 0x69, 0x64, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x45, 0x78,
    0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x49, 0x44, 0x12, 0x20, 0x0a, 0x08, 0x73, 0x6c, 0x61, 0x76,
    0x65, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x73,
    0x6f, 0x73, 0x2e, 0x53, 0x6c, 0x61, 0x76, 0x65, 0x49, 0x44, 0x12, 0x21, 0x0a, 0x06, 0x73, 0x74,
    0x61, 0x74, 0x75, 0x73, 0x18, 0x04, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6d, 0x65, 0x73,
    0x6f, 0x73, 0x2e, 0x54, 0x61, 0x73, 0x6b, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x11, 0x0a,
    0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x05, 0x20, 0x02, 0x28, 0x01,
    0x12, 0x0c, 0x0a, 0x04, 0x75, 0x75, 0x69, 0x64, 0x18, 0x06, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x26,
    0x0a, 0x0c, 0x6c, 0x61, 0x74, 0x65, 0x73, 0x74, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x07,
    0x20, 0x01, 0x28, 0x0e, 0x32, 0x10, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x54, 0x61, 0x73,
    0x6b, 0x53, 0x74, 0x61, 0x74, 0x65, 0x22, 0xa4, 0x01, 0x0a, 0x12, 0x53, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x52, 0x65, 0x63, 0x6f, 0x72, 0x64, 0x12, 0x35, 0x0a,
    0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x27, 0x2e, 0x6d, 0x65,
    0x73, 0x6f, 0x73, 0x2e, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x2e, 0x53, 0x74, 0x61,
    0x74, 0x75, 0x73, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x52, 0x65, 0x63, 0x6f, 0x72, 0x64, 0x2e,
    0x54, 0x79, 0x70, 0x65, 0x12, 0x2c, 0x0a, 0x06, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x69, 0x6e, 0x74,
    0x65, 0x72, 0x6e, 0x61, 0x6c, 0x2e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x55, 0x70, 0x64, 0x61,
    0x74, 0x65, 0x12, 0x0c, 0x0a, 0x04, 0x75, 0x75, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c,
    0x22, 0x1b, 0x0a, 0x04, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0a, 0x0a, 0x06, 0x55, 0x50, 0x44, 0x41,
    0x54, 0x45, 0x10, 0x00, 0x12, 0x07, 0x0a, 0x03, 0x41, 0x43, 0x4b, 0x10, 0x01, 0x22, 0x26, 0x0a,
    0x16, 0x53, 0x75, 0x62, 0x6d, 0x69, 0x74, 0x53, 0x63, 0x68, 0x65, 0x64, 0x75, 0x6c, 0x65, 0x72,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x09, 0x22, 0x27, 0x0a, 0x17, 0x53, 0x75, 0x62, 0x6d, 0x69, 0x74, 0x53,
    0x63, 0x68, 0x65, 0x64, 0x75, 0x6c, 0x65, 0x72, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x12, 0x0c, 0x0a, 0x04, 0x6f, 0x6b, 0x61, 0x79, 0x18, 0x01, 0x20, 0x02, 0x28, 0x08, 0x22, 0x9e,
    0x01, 0x0a, 0x1a, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x54, 0x6f, 0x46, 0x72, 0x61,
    0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x20, 0x0a,
    0x08, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32,
    0x0e, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x53, 0x6c, 0x61, 0x76, 0x65, 0x49, 0x44, 0x12,
    0x28, 0x0a, 0x0c, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x69, 0x64, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x46, 0x72,
    0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x49, 0x44, 0x12, 0x26, 0x0a, 0x0b, 0x65, 0x78, 0x65,
    0x63, 0x75, 0x74, 0x6f, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x11,
    0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x49,
    0x44, 0x12, 0x0c, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x04, 0x20, 0x02, 0x28, 0x0c, 0x22,
    0x9e, 0x01, 0x0a, 0x1a, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x54, 0x6f, 0x45,
    0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x20,
    0x0a, 0x08, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b,
    0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x53, 0x6c, 0x61, 0x76, 0x65, 0x49, 0x44,
    0x12, 0x28, 0x0a, 0x0c, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x69, 0x64,
    0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x46,
    0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x49, 0x44, 0x12, 0x26, 0x0a, 0x0b, 0x65, 0x78,
    0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0b, 0x32,
    0x11, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72,
    0x49, 0x44, 0x12, 0x0c, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x04, 0x20, 0x02, 0x28, 0x0c,
    0x22, 0x43, 0x0a, 0x18, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x46, 0x72, 0x61, 0x6d,
    0x65, 0x77, 0x6f, 0x72, 0x6b, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x27, 0x0a, 0x09,
    0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32,
    0x14, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72,
    0x6b, 0x49, 0x6e, 0x66, 0x6f, 0x22, 0x57, 0x0a, 0x1a, 0x52, 0x65, 0x72, 0x65, 0x67, 0x69, 0x73,
    0x74, 0x65, 0x72, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x4d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x12, 0x27, 0x0a, 0x09, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b,
    0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x46,
    0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x10, 0x0a, 0x08,
    0x66, 0x61, 0x69, 0x6c, 0x6f, 0x76, 0x65, 0x72, 0x18, 0x03, 0x20, 0x02, 0x28, 0x08, 0x22, 0x6e,
    0x0a, 0x1a, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x52, 0x65, 0x67, 0x69, 0x73,
    0x74, 0x65, 0x72, 0x65, 0x64, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x28, 0x0a, 0x0c,
    0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x0b, 0x32, 0x12, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x46, 0x72, 0x61, 0x6d, 0x65,
    0x77, 0x6f, 0x72, 0x6b, 0x49, 0x44, 0x12, 0x26, 0x0a, 0x0b, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72,
    0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6d, 0x65,
    0x73, 0x6f, 0x73, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x22, 0x70,
    0x0a, 0x1c, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x52, 0x65, 0x72, 0x65, 0x67,
    0x69, 0x73, 0x74, 0x65, 0x72, 0x65, 0x64, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x28,
    0x0a, 0x0c, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x69, 0x64, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x46, 0x72, 0x61,
    0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x49, 0x44, 0x12, 0x26, 0x0a, 0x0b, 0x6d, 0x61, 0x73, 0x74,
    0x65, 0x72, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x11, 0x2e,
    0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f,
    0x22, 0x46, 0x0a, 0x1a, 0x55, 0x6e, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x46, 0x72,
    0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x28,
    0x0a, 0x0c, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x69, 0x64, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x46, 0x72, 0x61,
    0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x49, 0x44, 0x22, 0x46, 0x0a, 0x1a, 0x44, 0x65, 0x61, 0x63,
    0x74, 0x69, 0x76, 0x61, 0x74, 0x65, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x4d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x28, 0x0a, 0x0c, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77,
    0x6f, 0x72, 0x6b, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x6d,
    0x65, 0x73, 0x6f, 0x73, 0x2e, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x49, 0x44,
    0x22, 0x64, 0x0a, 0x16, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x28, 0x0a, 0x0c, 0x66, 0x72,
    0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b,
    0x32, 0x12, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f,
    0x72, 0x6b, 0x49, 0x44, 0x12, 0x20, 0x0a, 0x08, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73,
    0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x43, 0x0a, 0x15, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72,
    0x63, 0x65, 0x4f, 0x66, 0x66, 0x65, 0x72, 0x73, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12,
    0x1c, 0x0a, 0x06, 0x6f, 0x66, 0x66, 0x65, 0x72, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x0c, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x4f, 0x66, 0x66, 0x65, 0x72, 0x12, 0x0c, 0x0a,
    0x04, 0x70, 0x69, 0x64, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x09, 0x22, 0xa2, 0x01, 0x0a, 0x12,
    0x4c, 0x61, 0x75, 0x6e, 0x63, 0x68, 0x54, 0x61, 0x73, 0x6b, 0x73, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x12, 0x28, 0x0a, 0x0c, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x5f,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73,
    0x2e, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x49, 0x44, 0x12, 0x1e, 0x0a, 0x05,
    0x74, 0x61, 0x73, 0x6b, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x6d, 0x65,
    0x73, 0x6f, 0x73, 0x2e, 0x54, 0x61, 0x73, 0x6b, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x1f, 0x0a, 0x07,
    0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x18, 0x05, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x0e, 0x2e,
    0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x46, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x12, 0x21, 0x0a,
    0x09, 0x6f, 0x66, 0x66, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x06, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x4f, 0x66, 0x66, 0x65, 0x72, 0x49, 0x44,
    0x22, 0x3f, 0x0a, 0x1b, 0x52, 0x65, 0x73, 0x63, 0x69, 0x6e, 0x64, 0x52, 0x65, 0x73, 0x6f, 0x75,
    0x72, 0x63, 0x65, 0x4f, 0x66, 0x66, 0x65, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12,
    0x20, 0x0a, 0x08, 0x6f, 0x66, 0x66, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x0b, 0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x4f, 0x66, 0x66, 0x65, 0x72, 0x49,
    0x44, 0x22, 0x3f, 0x0a, 0x13, 0x52, 0x65, 0x76, 0x69, 0x76, 0x65, 0x4f, 0x66, 0x66, 0x65, 0x72,
    0x73, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x28, 0x0a, 0x0c, 0x66, 0x72, 0x61, 0x6d,
    0x65, 0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x12,
    0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b,
    0x49, 0x44, 0x22, 0x93, 0x01, 0x0a, 0x0e, 0x52, 0x75, 0x6e, 0x54, 0x61, 0x73, 0x6b, 0x4d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x2c, 0x0a, 0x0c, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f,
    0x72, 0x6b, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x6d, 0x65,
    0x73, 0x6f, 0x73, 0x2e, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x49, 0x44, 0x42,
    0x02, 0x18, 0x01, 0x12, 0x27, 0x0a, 0x09, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b,
    0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x46,
    0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x0b, 0x0a, 0x03,
    0x70, 0x69, 0x64, 0x18, 0x03, 0x20, 0x02, 0x28, 0x09, 0x12, 0x1d, 0x0a, 0x04, 0x74, 0x61, 0x73,
    0x6b, 0x18, 0x04, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e,
    0x54, 0x61, 0x73, 0x6b, 0x49, 0x6e, 0x66, 0x6f, 0x22, 0x5b, 0x0a, 0x0f, 0x4b, 0x69, 0x6c, 0x6c,
    0x54, 0x61, 0x73, 0x6b, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x28, 0x0a, 0x0c, 0x66,
    0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x0b, 0x32, 0x12, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77,
    0x6f, 0x72, 0x6b, 0x49, 0x44, 0x12, 0x1e, 0x0a, 0x07, 0x74, 0x61, 0x73, 0x6b, 0x5f, 0x69, 0x64,
    0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x54,
    0x61, 0x73, 0x6b, 0x49, 0x44, 0x22, 0x50, 0x0a, 0x13, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x55,
    0x70, 0x64, 0x61, 0x74, 0x65, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x2c, 0x0a, 0x06,
    0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x6d,
    0x65, 0x73, 0x6f, 0x73, 0x2e, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x2e, 0x53, 0x74,
    0x61, 0x74, 0x75, 0x73, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x12, 0x0b, 0x0a, 0x03, 0x70, 0x69,
    0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x22, 0x9e, 0x01, 0x0a, 0x22, 0x53, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x41, 0x63, 0x6b, 0x6e, 0x6f, 0x77, 0x6c, 0x65,
    0x64, 0x67, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x20,
    0x0a, 0x08, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b,
    0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x53, 0x6c, 0x61, 0x76, 0x65, 0x49, 0x44,
    0x12, 0x28, 0x0a, 0x0c, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x69, 0x64,
    0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x46,
    0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x49, 0x44, 0x12, 0x1e, 0x0a, 0x07, 0x74, 0x61,
    0x73, 0x6b, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x6d, 0x65,
    0x73, 0x6f, 0x73, 0x2e, 0x54, 0x61, 0x73, 0x6b, 0x49, 0x44, 0x12, 0x0c, 0x0a, 0x04, 0x75, 0x75,
    0x69, 0x64, 0x18, 0x04, 0x20, 0x02, 0x28, 0x0c, 0x22, 0x34, 0x0a, 0x10, 0x4c, 0x6f, 0x73, 0x74,
    0x53, 0x6c, 0x61, 0x76, 0x65, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x20, 0x0a, 0x08,
    0x73, 0x6c, 0x61, 0x76, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x0e,
    0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x53, 0x6c, 0x61, 0x76, 0x65, 0x49, 0x44, 0x22, 0x66,
    0x0a, 0x15, 0x52, 0x65, 0x63, 0x6f, 0x6e, 0x63, 0x69, 0x6c, 0x65, 0x54, 0x61, 0x73, 0x6b, 0x73,
    0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x28, 0x0a, 0x0c, 0x66, 0x72, 0x61, 0x6d, 0x65,
    0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x12, 0x2e,
    0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x49,
    0x44, 0x12, 0x23, 0x0a, 0x08, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x65, 0x73, 0x18, 0x02, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x54, 0x61, 0x73, 0x6b,
    0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x22, 0x28, 0x0a, 0x15, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77,
    0x6f, 0x72, 0x6b, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12,
    0x0f, 0x0a, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09,
    0x22, 0x79, 0x0a, 0x14, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x53, 0x6c, 0x61, 0x76,
    0x65, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x1f, 0x0a, 0x05, 0x73, 0x6c, 0x61, 0x76,
    0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e,
    0x53, 0x6c, 0x61, 0x76, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x2f, 0x0a, 0x16, 0x63, 0x68, 0x65,
    0x63, 0x6b, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x65, 0x64, 0x5f, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72,
    0x63, 0x65, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x6d, 0x65, 0x73, 0x6f,
    0x73, 0x2e, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x12, 0x0f, 0x0a, 0x07, 0x76, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x22, 0x8e, 0x02, 0x0a, 0x16,
    0x52, 0x65, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x53, 0x6c, 0x61, 0x76, 0x65, 0x4d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x1f, 0x0a, 0x05, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x53, 0x6c,
    0x61, 0x76, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x2f, 0x0a, 0x16, 0x63, 0x68, 0x65, 0x63, 0x6b,
    0x70, 0x6f, 0x69, 0x6e, 0x74, 0x65, 0x64, 0x5f, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65,
    0x73, 0x18, 0x07, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e,
    0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x12, 0x2b, 0x0a, 0x0e, 0x65, 0x78, 0x65, 0x63,
    0x75, 0x74, 0x6f, 0x72, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x13, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f,
    0x72, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x23, 0x0a, 0x05, 0x74, 0x61, 0x73, 0x6b, 0x73, 0x18, 0x03,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x69, 0x6e, 0x74,
    0x65, 0x72, 0x6e, 0x61, 0x6c, 0x2e, 0x54, 0x61, 0x73, 0x6b, 0x12, 0x3f, 0x0a, 0x14, 0x63, 0x6f,
    0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x5f, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72,
    0x6b, 0x73, 0x18, 0x05, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73,
    0x2e, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x2e, 0x41, 0x72, 0x63, 0x68, 0x69, 0x76,
    0x65, 0x2e, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x12, 0x0f, 0x0a, 0x07, 0x76,
    0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x22, 0x3a, 0x0a, 0x16,
    0x53, 0x6c, 0x61, 0x76, 0x65, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x65, 0x64, 0x4d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x20, 0x0a, 0x08, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x5f,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73,
    0x2e, 0x53, 0x6c, 0x61, 0x76, 0x65, 0x49, 0x44, 0x22, 0x7c, 0x0a, 0x18, 0x53, 0x6c, 0x61, 0x76,
    0x65, 0x52, 0x65, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x65, 0x64, 0x4d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x12, 0x20, 0x0a, 0x08, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x5f, 0x69, 0x64,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x53,
    0x6c, 0x61, 0x76, 0x65, 0x49, 0x44, 0x12, 0x3e, 0x0a, 0x0f, 0x72, 0x65, 0x63, 0x6f, 0x6e, 0x63,
    0x69, 0x6c, 0x69, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32,
    0x25, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c,
    0x2e, 0x52, 0x65, 0x63, 0x6f, 0x6e, 0x63, 0x69, 0x6c, 0x65, 0x54, 0x61, 0x73, 0x6b, 0x73, 0x4d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x22, 0x3a, 0x0a, 0x16, 0x55, 0x6e, 0x72, 0x65, 0x67, 0x69,
    0x73, 0x74, 0x65, 0x72, 0x53, 0x6c, 0x61, 0x76, 0x65, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x12, 0x20, 0x0a, 0x08, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x53, 0x6c, 0x61, 0x76, 0x65,
    0x49, 0x44, 0x22, 0x25, 0x0a, 0x10, 0x50, 0x69, 0x6e, 0x67, 0x53, 0x6c, 0x61, 0x76, 0x65, 0x4d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x11, 0x0a, 0x09, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63,
    0x74, 0x65, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x08, 0x22, 0x12, 0x0a, 0x10, 0x50, 0x6f, 0x6e,
    0x67, 0x53, 0x6c, 0x61, 0x76, 0x65, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x22, 0x44, 0x0a,
    0x18, 0x53, 0x68, 0x75, 0x74, 0x64, 0x6f, 0x77, 0x6e, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f,
    0x72, 0x6b, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x28, 0x0a, 0x0c, 0x66, 0x72, 0x61,
    0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32,
    0x12, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72,
    0x6b, 0x49, 0x44, 0x22, 0x6b, 0x0a, 0x17, 0x53, 0x68, 0x75, 0x74, 0x64, 0x6f, 0x77, 0x6e, 0x45,
    0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x26,
    0x0a, 0x0b, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x45, 0x78, 0x65, 0x63,
    0x75, 0x74, 0x6f, 0x72, 0x49, 0x44, 0x12, 0x28, 0x0a, 0x0c, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77,
    0x6f, 0x72, 0x6b, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x6d,
    0x65, 0x73, 0x6f, 0x73, 0x2e, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x49, 0x44,
    0x22, 0x4f, 0x0a, 0x16, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77,
    0x6f, 0x72, 0x6b, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x28, 0x0a, 0x0c, 0x66, 0x72,
    0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b,
    0x32, 0x12, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f,
    0x72, 0x6b, 0x49, 0x44, 0x12, 0x0b, 0x0a, 0x03, 0x70, 0x69, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28,
    0x09, 0x22, 0x40, 0x0a, 0x1a, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x52,
    0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x73, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12,
    0x22, 0x0a, 0x09, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x52, 0x65, 0x73, 0x6f, 0x75,
    0x72, 0x63, 0x65, 0x22, 0x65, 0x0a, 0x1d, 0x4f, 0x76, 0x65, 0x72, 0x73, 0x75, 0x62, 0x73, 0x63,
    0x72, 0x69, 0x62, 0x65, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x73, 0x4d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x12, 0x20, 0x0a, 0x08, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x5f, 0x69, 0x64,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x53,
    0x6c, 0x61, 0x76, 0x65, 0x49, 0x44, 0x12, 0x22, 0x0a, 0x09, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72,
    0x63, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x6d, 0x65, 0x73, 0x6f,
    0x73, 0x2e, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x22, 0x6b, 0x0a, 0x17, 0x52, 0x65,
    0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x4d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x28, 0x0a, 0x0c, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f,
    0x72, 0x6b, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x6d, 0x65,
    0x73, 0x6f, 0x73, 0x2e, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x49, 0x44, 0x12,
    0x26, 0x0a, 0x0b, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x02,
    0x20, 0x02, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x45, 0x78, 0x65,
    0x63, 0x75, 0x74, 0x6f, 0x72, 0x49, 0x44, 0x22, 0xe7, 0x01, 0x0a, 0x19, 0x45, 0x78, 0x65, 0x63,
    0x75, 0x74, 0x6f, 0x72, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x65, 0x64, 0x4d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x2a, 0x0a, 0x0d, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f,
    0x72, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x6d,
    0x65, 0x73, 0x6f, 0x73, 0x2e, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x49, 0x6e, 0x66,
    0x6f, 0x12, 0x28, 0x0a, 0x0c, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x69,
    0x64, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e,
    0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x49, 0x44, 0x12, 0x2c, 0x0a, 0x0e, 0x66,
    0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x18, 0x04, 0x20,
    0x02, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x46, 0x72, 0x61, 0x6d,
    0x65, 0x77, 0x6f, 0x72, 0x6b, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x20, 0x0a, 0x08, 0x73, 0x6c, 0x61,
    0x76, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x05, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6d, 0x65,
    0x73, 0x6f, 0x73, 0x2e, 0x53, 0x6c, 0x61, 0x76, 0x65, 0x49, 0x44, 0x12, 0x24, 0x0a, 0x0a, 0x73,
    0x6c, 0x61, 0x76, 0x65, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x18, 0x06, 0x20, 0x02, 0x28, 0x0b, 0x32,
    0x10, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x53, 0x6c, 0x61, 0x76, 0x65, 0x49, 0x6e, 0x66,
    0x6f, 0x22, 0x65, 0x0a, 0x1b, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x52, 0x65, 0x72,
    0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x65, 0x64, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x12, 0x20, 0x0a, 0x08, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x53, 0x6c, 0x61, 0x76, 0x65,
    0x49, 0x44, 0x12, 0x24, 0x0a, 0x0a, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x5f, 0x69, 0x6e, 0x66, 0x6f,
    0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x53,
    0x6c, 0x61, 0x76, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x22, 0x9b, 0x01, 0x0a, 0x15, 0x45, 0x78, 0x69,
    0x74, 0x65, 0x64, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x12, 0x20, 0x0a, 0x08, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x53, 0x6c, 0x61,
    0x76, 0x65, 0x49, 0x44, 0x12, 0x28, 0x0a, 0x0c, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72,
    0x6b, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x6d, 0x65, 0x73,
    0x6f, 0x73, 0x2e, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x49, 0x44, 0x12, 0x26,
    0x0a, 0x0b, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20,
    0x02, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x45, 0x78, 0x65, 0x63,
    0x75, 0x74, 0x6f, 0x72, 0x49, 0x44, 0x12, 0x0e, 0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x18, 0x04, 0x20, 0x02, 0x28, 0x05, 0x22, 0x3c, 0x0a, 0x18, 0x52, 0x65, 0x63, 0x6f, 0x6e, 0x6e,
    0x65, 0x63, 0x74, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x12, 0x20, 0x0a, 0x08, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x53, 0x6c, 0x61,
    0x76, 0x65, 0x49, 0x44, 0x22, 0xbc, 0x01, 0x0a, 0x19, 0x52, 0x65, 0x72, 0x65, 0x67, 0x69, 0x73,
    0x74, 0x65, 0x72, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x12, 0x26, 0x0a, 0x0b, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x5f, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e,
    0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x49, 0x44, 0x12, 0x28, 0x0a, 0x0c, 0x66, 0x72,
    0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b,
    0x32, 0x12, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f,
    0x72, 0x6b, 0x49, 0x44, 0x12, 0x1e, 0x0a, 0x05, 0x74, 0x61, 0x73, 0x6b, 0x73, 0x18, 0x03, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x54, 0x61, 0x73, 0x6b,
    0x49, 0x6e, 0x66, 0x6f, 0x12, 0x2d, 0x0a, 0x07, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73, 0x18,
    0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x69, 0x6e,
    0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x2e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x55, 0x70, 0x64,
    0x61, 0x74, 0x65, 0x22, 0x22, 0x0a, 0x0f, 0x53, 0x68, 0x75, 0x74, 0x64, 0x6f, 0x77, 0x6e, 0x4d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x0f, 0x0a, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x22, 0xad, 0x01, 0x0a, 0x07, 0x41, 0x72, 0x63, 0x68,
    0x69, 0x76, 0x65, 0x12, 0x35, 0x0a, 0x0a, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b,
    0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e,
    0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x2e, 0x41, 0x72, 0x63, 0x68, 0x69, 0x76, 0x65,
    0x2e, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x1a, 0x6b, 0x0a, 0x09, 0x46, 0x72,
    0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x12, 0x2c, 0x0a, 0x0e, 0x66, 0x72, 0x61, 0x6d, 0x65,
    0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32,
    0x14, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72,
    0x6b, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x0b, 0x0a, 0x03, 0x70, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x09, 0x12, 0x23, 0x0a, 0x05, 0x74, 0x61, 0x73, 0x6b, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x14, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e,
    0x61, 0x6c, 0x2e, 0x54, 0x61, 0x73, 0x6b, 0x22, 0x7b, 0x0a, 0x10, 0x54, 0x61, 0x73, 0x6b, 0x48,
    0x65, 0x61, 0x6c, 0x74, 0x68, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x1e, 0x0a, 0x07, 0x74,
    0x61, 0x73, 0x6b, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x6d,
    0x65, 0x73, 0x6f, 0x73, 0x2e, 0x54, 0x61, 0x73, 0x6b, 0x49, 0x44, 0x12, 0x0f, 0x0a, 0x07, 0x68,
    0x65, 0x61, 0x6c, 0x74, 0x68, 0x79, 0x18, 0x02, 0x20, 0x02, 0x28, 0x08, 0x12, 0x18, 0x0a, 0x09,
    0x6b, 0x69, 0x6c, 0x6c, 0x5f, 0x74, 0x61, 0x73, 0x6b, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x3a,
    0x05, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x12, 0x1c, 0x0a, 0x14, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x63,
    0x75, 0x74, 0x69, 0x76, 0x65, 0x5f, 0x66, 0x61, 0x69, 0x6c, 0x75, 0x72, 0x65, 0x73, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x05, 0x22, 0x1e, 0x0a, 0x0c, 0x48, 0x6f, 0x6f, 0x6b, 0x45, 0x78, 0x65, 0x63,
    0x75, 0x74, 0x65, 0x64, 0x12, 0x0e, 0x0a, 0x06, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x09, 0x4a, 0xbf, 0x70, 0x0a, 0x07, 0x12, 0x05, 0x12, 0x00, 0xac, 0x03, 0x01,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x12, 0x07, 0x14, 0x0a, 0x08, 0x0a, 0x01, 0x02,
    0x12, 0x03, 0x14, 0x08, 0x16, 0x0a, 0x8d, 0x05, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x28, 0x00,
    0x3f, 0x01, 0x1a, 0x80, 0x05, 0x20, 0x54, 0x4f, 0x44, 0x4f, 0x28, 0x62, 0x65, 0x6e, 0x68, 0x29,
    0x3a, 0x20, 0x49, 0x74, 0x20, 0x77, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x67, 0x72,
    0x65, 0x61, 0x74, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x63, 0x6f, 0x75, 0x6c,
    0x64, 0x20, 0x6a, 0x75, 0x73, 0x74, 0x20, 0x62, 0x65, 0x20, 0x61, 0x0a, 0x20, 0x54, 0x61, 0x73,
    0x6b, 0x49, 0x6e, 0x66, 0x6f, 0x20, 0x77, 0x68, 0x65, 0x72, 0x65, 0x76, 0x65, 0x72, 0x20, 0x69,
    0x74, 0x20, 0x67, 0x65, 0x74, 0x73, 0x20, 0x75, 0x73, 0x65, 0x64, 0x21, 0x20, 0x48, 0x6f, 0x77,
    0x65, 0x76, 0x65, 0x72, 0x2c, 0x20, 0x64, 0x6f, 0x69, 0x6e, 0x67, 0x20, 0x73, 0x6f, 0x20, 0x77,
    0x6f, 0x75, 0x6c, 0x64, 0x0a, 0x20, 0x72, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x20, 0x61, 0x64,
    0x64, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f,
    0x72, 0x6b, 0x5f, 0x69, 0x64, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x2c, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x5f, 0x69, 0x64, 0x20, 0x66, 0x69, 0x65,
    0x6c, 0x64, 0x2c, 0x20, 0x61, 0x6e, 0x64, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x74, 0x61,
    0x74, 0x65, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x69, 0x6e, 0x74, 0x6f, 0x20, 0x54, 0x61,
    0x73, 0x6b, 0x49, 0x6e, 0x66, 0x6f, 0x20, 0x74, 0x68, 0x6f, 0x75, 0x67, 0x68, 0x20, 0x28, 0x6f,
    0x72, 0x20, 0x73, 0x65, 0x6e, 0x64, 0x20, 0x74, 0x68, 0x65, 0x6d, 0x20, 0x61, 0x6e, 0x6f, 0x74,
    0x68, 0x65, 0x72, 0x0a, 0x20, 0x77, 0x61, 0x79, 0x29, 0x2e, 0x20, 0x41, 0x6c, 0x73, 0x6f, 0x2c,
    0x20, 0x6f, 0x6e, 0x65, 0x20, 0x70, 0x65, 0x72, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x6e, 0x63, 0x65,
    0x20, 0x72, 0x65, 0x61, 0x73, 0x6f, 0x6e, 0x20, 0x77, 0x68, 0x79, 0x20, 0x77, 0x65, 0x20, 0x64,
    0x6f, 0x6e, 0x27, 0x74, 0x20, 0x64, 0x6f, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x6e, 0x6f, 0x77,
    0x20, 0x69, 0x73, 0x0a, 0x20, 0x62, 0x65, 0x63, 0x61, 0x75, 0x73, 0x65, 0x20, 0x73, 0x74, 0x6f,
    0x72, 0x69, 0x6e, 0x67, 0x20, 0x77, 0x68, 0x61, 0x74, 0x65, 0x76, 0x65, 0x72, 0x20, 0x64, 0x61,
    0x74, 0x61, 0x20, 0x69, 0x73, 0x20, 0x63, 0x6f, 0x75, 0x70, 0x6c, 0x65, 0x64, 0x20, 0x77, 0x69,
    0x74, 0x68, 0x20, 0x61, 0x20, 0x54, 0x61, 0x73, 0x6b, 0x49, 0x6e, 0x66, 0x6f, 0x0a, 0x20, 0x63,
    0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x6c, 0x61, 0x72, 0x67, 0x65, 0x20, 0x61, 0x6e,
    0x64, 0x20, 0x75, 0x6e, 0x6e, 0x65, 0x63, 0x65, 0x73, 0x73, 0x61, 0x72, 0x79, 0x2e, 0x0a, 0x20,
    0x54, 0x4f, 0x44, 0x4f, 0x28, 0x62, 0x6d, 0x61, 0x68, 0x6c, 0x65, 0x72, 0x29, 0x3a, 0x20, 0x41,
    0x64, 0x64, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x5f, 0x75, 0x75, 0x69, 0x64,
    0x20, 0x68, 0x65, 0x72, 0x65, 0x2c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x73, 0x65, 0x6e, 0x64, 0x20,
    0x69, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72,
    0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x0a, 0x20, 0x61, 0x6c, 0x6c,
    0x6f, 0x77, 0x20, 0x75, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x65, 0x78, 0x70, 0x6f, 0x73, 0x65, 0x20,
    0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x20, 0x77, 0x6f, 0x72, 0x6b, 0x20, 0x64, 0x69,
    0x72, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x69, 0x65, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x61,
    0x73, 0x6b, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x77, 0x65, 0x62, 0x75, 0x69,
    0x20, 0x77, 0x68, 0x65, 0x6e, 0x0a, 0x20, 0x6c, 0x6f, 0x6f, 0x6b, 0x69, 0x6e, 0x67, 0x20, 0x66,
    0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x20, 0x6c,
    0x65, 0x76, 0x65, 0x6c, 0x2e, 0x20, 0x43, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x6c, 0x79, 0x20,
    0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x20, 0x6b,
    0x6e, 0x6f, 0x77, 0x73, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x72, 0x75, 0x6e, 0x20, 0x74,
    0x68, 0x65, 0x0a, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x20, 0x62, 0x65, 0x6c, 0x6f, 0x6e, 0x67, 0x73,
    0x20, 0x74, 0x6f, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x28, 0x08,
    0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x29, 0x02, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x29, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x29, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x29, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x29, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x2a, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2a, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x2a, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2a, 0x12, 0x19, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2a, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x02, 0x12, 0x03, 0x2b, 0x02, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x2b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12,
    0x03, 0x2b, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2b,
    0x17, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2b, 0x26, 0x27,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x2c, 0x02, 0x26, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x2c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x2c, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x2c, 0x16, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x2c, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x2d,
    0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x2d, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x06, 0x12, 0x03, 0x2d, 0x0b, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x2d, 0x13, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x2d, 0x1e, 0x1f, 0x0a, 0x28, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x05, 0x12, 0x03, 0x2e, 0x02, 0x1f, 0x22, 0x1b, 0x20, 0x4c, 0x61, 0x74, 0x65, 0x73, 0x74,
    0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x61,
    0x73, 0x6b, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x2e,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x06, 0x12, 0x03, 0x2e, 0x0b, 0x14,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x2e, 0x15, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x2e, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x2f, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x06, 0x04, 0x12, 0x03, 0x2f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x06,
    0x12, 0x03, 0x2f, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03,
    0x2f, 0x14, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12, 0x03, 0x2f, 0x20,
    0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x30, 0x02, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x04, 0x12, 0x03, 0x30, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x07, 0x06, 0x12, 0x03, 0x30, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x07, 0x01, 0x12, 0x03, 0x30, 0x16, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07,
    0x03, 0x12, 0x03, 0x30, 0x21, 0x22, 0x0a, 0xb4, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08, 0x12,
    0x03, 0x35, 0x02, 0x2d, 0x1a, 0xa6, 0x01, 0x20, 0x54, 0x68, 0x65, 0x73, 0x65, 0x20, 0x66, 0x69,
    0x65, 0x6c, 0x64, 0x73, 0x20, 0x63, 0x6f, 0x72, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x64, 0x20,
    0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x20, 0x61, 0x6e, 0x64,
    0x20, 0x75, 0x75, 0x69, 0x64, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x61, 0x74,
    0x65, 0x73, 0x74, 0x0a, 0x20, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x20, 0x75, 0x70, 0x64, 0x61,
    0x74, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x0a, 0x20, 0x4e, 0x4f, 0x54,
    0x45, 0x3a, 0x20, 0x45, 0x69, 0x74, 0x68, 0x65, 0x72, 0x20, 0x62, 0x6f, 0x74, 0x68, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x20, 0x6d, 0x75, 0x73, 0x74, 0x20, 0x62,
    0x65, 0x20, 0x73, 0x65, 0x74, 0x20, 0x6f, 0x72, 0x20, 0x62, 0x6f, 0x74, 0x68, 0x20, 0x6d, 0x75,
    0x73, 0x74, 0x20, 0x62, 0x65, 0x20, 0x75, 0x6e, 0x73, 0x65, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x08, 0x04, 0x12, 0x03, 0x35, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x08, 0x06, 0x12, 0x03, 0x35, 0x0b, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x08, 0x01, 0x12, 0x03, 0x35, 0x15, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x03,
    0x12, 0x03, 0x35, 0x2b, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x09, 0x12, 0x03, 0x36,
    0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x04, 0x12, 0x03, 0x36, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x05, 0x12, 0x03, 0x36, 0x0b, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x36, 0x11, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x09, 0x03, 0x12, 0x03, 0x36, 0x26, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x0a, 0x12, 0x03, 0x38, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x04,
    0x12, 0x03, 0x38, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x06, 0x12, 0x03,
    0x38, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x38, 0x12,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x38, 0x1b, 0x1d, 0x0a,
    0xf0, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x0b, 0x12, 0x03, 0x3e, 0x02, 0x28, 0x1a, 0xe2, 0x01,
    0x20, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x64, 0x69, 0x73, 0x63, 0x6f, 0x76, 0x65,
    0x72, 0x79, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66,
    0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x2e, 0x20, 0x49, 0x74, 0x20,
    0x69, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x70, 0x72, 0x65, 0x74,
    0x65, 0x64, 0x0a, 0x20, 0x6f, 0x72, 0x20, 0x61, 0x63, 0x74, 0x65, 0x64, 0x20, 0x75, 0x70, 0x6f,
    0x6e, 0x20, 0x62, 0x79, 0x20, 0x4d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x20, 0x49, 0x74, 0x20, 0x69,
    0x73, 0x20, 0x75, 0x70, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x20, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63,
    0x65, 0x20, 0x64, 0x69, 0x73, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x20, 0x73, 0x79, 0x73, 0x74,
    0x65, 0x6d, 0x0a, 0x20, 0x74, 0x6f, 0x20, 0x75, 0x73, 0x65, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20,
    0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x61, 0x73, 0x20, 0x6e,
    0x65, 0x65, 0x64, 0x65, 0x64, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x68, 0x61, 0x6e,
    0x64, 0x6c, 0x65, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x73, 0x20, 0x77, 0x69, 0x74, 0x68, 0x6f, 0x75,
    0x74, 0x0a, 0x20, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x64, 0x69, 0x73, 0x63, 0x6f,
    0x76, 0x65, 0x72, 0x79, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x3e, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x06, 0x12, 0x03, 0x3e, 0x0b, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x3e, 0x19, 0x22, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x3e, 0x25, 0x27, 0x0a, 0x3a, 0x0a, 0x02, 0x04, 0x01,
    0x12, 0x04, 0x43, 0x00, 0x51, 0x01, 0x1a, 0x2e, 0x20, 0x54, 0x4f, 0x44, 0x4f, 0x28, 0x76, 0x69,
    0x6e, 0x6f, 0x64, 0x29, 0x3a, 0x20, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x20, 0x61, 0x20, 0x6e,
    0x65, 0x77, 0x20, 0x55, 0x55, 0x49, 0x44, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20,
    0x74, 0x79, 0x70, 0x65, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x43,
    0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x44, 0x02, 0x28, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x44, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x44, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x44, 0x17, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x44, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12,
    0x03, 0x45, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x45,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x45, 0x0b, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x45, 0x16, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x45, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x46, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x46, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x06,
    0x12, 0x03, 0x46, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x46, 0x13, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x46, 0x1e,
    0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x47, 0x02, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x03, 0x47, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x03, 0x06, 0x12, 0x03, 0x47, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x47, 0x16, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x47, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12, 0x03,
    0x48, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x04, 0x12, 0x03, 0x48, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x05, 0x12, 0x03, 0x48, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x48, 0x12, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x48, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x05, 0x12, 0x03, 0x49, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05,
    0x04, 0x12, 0x03, 0x49, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x05, 0x12,
    0x03, 0x49, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x01, 0x12, 0x03, 0x49,
    0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x03, 0x12, 0x03, 0x49, 0x18, 0x19,
    0x0a, 0xd9, 0x02, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x06, 0x12, 0x03, 0x50, 0x02, 0x26, 0x1a, 0xcb,
    0x02, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x63, 0x6f, 0x72, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x64, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x61, 0x74, 0x65, 0x73, 0x74,
    0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x61,
    0x73, 0x6b, 0x20, 0x61, 0x63, 0x63, 0x6f, 0x72, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x6f, 0x20,
    0x74, 0x68, 0x65, 0x0a, 0x20, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x2e, 0x20, 0x4e, 0x6f, 0x74, 0x65,
    0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65,
    0x20, 0x6d, 0x69, 0x67, 0x68, 0x74, 0x20, 0x62, 0x65, 0x20, 0x64, 0x69, 0x66, 0x66, 0x65, 0x72,
    0x65, 0x6e, 0x74, 0x20, 0x74, 0x68, 0x61, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x74, 0x61,
    0x74, 0x65, 0x20, 0x69, 0x6e, 0x0a, 0x20, 0x27, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x27, 0x20,
    0x62, 0x65, 0x63, 0x61, 0x75, 0x73, 0x65, 0x20, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x20, 0x75,
    0x70, 0x64, 0x61, 0x74, 0x65, 0x20, 0x6d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x20, 0x71, 0x75,
    0x65, 0x75, 0x65, 0x73, 0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73, 0x2e, 0x20, 0x49, 0x6e,
    0x20, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x0a, 0x20, 0x77, 0x6f, 0x72, 0x64, 0x73, 0x2c, 0x20, 0x27,
    0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x27, 0x20, 0x63, 0x6f, 0x72, 0x72, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x64, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x75, 0x70, 0x64, 0x61, 0x74,
    0x65, 0x20, 0x61, 0x74, 0x20, 0x74, 0x6f, 0x70, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x71, 0x75, 0x65, 0x75, 0x65, 0x20, 0x61, 0x6e, 0x64, 0x0a, 0x20, 0x27, 0x6c, 0x61, 0x74, 0x65,
    0x73, 0x74, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x65, 0x27, 0x20, 0x63, 0x6f, 0x72, 0x72, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x64, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x75, 0x70, 0x64,
    0x61, 0x74, 0x65, 0x20, 0x61, 0x74, 0x20, 0x62, 0x6f, 0x74, 0x74, 0x6f, 0x6d, 0x20, 0x6f, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x71, 0x75, 0x65, 0x75, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x06, 0x04, 0x12, 0x03, 0x50, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x06, 0x06, 0x12, 0x03, 0x50, 0x0b, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06,
    0x01, 0x12, 0x03, 0x50, 0x15, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x03, 0x12,
    0x03, 0x50, 0x24, 0x25, 0x0a, 0xc2, 0x01, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x57, 0x00, 0x5f,
    0x01, 0x1a, 0xb5, 0x01, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x20, 0x65, 0x6e, 0x63, 0x61, 0x70, 0x73, 0x75, 0x6c, 0x61, 0x74, 0x65, 0x73, 0x20, 0x68,
    0x6f, 0x77, 0x20, 0x77, 0x65, 0x20, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x70, 0x6f, 0x69, 0x6e, 0x74,
    0x20, 0x61, 0x20, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65,
    0x20, 0x74, 0x6f, 0x20, 0x64, 0x69, 0x73, 0x6b, 0x2e, 0x0a, 0x20, 0x4e, 0x4f, 0x54, 0x45, 0x3a,
    0x20, 0x49, 0x66, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x3d, 0x3d, 0x20, 0x55, 0x50, 0x44, 0x41,
    0x54, 0x45, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x27, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x27,
    0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x69, 0x73, 0x20, 0x72, 0x65, 0x71, 0x75, 0x69, 0x72,
    0x65, 0x64, 0x2e, 0x0a, 0x20, 0x4e, 0x4f, 0x54, 0x45, 0x3a, 0x20, 0x49, 0x66, 0x20, 0x74, 0x79,
    0x70, 0x65, 0x20, 0x3d, 0x3d, 0x20, 0x41, 0x43, 0x4b, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x27,
    0x75, 0x75, 0x69, 0x64, 0x27, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x69, 0x73, 0x20, 0x72,
    0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01,
    0x12, 0x03, 0x57, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x02, 0x04, 0x00, 0x12, 0x04, 0x58,
    0x02, 0x5b, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x04, 0x00, 0x01, 0x12, 0x03, 0x58, 0x07,
    0x0b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x02, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x59, 0x04, 0x0f,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x59, 0x04, 0x0a,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x02, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x59, 0x0d, 0x0e,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x02, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x5a, 0x04, 0x0c, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x02, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x5a, 0x04, 0x07, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x02, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x5a, 0x0a, 0x0b, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x5c, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x5c, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x5c, 0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x5c, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x5d, 0x02,
    0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x5d, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x06, 0x12, 0x03, 0x5d, 0x0b, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x5d, 0x18, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x5d, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x02, 0x12, 0x03, 0x5e, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x5e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x5e,
    0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x5e, 0x11, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x5e, 0x18, 0x19, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x62, 0x00, 0x65, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03,
    0x01, 0x12, 0x03, 0x62, 0x08, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03,
    0x64, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x64, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x64, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x64, 0x12, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x64, 0x19, 0x1a, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x04, 0x12, 0x04, 0x68, 0x00, 0x6b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03,
    0x68, 0x08, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x6a, 0x02, 0x19,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x6a, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x6a, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6a, 0x10, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x6a, 0x17, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04,
    0x6e, 0x00, 0x73, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x6e, 0x08, 0x22,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x6f, 0x02, 0x20, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x6f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x00, 0x06, 0x12, 0x03, 0x6f, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x6f, 0x13, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x6f, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x70,
    0x02, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x70, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x06, 0x12, 0x03, 0x70, 0x0b, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x70, 0x17, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x70, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05,
    0x02, 0x02, 0x12, 0x03, 0x71, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04,
    0x12, 0x03, 0x71, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x06, 0x12, 0x03,
    0x71, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x71, 0x16,
    0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x71, 0x24, 0x25, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x03, 0x12, 0x03, 0x72, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x03, 0x04, 0x12, 0x03, 0x72, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x72, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x72, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x72, 0x18, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x76, 0x00, 0x7b, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x76, 0x08, 0x22, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x77, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x77, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x77, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x77, 0x13, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x77, 0x1e,
    0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x78, 0x02, 0x28, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x03, 0x78, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x01, 0x06, 0x12, 0x03, 0x78, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x78, 0x17, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x78, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x03,
    0x79, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x04, 0x12, 0x03, 0x79, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x06, 0x12, 0x03, 0x79, 0x0b, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12, 0x03, 0x79, 0x16, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03, 0x79, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x06, 0x02, 0x03, 0x12, 0x03, 0x7a, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03,
    0x04, 0x12, 0x03, 0x7a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x05, 0x12,
    0x03, 0x7a, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x01, 0x12, 0x03, 0x7a,
    0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x03, 0x12, 0x03, 0x7a, 0x18, 0x19,
    0x0a, 0x0b, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x05, 0x7e, 0x00, 0x80, 0x01, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x7e, 0x08, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02,
    0x00, 0x12, 0x03, 0x7f, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x7f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x06, 0x12, 0x03, 0x7f,
    0x0b, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x7f, 0x19, 0x22,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x7f, 0x25, 0x26, 0x0a, 0x0c,
    0x0a, 0x02, 0x04, 0x08, 0x12, 0x06, 0x83, 0x01, 0x00, 0x86, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x08, 0x01, 0x12, 0x04, 0x83, 0x01, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x02,
    0x00, 0x12, 0x04, 0x84, 0x01, 0x02, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x84, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x06, 0x12,
    0x04, 0x84, 0x01, 0x0b, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x04,
    0x84, 0x01, 0x19, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x04, 0x84,
    0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x04, 0x85, 0x01, 0x02,
    0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x04, 0x85, 0x01, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x05, 0x12, 0x04, 0x85, 0x01, 0x0b, 0x0f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x04, 0x85, 0x01, 0x10, 0x18, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x04, 0x85, 0x01, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x09, 0x12, 0x06, 0x89, 0x01, 0x00, 0x8c, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x09, 0x01, 0x12, 0x04, 0x89, 0x01, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00,
    0x12, 0x04, 0x8a, 0x01, 0x02, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x8a, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x06, 0x12, 0x04,
    0x8a, 0x01, 0x0b, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8a,
    0x01, 0x17, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8a, 0x01,
    0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x04, 0x8b, 0x01, 0x02, 0x26,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x04, 0x12, 0x04, 0x8b, 0x01, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x06, 0x12, 0x04, 0x8b, 0x01, 0x0b, 0x15, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8b, 0x01, 0x16, 0x21, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8b, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x0a, 0x12, 0x06, 0x8e, 0x01, 0x00, 0x91, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0a,
    0x01, 0x12, 0x04, 0x8e, 0x01, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12,
    0x04, 0x8f, 0x01, 0x02, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x8f, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x06, 0x12, 0x04, 0x8f,
    0x01, 0x0b, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8f, 0x01,
    0x17, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8f, 0x01, 0x26,
    0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x01, 0x12, 0x04, 0x90, 0x01, 0x02, 0x26, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x04, 0x12, 0x04, 0x90, 0x01, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x06, 0x12, 0x04, 0x90, 0x01, 0x0b, 0x15, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0a, 0x02, 0x01, 0x01, 0x12, 0x04, 0x90, 0x01, 0x16, 0x21, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x01, 0x03, 0x12, 0x04, 0x90, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x0b, 0x12, 0x06, 0x93, 0x01, 0x00, 0x95, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0b, 0x01,
    0x12, 0x04, 0x93, 0x01, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x04,
    0x94, 0x01, 0x02, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x04, 0x94,
    0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x06, 0x12, 0x04, 0x94, 0x01,
    0x0b, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x04, 0x94, 0x01, 0x17,
    0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x04, 0x94, 0x01, 0x26, 0x27,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x06, 0x98, 0x01, 0x00, 0x9a, 0x01, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x04, 0x98, 0x01, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0c, 0x02, 0x00, 0x12, 0x04, 0x99, 0x01, 0x02, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x99, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00,
    0x06, 0x12, 0x04, 0x99, 0x01, 0x0b, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x99, 0x01, 0x17, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12,
    0x04, 0x99, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0d, 0x12, 0x06, 0x9d, 0x01, 0x00,
    0xa0, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0d, 0x01, 0x12, 0x04, 0x9d, 0x01, 0x08, 0x1e,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x04, 0x9e, 0x01, 0x02, 0x28, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x04, 0x9e, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x00, 0x06, 0x12, 0x04, 0x9e, 0x01, 0x0b, 0x16, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9e, 0x01, 0x17, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9e, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d,
    0x02, 0x01, 0x12, 0x04, 0x9f, 0x01, 0x02, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01,
    0x04, 0x12, 0x04, 0x9f, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x06,
    0x12, 0x04, 0x9f, 0x01, 0x0b, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x01, 0x12,
    0x04, 0x9f, 0x01, 0x13, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x03, 0x12, 0x04,
    0x9f, 0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0e, 0x12, 0x06, 0xa3, 0x01, 0x00, 0xa6,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x04, 0xa3, 0x01, 0x08, 0x1d, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x04, 0xa4, 0x01, 0x02, 0x1c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa4, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x00, 0x06, 0x12, 0x04, 0xa4, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa4, 0x01, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xa4, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02,
    0x01, 0x12, 0x04, 0xa5, 0x01, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x04,
    0x12, 0x04, 0xa5, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x05, 0x12,
    0x04, 0xa5, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xa5, 0x01, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa5,
    0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x06, 0xa9, 0x01, 0x00, 0xae, 0x01,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x04, 0xa9, 0x01, 0x08, 0x1a, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x0f, 0x02, 0x00, 0x12, 0x04, 0xaa, 0x01, 0x02, 0x28, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x00, 0x04, 0x12, 0x04, 0xaa, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x00, 0x06, 0x12, 0x04, 0xaa, 0x01, 0x0b, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xaa, 0x01, 0x17, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xaa, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x01,
    0x12, 0x04, 0xab, 0x01, 0x02, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x04, 0x12,
    0x04, 0xab, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x06, 0x12, 0x04,
    0xab, 0x01, 0x0b, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x01, 0x12, 0x04, 0xab,
    0x01, 0x14, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x03, 0x12, 0x04, 0xab, 0x01,
    0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x02, 0x12, 0x04, 0xac, 0x01, 0x02, 0x1f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x04, 0x12, 0x04, 0xac, 0x01, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x06, 0x12, 0x04, 0xac, 0x01, 0x0b, 0x12, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x01, 0x12, 0x04, 0xac, 0x01, 0x13, 0x1a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x02, 0x03, 0x12, 0x04, 0xac, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x0f, 0x02, 0x03, 0x12, 0x04, 0xad, 0x01, 0x02, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x03, 0x04, 0x12, 0x04, 0xad, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x03, 0x06, 0x12, 0x04, 0xad, 0x01, 0x0b, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03,
    0x01, 0x12, 0x04, 0xad, 0x01, 0x13, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x03,
    0x12, 0x04, 0xad, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x06, 0xb1, 0x01,
    0x00, 0xb3, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x04, 0xb1, 0x01, 0x08,
    0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x00, 0x12, 0x04, 0xb2, 0x01, 0x02, 0x20, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb2, 0x01, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x06, 0x12, 0x04, 0xb2, 0x01, 0x0b, 0x12, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb2, 0x01, 0x13, 0x1b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb2, 0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x11, 0x12, 0x06, 0xb6, 0x01, 0x00, 0xb8, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x11, 0x01,
    0x12, 0x04, 0xb6, 0x01, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00, 0x12, 0x04,
    0xb7, 0x01, 0x02, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb7,
    0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x06, 0x12, 0x04, 0xb7, 0x01,
    0x0b, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb7, 0x01, 0x17,
    0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb7, 0x01, 0x26, 0x27,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x12, 0x12, 0x06, 0xbb, 0x01, 0x00, 0xc1, 0x01, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x12, 0x01, 0x12, 0x04, 0xbb, 0x01, 0x08, 0x16, 0x0a, 0x4e, 0x0a, 0x04, 0x04,
    0x12, 0x02, 0x00, 0x12, 0x04, 0xbd, 0x01, 0x02, 0x3c, 0x1a, 0x40, 0x20, 0x54, 0x4f, 0x44, 0x4f,
    0x28, 0x6b, 0x61, 0x72, 0x79, 0x61, 0x29, 0x3a, 0x20, 0x52, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x20,
    0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x69, 0x64, 0x20, 0x61, 0x66, 0x74,
    0x65, 0x72, 0x20, 0x4d, 0x45, 0x53, 0x4f, 0x53, 0x2d, 0x32, 0x35, 0x35, 0x39, 0x20, 0x68, 0x61,
    0x73, 0x20, 0x73, 0x68, 0x69, 0x70, 0x70, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x12, 0x02, 0x00, 0x04, 0x12, 0x04, 0xbd, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12,
    0x02, 0x00, 0x06, 0x12, 0x04, 0xbd, 0x01, 0x0b, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xbd, 0x01, 0x17, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xbd, 0x01, 0x26, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x08,
    0x12, 0x04, 0xbd, 0x01, 0x28, 0x3b, 0x0a, 0x10, 0x0a, 0x08, 0x04, 0x12, 0x02, 0x00, 0x08, 0xe7,
    0x07, 0x00, 0x12, 0x04, 0xbd, 0x01, 0x29, 0x3a, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x12, 0x02, 0x00,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x04, 0xbd, 0x01, 0x29, 0x33, 0x0a, 0x12, 0x0a, 0x0a, 0x04,
    0x12, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x04, 0xbd, 0x01, 0x29, 0x33, 0x0a,
    0x13, 0x0a, 0x0b, 0x04, 0x12, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xbd, 0x01, 0x29, 0x33, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x12, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00,
    0x03, 0x12, 0x04, 0xbd, 0x01, 0x36, 0x3a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x01, 0x12,
    0x04, 0xbe, 0x01, 0x02, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x04, 0x12, 0x04,
    0xbe, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x06, 0x12, 0x04, 0xbe,
    0x01, 0x0b, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x01, 0x12, 0x04, 0xbe, 0x01,
    0x19, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x03, 0x12, 0x04, 0xbe, 0x01, 0x25,
    0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x02, 0x12, 0x04, 0xbf, 0x01, 0x02, 0x1a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02, 0x04, 0x12, 0x04, 0xbf, 0x01, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x12, 0x02, 0x02, 0x05, 0x12, 0x04, 0xbf, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x12, 0x02, 0x02, 0x01, 0x12, 0x04, 0xbf, 0x01, 0x12, 0x15, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x12, 0x02, 0x02, 0x03, 0x12, 0x04, 0xbf, 0x01, 0x18, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x12, 0x02, 0x03, 0x12, 0x04, 0xc0, 0x01, 0x02, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02,
    0x03, 0x04, 0x12, 0x04, 0xc0, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x03,
    0x06, 0x12, 0x04, 0xc0, 0x01, 0x0b, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x03, 0x01,
    0x12, 0x04, 0xc0, 0x01, 0x14, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x03, 0x03, 0x12,
    0x04, 0xc0, 0x01, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x13, 0x12, 0x06, 0xc4, 0x01, 0x00,
    0xc9, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x13, 0x01, 0x12, 0x04, 0xc4, 0x01, 0x08, 0x17,
    0x0a, 0x7d, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x00, 0x12, 0x04, 0xc7, 0x01, 0x02, 0x28, 0x1a, 0x6f,
    0x20, 0x54, 0x4f, 0x44, 0x4f, 0x28, 0x62, 0x6d, 0x61, 0x68, 0x6c, 0x65, 0x72, 0x29, 0x3a, 0x20,
    0x49, 0x6e, 0x63, 0x6c, 0x75, 0x64, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x53, 0x6c, 0x61, 0x76,
    0x65, 0x49, 0x44, 0x20, 0x68, 0x65, 0x72, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x69, 0x6d, 0x70, 0x72,
    0x6f, 0x76, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x27, 0x73,
    0x0a, 0x20, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x6e, 0x6f, 0x6e, 0x2d, 0x61, 0x63, 0x74,
    0x69, 0x76, 0x61, 0x74, 0x65, 0x64, 0x20, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x73, 0x2e, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc7, 0x01, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x06, 0x12, 0x04, 0xc7, 0x01, 0x0b, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc7, 0x01, 0x17, 0x23, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc7, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x13, 0x02, 0x01, 0x12, 0x04, 0xc8, 0x01, 0x02, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xc8, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01,
    0x06, 0x12, 0x04, 0xc8, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xc8, 0x01, 0x12, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xc8, 0x01, 0x1c, 0x1d, 0x0a, 0x61, 0x0a, 0x02, 0x04, 0x14, 0x12, 0x06, 0xce, 0x01, 0x00,
    0xd1, 0x01, 0x01, 0x1a, 0x53, 0x20, 0x4e, 0x4f, 0x54, 0x45, 0x3a, 0x20, 0x49, 0x66, 0x20, 0x27,
    0x70, 0x69, 0x64, 0x27, 0x20, 0x69, 0x73, 0x20, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x2c,
    0x20, 0x73, 0x63, 0x68, 0x65, 0x64, 0x75, 0x6c, 0x65, 0x72, 0x20, 0x64, 0x72, 0x69, 0x76, 0x65,
    0x72, 0x20, 0x73, 0x65, 0x6e, 0x64, 0x73, 0x20, 0x61, 0x6e, 0x0a, 0x20, 0x61, 0x63, 0x6b, 0x6e,
    0x6f, 0x77, 0x6c, 0x65, 0x64, 0x67, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x70, 0x69, 0x64, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x14, 0x01, 0x12,
    0x04, 0xce, 0x01, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x00, 0x12, 0x04, 0xcf,
    0x01, 0x02, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x04, 0x12, 0x04, 0xcf, 0x01,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x06, 0x12, 0x04, 0xcf, 0x01, 0x0b,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x01, 0x12, 0x04, 0xcf, 0x01, 0x18, 0x1e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x03, 0x12, 0x04, 0xcf, 0x01, 0x21, 0x22, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x01, 0x12, 0x04, 0xd0, 0x01, 0x02, 0x1a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd0, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x14, 0x02, 0x01, 0x05, 0x12, 0x04, 0xd0, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd0, 0x01, 0x12, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14,
    0x02, 0x01, 0x03, 0x12, 0x04, 0xd0, 0x01, 0x18, 0x19, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x15, 0x12,
    0x06, 0xd4, 0x01, 0x00, 0xd9, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x15, 0x01, 0x12, 0x04,
    0xd4, 0x01, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x00, 0x12, 0x04, 0xd5, 0x01,
    0x02, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd5, 0x01, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x06, 0x12, 0x04, 0xd5, 0x01, 0x0b, 0x12,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd5, 0x01, 0x13, 0x1b, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd5, 0x01, 0x1e, 0x1f, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x15, 0x02, 0x01, 0x12, 0x04, 0xd6, 0x01, 0x02, 0x28, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd6, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x01, 0x06, 0x12, 0x04, 0xd6, 0x01, 0x0b, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xd6, 0x01, 0x17, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xd6, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x02,
    0x12, 0x04, 0xd7, 0x01, 0x02, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x04, 0x12,
    0x04, 0xd7, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x06, 0x12, 0x04,
    0xd7, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x01, 0x12, 0x04, 0xd7,
    0x01, 0x12, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x03, 0x12, 0x04, 0xd7, 0x01,
    0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x03, 0x12, 0x04, 0xd8, 0x01, 0x02, 0x1a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x04, 0x12, 0x04, 0xd8, 0x01, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x05, 0x12, 0x04, 0xd8, 0x01, 0x0b, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x01, 0x12, 0x04, 0xd8, 0x01, 0x11, 0x15, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x03, 0x03, 0x12, 0x04, 0xd8, 0x01, 0x18, 0x19, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x16, 0x12, 0x06, 0xdc, 0x01, 0x00, 0xde, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x16,
    0x01, 0x12, 0x04, 0xdc, 0x01, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x00, 0x12,
    0x04, 0xdd, 0x01, 0x02, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xdd, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x06, 0x12, 0x04, 0xdd,
    0x01, 0x0b, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x01, 0x12, 0x04, 0xdd, 0x01,
    0x13, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x03, 0x12, 0x04, 0xdd, 0x01, 0x1e,
    0x1f, 0x0a, 0xde, 0x02, 0x0a, 0x02, 0x04, 0x17, 0x12, 0x06, 0xe7, 0x01, 0x00, 0xea, 0x01, 0x01,
    0x1a, 0xcf, 0x02, 0x20, 0x41, 0x6c, 0x6c, 0x6f, 0x77, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66,
    0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x20, 0x74, 0x6f, 0x20, 0x71, 0x75, 0x65, 0x72,
    0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x20, 0x66, 0x6f, 0x72,
    0x20, 0x6e, 0x6f, 0x6e, 0x2d, 0x74, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x61, 0x6c, 0x20, 0x74, 0x61,
    0x73, 0x6b, 0x73, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x63, 0x61, 0x75, 0x73, 0x65,
    0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20,
    0x73, 0x65, 0x6e, 0x64, 0x20, 0x62, 0x61, 0x63, 0x6b, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x61,
    0x74, 0x65, 0x73, 0x74, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x20, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x20, 0x66, 0x6f, 0x72, 0x0a, 0x20, 0x65, 0x61, 0x63, 0x68, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x20,
    0x69, 0x6e, 0x20, 0x27, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x65, 0x73, 0x27, 0x2c, 0x20, 0x69,
    0x66, 0x20, 0x70, 0x6f, 0x73, 0x73, 0x69, 0x62, 0x6c, 0x65, 0x2e, 0x20, 0x54, 0x61, 0x73, 0x6b,
    0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x61, 0x72, 0x65, 0x20, 0x6e, 0x6f, 0x20, 0x6c, 0x6f,
    0x6e, 0x67, 0x65, 0x72, 0x0a, 0x20, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x20, 0x77, 0x69, 0x6c, 0x6c,
    0x20, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x20, 0x69, 0x6e, 0x20, 0x61, 0x20, 0x54, 0x41, 0x53,
    0x4b, 0x5f, 0x4c, 0x4f, 0x53, 0x54, 0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x2e, 0x20, 0x49,
    0x66, 0x20, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x65, 0x73, 0x20, 0x69, 0x73, 0x20, 0x65, 0x6d,
    0x70, 0x74, 0x79, 0x2c, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d,
    0x61, 0x73, 0x74, 0x65, 0x72, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x73, 0x65, 0x6e, 0x64, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x6c, 0x61, 0x74, 0x65, 0x73, 0x74, 0x20, 0x73, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x65, 0x61, 0x63, 0x68, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x0a,
    0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x6c, 0x79, 0x20, 0x6b, 0x6e, 0x6f, 0x77, 0x6e,
    0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x17, 0x01, 0x12, 0x04, 0xe7, 0x01, 0x08, 0x1d, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x00, 0x12, 0x04, 0xe8, 0x01, 0x02, 0x28, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x17, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe8, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x17, 0x02, 0x00, 0x06, 0x12, 0x04, 0xe8, 0x01, 0x0b, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x17, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe8, 0x01, 0x17, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xe8, 0x01, 0x26, 0x27, 0x0a, 0x2c, 0x0a, 0x04, 0x04, 0x17, 0x02,
    0x01, 0x12, 0x04, 0xe9, 0x01, 0x02, 0x23, 0x22, 0x1e, 0x20, 0x53, 0x68, 0x6f, 0x75, 0x6c, 0x64,
    0x20, 0x62, 0x65, 0x20, 0x6e, 0x6f, 0x6e, 0x2d, 0x74, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x61, 0x6c,
    0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x04,
    0x12, 0x04, 0xe9, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x06, 0x12,
    0x04, 0xe9, 0x01, 0x0b, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xe9, 0x01, 0x16, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x03, 0x12, 0x04, 0xe9,
    0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x18, 0x12, 0x06, 0xed, 0x01, 0x00, 0xef, 0x01,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x18, 0x01, 0x12, 0x04, 0xed, 0x01, 0x08, 0x1d, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x18, 0x02, 0x00, 0x12, 0x04, 0xee, 0x01, 0x02, 0x1e, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x18, 0x02, 0x00, 0x04, 0x12, 0x04, 0xee, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x18, 0x02, 0x00, 0x05, 0x12, 0x04, 0xee, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xee, 0x01, 0x12, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xee, 0x01, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x19, 0x12, 0x06,
    0xf2, 0x01, 0x00, 0xfe, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x19, 0x01, 0x12, 0x04, 0xf2,
    0x01, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x00, 0x12, 0x04, 0xf3, 0x01, 0x02,
    0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x04, 0x12, 0x04, 0xf3, 0x01, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x06, 0x12, 0x04, 0xf3, 0x01, 0x0b, 0x14, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x01, 0x12, 0x04, 0xf3, 0x01, 0x15, 0x1a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x03, 0x12, 0x04, 0xf3, 0x01, 0x1d, 0x1e, 0x0a, 0xaf, 0x01,
    0x0a, 0x04, 0x04, 0x19, 0x02, 0x01, 0x12, 0x04, 0xf8, 0x01, 0x02, 0x2f, 0x1a, 0xa0, 0x01, 0x20,
    0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x61,
    0x72, 0x65, 0x20, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x65, 0x64, 0x20,
    0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x20, 0x28, 0x65, 0x2e,
    0x67, 0x2e, 0x2c, 0x20, 0x70, 0x65, 0x72, 0x73, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x74, 0x0a, 0x20,
    0x76, 0x6f, 0x6c, 0x75, 0x6d, 0x65, 0x20, 0x6f, 0x72, 0x20, 0x64, 0x79, 0x6e, 0x61, 0x6d, 0x69,
    0x63, 0x20, 0x72, 0x65, 0x73, 0x65, 0x72, 0x76, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x29, 0x2e, 0x20,
    0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x73, 0x20, 0x6e, 0x65, 0x65, 0x64, 0x20,
    0x74, 0x6f, 0x20, 0x72, 0x65, 0x6c, 0x65, 0x61, 0x73, 0x65, 0x0a, 0x20, 0x63, 0x68, 0x65, 0x63,
    0x6b, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x65, 0x64, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63,
    0x65, 0x73, 0x20, 0x65, 0x78, 0x70, 0x6c, 0x69, 0x63, 0x69, 0x74, 0x6c, 0x79, 0x2e, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x04, 0x12, 0x04, 0xf8, 0x01, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x06, 0x12, 0x04, 0xf8, 0x01, 0x0b, 0x13, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x19, 0x02, 0x01, 0x01, 0x12, 0x04, 0xf8, 0x01, 0x14, 0x2a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x19, 0x02, 0x01, 0x03, 0x12, 0x04, 0xf8, 0x01, 0x2d, 0x2e, 0x0a, 0xa5, 0x01, 0x0a, 0x04,
    0x04, 0x19, 0x02, 0x02, 0x12, 0x04, 0xfd, 0x01, 0x02, 0x1e, 0x1a, 0x96, 0x01, 0x20, 0x4e, 0x4f,
    0x54, 0x45, 0x3a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x68, 0x61,
    0x63, 0x6b, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x73, 0x74, 0x65,
    0x72, 0x20, 0x74, 0x6f, 0x20, 0x64, 0x65, 0x74, 0x65, 0x63, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x73, 0x6c, 0x61, 0x76, 0x65, 0x27, 0x73, 0x0a, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e,
    0x2e, 0x20, 0x49, 0x66, 0x20, 0x75, 0x6e, 0x73, 0x65, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73,
    0x6c, 0x61, 0x76, 0x65, 0x20, 0x69, 0x73, 0x20, 0x3c, 0x20, 0x30, 0x2e, 0x32, 0x31, 0x2e, 0x30,
    0x2e, 0x0a, 0x20, 0x54, 0x4f, 0x44, 0x4f, 0x28, 0x62, 0x6d, 0x61, 0x68, 0x6c, 0x65, 0x72, 0x29,
    0x3a, 0x20, 0x44, 0x6f, 0x20, 0x70, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x20, 0x76, 0x65, 0x72, 0x73,
    0x69, 0x6f, 0x6e, 0x69, 0x6e, 0x67, 0x3a, 0x20, 0x4d, 0x45, 0x53, 0x4f, 0x53, 0x2d, 0x39, 0x38,
    0x36, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x04, 0x12, 0x04, 0xfd, 0x01,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x05, 0x12, 0x04, 0xfd, 0x01, 0x0b,
    0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x01, 0x12, 0x04, 0xfd, 0x01, 0x12, 0x19,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x03, 0x12, 0x04, 0xfd, 0x01, 0x1c, 0x1d, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x1a, 0x12, 0x06, 0x81, 0x02, 0x00, 0x91, 0x02, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x1a, 0x01, 0x12, 0x04, 0x81, 0x02, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a,
    0x02, 0x00, 0x12, 0x04, 0x82, 0x02, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x82, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x06,
    0x12, 0x04, 0x82, 0x02, 0x0b, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x01, 0x12,
    0x04, 0x82, 0x02, 0x15, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x03, 0x12, 0x04,
    0x82, 0x02, 0x1d, 0x1e, 0x0a, 0xaf, 0x01, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x01, 0x12, 0x04, 0x87,
    0x02, 0x02, 0x2f, 0x1a, 0xa0, 0x01, 0x20, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x73,
    0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x61, 0x72, 0x65, 0x20, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x70,
    0x6f, 0x69, 0x6e, 0x74, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x6c,
    0x61, 0x76, 0x65, 0x20, 0x28, 0x65, 0x2e, 0x67, 0x2e, 0x2c, 0x20, 0x70, 0x65, 0x72, 0x73, 0x69,
    0x73, 0x74, 0x65, 0x6e, 0x74, 0x0a, 0x20, 0x76, 0x6f, 0x6c, 0x75, 0x6d, 0x65, 0x20, 0x6f, 0x72,
    0x20, 0x64, 0x79, 0x6e, 0x61, 0x6d, 0x69, 0x63, 0x20, 0x72, 0x65, 0x73, 0x65, 0x72, 0x76, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x29, 0x2e, 0x20, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b,
    0x73, 0x20, 0x6e, 0x65, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x6c, 0x65, 0x61, 0x73,
    0x65, 0x0a, 0x20, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x65, 0x64, 0x20,
    0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x73, 0x20, 0x65, 0x78, 0x70, 0x6c, 0x69, 0x63,
    0x69, 0x74, 0x6c, 0x79, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x04, 0x12,
    0x04, 0x87, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x06, 0x12, 0x04,
    0x87, 0x02, 0x0b, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x01, 0x12, 0x04, 0x87,
    0x02, 0x14, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x03, 0x12, 0x04, 0x87, 0x02,
    0x2d, 0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x02, 0x12, 0x04, 0x89, 0x02, 0x02, 0x2b,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x02, 0x04, 0x12, 0x04, 0x89, 0x02, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x02, 0x06, 0x12, 0x04, 0x89, 0x02, 0x0b, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1a, 0x02, 0x02, 0x01, 0x12, 0x04, 0x89, 0x02, 0x18, 0x26, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1a, 0x02, 0x02, 0x03, 0x12, 0x04, 0x89, 0x02, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x1a, 0x02, 0x03, 0x12, 0x04, 0x8a, 0x02, 0x02, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a,
    0x02, 0x03, 0x04, 0x12, 0x04, 0x8a, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02,
    0x03, 0x06, 0x12, 0x04, 0x8a, 0x02, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x03,
    0x01, 0x12, 0x04, 0x8a, 0x02, 0x10, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x03, 0x03,
    0x12, 0x04, 0x8a, 0x02, 0x18, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x04, 0x12, 0x04,
    0x8b, 0x02, 0x02, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x8b,
    0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x8b, 0x02,
    0x0b, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x8b, 0x02, 0x1d,
    0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x8b, 0x02, 0x34, 0x35,
    0x0a, 0xa5, 0x01, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x05, 0x12, 0x04, 0x90, 0x02, 0x02, 0x1e, 0x1a,
    0x96, 0x01, 0x20, 0x4e, 0x4f, 0x54, 0x45, 0x3a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73,
    0x20, 0x61, 0x20, 0x68, 0x61, 0x63, 0x6b, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x64, 0x65, 0x74, 0x65, 0x63, 0x74,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x27, 0x73, 0x0a, 0x20, 0x76, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x75, 0x6e, 0x73, 0x65, 0x74, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x20, 0x69, 0x73, 0x20, 0x3c, 0x20, 0x30,
    0x2e, 0x32, 0x31, 0x2e, 0x30, 0x2e, 0x0a, 0x20, 0x54, 0x4f, 0x44, 0x4f, 0x28, 0x62, 0x6d, 0x61,
    0x68, 0x6c, 0x65, 0x72, 0x29, 0x3a, 0x20, 0x44, 0x6f, 0x20, 0x70, 0x72, 0x6f, 0x70, 0x65, 0x72,
    0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x69, 0x6e, 0x67, 0x3a, 0x20, 0x4d, 0x45, 0x53,
    0x4f, 0x53, 0x2d, 0x39, 0x38, 0x36, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x05,
    0x04, 0x12, 0x04, 0x90, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x05, 0x05,
    0x12, 0x04, 0x90, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x05, 0x01, 0x12,
    0x04, 0x90, 0x02, 0x12, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x05, 0x03, 0x12, 0x04,
    0x90, 0x02, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1b, 0x12, 0x06, 0x94, 0x02, 0x00, 0x96,
    0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1b, 0x01, 0x12, 0x04, 0x94, 0x02, 0x08, 0x1e, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x00, 0x12, 0x04, 0x95, 0x02, 0x02, 0x20, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1b, 0x02, 0x00, 0x04, 0x12, 0x04, 0x95, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1b, 0x02, 0x00, 0x06, 0x12, 0x04, 0x95, 0x02, 0x0b, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1b, 0x02, 0x00, 0x01, 0x12, 0x04, 0x95, 0x02, 0x13, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b,
    0x02, 0x00, 0x03, 0x12, 0x04, 0x95, 0x02, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1c, 0x12,
    0x06, 0x99, 0x02, 0x00, 0x9d, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1c, 0x01, 0x12, 0x04,
    0x99, 0x02, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x00, 0x12, 0x04, 0x9a, 0x02,
    0x02, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x04, 0x12, 0x04, 0x9a, 0x02, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x06, 0x12, 0x04, 0x9a, 0x02, 0x0b, 0x12,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9a, 0x02, 0x13, 0x1b, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9a, 0x02, 0x1e, 0x1f, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x1c, 0x02, 0x01, 0x12, 0x04, 0x9c, 0x02, 0x02, 0x35, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1c, 0x02, 0x01, 0x04, 0x12, 0x04, 0x9c, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1c, 0x02, 0x01, 0x06, 0x12, 0x04, 0x9c, 0x02, 0x0b, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c,
    0x02, 0x01, 0x01, 0x12, 0x04, 0x9c, 0x02, 0x21, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02,
    0x01, 0x03, 0x12, 0x04, 0x9c, 0x02, 0x33, 0x34, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1d, 0x12, 0x06,
    0xa0, 0x02, 0x00, 0xa2, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1d, 0x01, 0x12, 0x04, 0xa0,
    0x02, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x00, 0x12, 0x04, 0xa1, 0x02, 0x02,
    0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa1, 0x02, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x06, 0x12, 0x04, 0xa1, 0x02, 0x0b, 0x12, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa1, 0x02, 0x13, 0x1b, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa1, 0x02, 0x1e, 0x1f, 0x0a, 0x8c, 0x01,
    0x0a, 0x02, 0x04, 0x1e, 0x12, 0x06, 0xa7, 0x02, 0x00, 0xa9, 0x02, 0x01, 0x1a, 0x7e, 0x20, 0x54,
    0x68, 0x69, 0x73, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x69, 0x73, 0x20, 0x70,
    0x65, 0x72, 0x69, 0x6f, 0x64, 0x69, 0x63, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x73, 0x65, 0x6e, 0x74,
    0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x20, 0x74,
    0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x2e, 0x0a, 0x20, 0x49, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x20, 0x69, 0x73, 0x20, 0x63, 0x6f,
    0x6e, 0x6e, 0x65, 0x63, 0x74, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d,
    0x61, 0x73, 0x74, 0x65, 0x72, 0x2c, 0x20, 0x22, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x65,
    0x64, 0x22, 0x20, 0x69, 0x73, 0x20, 0x74, 0x72, 0x75, 0x65, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x1e, 0x01, 0x12, 0x04, 0xa7, 0x02, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1e, 0x02,
    0x00, 0x12, 0x04, 0xa8, 0x02, 0x02, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x04,
    0x12, 0x04, 0xa8, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x05, 0x12,
    0x04, 0xa8, 0x02, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xa8, 0x02, 0x10, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa8,
    0x02, 0x1c, 0x1d, 0x0a, 0x63, 0x0a, 0x02, 0x04, 0x1f, 0x12, 0x04, 0xae, 0x02, 0x00, 0x1b, 0x1a,
    0x57, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x69,
    0x73, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x6c,
    0x61, 0x76, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x73, 0x74, 0x65,
    0x72, 0x20, 0x69, 0x6e, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x74, 0x6f,
    0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x50, 0x69, 0x6e, 0x67, 0x53, 0x6c, 0x61, 0x76, 0x65, 0x4d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1f, 0x01, 0x12,
    0x04, 0xae, 0x02, 0x08, 0x18, 0x0a, 0x50, 0x0a, 0x02, 0x04, 0x20, 0x12, 0x06, 0xb2, 0x02, 0x00,
    0xb4, 0x02, 0x01, 0x1a, 0x42, 0x20, 0x54, 0x65, 0x6c, 0x6c, 0x73, 0x20, 0x61, 0x20, 0x73, 0x6c,
    0x61, 0x76, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x73, 0x68, 0x75, 0x74, 0x20, 0x64, 0x6f, 0x77, 0x6e,
    0x20, 0x61, 0x6c, 0x6c, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x73, 0x20, 0x6f,
    0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x67, 0x69, 0x76, 0x65, 0x6e, 0x20, 0x66, 0x72, 0x61, 0x6d,
    0x65, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x20, 0x01, 0x12, 0x04,
    0xb2, 0x02, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x00, 0x12, 0x04, 0xb3, 0x02,
    0x02, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb3, 0x02, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x06, 0x12, 0x04, 0xb3, 0x02, 0x0b, 0x16,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb3, 0x02, 0x17, 0x23, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb3, 0x02, 0x26, 0x27, 0x0a, 0x52,
    0x0a, 0x02, 0x04, 0x21, 0x12, 0x06, 0xb8, 0x02, 0x00, 0xbe, 0x02, 0x01, 0x1a, 0x44, 0x20, 0x54,
    0x65, 0x6c, 0x6c, 0x73, 0x20, 0x61, 0x20, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x20, 0x28, 0x61, 0x6e,
    0x64, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x74, 0x6c, 0x79, 0x20, 0x65,
    0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x29, 0x20, 0x74, 0x6f, 0x20, 0x73, 0x68, 0x75, 0x74,
    0x64, 0x6f, 0x77, 0x6e, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72,
    0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x21, 0x01, 0x12, 0x04, 0xb8, 0x02, 0x08, 0x1f, 0x0a,
    0xa4, 0x01, 0x0a, 0x04, 0x04, 0x21, 0x02, 0x00, 0x12, 0x04, 0xbc, 0x02, 0x02, 0x26, 0x1a, 0x95,
    0x01, 0x20, 0x54, 0x4f, 0x44, 0x4f, 0x28, 0x76, 0x69, 0x6e, 0x6f, 0x64, 0x29, 0x3a, 0x20, 0x4d,
    0x61, 0x6b, 0x65, 0x20, 0x74, 0x68, 0x65, 0x73, 0x65, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73,
    0x20, 0x72, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x73, 0x65,
    0x20, 0x61, 0x72, 0x65, 0x20, 0x6d, 0x61, 0x64, 0x65, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e,
    0x61, 0x6c, 0x0a, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x62, 0x61, 0x63, 0x6b, 0x77, 0x61, 0x72, 0x64,
    0x73, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x61, 0x74, 0x69, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x20,
    0x62, 0x65, 0x74, 0x77, 0x65, 0x65, 0x6e, 0x20, 0x30, 0x2e, 0x32, 0x33, 0x2e, 0x30, 0x20, 0x73,
    0x6c, 0x61, 0x76, 0x65, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x70, 0x72, 0x65, 0x20, 0x30, 0x2e, 0x32,
    0x33, 0x2e, 0x30, 0x0a, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x20, 0x64, 0x72,
    0x69, 0x76, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xbc, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00, 0x06, 0x12, 0x04,
    0xbc, 0x02, 0x0b, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbc,
    0x02, 0x16, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00, 0x03, 0x12, 0x04, 0xbc, 0x02,
    0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x21, 0x02, 0x01, 0x12, 0x04, 0xbd, 0x02, 0x02, 0x28,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x04, 0x12, 0x04, 0xbd, 0x02, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x06, 0x12, 0x04, 0xbd, 0x02, 0x0b, 0x16, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x01, 0x12, 0x04, 0xbd, 0x02, 0x17, 0x23, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x21, 0x02, 0x01, 0x03, 0x12, 0x04, 0xbd, 0x02, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x22, 0x12, 0x06, 0xc1, 0x02, 0x00, 0xc4, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x22,
    0x01, 0x12, 0x04, 0xc1, 0x02, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x22, 0x02, 0x00, 0x12,
    0x04, 0xc2, 0x02, 0x02, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xc2, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x06, 0x12, 0x04, 0xc2,
    0x02, 0x0b, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc2, 0x02,
    0x17, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc2, 0x02, 0x26,
    0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x22, 0x02, 0x01, 0x12, 0x04, 0xc3, 0x02, 0x02, 0x1a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x01, 0x04, 0x12, 0x04, 0xc3, 0x02, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x22, 0x02, 0x01, 0x05, 0x12, 0x04, 0xc3, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x22, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc3, 0x02, 0x12, 0x15, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x22, 0x02, 0x01, 0x03, 0x12, 0x04, 0xc3, 0x02, 0x18, 0x19, 0x0a, 0xb0, 0x01, 0x0a, 0x02,
    0x04, 0x23, 0x12, 0x06, 0xca, 0x02, 0x00, 0xcc, 0x02, 0x01, 0x1a, 0xa1, 0x01, 0x20, 0x54, 0x68,
    0x69, 0x73, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x69, 0x73, 0x20, 0x73, 0x65,
    0x6e, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x20,
    0x77, 0x68, 0x65, 0x6e, 0x65, 0x76, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x72, 0x65, 0x20, 0x69,
    0x73, 0x20, 0x61, 0x6e, 0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x20, 0x6f, 0x66, 0x0a, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x73, 0x20, 0x74, 0x68,
    0x61, 0x74, 0x20, 0x6e, 0x65, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x63, 0x68,
    0x65, 0x63, 0x6b, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x65, 0x64, 0x20, 0x28, 0x65, 0x2e, 0x67, 0x2e,
    0x2c, 0x20, 0x70, 0x65, 0x72, 0x73, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x74, 0x20, 0x76, 0x6f, 0x6c,
    0x75, 0x6d, 0x65, 0x0a, 0x20, 0x6f, 0x72, 0x20, 0x64, 0x79, 0x6e, 0x61, 0x6d, 0x69, 0x63, 0x20,
    0x72, 0x65, 0x73, 0x65, 0x72, 0x76, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x29, 0x2e, 0x0a, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x23, 0x01, 0x12, 0x04, 0xca, 0x02, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x23, 0x02, 0x00, 0x12, 0x04, 0xcb, 0x02, 0x02, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xcb, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x00,
    0x06, 0x12, 0x04, 0xcb, 0x02, 0x0b, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xcb, 0x02, 0x14, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x00, 0x03, 0x12,
    0x04, 0xcb, 0x02, 0x20, 0x21, 0x0a, 0x85, 0x01, 0x0a, 0x02, 0x04, 0x24, 0x12, 0x06, 0xd1, 0x02,
    0x00, 0xd4, 0x02, 0x01, 0x1a, 0x77, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x6d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x20, 0x69, 0x73, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x62, 0x79, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d,
    0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x20, 0x61, 0x62, 0x6f,
    0x75, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x6c, 0x79,
    0x20, 0x6f, 0x76, 0x65, 0x72, 0x73, 0x75, 0x62, 0x73, 0x63, 0x72, 0x69, 0x62, 0x61, 0x62, 0x6c,
    0x65, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x24, 0x01, 0x12, 0x04, 0xd1, 0x02, 0x08, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x24,
    0x02, 0x00, 0x12, 0x04, 0xd2, 0x02, 0x02, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xd2, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x00, 0x06,
    0x12, 0x04, 0xd2, 0x02, 0x0b, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xd2, 0x02, 0x13, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xd2, 0x02, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x24, 0x02, 0x01, 0x12, 0x04, 0xd3, 0x02,
    0x02, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd3, 0x02, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x01, 0x06, 0x12, 0x04, 0xd3, 0x02, 0x0b, 0x13,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd3, 0x02, 0x14, 0x1d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x01, 0x03, 0x12, 0x04, 0xd3, 0x02, 0x20, 0x21, 0x0a, 0x0c,
    0x0a, 0x02, 0x04, 0x25, 0x12, 0x06, 0xd7, 0x02, 0x00, 0xda, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x25, 0x01, 0x12, 0x04, 0xd7, 0x02, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02,
    0x00, 0x12, 0x04, 0xd8, 0x02, 0x02, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x00, 0x04,
    0x12, 0x04, 0xd8, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x00, 0x06, 0x12,
    0x04, 0xd8, 0x02, 0x0b, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xd8, 0x02, 0x17, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd8,
    0x02, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x01, 0x12, 0x04, 0xd9, 0x02, 0x02,
    0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd9, 0x02, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x06, 0x12, 0x04, 0xd9, 0x02, 0x0b, 0x15, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd9, 0x02, 0x16, 0x21, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x03, 0x12, 0x04, 0xd9, 0x02, 0x24, 0x25, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x26, 0x12, 0x06, 0xdd, 0x02, 0x00, 0xe3, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x26, 0x01, 0x12, 0x04, 0xdd, 0x02, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x00,
    0x12, 0x04, 0xde, 0x02, 0x02, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xde, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x00, 0x06, 0x12, 0x04,
    0xde, 0x02, 0x0b, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x00, 0x01, 0x12, 0x04, 0xde,
    0x02, 0x18, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x00, 0x03, 0x12, 0x04, 0xde, 0x02,
    0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x01, 0x12, 0x04, 0xdf, 0x02, 0x02, 0x28,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x01, 0x04, 0x12, 0x04, 0xdf, 0x02, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x01, 0x06, 0x12, 0x04, 0xdf, 0x02, 0x0b, 0x16, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x26, 0x02, 0x01, 0x01, 0x12, 0x04, 0xdf, 0x02, 0x17, 0x23, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x26, 0x02, 0x01, 0x03, 0x12, 0x04, 0xdf, 0x02, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x26, 0x02, 0x02, 0x12, 0x04, 0xe0, 0x02, 0x02, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26,
    0x02, 0x02, 0x04, 0x12, 0x04, 0xe0, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02,
    0x02, 0x06, 0x12, 0x04, 0xe0, 0x02, 0x0b, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x02,
    0x01, 0x12, 0x04, 0xe0, 0x02, 0x19, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x02, 0x03,
    0x12, 0x04, 0xe0, 0x02, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x03, 0x12, 0x04,
    0xe1, 0x02, 0x02, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x03, 0x04, 0x12, 0x04, 0xe1,
    0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x03, 0x06, 0x12, 0x04, 0xe1, 0x02,
    0x0b, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x03, 0x01, 0x12, 0x04, 0xe1, 0x02, 0x13,
    0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x03, 0x03, 0x12, 0x04, 0xe1, 0x02, 0x1e, 0x1f,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x04, 0x12, 0x04, 0xe2, 0x02, 0x02, 0x24, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x26, 0x02, 0x04, 0x04, 0x12, 0x04, 0xe2, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x26, 0x02, 0x04, 0x06, 0x12, 0x04, 0xe2, 0x02, 0x0b, 0x14, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x26, 0x02, 0x04, 0x01, 0x12, 0x04, 0xe2, 0x02, 0x15, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x26, 0x02, 0x04, 0x03, 0x12, 0x04, 0xe2, 0x02, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x27,
    0x12, 0x06, 0xe6, 0x02, 0x00, 0xe9, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x27, 0x01, 0x12,
    0x04, 0xe6, 0x02, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x27, 0x02, 0x00, 0x12, 0x04, 0xe7,
    0x02, 0x02, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe7, 0x02,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x00, 0x06, 0x12, 0x04, 0xe7, 0x02, 0x0b,
    0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe7, 0x02, 0x13, 0x1b,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe7, 0x02, 0x1e, 0x1f, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x27, 0x02, 0x01, 0x12, 0x04, 0xe8, 0x02, 0x02, 0x24, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x27, 0x02, 0x01, 0x04, 0x12, 0x04, 0xe8, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x27, 0x02, 0x01, 0x06, 0x12, 0x04, 0xe8, 0x02, 0x0b, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x27, 0x02, 0x01, 0x01, 0x12, 0x04, 0xe8, 0x02, 0x15, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27,
    0x02, 0x01, 0x03, 0x12, 0x04, 0xe8, 0x02, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x28, 0x12,
    0x06, 0xec, 0x02, 0x00, 0xf1, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x28, 0x01, 0x12, 0x04,
    0xec, 0x02, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x28, 0x02, 0x00, 0x12, 0x04, 0xed, 0x02,
    0x02, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x00, 0x04, 0x12, 0x04, 0xed, 0x02, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x00, 0x06, 0x12, 0x04, 0xed, 0x02, 0x0b, 0x12,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x00, 0x01, 0x12, 0x04, 0xed, 0x02, 0x13, 0x1b, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x00, 0x03, 0x12, 0x04, 0xed, 0x02, 0x1e, 0x1f, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x28, 0x02, 0x01, 0x12, 0x04, 0xee, 0x02, 0x02, 0x28, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x28, 0x02, 0x01, 0x04, 0x12, 0x04, 0xee, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x28, 0x02, 0x01, 0x06, 0x12, 0x04, 0xee, 0x02, 0x0b, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xee, 0x02, 0x17, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xee, 0x02, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x28, 0x02, 0x02,
    0x12, 0x04, 0xef, 0x02, 0x02, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x02, 0x04, 0x12,
    0x04, 0xef, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x02, 0x06, 0x12, 0x04,
    0xef, 0x02, 0x0b, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x02, 0x01, 0x12, 0x04, 0xef,
    0x02, 0x16, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x02, 0x03, 0x12, 0x04, 0xef, 0x02,
    0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x28, 0x02, 0x03, 0x12, 0x04, 0xf0, 0x02, 0x02, 0x1c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x03, 0x04, 0x12, 0x04, 0xf0, 0x02, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x03, 0x05, 0x12, 0x04, 0xf0, 0x02, 0x0b, 0x10, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x28, 0x02, 0x03, 0x01, 0x12, 0x04, 0xf0, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x28, 0x02, 0x03, 0x03, 0x12, 0x04, 0xf0, 0x02, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x29, 0x12, 0x06, 0xf4, 0x02, 0x00, 0xf6, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x29,
    0x01, 0x12, 0x04, 0xf4, 0x02, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x00, 0x12,
    0x04, 0xf5, 0x02, 0x02, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xf5, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x00, 0x06, 0x12, 0x04, 0xf5,
    0x02, 0x0b, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x00, 0x01, 0x12, 0x04, 0xf5, 0x02,
    0x13, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x00, 0x03, 0x12, 0x04, 0xf5, 0x02, 0x1e,
    0x1f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2a, 0x12, 0x06, 0xf9, 0x02, 0x00, 0xfe, 0x02, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x2a, 0x01, 0x12, 0x04, 0xf9, 0x02, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x2a, 0x02, 0x00, 0x12, 0x04, 0xfa, 0x02, 0x02, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xfa, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02,
    0x00, 0x06, 0x12, 0x04, 0xfa, 0x02, 0x0b, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xfa, 0x02, 0x16, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xfa, 0x02, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2a, 0x02, 0x01, 0x12, 0x04,
    0xfb, 0x02, 0x02, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x01, 0x04, 0x12, 0x04, 0xfb,
    0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x01, 0x06, 0x12, 0x04, 0xfb, 0x02,
    0x0b, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x01, 0x01, 0x12, 0x04, 0xfb, 0x02, 0x17,
    0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x01, 0x03, 0x12, 0x04, 0xfb, 0x02, 0x26, 0x27,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2a, 0x02, 0x02, 0x12, 0x04, 0xfc, 0x02, 0x02, 0x1e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x2a, 0x02, 0x02, 0x04, 0x12, 0x04, 0xfc, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x2a, 0x02, 0x02, 0x06, 0x12, 0x04, 0xfc, 0x02, 0x0b, 0x13, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2a, 0x02, 0x02, 0x01, 0x12, 0x04, 0xfc, 0x02, 0x14, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2a, 0x02, 0x02, 0x03, 0x12, 0x04, 0xfc, 0x02, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2a,
    0x02, 0x03, 0x12, 0x04, 0xfd, 0x02, 0x02, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x03,
    0x04, 0x12, 0x04, 0xfd, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x03, 0x06,
    0x12, 0x04, 0xfd, 0x02, 0x0b, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x03, 0x01, 0x12,
    0x04, 0xfd, 0x02, 0x18, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x03, 0x03, 0x12, 0x04,
    0xfd, 0x02, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x2b, 0x12, 0x06, 0x81, 0x03, 0x00, 0x83,
    0x03, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2b, 0x01, 0x12, 0x04, 0x81, 0x03, 0x08, 0x17, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x2b, 0x02, 0x00, 0x12, 0x04, 0x82, 0x03, 0x02, 0x1e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x2b, 0x02, 0x00, 0x04, 0x12, 0x04, 0x82, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2b, 0x02, 0x00, 0x05, 0x12, 0x04, 0x82, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2b, 0x02, 0x00, 0x01, 0x12, 0x04, 0x82, 0x03, 0x12, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2b,
    0x02, 0x00, 0x03, 0x12, 0x04, 0x82, 0x03, 0x1c, 0x1d, 0x0a, 0x44, 0x0a, 0x02, 0x04, 0x2c, 0x12,
    0x06, 0x8a, 0x03, 0x00, 0x91, 0x03, 0x01, 0x1a, 0x36, 0x2a, 0x0a, 0x20, 0x44, 0x65, 0x73, 0x63,
    0x72, 0x69, 0x62, 0x65, 0x73, 0x20, 0x43, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x20,
    0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x73, 0x2c, 0x20, 0x65, 0x74, 0x63, 0x2e,
    0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x72, 0x63, 0x68, 0x69, 0x76, 0x61, 0x6c, 0x2e, 0x0a, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x2c, 0x01, 0x12, 0x04, 0x8a, 0x03, 0x08, 0x0f, 0x0a, 0x0e, 0x0a, 0x04,
    0x04, 0x2c, 0x03, 0x00, 0x12, 0x06, 0x8b, 0x03, 0x02, 0x8f, 0x03, 0x03, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2c, 0x03, 0x00, 0x01, 0x12, 0x04, 0x8b, 0x03, 0x0a, 0x13, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x2c, 0x03, 0x00, 0x02, 0x00, 0x12, 0x04, 0x8c, 0x03, 0x04, 0x2e, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x2c, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x8c, 0x03, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x2c, 0x03, 0x00, 0x02, 0x00, 0x06, 0x12, 0x04, 0x8c, 0x03, 0x0d, 0x1a, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x2c, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8c, 0x03, 0x1b, 0x29, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x2c, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8c, 0x03, 0x2c, 0x2d, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x2c, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0x8d, 0x03, 0x04, 0x1c, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x2c, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x8d, 0x03, 0x04, 0x0c,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x2c, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x04, 0x8d, 0x03, 0x0d,
    0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x2c, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8d, 0x03,
    0x14, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x2c, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8d,
    0x03, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x2c, 0x03, 0x00, 0x02, 0x02, 0x12, 0x04, 0x8e,
    0x03, 0x04, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x2c, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04,
    0x8e, 0x03, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x2c, 0x03, 0x00, 0x02, 0x02, 0x06, 0x12,
    0x04, 0x8e, 0x03, 0x0d, 0x11, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x2c, 0x03, 0x00, 0x02, 0x02, 0x01,
    0x12, 0x04, 0x8e, 0x03, 0x12, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x2c, 0x03, 0x00, 0x02, 0x02,
    0x03, 0x12, 0x04, 0x8e, 0x03, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2c, 0x02, 0x00, 0x12,
    0x04, 0x90, 0x03, 0x02, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x90, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x00, 0x06, 0x12, 0x04, 0x90,
    0x03, 0x0b, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x00, 0x01, 0x12, 0x04, 0x90, 0x03,
    0x15, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x00, 0x03, 0x12, 0x04, 0x90, 0x03, 0x22,
    0x23, 0x0a, 0xe9, 0x02, 0x0a, 0x02, 0x04, 0x2d, 0x12, 0x06, 0x99, 0x03, 0x00, 0xa4, 0x03, 0x01,
    0x1a, 0xda, 0x02, 0x20, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x64, 0x65, 0x73, 0x63,
    0x72, 0x69, 0x62, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x20, 0x63, 0x75, 0x72, 0x72,
    0x65, 0x6e, 0x74, 0x20, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x20, 0x73, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x69, 0x73, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x62,
    0x79, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x20, 0x68, 0x65, 0x61, 0x6c,
    0x74, 0x68, 0x20, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74,
    0x6f, 0x72, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64,
    0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x20, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74,
    0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x20, 0x73, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x20, 0x62, 0x61, 0x63, 0x6b, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x6f,
    0x6e, 0x20, 0x65, 0x61, 0x63, 0x68, 0x20, 0x72, 0x65, 0x63, 0x65, 0x69, 0x76, 0x65, 0x2e, 0x20,
    0x49, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x20, 0x63, 0x68,
    0x65, 0x63, 0x6b, 0x65, 0x72, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x65, 0x64,
    0x20, 0x66, 0x61, 0x69, 0x75, 0x72, 0x65, 0x0a, 0x20, 0x63, 0x6f, 0x6e, 0x64, 0x69, 0x74, 0x69,
    0x6f, 0x6e, 0x20, 0x6d, 0x65, 0x65, 0x74, 0x73, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x6e, 0x20, 0x6b,
    0x69, 0x6c, 0x6c, 0x5f, 0x74, 0x61, 0x73, 0x6b, 0x20, 0x66, 0x6c, 0x61, 0x67, 0x20, 0x77, 0x69,
    0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x73, 0x65, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x72, 0x75,
    0x65, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x78, 0x65,
    0x63, 0x75, 0x74, 0x6f, 0x72, 0x20, 0x6f, 0x6e, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x20, 0x72, 0x65, 0x63, 0x65, 0x69, 0x76, 0x65, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x6b, 0x69,
    0x6c, 0x6c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x2d, 0x01, 0x12, 0x04, 0x99, 0x03, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2d,
    0x02, 0x00, 0x12, 0x04, 0x9a, 0x03, 0x02, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x9a, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x00, 0x06,
    0x12, 0x04, 0x9a, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x00, 0x01, 0x12,
    0x04, 0x9a, 0x03, 0x12, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x00, 0x03, 0x12, 0x04,
    0x9a, 0x03, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2d, 0x02, 0x01, 0x12, 0x04, 0x9c, 0x03,
    0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x01, 0x04, 0x12, 0x04, 0x9c, 0x03, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x01, 0x05, 0x12, 0x04, 0x9c, 0x03, 0x0b, 0x0f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x01, 0x01, 0x12, 0x04, 0x9c, 0x03, 0x10, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x01, 0x03, 0x12, 0x04, 0x9c, 0x03, 0x1a, 0x1b, 0x0a, 0x2b,
    0x0a, 0x04, 0x04, 0x2d, 0x02, 0x02, 0x12, 0x04, 0x9f, 0x03, 0x02, 0x30, 0x1a, 0x1d, 0x20, 0x46,
    0x6c, 0x61, 0x67, 0x20, 0x74, 0x6f, 0x20, 0x69, 0x6e, 0x69, 0x74, 0x69, 0x61, 0x74, 0x65, 0x20,
    0x74, 0x61, 0x73, 0x6b, 0x20, 0x6b, 0x69, 0x6c, 0x6c, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2d, 0x02, 0x02, 0x04, 0x12, 0x04, 0x9f, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d,
    0x02, 0x02, 0x05, 0x12, 0x04, 0x9f, 0x03, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02,
    0x02, 0x01, 0x12, 0x04, 0x9f, 0x03, 0x10, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x02,
    0x03, 0x12, 0x04, 0x9f, 0x03, 0x1c, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x02, 0x08,
    0x12, 0x04, 0x9f, 0x03, 0x1e, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x02, 0x07, 0x12,
    0x04, 0x9f, 0x03, 0x29, 0x2e, 0x0a, 0x6f, 0x0a, 0x04, 0x04, 0x2d, 0x02, 0x03, 0x12, 0x04, 0xa3,
    0x03, 0x02, 0x2a, 0x1a, 0x61, 0x20, 0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20,
    0x63, 0x6f, 0x6e, 0x73, 0x65, 0x63, 0x75, 0x74, 0x69, 0x76, 0x65, 0x20, 0x63, 0x6f, 0x75, 0x6e,
    0x74, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x73, 0x74,
    0x61, 0x74, 0x75, 0x73, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x77, 0x69, 0x6c, 0x6c,
    0x20, 0x6e, 0x6f, 0x74, 0x20, 0x62, 0x65, 0x20, 0x70, 0x6f, 0x70, 0x75, 0x6c, 0x61, 0x74, 0x65,
    0x64, 0x20, 0x69, 0x66, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x20, 0x69, 0x73, 0x20, 0x68, 0x65, 0x61,
    0x6c, 0x74, 0x68, 0x79, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x03, 0x04, 0x12,
    0x04, 0xa3, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x03, 0x05, 0x12, 0x04,
    0xa3, 0x03, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x03, 0x01, 0x12, 0x04, 0xa3,
    0x03, 0x11, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x03, 0x03, 0x12, 0x04, 0xa3, 0x03,
    0x28, 0x29, 0x0a, 0x4b, 0x0a, 0x02, 0x04, 0x2e, 0x12, 0x06, 0xaa, 0x03, 0x00, 0xac, 0x03, 0x01,
    0x1a, 0x3d, 0x2a, 0x0a, 0x20, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x74, 0x6f, 0x20,
    0x73, 0x69, 0x67, 0x6e, 0x61, 0x6c, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x69, 0x6f,
    0x6e, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x20, 0x77, 0x69,
    0x74, 0x68, 0x69, 0x6e, 0x20, 0x61, 0x20, 0x6d, 0x6f, 0x64, 0x75, 0x6c, 0x65, 0x2e, 0x0a, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x2e, 0x01, 0x12, 0x04, 0xaa, 0x03, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x2e, 0x02, 0x00, 0x12, 0x04, 0xab, 0x03, 0x02, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xab, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02,
    0x00, 0x05, 0x12, 0x04, 0xab, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xab, 0x03, 0x12, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xab, 0x03, 0x1b, 0x1c,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
