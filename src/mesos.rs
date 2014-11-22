// This file is generated. Do not edit

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]


#[deriving(Clone,PartialEq,Default)]
pub struct FrameworkID {
    value: ::protobuf::SingularField<::std::string::String>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> FrameworkID {
    pub fn new() -> FrameworkID {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FrameworkID {
        static mut instance: ::protobuf::lazy::Lazy<FrameworkID> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FrameworkID,
        };
        unsafe {
            instance.get(|| {
                FrameworkID {
                    value: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // required string value = 1;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&'a mut self) -> &'a mut ::std::string::String {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    pub fn get_value(&'a self) -> &'a str {
        match self.value.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }
}

impl ::protobuf::Message for FrameworkID {
    fn new() -> FrameworkID {
        FrameworkID::new()
    }

    fn is_initialized(&self) -> bool {
        if self.value.is_none() {
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
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.value.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.value.iter() {
            my_size += ::protobuf::rt::string_size(1, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.value.as_ref() {
            Some(v) => {
                try!(os.write_string(1, v.as_slice()));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<FrameworkID>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<FrameworkID>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&FrameworkID_value_acc as &'static ::protobuf::reflect::FieldAccessor<FrameworkID>) });
                ::protobuf::reflect::MessageDescriptor::new::<FrameworkID>(
                    "FrameworkID",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<FrameworkID>()
    }
}

impl ::protobuf::Clear for FrameworkID {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for FrameworkID {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct FrameworkID_value_acc_type;
static FrameworkID_value_acc: FrameworkID_value_acc_type = FrameworkID_value_acc_type;

impl ::protobuf::reflect::FieldAccessor<FrameworkID> for FrameworkID_value_acc_type {
    fn name(&self) -> &'static str {
        "value"
    }

    fn has_field(&self, m: &FrameworkID) -> bool {
        m.has_value()
    }

    fn get_str<'a>(&self, m: &'a FrameworkID) -> &'a str {
        m.get_value()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct OfferID {
    value: ::protobuf::SingularField<::std::string::String>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> OfferID {
    pub fn new() -> OfferID {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OfferID {
        static mut instance: ::protobuf::lazy::Lazy<OfferID> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OfferID,
        };
        unsafe {
            instance.get(|| {
                OfferID {
                    value: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // required string value = 1;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&'a mut self) -> &'a mut ::std::string::String {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    pub fn get_value(&'a self) -> &'a str {
        match self.value.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }
}

impl ::protobuf::Message for OfferID {
    fn new() -> OfferID {
        OfferID::new()
    }

    fn is_initialized(&self) -> bool {
        if self.value.is_none() {
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
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.value.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.value.iter() {
            my_size += ::protobuf::rt::string_size(1, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.value.as_ref() {
            Some(v) => {
                try!(os.write_string(1, v.as_slice()));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<OfferID>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<OfferID>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&OfferID_value_acc as &'static ::protobuf::reflect::FieldAccessor<OfferID>) });
                ::protobuf::reflect::MessageDescriptor::new::<OfferID>(
                    "OfferID",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<OfferID>()
    }
}

impl ::protobuf::Clear for OfferID {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for OfferID {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct OfferID_value_acc_type;
static OfferID_value_acc: OfferID_value_acc_type = OfferID_value_acc_type;

impl ::protobuf::reflect::FieldAccessor<OfferID> for OfferID_value_acc_type {
    fn name(&self) -> &'static str {
        "value"
    }

    fn has_field(&self, m: &OfferID) -> bool {
        m.has_value()
    }

    fn get_str<'a>(&self, m: &'a OfferID) -> &'a str {
        m.get_value()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct SlaveID {
    value: ::protobuf::SingularField<::std::string::String>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> SlaveID {
    pub fn new() -> SlaveID {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SlaveID {
        static mut instance: ::protobuf::lazy::Lazy<SlaveID> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SlaveID,
        };
        unsafe {
            instance.get(|| {
                SlaveID {
                    value: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // required string value = 1;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&'a mut self) -> &'a mut ::std::string::String {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    pub fn get_value(&'a self) -> &'a str {
        match self.value.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }
}

impl ::protobuf::Message for SlaveID {
    fn new() -> SlaveID {
        SlaveID::new()
    }

    fn is_initialized(&self) -> bool {
        if self.value.is_none() {
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
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.value.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.value.iter() {
            my_size += ::protobuf::rt::string_size(1, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.value.as_ref() {
            Some(v) => {
                try!(os.write_string(1, v.as_slice()));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<SlaveID>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<SlaveID>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&SlaveID_value_acc as &'static ::protobuf::reflect::FieldAccessor<SlaveID>) });
                ::protobuf::reflect::MessageDescriptor::new::<SlaveID>(
                    "SlaveID",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<SlaveID>()
    }
}

impl ::protobuf::Clear for SlaveID {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for SlaveID {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct SlaveID_value_acc_type;
static SlaveID_value_acc: SlaveID_value_acc_type = SlaveID_value_acc_type;

impl ::protobuf::reflect::FieldAccessor<SlaveID> for SlaveID_value_acc_type {
    fn name(&self) -> &'static str {
        "value"
    }

    fn has_field(&self, m: &SlaveID) -> bool {
        m.has_value()
    }

    fn get_str<'a>(&self, m: &'a SlaveID) -> &'a str {
        m.get_value()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct TaskID {
    value: ::protobuf::SingularField<::std::string::String>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> TaskID {
    pub fn new() -> TaskID {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TaskID {
        static mut instance: ::protobuf::lazy::Lazy<TaskID> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TaskID,
        };
        unsafe {
            instance.get(|| {
                TaskID {
                    value: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // required string value = 1;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&'a mut self) -> &'a mut ::std::string::String {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    pub fn get_value(&'a self) -> &'a str {
        match self.value.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }
}

impl ::protobuf::Message for TaskID {
    fn new() -> TaskID {
        TaskID::new()
    }

    fn is_initialized(&self) -> bool {
        if self.value.is_none() {
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
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.value.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.value.iter() {
            my_size += ::protobuf::rt::string_size(1, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.value.as_ref() {
            Some(v) => {
                try!(os.write_string(1, v.as_slice()));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<TaskID>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<TaskID>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&TaskID_value_acc as &'static ::protobuf::reflect::FieldAccessor<TaskID>) });
                ::protobuf::reflect::MessageDescriptor::new::<TaskID>(
                    "TaskID",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<TaskID>()
    }
}

impl ::protobuf::Clear for TaskID {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for TaskID {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct TaskID_value_acc_type;
static TaskID_value_acc: TaskID_value_acc_type = TaskID_value_acc_type;

impl ::protobuf::reflect::FieldAccessor<TaskID> for TaskID_value_acc_type {
    fn name(&self) -> &'static str {
        "value"
    }

    fn has_field(&self, m: &TaskID) -> bool {
        m.has_value()
    }

    fn get_str<'a>(&self, m: &'a TaskID) -> &'a str {
        m.get_value()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct ExecutorID {
    value: ::protobuf::SingularField<::std::string::String>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> ExecutorID {
    pub fn new() -> ExecutorID {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExecutorID {
        static mut instance: ::protobuf::lazy::Lazy<ExecutorID> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExecutorID,
        };
        unsafe {
            instance.get(|| {
                ExecutorID {
                    value: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // required string value = 1;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&'a mut self) -> &'a mut ::std::string::String {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    pub fn get_value(&'a self) -> &'a str {
        match self.value.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }
}

impl ::protobuf::Message for ExecutorID {
    fn new() -> ExecutorID {
        ExecutorID::new()
    }

    fn is_initialized(&self) -> bool {
        if self.value.is_none() {
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
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.value.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.value.iter() {
            my_size += ::protobuf::rt::string_size(1, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.value.as_ref() {
            Some(v) => {
                try!(os.write_string(1, v.as_slice()));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<ExecutorID>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<ExecutorID>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&ExecutorID_value_acc as &'static ::protobuf::reflect::FieldAccessor<ExecutorID>) });
                ::protobuf::reflect::MessageDescriptor::new::<ExecutorID>(
                    "ExecutorID",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<ExecutorID>()
    }
}

impl ::protobuf::Clear for ExecutorID {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for ExecutorID {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct ExecutorID_value_acc_type;
static ExecutorID_value_acc: ExecutorID_value_acc_type = ExecutorID_value_acc_type;

impl ::protobuf::reflect::FieldAccessor<ExecutorID> for ExecutorID_value_acc_type {
    fn name(&self) -> &'static str {
        "value"
    }

    fn has_field(&self, m: &ExecutorID) -> bool {
        m.has_value()
    }

    fn get_str<'a>(&self, m: &'a ExecutorID) -> &'a str {
        m.get_value()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct ContainerID {
    value: ::protobuf::SingularField<::std::string::String>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> ContainerID {
    pub fn new() -> ContainerID {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ContainerID {
        static mut instance: ::protobuf::lazy::Lazy<ContainerID> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ContainerID,
        };
        unsafe {
            instance.get(|| {
                ContainerID {
                    value: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // required string value = 1;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&'a mut self) -> &'a mut ::std::string::String {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    pub fn get_value(&'a self) -> &'a str {
        match self.value.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }
}

impl ::protobuf::Message for ContainerID {
    fn new() -> ContainerID {
        ContainerID::new()
    }

    fn is_initialized(&self) -> bool {
        if self.value.is_none() {
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
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.value.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.value.iter() {
            my_size += ::protobuf::rt::string_size(1, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.value.as_ref() {
            Some(v) => {
                try!(os.write_string(1, v.as_slice()));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<ContainerID>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<ContainerID>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&ContainerID_value_acc as &'static ::protobuf::reflect::FieldAccessor<ContainerID>) });
                ::protobuf::reflect::MessageDescriptor::new::<ContainerID>(
                    "ContainerID",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<ContainerID>()
    }
}

impl ::protobuf::Clear for ContainerID {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for ContainerID {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct ContainerID_value_acc_type;
static ContainerID_value_acc: ContainerID_value_acc_type = ContainerID_value_acc_type;

impl ::protobuf::reflect::FieldAccessor<ContainerID> for ContainerID_value_acc_type {
    fn name(&self) -> &'static str {
        "value"
    }

    fn has_field(&self, m: &ContainerID) -> bool {
        m.has_value()
    }

    fn get_str<'a>(&self, m: &'a ContainerID) -> &'a str {
        m.get_value()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct FrameworkInfo {
    user: ::protobuf::SingularField<::std::string::String>,
    name: ::protobuf::SingularField<::std::string::String>,
    id: ::protobuf::SingularPtrField<FrameworkID>,
    failover_timeout: ::std::option::Option<f64>,
    checkpoint: ::std::option::Option<bool>,
    role: ::protobuf::SingularField<::std::string::String>,
    hostname: ::protobuf::SingularField<::std::string::String>,
    principal: ::protobuf::SingularField<::std::string::String>,
    webui_url: ::protobuf::SingularField<::std::string::String>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> FrameworkInfo {
    pub fn new() -> FrameworkInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FrameworkInfo {
        static mut instance: ::protobuf::lazy::Lazy<FrameworkInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FrameworkInfo,
        };
        unsafe {
            instance.get(|| {
                FrameworkInfo {
                    user: ::protobuf::SingularField::none(),
                    name: ::protobuf::SingularField::none(),
                    id: ::protobuf::SingularPtrField::none(),
                    failover_timeout: ::std::option::None,
                    checkpoint: ::std::option::None,
                    role: ::protobuf::SingularField::none(),
                    hostname: ::protobuf::SingularField::none(),
                    principal: ::protobuf::SingularField::none(),
                    webui_url: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // required string user = 1;

    pub fn clear_user(&mut self) {
        self.user.clear();
    }

    pub fn has_user(&self) -> bool {
        self.user.is_some()
    }

    // Param is passed by value, moved
    pub fn set_user(&mut self, v: ::std::string::String) {
        self.user = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_user(&'a mut self) -> &'a mut ::std::string::String {
        if self.user.is_none() {
            self.user.set_default();
        };
        self.user.as_mut().unwrap()
    }

    pub fn get_user(&'a self) -> &'a str {
        match self.user.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // required string name = 2;

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
    pub fn mut_name(&'a mut self) -> &'a mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    pub fn get_name(&'a self) -> &'a str {
        match self.name.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // optional .mesos.FrameworkID id = 3;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: FrameworkID) {
        self.id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&'a mut self) -> &'a mut FrameworkID {
        if self.id.is_none() {
            self.id.set_default();
        };
        self.id.as_mut().unwrap()
    }

    pub fn get_id(&'a self) -> &'a FrameworkID {
        self.id.as_ref().unwrap_or_else(|| FrameworkID::default_instance())
    }

    // optional double failover_timeout = 4;

    pub fn clear_failover_timeout(&mut self) {
        self.failover_timeout = None;
    }

    pub fn has_failover_timeout(&self) -> bool {
        self.failover_timeout.is_some()
    }

    // Param is passed by value, moved
    pub fn set_failover_timeout(&mut self, v: f64) {
        self.failover_timeout = Some(v);
    }

    pub fn get_failover_timeout(&self) -> f64 {
        self.failover_timeout.unwrap_or(0f64)
    }

    // optional bool checkpoint = 5;

    pub fn clear_checkpoint(&mut self) {
        self.checkpoint = None;
    }

    pub fn has_checkpoint(&self) -> bool {
        self.checkpoint.is_some()
    }

    // Param is passed by value, moved
    pub fn set_checkpoint(&mut self, v: bool) {
        self.checkpoint = Some(v);
    }

    pub fn get_checkpoint(&self) -> bool {
        self.checkpoint.unwrap_or(false)
    }

    // optional string role = 6;

    pub fn clear_role(&mut self) {
        self.role.clear();
    }

    pub fn has_role(&self) -> bool {
        self.role.is_some()
    }

    // Param is passed by value, moved
    pub fn set_role(&mut self, v: ::std::string::String) {
        self.role = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_role(&'a mut self) -> &'a mut ::std::string::String {
        if self.role.is_none() {
            self.role.set_default();
        };
        self.role.as_mut().unwrap()
    }

    pub fn get_role(&'a self) -> &'a str {
        match self.role.as_ref() {
            Some(v) => v.as_slice(),
            None => "*",
        }
    }

    // optional string hostname = 7;

    pub fn clear_hostname(&mut self) {
        self.hostname.clear();
    }

    pub fn has_hostname(&self) -> bool {
        self.hostname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hostname(&mut self, v: ::std::string::String) {
        self.hostname = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hostname(&'a mut self) -> &'a mut ::std::string::String {
        if self.hostname.is_none() {
            self.hostname.set_default();
        };
        self.hostname.as_mut().unwrap()
    }

    pub fn get_hostname(&'a self) -> &'a str {
        match self.hostname.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // optional string principal = 8;

    pub fn clear_principal(&mut self) {
        self.principal.clear();
    }

    pub fn has_principal(&self) -> bool {
        self.principal.is_some()
    }

    // Param is passed by value, moved
    pub fn set_principal(&mut self, v: ::std::string::String) {
        self.principal = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_principal(&'a mut self) -> &'a mut ::std::string::String {
        if self.principal.is_none() {
            self.principal.set_default();
        };
        self.principal.as_mut().unwrap()
    }

    pub fn get_principal(&'a self) -> &'a str {
        match self.principal.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // optional string webui_url = 9;

    pub fn clear_webui_url(&mut self) {
        self.webui_url.clear();
    }

    pub fn has_webui_url(&self) -> bool {
        self.webui_url.is_some()
    }

    // Param is passed by value, moved
    pub fn set_webui_url(&mut self, v: ::std::string::String) {
        self.webui_url = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_webui_url(&'a mut self) -> &'a mut ::std::string::String {
        if self.webui_url.is_none() {
            self.webui_url.set_default();
        };
        self.webui_url.as_mut().unwrap()
    }

    pub fn get_webui_url(&'a self) -> &'a str {
        match self.webui_url.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }
}

impl ::protobuf::Message for FrameworkInfo {
    fn new() -> FrameworkInfo {
        FrameworkInfo::new()
    }

    fn is_initialized(&self) -> bool {
        if self.user.is_none() {
            return false;
        };
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
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.user.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.name.set_default();
                    try!(is.read_string_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.id.set_default();
                    try!(is.merge_message(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.failover_timeout = Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.checkpoint = Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.role.set_default();
                    try!(is.read_string_into(tmp))
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.hostname.set_default();
                    try!(is.read_string_into(tmp))
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.principal.set_default();
                    try!(is.read_string_into(tmp))
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.webui_url.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.user.iter() {
            my_size += ::protobuf::rt::string_size(1, value.as_slice());
        };
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(2, value.as_slice());
        };
        for value in self.id.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.failover_timeout.is_some() {
            my_size += 9;
        };
        if self.checkpoint.is_some() {
            my_size += 2;
        };
        for value in self.role.iter() {
            my_size += ::protobuf::rt::string_size(6, value.as_slice());
        };
        for value in self.hostname.iter() {
            my_size += ::protobuf::rt::string_size(7, value.as_slice());
        };
        for value in self.principal.iter() {
            my_size += ::protobuf::rt::string_size(8, value.as_slice());
        };
        for value in self.webui_url.iter() {
            my_size += ::protobuf::rt::string_size(9, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.user.as_ref() {
            Some(v) => {
                try!(os.write_string(1, v.as_slice()));
            },
            None => {},
        };
        match self.name.as_ref() {
            Some(v) => {
                try!(os.write_string(2, v.as_slice()));
            },
            None => {},
        };
        match self.id.as_ref() {
            Some(v) => {
                try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.failover_timeout {
            Some(v) => {
                try!(os.write_double(4, v));
            },
            None => {},
        };
        match self.checkpoint {
            Some(v) => {
                try!(os.write_bool(5, v));
            },
            None => {},
        };
        match self.role.as_ref() {
            Some(v) => {
                try!(os.write_string(6, v.as_slice()));
            },
            None => {},
        };
        match self.hostname.as_ref() {
            Some(v) => {
                try!(os.write_string(7, v.as_slice()));
            },
            None => {},
        };
        match self.principal.as_ref() {
            Some(v) => {
                try!(os.write_string(8, v.as_slice()));
            },
            None => {},
        };
        match self.webui_url.as_ref() {
            Some(v) => {
                try!(os.write_string(9, v.as_slice()));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<FrameworkInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<FrameworkInfo>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&FrameworkInfo_user_acc as &'static ::protobuf::reflect::FieldAccessor<FrameworkInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&FrameworkInfo_name_acc as &'static ::protobuf::reflect::FieldAccessor<FrameworkInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&FrameworkInfo_id_acc as &'static ::protobuf::reflect::FieldAccessor<FrameworkInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&FrameworkInfo_failover_timeout_acc as &'static ::protobuf::reflect::FieldAccessor<FrameworkInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&FrameworkInfo_checkpoint_acc as &'static ::protobuf::reflect::FieldAccessor<FrameworkInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&FrameworkInfo_role_acc as &'static ::protobuf::reflect::FieldAccessor<FrameworkInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&FrameworkInfo_hostname_acc as &'static ::protobuf::reflect::FieldAccessor<FrameworkInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&FrameworkInfo_principal_acc as &'static ::protobuf::reflect::FieldAccessor<FrameworkInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&FrameworkInfo_webui_url_acc as &'static ::protobuf::reflect::FieldAccessor<FrameworkInfo>) });
                ::protobuf::reflect::MessageDescriptor::new::<FrameworkInfo>(
                    "FrameworkInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<FrameworkInfo>()
    }
}

impl ::protobuf::Clear for FrameworkInfo {
    fn clear(&mut self) {
        self.clear_user();
        self.clear_name();
        self.clear_id();
        self.clear_failover_timeout();
        self.clear_checkpoint();
        self.clear_role();
        self.clear_hostname();
        self.clear_principal();
        self.clear_webui_url();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for FrameworkInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct FrameworkInfo_user_acc_type;
static FrameworkInfo_user_acc: FrameworkInfo_user_acc_type = FrameworkInfo_user_acc_type;

impl ::protobuf::reflect::FieldAccessor<FrameworkInfo> for FrameworkInfo_user_acc_type {
    fn name(&self) -> &'static str {
        "user"
    }

    fn has_field(&self, m: &FrameworkInfo) -> bool {
        m.has_user()
    }

    fn get_str<'a>(&self, m: &'a FrameworkInfo) -> &'a str {
        m.get_user()
    }
}

#[allow(non_camel_case_types)]
struct FrameworkInfo_name_acc_type;
static FrameworkInfo_name_acc: FrameworkInfo_name_acc_type = FrameworkInfo_name_acc_type;

impl ::protobuf::reflect::FieldAccessor<FrameworkInfo> for FrameworkInfo_name_acc_type {
    fn name(&self) -> &'static str {
        "name"
    }

    fn has_field(&self, m: &FrameworkInfo) -> bool {
        m.has_name()
    }

    fn get_str<'a>(&self, m: &'a FrameworkInfo) -> &'a str {
        m.get_name()
    }
}

#[allow(non_camel_case_types)]
struct FrameworkInfo_id_acc_type;
static FrameworkInfo_id_acc: FrameworkInfo_id_acc_type = FrameworkInfo_id_acc_type;

impl ::protobuf::reflect::FieldAccessor<FrameworkInfo> for FrameworkInfo_id_acc_type {
    fn name(&self) -> &'static str {
        "id"
    }

    fn has_field(&self, m: &FrameworkInfo) -> bool {
        m.has_id()
    }

    fn get_message<'a>(&self, m: &'a FrameworkInfo) -> &'a ::protobuf::Message {
        m.get_id() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct FrameworkInfo_failover_timeout_acc_type;
static FrameworkInfo_failover_timeout_acc: FrameworkInfo_failover_timeout_acc_type = FrameworkInfo_failover_timeout_acc_type;

impl ::protobuf::reflect::FieldAccessor<FrameworkInfo> for FrameworkInfo_failover_timeout_acc_type {
    fn name(&self) -> &'static str {
        "failover_timeout"
    }

    fn has_field(&self, m: &FrameworkInfo) -> bool {
        m.has_failover_timeout()
    }

    fn get_f64(&self, m: &FrameworkInfo) -> f64 {
        m.get_failover_timeout()
    }
}

#[allow(non_camel_case_types)]
struct FrameworkInfo_checkpoint_acc_type;
static FrameworkInfo_checkpoint_acc: FrameworkInfo_checkpoint_acc_type = FrameworkInfo_checkpoint_acc_type;

impl ::protobuf::reflect::FieldAccessor<FrameworkInfo> for FrameworkInfo_checkpoint_acc_type {
    fn name(&self) -> &'static str {
        "checkpoint"
    }

    fn has_field(&self, m: &FrameworkInfo) -> bool {
        m.has_checkpoint()
    }

    fn get_bool(&self, m: &FrameworkInfo) -> bool {
        m.get_checkpoint()
    }
}

#[allow(non_camel_case_types)]
struct FrameworkInfo_role_acc_type;
static FrameworkInfo_role_acc: FrameworkInfo_role_acc_type = FrameworkInfo_role_acc_type;

impl ::protobuf::reflect::FieldAccessor<FrameworkInfo> for FrameworkInfo_role_acc_type {
    fn name(&self) -> &'static str {
        "role"
    }

    fn has_field(&self, m: &FrameworkInfo) -> bool {
        m.has_role()
    }

    fn get_str<'a>(&self, m: &'a FrameworkInfo) -> &'a str {
        m.get_role()
    }
}

#[allow(non_camel_case_types)]
struct FrameworkInfo_hostname_acc_type;
static FrameworkInfo_hostname_acc: FrameworkInfo_hostname_acc_type = FrameworkInfo_hostname_acc_type;

impl ::protobuf::reflect::FieldAccessor<FrameworkInfo> for FrameworkInfo_hostname_acc_type {
    fn name(&self) -> &'static str {
        "hostname"
    }

    fn has_field(&self, m: &FrameworkInfo) -> bool {
        m.has_hostname()
    }

    fn get_str<'a>(&self, m: &'a FrameworkInfo) -> &'a str {
        m.get_hostname()
    }
}

#[allow(non_camel_case_types)]
struct FrameworkInfo_principal_acc_type;
static FrameworkInfo_principal_acc: FrameworkInfo_principal_acc_type = FrameworkInfo_principal_acc_type;

impl ::protobuf::reflect::FieldAccessor<FrameworkInfo> for FrameworkInfo_principal_acc_type {
    fn name(&self) -> &'static str {
        "principal"
    }

    fn has_field(&self, m: &FrameworkInfo) -> bool {
        m.has_principal()
    }

    fn get_str<'a>(&self, m: &'a FrameworkInfo) -> &'a str {
        m.get_principal()
    }
}

#[allow(non_camel_case_types)]
struct FrameworkInfo_webui_url_acc_type;
static FrameworkInfo_webui_url_acc: FrameworkInfo_webui_url_acc_type = FrameworkInfo_webui_url_acc_type;

impl ::protobuf::reflect::FieldAccessor<FrameworkInfo> for FrameworkInfo_webui_url_acc_type {
    fn name(&self) -> &'static str {
        "webui_url"
    }

    fn has_field(&self, m: &FrameworkInfo) -> bool {
        m.has_webui_url()
    }

    fn get_str<'a>(&self, m: &'a FrameworkInfo) -> &'a str {
        m.get_webui_url()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct HealthCheck {
    http: ::protobuf::SingularPtrField<HealthCheck_HTTP>,
    delay_seconds: ::std::option::Option<f64>,
    interval_seconds: ::std::option::Option<f64>,
    timeout_seconds: ::std::option::Option<f64>,
    consecutive_failures: ::std::option::Option<u32>,
    grace_period_seconds: ::std::option::Option<f64>,
    command: ::protobuf::SingularPtrField<CommandInfo>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> HealthCheck {
    pub fn new() -> HealthCheck {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HealthCheck {
        static mut instance: ::protobuf::lazy::Lazy<HealthCheck> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HealthCheck,
        };
        unsafe {
            instance.get(|| {
                HealthCheck {
                    http: ::protobuf::SingularPtrField::none(),
                    delay_seconds: ::std::option::None,
                    interval_seconds: ::std::option::None,
                    timeout_seconds: ::std::option::None,
                    consecutive_failures: ::std::option::None,
                    grace_period_seconds: ::std::option::None,
                    command: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // optional .mesos.HealthCheck.HTTP http = 1;

    pub fn clear_http(&mut self) {
        self.http.clear();
    }

    pub fn has_http(&self) -> bool {
        self.http.is_some()
    }

    // Param is passed by value, moved
    pub fn set_http(&mut self, v: HealthCheck_HTTP) {
        self.http = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_http(&'a mut self) -> &'a mut HealthCheck_HTTP {
        if self.http.is_none() {
            self.http.set_default();
        };
        self.http.as_mut().unwrap()
    }

    pub fn get_http(&'a self) -> &'a HealthCheck_HTTP {
        self.http.as_ref().unwrap_or_else(|| HealthCheck_HTTP::default_instance())
    }

    // optional double delay_seconds = 2;

    pub fn clear_delay_seconds(&mut self) {
        self.delay_seconds = None;
    }

    pub fn has_delay_seconds(&self) -> bool {
        self.delay_seconds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_delay_seconds(&mut self, v: f64) {
        self.delay_seconds = Some(v);
    }

    pub fn get_delay_seconds(&self) -> f64 {
        self.delay_seconds.unwrap_or(15f64)
    }

    // optional double interval_seconds = 3;

    pub fn clear_interval_seconds(&mut self) {
        self.interval_seconds = None;
    }

    pub fn has_interval_seconds(&self) -> bool {
        self.interval_seconds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_interval_seconds(&mut self, v: f64) {
        self.interval_seconds = Some(v);
    }

    pub fn get_interval_seconds(&self) -> f64 {
        self.interval_seconds.unwrap_or(10f64)
    }

    // optional double timeout_seconds = 4;

    pub fn clear_timeout_seconds(&mut self) {
        self.timeout_seconds = None;
    }

    pub fn has_timeout_seconds(&self) -> bool {
        self.timeout_seconds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timeout_seconds(&mut self, v: f64) {
        self.timeout_seconds = Some(v);
    }

    pub fn get_timeout_seconds(&self) -> f64 {
        self.timeout_seconds.unwrap_or(20f64)
    }

    // optional uint32 consecutive_failures = 5;

    pub fn clear_consecutive_failures(&mut self) {
        self.consecutive_failures = None;
    }

    pub fn has_consecutive_failures(&self) -> bool {
        self.consecutive_failures.is_some()
    }

    // Param is passed by value, moved
    pub fn set_consecutive_failures(&mut self, v: u32) {
        self.consecutive_failures = Some(v);
    }

    pub fn get_consecutive_failures(&self) -> u32 {
        self.consecutive_failures.unwrap_or(3u32)
    }

    // optional double grace_period_seconds = 6;

    pub fn clear_grace_period_seconds(&mut self) {
        self.grace_period_seconds = None;
    }

    pub fn has_grace_period_seconds(&self) -> bool {
        self.grace_period_seconds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_grace_period_seconds(&mut self, v: f64) {
        self.grace_period_seconds = Some(v);
    }

    pub fn get_grace_period_seconds(&self) -> f64 {
        self.grace_period_seconds.unwrap_or(10f64)
    }

    // optional .mesos.CommandInfo command = 7;

    pub fn clear_command(&mut self) {
        self.command.clear();
    }

    pub fn has_command(&self) -> bool {
        self.command.is_some()
    }

    // Param is passed by value, moved
    pub fn set_command(&mut self, v: CommandInfo) {
        self.command = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_command(&'a mut self) -> &'a mut CommandInfo {
        if self.command.is_none() {
            self.command.set_default();
        };
        self.command.as_mut().unwrap()
    }

    pub fn get_command(&'a self) -> &'a CommandInfo {
        self.command.as_ref().unwrap_or_else(|| CommandInfo::default_instance())
    }
}

impl ::protobuf::Message for HealthCheck {
    fn new() -> HealthCheck {
        HealthCheck::new()
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.http.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.delay_seconds = Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.interval_seconds = Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.timeout_seconds = Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.consecutive_failures = Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.grace_period_seconds = Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.command.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.http.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.delay_seconds.is_some() {
            my_size += 9;
        };
        if self.interval_seconds.is_some() {
            my_size += 9;
        };
        if self.timeout_seconds.is_some() {
            my_size += 9;
        };
        for value in self.consecutive_failures.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.grace_period_seconds.is_some() {
            my_size += 9;
        };
        for value in self.command.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.http.as_ref() {
            Some(v) => {
                try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.delay_seconds {
            Some(v) => {
                try!(os.write_double(2, v));
            },
            None => {},
        };
        match self.interval_seconds {
            Some(v) => {
                try!(os.write_double(3, v));
            },
            None => {},
        };
        match self.timeout_seconds {
            Some(v) => {
                try!(os.write_double(4, v));
            },
            None => {},
        };
        match self.consecutive_failures {
            Some(v) => {
                try!(os.write_uint32(5, v));
            },
            None => {},
        };
        match self.grace_period_seconds {
            Some(v) => {
                try!(os.write_double(6, v));
            },
            None => {},
        };
        match self.command.as_ref() {
            Some(v) => {
                try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<HealthCheck>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<HealthCheck>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&HealthCheck_http_acc as &'static ::protobuf::reflect::FieldAccessor<HealthCheck>) });
                fields.push(unsafe { ::std::mem::transmute(&HealthCheck_delay_seconds_acc as &'static ::protobuf::reflect::FieldAccessor<HealthCheck>) });
                fields.push(unsafe { ::std::mem::transmute(&HealthCheck_interval_seconds_acc as &'static ::protobuf::reflect::FieldAccessor<HealthCheck>) });
                fields.push(unsafe { ::std::mem::transmute(&HealthCheck_timeout_seconds_acc as &'static ::protobuf::reflect::FieldAccessor<HealthCheck>) });
                fields.push(unsafe { ::std::mem::transmute(&HealthCheck_consecutive_failures_acc as &'static ::protobuf::reflect::FieldAccessor<HealthCheck>) });
                fields.push(unsafe { ::std::mem::transmute(&HealthCheck_grace_period_seconds_acc as &'static ::protobuf::reflect::FieldAccessor<HealthCheck>) });
                fields.push(unsafe { ::std::mem::transmute(&HealthCheck_command_acc as &'static ::protobuf::reflect::FieldAccessor<HealthCheck>) });
                ::protobuf::reflect::MessageDescriptor::new::<HealthCheck>(
                    "HealthCheck",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<HealthCheck>()
    }
}

impl ::protobuf::Clear for HealthCheck {
    fn clear(&mut self) {
        self.clear_http();
        self.clear_delay_seconds();
        self.clear_interval_seconds();
        self.clear_timeout_seconds();
        self.clear_consecutive_failures();
        self.clear_grace_period_seconds();
        self.clear_command();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for HealthCheck {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct HealthCheck_http_acc_type;
static HealthCheck_http_acc: HealthCheck_http_acc_type = HealthCheck_http_acc_type;

impl ::protobuf::reflect::FieldAccessor<HealthCheck> for HealthCheck_http_acc_type {
    fn name(&self) -> &'static str {
        "http"
    }

    fn has_field(&self, m: &HealthCheck) -> bool {
        m.has_http()
    }

    fn get_message<'a>(&self, m: &'a HealthCheck) -> &'a ::protobuf::Message {
        m.get_http() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct HealthCheck_delay_seconds_acc_type;
static HealthCheck_delay_seconds_acc: HealthCheck_delay_seconds_acc_type = HealthCheck_delay_seconds_acc_type;

impl ::protobuf::reflect::FieldAccessor<HealthCheck> for HealthCheck_delay_seconds_acc_type {
    fn name(&self) -> &'static str {
        "delay_seconds"
    }

    fn has_field(&self, m: &HealthCheck) -> bool {
        m.has_delay_seconds()
    }

    fn get_f64(&self, m: &HealthCheck) -> f64 {
        m.get_delay_seconds()
    }
}

#[allow(non_camel_case_types)]
struct HealthCheck_interval_seconds_acc_type;
static HealthCheck_interval_seconds_acc: HealthCheck_interval_seconds_acc_type = HealthCheck_interval_seconds_acc_type;

impl ::protobuf::reflect::FieldAccessor<HealthCheck> for HealthCheck_interval_seconds_acc_type {
    fn name(&self) -> &'static str {
        "interval_seconds"
    }

    fn has_field(&self, m: &HealthCheck) -> bool {
        m.has_interval_seconds()
    }

    fn get_f64(&self, m: &HealthCheck) -> f64 {
        m.get_interval_seconds()
    }
}

#[allow(non_camel_case_types)]
struct HealthCheck_timeout_seconds_acc_type;
static HealthCheck_timeout_seconds_acc: HealthCheck_timeout_seconds_acc_type = HealthCheck_timeout_seconds_acc_type;

impl ::protobuf::reflect::FieldAccessor<HealthCheck> for HealthCheck_timeout_seconds_acc_type {
    fn name(&self) -> &'static str {
        "timeout_seconds"
    }

    fn has_field(&self, m: &HealthCheck) -> bool {
        m.has_timeout_seconds()
    }

    fn get_f64(&self, m: &HealthCheck) -> f64 {
        m.get_timeout_seconds()
    }
}

#[allow(non_camel_case_types)]
struct HealthCheck_consecutive_failures_acc_type;
static HealthCheck_consecutive_failures_acc: HealthCheck_consecutive_failures_acc_type = HealthCheck_consecutive_failures_acc_type;

impl ::protobuf::reflect::FieldAccessor<HealthCheck> for HealthCheck_consecutive_failures_acc_type {
    fn name(&self) -> &'static str {
        "consecutive_failures"
    }

    fn has_field(&self, m: &HealthCheck) -> bool {
        m.has_consecutive_failures()
    }

    fn get_u32(&self, m: &HealthCheck) -> u32 {
        m.get_consecutive_failures()
    }
}

#[allow(non_camel_case_types)]
struct HealthCheck_grace_period_seconds_acc_type;
static HealthCheck_grace_period_seconds_acc: HealthCheck_grace_period_seconds_acc_type = HealthCheck_grace_period_seconds_acc_type;

impl ::protobuf::reflect::FieldAccessor<HealthCheck> for HealthCheck_grace_period_seconds_acc_type {
    fn name(&self) -> &'static str {
        "grace_period_seconds"
    }

    fn has_field(&self, m: &HealthCheck) -> bool {
        m.has_grace_period_seconds()
    }

    fn get_f64(&self, m: &HealthCheck) -> f64 {
        m.get_grace_period_seconds()
    }
}

#[allow(non_camel_case_types)]
struct HealthCheck_command_acc_type;
static HealthCheck_command_acc: HealthCheck_command_acc_type = HealthCheck_command_acc_type;

impl ::protobuf::reflect::FieldAccessor<HealthCheck> for HealthCheck_command_acc_type {
    fn name(&self) -> &'static str {
        "command"
    }

    fn has_field(&self, m: &HealthCheck) -> bool {
        m.has_command()
    }

    fn get_message<'a>(&self, m: &'a HealthCheck) -> &'a ::protobuf::Message {
        m.get_command() as &'a ::protobuf::Message
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct HealthCheck_HTTP {
    port: ::std::option::Option<u32>,
    path: ::protobuf::SingularField<::std::string::String>,
    statuses: ::std::vec::Vec<u32>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> HealthCheck_HTTP {
    pub fn new() -> HealthCheck_HTTP {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HealthCheck_HTTP {
        static mut instance: ::protobuf::lazy::Lazy<HealthCheck_HTTP> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HealthCheck_HTTP,
        };
        unsafe {
            instance.get(|| {
                HealthCheck_HTTP {
                    port: ::std::option::None,
                    path: ::protobuf::SingularField::none(),
                    statuses: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // required uint32 port = 1;

    pub fn clear_port(&mut self) {
        self.port = None;
    }

    pub fn has_port(&self) -> bool {
        self.port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_port(&mut self, v: u32) {
        self.port = Some(v);
    }

    pub fn get_port(&self) -> u32 {
        self.port.unwrap_or(0)
    }

    // optional string path = 2;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&'a mut self) -> &'a mut ::std::string::String {
        if self.path.is_none() {
            self.path.set_default();
        };
        self.path.as_mut().unwrap()
    }

    pub fn get_path(&'a self) -> &'a str {
        match self.path.as_ref() {
            Some(v) => v.as_slice(),
            None => "/",
        }
    }

    // repeated uint32 statuses = 4;

    pub fn clear_statuses(&mut self) {
        self.statuses.clear();
    }

    // Param is passed by value, moved
    pub fn set_statuses(&mut self, v: ::std::vec::Vec<u32>) {
        self.statuses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_statuses(&'a mut self) -> &'a mut ::std::vec::Vec<u32> {
        &mut self.statuses
    }

    pub fn get_statuses(&'a self) -> &'a [u32] {
        self.statuses.as_slice()
    }
}

impl ::protobuf::Message for HealthCheck_HTTP {
    fn new() -> HealthCheck_HTTP {
        HealthCheck_HTTP::new()
    }

    fn is_initialized(&self) -> bool {
        if self.port.is_none() {
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
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.port = Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.path.set_default();
                    try!(is.read_string_into(tmp))
                },
                4 => {
                    if wire_type == ::protobuf::wire_format::WireTypeLengthDelimited {
                        let len = try!(is.read_raw_varint32());
                        let old_limit = is.push_limit(len);
                        while !try!(is.eof()) {
                            self.statuses.push(try!(is.read_uint32()));
                        }
                        is.pop_limit(old_limit);
                    } else {
                        if wire_type != ::protobuf::wire_format::WireTypeVarint {
                            return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                        };
                        self.statuses.push(try!(is.read_uint32()));
                    }
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.port.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.path.iter() {
            my_size += ::protobuf::rt::string_size(2, value.as_slice());
        };
        for value in self.statuses.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.port {
            Some(v) => {
                try!(os.write_uint32(1, v));
            },
            None => {},
        };
        match self.path.as_ref() {
            Some(v) => {
                try!(os.write_string(2, v.as_slice()));
            },
            None => {},
        };
        for v in self.statuses.iter() {
            try!(os.write_uint32(4, *v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<HealthCheck_HTTP>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<HealthCheck_HTTP>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&HealthCheck_HTTP_port_acc as &'static ::protobuf::reflect::FieldAccessor<HealthCheck_HTTP>) });
                fields.push(unsafe { ::std::mem::transmute(&HealthCheck_HTTP_path_acc as &'static ::protobuf::reflect::FieldAccessor<HealthCheck_HTTP>) });
                fields.push(unsafe { ::std::mem::transmute(&HealthCheck_HTTP_statuses_acc as &'static ::protobuf::reflect::FieldAccessor<HealthCheck_HTTP>) });
                ::protobuf::reflect::MessageDescriptor::new::<HealthCheck_HTTP>(
                    "HealthCheck_HTTP",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<HealthCheck_HTTP>()
    }
}

impl ::protobuf::Clear for HealthCheck_HTTP {
    fn clear(&mut self) {
        self.clear_port();
        self.clear_path();
        self.clear_statuses();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for HealthCheck_HTTP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct HealthCheck_HTTP_port_acc_type;
static HealthCheck_HTTP_port_acc: HealthCheck_HTTP_port_acc_type = HealthCheck_HTTP_port_acc_type;

impl ::protobuf::reflect::FieldAccessor<HealthCheck_HTTP> for HealthCheck_HTTP_port_acc_type {
    fn name(&self) -> &'static str {
        "port"
    }

    fn has_field(&self, m: &HealthCheck_HTTP) -> bool {
        m.has_port()
    }

    fn get_u32(&self, m: &HealthCheck_HTTP) -> u32 {
        m.get_port()
    }
}

#[allow(non_camel_case_types)]
struct HealthCheck_HTTP_path_acc_type;
static HealthCheck_HTTP_path_acc: HealthCheck_HTTP_path_acc_type = HealthCheck_HTTP_path_acc_type;

impl ::protobuf::reflect::FieldAccessor<HealthCheck_HTTP> for HealthCheck_HTTP_path_acc_type {
    fn name(&self) -> &'static str {
        "path"
    }

    fn has_field(&self, m: &HealthCheck_HTTP) -> bool {
        m.has_path()
    }

    fn get_str<'a>(&self, m: &'a HealthCheck_HTTP) -> &'a str {
        m.get_path()
    }
}

#[allow(non_camel_case_types)]
struct HealthCheck_HTTP_statuses_acc_type;
static HealthCheck_HTTP_statuses_acc: HealthCheck_HTTP_statuses_acc_type = HealthCheck_HTTP_statuses_acc_type;

impl ::protobuf::reflect::FieldAccessor<HealthCheck_HTTP> for HealthCheck_HTTP_statuses_acc_type {
    fn name(&self) -> &'static str {
        "statuses"
    }

    fn len_field(&self, m: &HealthCheck_HTTP) -> uint {
        m.get_statuses().len()
    }

    fn get_rep_u32<'a>(&self, m: &'a HealthCheck_HTTP) -> &'a [u32] {
        m.get_statuses()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct CommandInfo {
    container: ::protobuf::SingularPtrField<CommandInfo_ContainerInfo>,
    uris: ::protobuf::RepeatedField<CommandInfo_URI>,
    environment: ::protobuf::SingularPtrField<Environment>,
    shell: ::std::option::Option<bool>,
    value: ::protobuf::SingularField<::std::string::String>,
    arguments: ::protobuf::RepeatedField<::std::string::String>,
    user: ::protobuf::SingularField<::std::string::String>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> CommandInfo {
    pub fn new() -> CommandInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CommandInfo {
        static mut instance: ::protobuf::lazy::Lazy<CommandInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CommandInfo,
        };
        unsafe {
            instance.get(|| {
                CommandInfo {
                    container: ::protobuf::SingularPtrField::none(),
                    uris: ::protobuf::RepeatedField::new(),
                    environment: ::protobuf::SingularPtrField::none(),
                    shell: ::std::option::None,
                    value: ::protobuf::SingularField::none(),
                    arguments: ::protobuf::RepeatedField::new(),
                    user: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // optional .mesos.CommandInfo.ContainerInfo container = 4;

    pub fn clear_container(&mut self) {
        self.container.clear();
    }

    pub fn has_container(&self) -> bool {
        self.container.is_some()
    }

    // Param is passed by value, moved
    pub fn set_container(&mut self, v: CommandInfo_ContainerInfo) {
        self.container = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_container(&'a mut self) -> &'a mut CommandInfo_ContainerInfo {
        if self.container.is_none() {
            self.container.set_default();
        };
        self.container.as_mut().unwrap()
    }

    pub fn get_container(&'a self) -> &'a CommandInfo_ContainerInfo {
        self.container.as_ref().unwrap_or_else(|| CommandInfo_ContainerInfo::default_instance())
    }

    // repeated .mesos.CommandInfo.URI uris = 1;

    pub fn clear_uris(&mut self) {
        self.uris.clear();
    }

    // Param is passed by value, moved
    pub fn set_uris(&mut self, v: ::protobuf::RepeatedField<CommandInfo_URI>) {
        self.uris = v;
    }

    // Mutable pointer to the field.
    pub fn mut_uris(&'a mut self) -> &'a mut ::protobuf::RepeatedField<CommandInfo_URI> {
        &mut self.uris
    }

    pub fn get_uris(&'a self) -> &'a [CommandInfo_URI] {
        self.uris.as_slice()
    }

    // optional .mesos.Environment environment = 2;

    pub fn clear_environment(&mut self) {
        self.environment.clear();
    }

    pub fn has_environment(&self) -> bool {
        self.environment.is_some()
    }

    // Param is passed by value, moved
    pub fn set_environment(&mut self, v: Environment) {
        self.environment = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_environment(&'a mut self) -> &'a mut Environment {
        if self.environment.is_none() {
            self.environment.set_default();
        };
        self.environment.as_mut().unwrap()
    }

    pub fn get_environment(&'a self) -> &'a Environment {
        self.environment.as_ref().unwrap_or_else(|| Environment::default_instance())
    }

    // optional bool shell = 6;

    pub fn clear_shell(&mut self) {
        self.shell = None;
    }

    pub fn has_shell(&self) -> bool {
        self.shell.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shell(&mut self, v: bool) {
        self.shell = Some(v);
    }

    pub fn get_shell(&self) -> bool {
        self.shell.unwrap_or(true)
    }

    // optional string value = 3;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&'a mut self) -> &'a mut ::std::string::String {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    pub fn get_value(&'a self) -> &'a str {
        match self.value.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // repeated string arguments = 7;

    pub fn clear_arguments(&mut self) {
        self.arguments.clear();
    }

    // Param is passed by value, moved
    pub fn set_arguments(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.arguments = v;
    }

    // Mutable pointer to the field.
    pub fn mut_arguments(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.arguments
    }

    pub fn get_arguments(&'a self) -> &'a [::std::string::String] {
        self.arguments.as_slice()
    }

    // optional string user = 5;

    pub fn clear_user(&mut self) {
        self.user.clear();
    }

    pub fn has_user(&self) -> bool {
        self.user.is_some()
    }

    // Param is passed by value, moved
    pub fn set_user(&mut self, v: ::std::string::String) {
        self.user = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_user(&'a mut self) -> &'a mut ::std::string::String {
        if self.user.is_none() {
            self.user.set_default();
        };
        self.user.as_mut().unwrap()
    }

    pub fn get_user(&'a self) -> &'a str {
        match self.user.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }
}

impl ::protobuf::Message for CommandInfo {
    fn new() -> CommandInfo {
        CommandInfo::new()
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.container.set_default();
                    try!(is.merge_message(tmp))
                },
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.uris.push_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.environment.set_default();
                    try!(is.merge_message(tmp))
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.shell = Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.value.set_default();
                    try!(is.read_string_into(tmp))
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.arguments.push_default();
                    try!(is.read_string_into(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.user.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.container.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.uris.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.environment.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.shell.is_some() {
            my_size += 2;
        };
        for value in self.value.iter() {
            my_size += ::protobuf::rt::string_size(3, value.as_slice());
        };
        for value in self.arguments.iter() {
            my_size += ::protobuf::rt::string_size(7, value.as_slice());
        };
        for value in self.user.iter() {
            my_size += ::protobuf::rt::string_size(5, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.container.as_ref() {
            Some(v) => {
                try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        for v in self.uris.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(sizes[*sizes_pos]));
            *sizes_pos += 1;
            try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
        };
        match self.environment.as_ref() {
            Some(v) => {
                try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.shell {
            Some(v) => {
                try!(os.write_bool(6, v));
            },
            None => {},
        };
        match self.value.as_ref() {
            Some(v) => {
                try!(os.write_string(3, v.as_slice()));
            },
            None => {},
        };
        for v in self.arguments.iter() {
            try!(os.write_string(7, v.as_slice()));
        };
        match self.user.as_ref() {
            Some(v) => {
                try!(os.write_string(5, v.as_slice()));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CommandInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<CommandInfo>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&CommandInfo_container_acc as &'static ::protobuf::reflect::FieldAccessor<CommandInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&CommandInfo_uris_acc as &'static ::protobuf::reflect::FieldAccessor<CommandInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&CommandInfo_environment_acc as &'static ::protobuf::reflect::FieldAccessor<CommandInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&CommandInfo_shell_acc as &'static ::protobuf::reflect::FieldAccessor<CommandInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&CommandInfo_value_acc as &'static ::protobuf::reflect::FieldAccessor<CommandInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&CommandInfo_arguments_acc as &'static ::protobuf::reflect::FieldAccessor<CommandInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&CommandInfo_user_acc as &'static ::protobuf::reflect::FieldAccessor<CommandInfo>) });
                ::protobuf::reflect::MessageDescriptor::new::<CommandInfo>(
                    "CommandInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<CommandInfo>()
    }
}

impl ::protobuf::Clear for CommandInfo {
    fn clear(&mut self) {
        self.clear_container();
        self.clear_uris();
        self.clear_environment();
        self.clear_shell();
        self.clear_value();
        self.clear_arguments();
        self.clear_user();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for CommandInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct CommandInfo_container_acc_type;
static CommandInfo_container_acc: CommandInfo_container_acc_type = CommandInfo_container_acc_type;

impl ::protobuf::reflect::FieldAccessor<CommandInfo> for CommandInfo_container_acc_type {
    fn name(&self) -> &'static str {
        "container"
    }

    fn has_field(&self, m: &CommandInfo) -> bool {
        m.has_container()
    }

    fn get_message<'a>(&self, m: &'a CommandInfo) -> &'a ::protobuf::Message {
        m.get_container() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct CommandInfo_uris_acc_type;
static CommandInfo_uris_acc: CommandInfo_uris_acc_type = CommandInfo_uris_acc_type;

impl ::protobuf::reflect::FieldAccessor<CommandInfo> for CommandInfo_uris_acc_type {
    fn name(&self) -> &'static str {
        "uris"
    }

    fn len_field(&self, m: &CommandInfo) -> uint {
        m.get_uris().len()
    }

    fn get_rep_message_item<'a>(&self, m: &'a CommandInfo, index: uint) -> &'a ::protobuf::Message {
        &m.get_uris()[index] as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct CommandInfo_environment_acc_type;
static CommandInfo_environment_acc: CommandInfo_environment_acc_type = CommandInfo_environment_acc_type;

impl ::protobuf::reflect::FieldAccessor<CommandInfo> for CommandInfo_environment_acc_type {
    fn name(&self) -> &'static str {
        "environment"
    }

    fn has_field(&self, m: &CommandInfo) -> bool {
        m.has_environment()
    }

    fn get_message<'a>(&self, m: &'a CommandInfo) -> &'a ::protobuf::Message {
        m.get_environment() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct CommandInfo_shell_acc_type;
static CommandInfo_shell_acc: CommandInfo_shell_acc_type = CommandInfo_shell_acc_type;

impl ::protobuf::reflect::FieldAccessor<CommandInfo> for CommandInfo_shell_acc_type {
    fn name(&self) -> &'static str {
        "shell"
    }

    fn has_field(&self, m: &CommandInfo) -> bool {
        m.has_shell()
    }

    fn get_bool(&self, m: &CommandInfo) -> bool {
        m.get_shell()
    }
}

#[allow(non_camel_case_types)]
struct CommandInfo_value_acc_type;
static CommandInfo_value_acc: CommandInfo_value_acc_type = CommandInfo_value_acc_type;

impl ::protobuf::reflect::FieldAccessor<CommandInfo> for CommandInfo_value_acc_type {
    fn name(&self) -> &'static str {
        "value"
    }

    fn has_field(&self, m: &CommandInfo) -> bool {
        m.has_value()
    }

    fn get_str<'a>(&self, m: &'a CommandInfo) -> &'a str {
        m.get_value()
    }
}

#[allow(non_camel_case_types)]
struct CommandInfo_arguments_acc_type;
static CommandInfo_arguments_acc: CommandInfo_arguments_acc_type = CommandInfo_arguments_acc_type;

impl ::protobuf::reflect::FieldAccessor<CommandInfo> for CommandInfo_arguments_acc_type {
    fn name(&self) -> &'static str {
        "arguments"
    }

    fn len_field(&self, m: &CommandInfo) -> uint {
        m.get_arguments().len()
    }

    fn get_rep_str<'a>(&self, m: &'a CommandInfo) -> &'a [::std::string::String] {
        m.get_arguments()
    }
}

#[allow(non_camel_case_types)]
struct CommandInfo_user_acc_type;
static CommandInfo_user_acc: CommandInfo_user_acc_type = CommandInfo_user_acc_type;

impl ::protobuf::reflect::FieldAccessor<CommandInfo> for CommandInfo_user_acc_type {
    fn name(&self) -> &'static str {
        "user"
    }

    fn has_field(&self, m: &CommandInfo) -> bool {
        m.has_user()
    }

    fn get_str<'a>(&self, m: &'a CommandInfo) -> &'a str {
        m.get_user()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct CommandInfo_URI {
    value: ::protobuf::SingularField<::std::string::String>,
    executable: ::std::option::Option<bool>,
    extract: ::std::option::Option<bool>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> CommandInfo_URI {
    pub fn new() -> CommandInfo_URI {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CommandInfo_URI {
        static mut instance: ::protobuf::lazy::Lazy<CommandInfo_URI> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CommandInfo_URI,
        };
        unsafe {
            instance.get(|| {
                CommandInfo_URI {
                    value: ::protobuf::SingularField::none(),
                    executable: ::std::option::None,
                    extract: ::std::option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // required string value = 1;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&'a mut self) -> &'a mut ::std::string::String {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    pub fn get_value(&'a self) -> &'a str {
        match self.value.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // optional bool executable = 2;

    pub fn clear_executable(&mut self) {
        self.executable = None;
    }

    pub fn has_executable(&self) -> bool {
        self.executable.is_some()
    }

    // Param is passed by value, moved
    pub fn set_executable(&mut self, v: bool) {
        self.executable = Some(v);
    }

    pub fn get_executable(&self) -> bool {
        self.executable.unwrap_or(false)
    }

    // optional bool extract = 3;

    pub fn clear_extract(&mut self) {
        self.extract = None;
    }

    pub fn has_extract(&self) -> bool {
        self.extract.is_some()
    }

    // Param is passed by value, moved
    pub fn set_extract(&mut self, v: bool) {
        self.extract = Some(v);
    }

    pub fn get_extract(&self) -> bool {
        self.extract.unwrap_or(true)
    }
}

impl ::protobuf::Message for CommandInfo_URI {
    fn new() -> CommandInfo_URI {
        CommandInfo_URI::new()
    }

    fn is_initialized(&self) -> bool {
        if self.value.is_none() {
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
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.value.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.executable = Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.extract = Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.value.iter() {
            my_size += ::protobuf::rt::string_size(1, value.as_slice());
        };
        if self.executable.is_some() {
            my_size += 2;
        };
        if self.extract.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.value.as_ref() {
            Some(v) => {
                try!(os.write_string(1, v.as_slice()));
            },
            None => {},
        };
        match self.executable {
            Some(v) => {
                try!(os.write_bool(2, v));
            },
            None => {},
        };
        match self.extract {
            Some(v) => {
                try!(os.write_bool(3, v));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CommandInfo_URI>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<CommandInfo_URI>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&CommandInfo_URI_value_acc as &'static ::protobuf::reflect::FieldAccessor<CommandInfo_URI>) });
                fields.push(unsafe { ::std::mem::transmute(&CommandInfo_URI_executable_acc as &'static ::protobuf::reflect::FieldAccessor<CommandInfo_URI>) });
                fields.push(unsafe { ::std::mem::transmute(&CommandInfo_URI_extract_acc as &'static ::protobuf::reflect::FieldAccessor<CommandInfo_URI>) });
                ::protobuf::reflect::MessageDescriptor::new::<CommandInfo_URI>(
                    "CommandInfo_URI",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<CommandInfo_URI>()
    }
}

impl ::protobuf::Clear for CommandInfo_URI {
    fn clear(&mut self) {
        self.clear_value();
        self.clear_executable();
        self.clear_extract();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for CommandInfo_URI {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct CommandInfo_URI_value_acc_type;
static CommandInfo_URI_value_acc: CommandInfo_URI_value_acc_type = CommandInfo_URI_value_acc_type;

impl ::protobuf::reflect::FieldAccessor<CommandInfo_URI> for CommandInfo_URI_value_acc_type {
    fn name(&self) -> &'static str {
        "value"
    }

    fn has_field(&self, m: &CommandInfo_URI) -> bool {
        m.has_value()
    }

    fn get_str<'a>(&self, m: &'a CommandInfo_URI) -> &'a str {
        m.get_value()
    }
}

#[allow(non_camel_case_types)]
struct CommandInfo_URI_executable_acc_type;
static CommandInfo_URI_executable_acc: CommandInfo_URI_executable_acc_type = CommandInfo_URI_executable_acc_type;

impl ::protobuf::reflect::FieldAccessor<CommandInfo_URI> for CommandInfo_URI_executable_acc_type {
    fn name(&self) -> &'static str {
        "executable"
    }

    fn has_field(&self, m: &CommandInfo_URI) -> bool {
        m.has_executable()
    }

    fn get_bool(&self, m: &CommandInfo_URI) -> bool {
        m.get_executable()
    }
}

#[allow(non_camel_case_types)]
struct CommandInfo_URI_extract_acc_type;
static CommandInfo_URI_extract_acc: CommandInfo_URI_extract_acc_type = CommandInfo_URI_extract_acc_type;

impl ::protobuf::reflect::FieldAccessor<CommandInfo_URI> for CommandInfo_URI_extract_acc_type {
    fn name(&self) -> &'static str {
        "extract"
    }

    fn has_field(&self, m: &CommandInfo_URI) -> bool {
        m.has_extract()
    }

    fn get_bool(&self, m: &CommandInfo_URI) -> bool {
        m.get_extract()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct CommandInfo_ContainerInfo {
    image: ::protobuf::SingularField<::std::string::String>,
    options: ::protobuf::RepeatedField<::std::string::String>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> CommandInfo_ContainerInfo {
    pub fn new() -> CommandInfo_ContainerInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CommandInfo_ContainerInfo {
        static mut instance: ::protobuf::lazy::Lazy<CommandInfo_ContainerInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CommandInfo_ContainerInfo,
        };
        unsafe {
            instance.get(|| {
                CommandInfo_ContainerInfo {
                    image: ::protobuf::SingularField::none(),
                    options: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // required string image = 1;

    pub fn clear_image(&mut self) {
        self.image.clear();
    }

    pub fn has_image(&self) -> bool {
        self.image.is_some()
    }

    // Param is passed by value, moved
    pub fn set_image(&mut self, v: ::std::string::String) {
        self.image = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_image(&'a mut self) -> &'a mut ::std::string::String {
        if self.image.is_none() {
            self.image.set_default();
        };
        self.image.as_mut().unwrap()
    }

    pub fn get_image(&'a self) -> &'a str {
        match self.image.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // repeated string options = 2;

    pub fn clear_options(&mut self) {
        self.options.clear();
    }

    // Param is passed by value, moved
    pub fn set_options(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.options = v;
    }

    // Mutable pointer to the field.
    pub fn mut_options(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.options
    }

    pub fn get_options(&'a self) -> &'a [::std::string::String] {
        self.options.as_slice()
    }
}

impl ::protobuf::Message for CommandInfo_ContainerInfo {
    fn new() -> CommandInfo_ContainerInfo {
        CommandInfo_ContainerInfo::new()
    }

    fn is_initialized(&self) -> bool {
        if self.image.is_none() {
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
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.image.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.options.push_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.image.iter() {
            my_size += ::protobuf::rt::string_size(1, value.as_slice());
        };
        for value in self.options.iter() {
            my_size += ::protobuf::rt::string_size(2, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.image.as_ref() {
            Some(v) => {
                try!(os.write_string(1, v.as_slice()));
            },
            None => {},
        };
        for v in self.options.iter() {
            try!(os.write_string(2, v.as_slice()));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<CommandInfo_ContainerInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<CommandInfo_ContainerInfo>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&CommandInfo_ContainerInfo_image_acc as &'static ::protobuf::reflect::FieldAccessor<CommandInfo_ContainerInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&CommandInfo_ContainerInfo_options_acc as &'static ::protobuf::reflect::FieldAccessor<CommandInfo_ContainerInfo>) });
                ::protobuf::reflect::MessageDescriptor::new::<CommandInfo_ContainerInfo>(
                    "CommandInfo_ContainerInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<CommandInfo_ContainerInfo>()
    }
}

impl ::protobuf::Clear for CommandInfo_ContainerInfo {
    fn clear(&mut self) {
        self.clear_image();
        self.clear_options();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for CommandInfo_ContainerInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct CommandInfo_ContainerInfo_image_acc_type;
static CommandInfo_ContainerInfo_image_acc: CommandInfo_ContainerInfo_image_acc_type = CommandInfo_ContainerInfo_image_acc_type;

impl ::protobuf::reflect::FieldAccessor<CommandInfo_ContainerInfo> for CommandInfo_ContainerInfo_image_acc_type {
    fn name(&self) -> &'static str {
        "image"
    }

    fn has_field(&self, m: &CommandInfo_ContainerInfo) -> bool {
        m.has_image()
    }

    fn get_str<'a>(&self, m: &'a CommandInfo_ContainerInfo) -> &'a str {
        m.get_image()
    }
}

#[allow(non_camel_case_types)]
struct CommandInfo_ContainerInfo_options_acc_type;
static CommandInfo_ContainerInfo_options_acc: CommandInfo_ContainerInfo_options_acc_type = CommandInfo_ContainerInfo_options_acc_type;

impl ::protobuf::reflect::FieldAccessor<CommandInfo_ContainerInfo> for CommandInfo_ContainerInfo_options_acc_type {
    fn name(&self) -> &'static str {
        "options"
    }

    fn len_field(&self, m: &CommandInfo_ContainerInfo) -> uint {
        m.get_options().len()
    }

    fn get_rep_str<'a>(&self, m: &'a CommandInfo_ContainerInfo) -> &'a [::std::string::String] {
        m.get_options()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct ExecutorInfo {
    executor_id: ::protobuf::SingularPtrField<ExecutorID>,
    framework_id: ::protobuf::SingularPtrField<FrameworkID>,
    command: ::protobuf::SingularPtrField<CommandInfo>,
    container: ::protobuf::SingularPtrField<ContainerInfo>,
    resources: ::protobuf::RepeatedField<Resource>,
    name: ::protobuf::SingularField<::std::string::String>,
    source: ::protobuf::SingularField<::std::string::String>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> ExecutorInfo {
    pub fn new() -> ExecutorInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExecutorInfo {
        static mut instance: ::protobuf::lazy::Lazy<ExecutorInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExecutorInfo,
        };
        unsafe {
            instance.get(|| {
                ExecutorInfo {
                    executor_id: ::protobuf::SingularPtrField::none(),
                    framework_id: ::protobuf::SingularPtrField::none(),
                    command: ::protobuf::SingularPtrField::none(),
                    container: ::protobuf::SingularPtrField::none(),
                    resources: ::protobuf::RepeatedField::new(),
                    name: ::protobuf::SingularField::none(),
                    source: ::protobuf::SingularField::none(),
                    data: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
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
    pub fn mut_executor_id(&'a mut self) -> &'a mut ExecutorID {
        if self.executor_id.is_none() {
            self.executor_id.set_default();
        };
        self.executor_id.as_mut().unwrap()
    }

    pub fn get_executor_id(&'a self) -> &'a ExecutorID {
        self.executor_id.as_ref().unwrap_or_else(|| ExecutorID::default_instance())
    }

    // optional .mesos.FrameworkID framework_id = 8;

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
    pub fn mut_framework_id(&'a mut self) -> &'a mut FrameworkID {
        if self.framework_id.is_none() {
            self.framework_id.set_default();
        };
        self.framework_id.as_mut().unwrap()
    }

    pub fn get_framework_id(&'a self) -> &'a FrameworkID {
        self.framework_id.as_ref().unwrap_or_else(|| FrameworkID::default_instance())
    }

    // required .mesos.CommandInfo command = 7;

    pub fn clear_command(&mut self) {
        self.command.clear();
    }

    pub fn has_command(&self) -> bool {
        self.command.is_some()
    }

    // Param is passed by value, moved
    pub fn set_command(&mut self, v: CommandInfo) {
        self.command = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_command(&'a mut self) -> &'a mut CommandInfo {
        if self.command.is_none() {
            self.command.set_default();
        };
        self.command.as_mut().unwrap()
    }

    pub fn get_command(&'a self) -> &'a CommandInfo {
        self.command.as_ref().unwrap_or_else(|| CommandInfo::default_instance())
    }

    // optional .mesos.ContainerInfo container = 11;

    pub fn clear_container(&mut self) {
        self.container.clear();
    }

    pub fn has_container(&self) -> bool {
        self.container.is_some()
    }

    // Param is passed by value, moved
    pub fn set_container(&mut self, v: ContainerInfo) {
        self.container = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_container(&'a mut self) -> &'a mut ContainerInfo {
        if self.container.is_none() {
            self.container.set_default();
        };
        self.container.as_mut().unwrap()
    }

    pub fn get_container(&'a self) -> &'a ContainerInfo {
        self.container.as_ref().unwrap_or_else(|| ContainerInfo::default_instance())
    }

    // repeated .mesos.Resource resources = 5;

    pub fn clear_resources(&mut self) {
        self.resources.clear();
    }

    // Param is passed by value, moved
    pub fn set_resources(&mut self, v: ::protobuf::RepeatedField<Resource>) {
        self.resources = v;
    }

    // Mutable pointer to the field.
    pub fn mut_resources(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Resource> {
        &mut self.resources
    }

    pub fn get_resources(&'a self) -> &'a [Resource] {
        self.resources.as_slice()
    }

    // optional string name = 9;

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
    pub fn mut_name(&'a mut self) -> &'a mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    pub fn get_name(&'a self) -> &'a str {
        match self.name.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // optional string source = 10;

    pub fn clear_source(&mut self) {
        self.source.clear();
    }

    pub fn has_source(&self) -> bool {
        self.source.is_some()
    }

    // Param is passed by value, moved
    pub fn set_source(&mut self, v: ::std::string::String) {
        self.source = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_source(&'a mut self) -> &'a mut ::std::string::String {
        if self.source.is_none() {
            self.source.set_default();
        };
        self.source.as_mut().unwrap()
    }

    pub fn get_source(&'a self) -> &'a str {
        match self.source.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // optional bytes data = 4;

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
    pub fn mut_data(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        };
        self.data.as_mut().unwrap()
    }

    pub fn get_data(&'a self) -> &'a [u8] {
        match self.data.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }
}

impl ::protobuf::Message for ExecutorInfo {
    fn new() -> ExecutorInfo {
        ExecutorInfo::new()
    }

    fn is_initialized(&self) -> bool {
        if self.executor_id.is_none() {
            return false;
        };
        if self.command.is_none() {
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
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.executor_id.set_default();
                    try!(is.merge_message(tmp))
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.framework_id.set_default();
                    try!(is.merge_message(tmp))
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.command.set_default();
                    try!(is.merge_message(tmp))
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.container.set_default();
                    try!(is.merge_message(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.resources.push_default();
                    try!(is.merge_message(tmp))
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.name.set_default();
                    try!(is.read_string_into(tmp))
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.source.set_default();
                    try!(is.read_string_into(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
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
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.executor_id.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.framework_id.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.command.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.container.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.resources.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(9, value.as_slice());
        };
        for value in self.source.iter() {
            my_size += ::protobuf::rt::string_size(10, value.as_slice());
        };
        for value in self.data.iter() {
            my_size += ::protobuf::rt::bytes_size(4, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.executor_id.as_ref() {
            Some(v) => {
                try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.framework_id.as_ref() {
            Some(v) => {
                try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.command.as_ref() {
            Some(v) => {
                try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.container.as_ref() {
            Some(v) => {
                try!(os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        for v in self.resources.iter() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(sizes[*sizes_pos]));
            *sizes_pos += 1;
            try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
        };
        match self.name.as_ref() {
            Some(v) => {
                try!(os.write_string(9, v.as_slice()));
            },
            None => {},
        };
        match self.source.as_ref() {
            Some(v) => {
                try!(os.write_string(10, v.as_slice()));
            },
            None => {},
        };
        match self.data.as_ref() {
            Some(v) => {
                try!(os.write_bytes(4, v.as_slice()));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<ExecutorInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<ExecutorInfo>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&ExecutorInfo_executor_id_acc as &'static ::protobuf::reflect::FieldAccessor<ExecutorInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&ExecutorInfo_framework_id_acc as &'static ::protobuf::reflect::FieldAccessor<ExecutorInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&ExecutorInfo_command_acc as &'static ::protobuf::reflect::FieldAccessor<ExecutorInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&ExecutorInfo_container_acc as &'static ::protobuf::reflect::FieldAccessor<ExecutorInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&ExecutorInfo_resources_acc as &'static ::protobuf::reflect::FieldAccessor<ExecutorInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&ExecutorInfo_name_acc as &'static ::protobuf::reflect::FieldAccessor<ExecutorInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&ExecutorInfo_source_acc as &'static ::protobuf::reflect::FieldAccessor<ExecutorInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&ExecutorInfo_data_acc as &'static ::protobuf::reflect::FieldAccessor<ExecutorInfo>) });
                ::protobuf::reflect::MessageDescriptor::new::<ExecutorInfo>(
                    "ExecutorInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<ExecutorInfo>()
    }
}

impl ::protobuf::Clear for ExecutorInfo {
    fn clear(&mut self) {
        self.clear_executor_id();
        self.clear_framework_id();
        self.clear_command();
        self.clear_container();
        self.clear_resources();
        self.clear_name();
        self.clear_source();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for ExecutorInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct ExecutorInfo_executor_id_acc_type;
static ExecutorInfo_executor_id_acc: ExecutorInfo_executor_id_acc_type = ExecutorInfo_executor_id_acc_type;

impl ::protobuf::reflect::FieldAccessor<ExecutorInfo> for ExecutorInfo_executor_id_acc_type {
    fn name(&self) -> &'static str {
        "executor_id"
    }

    fn has_field(&self, m: &ExecutorInfo) -> bool {
        m.has_executor_id()
    }

    fn get_message<'a>(&self, m: &'a ExecutorInfo) -> &'a ::protobuf::Message {
        m.get_executor_id() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct ExecutorInfo_framework_id_acc_type;
static ExecutorInfo_framework_id_acc: ExecutorInfo_framework_id_acc_type = ExecutorInfo_framework_id_acc_type;

impl ::protobuf::reflect::FieldAccessor<ExecutorInfo> for ExecutorInfo_framework_id_acc_type {
    fn name(&self) -> &'static str {
        "framework_id"
    }

    fn has_field(&self, m: &ExecutorInfo) -> bool {
        m.has_framework_id()
    }

    fn get_message<'a>(&self, m: &'a ExecutorInfo) -> &'a ::protobuf::Message {
        m.get_framework_id() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct ExecutorInfo_command_acc_type;
static ExecutorInfo_command_acc: ExecutorInfo_command_acc_type = ExecutorInfo_command_acc_type;

impl ::protobuf::reflect::FieldAccessor<ExecutorInfo> for ExecutorInfo_command_acc_type {
    fn name(&self) -> &'static str {
        "command"
    }

    fn has_field(&self, m: &ExecutorInfo) -> bool {
        m.has_command()
    }

    fn get_message<'a>(&self, m: &'a ExecutorInfo) -> &'a ::protobuf::Message {
        m.get_command() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct ExecutorInfo_container_acc_type;
static ExecutorInfo_container_acc: ExecutorInfo_container_acc_type = ExecutorInfo_container_acc_type;

impl ::protobuf::reflect::FieldAccessor<ExecutorInfo> for ExecutorInfo_container_acc_type {
    fn name(&self) -> &'static str {
        "container"
    }

    fn has_field(&self, m: &ExecutorInfo) -> bool {
        m.has_container()
    }

    fn get_message<'a>(&self, m: &'a ExecutorInfo) -> &'a ::protobuf::Message {
        m.get_container() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct ExecutorInfo_resources_acc_type;
static ExecutorInfo_resources_acc: ExecutorInfo_resources_acc_type = ExecutorInfo_resources_acc_type;

impl ::protobuf::reflect::FieldAccessor<ExecutorInfo> for ExecutorInfo_resources_acc_type {
    fn name(&self) -> &'static str {
        "resources"
    }

    fn len_field(&self, m: &ExecutorInfo) -> uint {
        m.get_resources().len()
    }

    fn get_rep_message_item<'a>(&self, m: &'a ExecutorInfo, index: uint) -> &'a ::protobuf::Message {
        &m.get_resources()[index] as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct ExecutorInfo_name_acc_type;
static ExecutorInfo_name_acc: ExecutorInfo_name_acc_type = ExecutorInfo_name_acc_type;

impl ::protobuf::reflect::FieldAccessor<ExecutorInfo> for ExecutorInfo_name_acc_type {
    fn name(&self) -> &'static str {
        "name"
    }

    fn has_field(&self, m: &ExecutorInfo) -> bool {
        m.has_name()
    }

    fn get_str<'a>(&self, m: &'a ExecutorInfo) -> &'a str {
        m.get_name()
    }
}

#[allow(non_camel_case_types)]
struct ExecutorInfo_source_acc_type;
static ExecutorInfo_source_acc: ExecutorInfo_source_acc_type = ExecutorInfo_source_acc_type;

impl ::protobuf::reflect::FieldAccessor<ExecutorInfo> for ExecutorInfo_source_acc_type {
    fn name(&self) -> &'static str {
        "source"
    }

    fn has_field(&self, m: &ExecutorInfo) -> bool {
        m.has_source()
    }

    fn get_str<'a>(&self, m: &'a ExecutorInfo) -> &'a str {
        m.get_source()
    }
}

#[allow(non_camel_case_types)]
struct ExecutorInfo_data_acc_type;
static ExecutorInfo_data_acc: ExecutorInfo_data_acc_type = ExecutorInfo_data_acc_type;

impl ::protobuf::reflect::FieldAccessor<ExecutorInfo> for ExecutorInfo_data_acc_type {
    fn name(&self) -> &'static str {
        "data"
    }

    fn has_field(&self, m: &ExecutorInfo) -> bool {
        m.has_data()
    }

    fn get_bytes<'a>(&self, m: &'a ExecutorInfo) -> &'a [u8] {
        m.get_data()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct MasterInfo {
    id: ::protobuf::SingularField<::std::string::String>,
    ip: ::std::option::Option<u32>,
    port: ::std::option::Option<u32>,
    pid: ::protobuf::SingularField<::std::string::String>,
    hostname: ::protobuf::SingularField<::std::string::String>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> MasterInfo {
    pub fn new() -> MasterInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MasterInfo {
        static mut instance: ::protobuf::lazy::Lazy<MasterInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MasterInfo,
        };
        unsafe {
            instance.get(|| {
                MasterInfo {
                    id: ::protobuf::SingularField::none(),
                    ip: ::std::option::None,
                    port: ::std::option::None,
                    pid: ::protobuf::SingularField::none(),
                    hostname: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // required string id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&'a mut self) -> &'a mut ::std::string::String {
        if self.id.is_none() {
            self.id.set_default();
        };
        self.id.as_mut().unwrap()
    }

    pub fn get_id(&'a self) -> &'a str {
        match self.id.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // required uint32 ip = 2;

    pub fn clear_ip(&mut self) {
        self.ip = None;
    }

    pub fn has_ip(&self) -> bool {
        self.ip.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ip(&mut self, v: u32) {
        self.ip = Some(v);
    }

    pub fn get_ip(&self) -> u32 {
        self.ip.unwrap_or(0)
    }

    // required uint32 port = 3;

    pub fn clear_port(&mut self) {
        self.port = None;
    }

    pub fn has_port(&self) -> bool {
        self.port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_port(&mut self, v: u32) {
        self.port = Some(v);
    }

    pub fn get_port(&self) -> u32 {
        self.port.unwrap_or(5050u32)
    }

    // optional string pid = 4;

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
    pub fn mut_pid(&'a mut self) -> &'a mut ::std::string::String {
        if self.pid.is_none() {
            self.pid.set_default();
        };
        self.pid.as_mut().unwrap()
    }

    pub fn get_pid(&'a self) -> &'a str {
        match self.pid.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // optional string hostname = 5;

    pub fn clear_hostname(&mut self) {
        self.hostname.clear();
    }

    pub fn has_hostname(&self) -> bool {
        self.hostname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hostname(&mut self, v: ::std::string::String) {
        self.hostname = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hostname(&'a mut self) -> &'a mut ::std::string::String {
        if self.hostname.is_none() {
            self.hostname.set_default();
        };
        self.hostname.as_mut().unwrap()
    }

    pub fn get_hostname(&'a self) -> &'a str {
        match self.hostname.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }
}

impl ::protobuf::Message for MasterInfo {
    fn new() -> MasterInfo {
        MasterInfo::new()
    }

    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        };
        if self.ip.is_none() {
            return false;
        };
        if self.port.is_none() {
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
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.id.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.ip = Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.port = Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.pid.set_default();
                    try!(is.read_string_into(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.hostname.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.id.iter() {
            my_size += ::protobuf::rt::string_size(1, value.as_slice());
        };
        for value in self.ip.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.port.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.pid.iter() {
            my_size += ::protobuf::rt::string_size(4, value.as_slice());
        };
        for value in self.hostname.iter() {
            my_size += ::protobuf::rt::string_size(5, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.id.as_ref() {
            Some(v) => {
                try!(os.write_string(1, v.as_slice()));
            },
            None => {},
        };
        match self.ip {
            Some(v) => {
                try!(os.write_uint32(2, v));
            },
            None => {},
        };
        match self.port {
            Some(v) => {
                try!(os.write_uint32(3, v));
            },
            None => {},
        };
        match self.pid.as_ref() {
            Some(v) => {
                try!(os.write_string(4, v.as_slice()));
            },
            None => {},
        };
        match self.hostname.as_ref() {
            Some(v) => {
                try!(os.write_string(5, v.as_slice()));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<MasterInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<MasterInfo>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&MasterInfo_id_acc as &'static ::protobuf::reflect::FieldAccessor<MasterInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&MasterInfo_ip_acc as &'static ::protobuf::reflect::FieldAccessor<MasterInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&MasterInfo_port_acc as &'static ::protobuf::reflect::FieldAccessor<MasterInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&MasterInfo_pid_acc as &'static ::protobuf::reflect::FieldAccessor<MasterInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&MasterInfo_hostname_acc as &'static ::protobuf::reflect::FieldAccessor<MasterInfo>) });
                ::protobuf::reflect::MessageDescriptor::new::<MasterInfo>(
                    "MasterInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<MasterInfo>()
    }
}

impl ::protobuf::Clear for MasterInfo {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_ip();
        self.clear_port();
        self.clear_pid();
        self.clear_hostname();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for MasterInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct MasterInfo_id_acc_type;
static MasterInfo_id_acc: MasterInfo_id_acc_type = MasterInfo_id_acc_type;

impl ::protobuf::reflect::FieldAccessor<MasterInfo> for MasterInfo_id_acc_type {
    fn name(&self) -> &'static str {
        "id"
    }

    fn has_field(&self, m: &MasterInfo) -> bool {
        m.has_id()
    }

    fn get_str<'a>(&self, m: &'a MasterInfo) -> &'a str {
        m.get_id()
    }
}

#[allow(non_camel_case_types)]
struct MasterInfo_ip_acc_type;
static MasterInfo_ip_acc: MasterInfo_ip_acc_type = MasterInfo_ip_acc_type;

impl ::protobuf::reflect::FieldAccessor<MasterInfo> for MasterInfo_ip_acc_type {
    fn name(&self) -> &'static str {
        "ip"
    }

    fn has_field(&self, m: &MasterInfo) -> bool {
        m.has_ip()
    }

    fn get_u32(&self, m: &MasterInfo) -> u32 {
        m.get_ip()
    }
}

#[allow(non_camel_case_types)]
struct MasterInfo_port_acc_type;
static MasterInfo_port_acc: MasterInfo_port_acc_type = MasterInfo_port_acc_type;

impl ::protobuf::reflect::FieldAccessor<MasterInfo> for MasterInfo_port_acc_type {
    fn name(&self) -> &'static str {
        "port"
    }

    fn has_field(&self, m: &MasterInfo) -> bool {
        m.has_port()
    }

    fn get_u32(&self, m: &MasterInfo) -> u32 {
        m.get_port()
    }
}

#[allow(non_camel_case_types)]
struct MasterInfo_pid_acc_type;
static MasterInfo_pid_acc: MasterInfo_pid_acc_type = MasterInfo_pid_acc_type;

impl ::protobuf::reflect::FieldAccessor<MasterInfo> for MasterInfo_pid_acc_type {
    fn name(&self) -> &'static str {
        "pid"
    }

    fn has_field(&self, m: &MasterInfo) -> bool {
        m.has_pid()
    }

    fn get_str<'a>(&self, m: &'a MasterInfo) -> &'a str {
        m.get_pid()
    }
}

#[allow(non_camel_case_types)]
struct MasterInfo_hostname_acc_type;
static MasterInfo_hostname_acc: MasterInfo_hostname_acc_type = MasterInfo_hostname_acc_type;

impl ::protobuf::reflect::FieldAccessor<MasterInfo> for MasterInfo_hostname_acc_type {
    fn name(&self) -> &'static str {
        "hostname"
    }

    fn has_field(&self, m: &MasterInfo) -> bool {
        m.has_hostname()
    }

    fn get_str<'a>(&self, m: &'a MasterInfo) -> &'a str {
        m.get_hostname()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct SlaveInfo {
    hostname: ::protobuf::SingularField<::std::string::String>,
    port: ::std::option::Option<i32>,
    resources: ::protobuf::RepeatedField<Resource>,
    attributes: ::protobuf::RepeatedField<Attribute>,
    id: ::protobuf::SingularPtrField<SlaveID>,
    checkpoint: ::std::option::Option<bool>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> SlaveInfo {
    pub fn new() -> SlaveInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SlaveInfo {
        static mut instance: ::protobuf::lazy::Lazy<SlaveInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SlaveInfo,
        };
        unsafe {
            instance.get(|| {
                SlaveInfo {
                    hostname: ::protobuf::SingularField::none(),
                    port: ::std::option::None,
                    resources: ::protobuf::RepeatedField::new(),
                    attributes: ::protobuf::RepeatedField::new(),
                    id: ::protobuf::SingularPtrField::none(),
                    checkpoint: ::std::option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // required string hostname = 1;

    pub fn clear_hostname(&mut self) {
        self.hostname.clear();
    }

    pub fn has_hostname(&self) -> bool {
        self.hostname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hostname(&mut self, v: ::std::string::String) {
        self.hostname = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hostname(&'a mut self) -> &'a mut ::std::string::String {
        if self.hostname.is_none() {
            self.hostname.set_default();
        };
        self.hostname.as_mut().unwrap()
    }

    pub fn get_hostname(&'a self) -> &'a str {
        match self.hostname.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // optional int32 port = 8;

    pub fn clear_port(&mut self) {
        self.port = None;
    }

    pub fn has_port(&self) -> bool {
        self.port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_port(&mut self, v: i32) {
        self.port = Some(v);
    }

    pub fn get_port(&self) -> i32 {
        self.port.unwrap_or(5051i32)
    }

    // repeated .mesos.Resource resources = 3;

    pub fn clear_resources(&mut self) {
        self.resources.clear();
    }

    // Param is passed by value, moved
    pub fn set_resources(&mut self, v: ::protobuf::RepeatedField<Resource>) {
        self.resources = v;
    }

    // Mutable pointer to the field.
    pub fn mut_resources(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Resource> {
        &mut self.resources
    }

    pub fn get_resources(&'a self) -> &'a [Resource] {
        self.resources.as_slice()
    }

    // repeated .mesos.Attribute attributes = 5;

    pub fn clear_attributes(&mut self) {
        self.attributes.clear();
    }

    // Param is passed by value, moved
    pub fn set_attributes(&mut self, v: ::protobuf::RepeatedField<Attribute>) {
        self.attributes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_attributes(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Attribute> {
        &mut self.attributes
    }

    pub fn get_attributes(&'a self) -> &'a [Attribute] {
        self.attributes.as_slice()
    }

    // optional .mesos.SlaveID id = 6;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: SlaveID) {
        self.id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&'a mut self) -> &'a mut SlaveID {
        if self.id.is_none() {
            self.id.set_default();
        };
        self.id.as_mut().unwrap()
    }

    pub fn get_id(&'a self) -> &'a SlaveID {
        self.id.as_ref().unwrap_or_else(|| SlaveID::default_instance())
    }

    // optional bool checkpoint = 7;

    pub fn clear_checkpoint(&mut self) {
        self.checkpoint = None;
    }

    pub fn has_checkpoint(&self) -> bool {
        self.checkpoint.is_some()
    }

    // Param is passed by value, moved
    pub fn set_checkpoint(&mut self, v: bool) {
        self.checkpoint = Some(v);
    }

    pub fn get_checkpoint(&self) -> bool {
        self.checkpoint.unwrap_or(false)
    }
}

impl ::protobuf::Message for SlaveInfo {
    fn new() -> SlaveInfo {
        SlaveInfo::new()
    }

    fn is_initialized(&self) -> bool {
        if self.hostname.is_none() {
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
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.hostname.set_default();
                    try!(is.read_string_into(tmp))
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.port = Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.resources.push_default();
                    try!(is.merge_message(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.attributes.push_default();
                    try!(is.merge_message(tmp))
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.id.set_default();
                    try!(is.merge_message(tmp))
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.checkpoint = Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.hostname.iter() {
            my_size += ::protobuf::rt::string_size(1, value.as_slice());
        };
        for value in self.port.iter() {
            my_size += ::protobuf::rt::value_size(8, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.resources.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.attributes.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.id.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.checkpoint.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.hostname.as_ref() {
            Some(v) => {
                try!(os.write_string(1, v.as_slice()));
            },
            None => {},
        };
        match self.port {
            Some(v) => {
                try!(os.write_int32(8, v));
            },
            None => {},
        };
        for v in self.resources.iter() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(sizes[*sizes_pos]));
            *sizes_pos += 1;
            try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
        };
        for v in self.attributes.iter() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(sizes[*sizes_pos]));
            *sizes_pos += 1;
            try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
        };
        match self.id.as_ref() {
            Some(v) => {
                try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.checkpoint {
            Some(v) => {
                try!(os.write_bool(7, v));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<SlaveInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<SlaveInfo>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&SlaveInfo_hostname_acc as &'static ::protobuf::reflect::FieldAccessor<SlaveInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&SlaveInfo_port_acc as &'static ::protobuf::reflect::FieldAccessor<SlaveInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&SlaveInfo_resources_acc as &'static ::protobuf::reflect::FieldAccessor<SlaveInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&SlaveInfo_attributes_acc as &'static ::protobuf::reflect::FieldAccessor<SlaveInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&SlaveInfo_id_acc as &'static ::protobuf::reflect::FieldAccessor<SlaveInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&SlaveInfo_checkpoint_acc as &'static ::protobuf::reflect::FieldAccessor<SlaveInfo>) });
                ::protobuf::reflect::MessageDescriptor::new::<SlaveInfo>(
                    "SlaveInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<SlaveInfo>()
    }
}

impl ::protobuf::Clear for SlaveInfo {
    fn clear(&mut self) {
        self.clear_hostname();
        self.clear_port();
        self.clear_resources();
        self.clear_attributes();
        self.clear_id();
        self.clear_checkpoint();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for SlaveInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct SlaveInfo_hostname_acc_type;
static SlaveInfo_hostname_acc: SlaveInfo_hostname_acc_type = SlaveInfo_hostname_acc_type;

impl ::protobuf::reflect::FieldAccessor<SlaveInfo> for SlaveInfo_hostname_acc_type {
    fn name(&self) -> &'static str {
        "hostname"
    }

    fn has_field(&self, m: &SlaveInfo) -> bool {
        m.has_hostname()
    }

    fn get_str<'a>(&self, m: &'a SlaveInfo) -> &'a str {
        m.get_hostname()
    }
}

#[allow(non_camel_case_types)]
struct SlaveInfo_port_acc_type;
static SlaveInfo_port_acc: SlaveInfo_port_acc_type = SlaveInfo_port_acc_type;

impl ::protobuf::reflect::FieldAccessor<SlaveInfo> for SlaveInfo_port_acc_type {
    fn name(&self) -> &'static str {
        "port"
    }

    fn has_field(&self, m: &SlaveInfo) -> bool {
        m.has_port()
    }

    fn get_i32(&self, m: &SlaveInfo) -> i32 {
        m.get_port()
    }
}

#[allow(non_camel_case_types)]
struct SlaveInfo_resources_acc_type;
static SlaveInfo_resources_acc: SlaveInfo_resources_acc_type = SlaveInfo_resources_acc_type;

impl ::protobuf::reflect::FieldAccessor<SlaveInfo> for SlaveInfo_resources_acc_type {
    fn name(&self) -> &'static str {
        "resources"
    }

    fn len_field(&self, m: &SlaveInfo) -> uint {
        m.get_resources().len()
    }

    fn get_rep_message_item<'a>(&self, m: &'a SlaveInfo, index: uint) -> &'a ::protobuf::Message {
        &m.get_resources()[index] as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct SlaveInfo_attributes_acc_type;
static SlaveInfo_attributes_acc: SlaveInfo_attributes_acc_type = SlaveInfo_attributes_acc_type;

impl ::protobuf::reflect::FieldAccessor<SlaveInfo> for SlaveInfo_attributes_acc_type {
    fn name(&self) -> &'static str {
        "attributes"
    }

    fn len_field(&self, m: &SlaveInfo) -> uint {
        m.get_attributes().len()
    }

    fn get_rep_message_item<'a>(&self, m: &'a SlaveInfo, index: uint) -> &'a ::protobuf::Message {
        &m.get_attributes()[index] as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct SlaveInfo_id_acc_type;
static SlaveInfo_id_acc: SlaveInfo_id_acc_type = SlaveInfo_id_acc_type;

impl ::protobuf::reflect::FieldAccessor<SlaveInfo> for SlaveInfo_id_acc_type {
    fn name(&self) -> &'static str {
        "id"
    }

    fn has_field(&self, m: &SlaveInfo) -> bool {
        m.has_id()
    }

    fn get_message<'a>(&self, m: &'a SlaveInfo) -> &'a ::protobuf::Message {
        m.get_id() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct SlaveInfo_checkpoint_acc_type;
static SlaveInfo_checkpoint_acc: SlaveInfo_checkpoint_acc_type = SlaveInfo_checkpoint_acc_type;

impl ::protobuf::reflect::FieldAccessor<SlaveInfo> for SlaveInfo_checkpoint_acc_type {
    fn name(&self) -> &'static str {
        "checkpoint"
    }

    fn has_field(&self, m: &SlaveInfo) -> bool {
        m.has_checkpoint()
    }

    fn get_bool(&self, m: &SlaveInfo) -> bool {
        m.get_checkpoint()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct Value {
    field_type: ::std::option::Option<Value_Type>,
    scalar: ::protobuf::SingularPtrField<Value_Scalar>,
    ranges: ::protobuf::SingularPtrField<Value_Ranges>,
    set: ::protobuf::SingularPtrField<Value_Set>,
    text: ::protobuf::SingularPtrField<Value_Text>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> Value {
    pub fn new() -> Value {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Value {
        static mut instance: ::protobuf::lazy::Lazy<Value> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Value,
        };
        unsafe {
            instance.get(|| {
                Value {
                    field_type: ::std::option::None,
                    scalar: ::protobuf::SingularPtrField::none(),
                    ranges: ::protobuf::SingularPtrField::none(),
                    set: ::protobuf::SingularPtrField::none(),
                    text: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // required .mesos.Value.Type type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Value_Type) {
        self.field_type = Some(v);
    }

    pub fn get_field_type(&self) -> Value_Type {
        self.field_type.unwrap_or(Value_Type::SCALAR)
    }

    // optional .mesos.Value.Scalar scalar = 2;

    pub fn clear_scalar(&mut self) {
        self.scalar.clear();
    }

    pub fn has_scalar(&self) -> bool {
        self.scalar.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scalar(&mut self, v: Value_Scalar) {
        self.scalar = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_scalar(&'a mut self) -> &'a mut Value_Scalar {
        if self.scalar.is_none() {
            self.scalar.set_default();
        };
        self.scalar.as_mut().unwrap()
    }

    pub fn get_scalar(&'a self) -> &'a Value_Scalar {
        self.scalar.as_ref().unwrap_or_else(|| Value_Scalar::default_instance())
    }

    // optional .mesos.Value.Ranges ranges = 3;

    pub fn clear_ranges(&mut self) {
        self.ranges.clear();
    }

    pub fn has_ranges(&self) -> bool {
        self.ranges.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ranges(&mut self, v: Value_Ranges) {
        self.ranges = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ranges(&'a mut self) -> &'a mut Value_Ranges {
        if self.ranges.is_none() {
            self.ranges.set_default();
        };
        self.ranges.as_mut().unwrap()
    }

    pub fn get_ranges(&'a self) -> &'a Value_Ranges {
        self.ranges.as_ref().unwrap_or_else(|| Value_Ranges::default_instance())
    }

    // optional .mesos.Value.Set set = 4;

    pub fn clear_set(&mut self) {
        self.set.clear();
    }

    pub fn has_set(&self) -> bool {
        self.set.is_some()
    }

    // Param is passed by value, moved
    pub fn set_set(&mut self, v: Value_Set) {
        self.set = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_set(&'a mut self) -> &'a mut Value_Set {
        if self.set.is_none() {
            self.set.set_default();
        };
        self.set.as_mut().unwrap()
    }

    pub fn get_set(&'a self) -> &'a Value_Set {
        self.set.as_ref().unwrap_or_else(|| Value_Set::default_instance())
    }

    // optional .mesos.Value.Text text = 5;

    pub fn clear_text(&mut self) {
        self.text.clear();
    }

    pub fn has_text(&self) -> bool {
        self.text.is_some()
    }

    // Param is passed by value, moved
    pub fn set_text(&mut self, v: Value_Text) {
        self.text = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_text(&'a mut self) -> &'a mut Value_Text {
        if self.text.is_none() {
            self.text.set_default();
        };
        self.text.as_mut().unwrap()
    }

    pub fn get_text(&'a self) -> &'a Value_Text {
        self.text.as_ref().unwrap_or_else(|| Value_Text::default_instance())
    }
}

impl ::protobuf::Message for Value {
    fn new() -> Value {
        Value::new()
    }

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
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = Value_Type::new(try!(is.read_int32()));
                    self.field_type = Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.scalar.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.ranges.set_default();
                    try!(is.merge_message(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.set.set_default();
                    try!(is.merge_message(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.text.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.field_type.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.scalar.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.ranges.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.set.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.text.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.field_type {
            Some(v) => {
                try!(os.write_enum(1, v as i32));
            },
            None => {},
        };
        match self.scalar.as_ref() {
            Some(v) => {
                try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.ranges.as_ref() {
            Some(v) => {
                try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.set.as_ref() {
            Some(v) => {
                try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.text.as_ref() {
            Some(v) => {
                try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<Value>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<Value>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&Value_field_type_acc as &'static ::protobuf::reflect::FieldAccessor<Value>) });
                fields.push(unsafe { ::std::mem::transmute(&Value_scalar_acc as &'static ::protobuf::reflect::FieldAccessor<Value>) });
                fields.push(unsafe { ::std::mem::transmute(&Value_ranges_acc as &'static ::protobuf::reflect::FieldAccessor<Value>) });
                fields.push(unsafe { ::std::mem::transmute(&Value_set_acc as &'static ::protobuf::reflect::FieldAccessor<Value>) });
                fields.push(unsafe { ::std::mem::transmute(&Value_text_acc as &'static ::protobuf::reflect::FieldAccessor<Value>) });
                ::protobuf::reflect::MessageDescriptor::new::<Value>(
                    "Value",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<Value>()
    }
}

impl ::protobuf::Clear for Value {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_scalar();
        self.clear_ranges();
        self.clear_set();
        self.clear_text();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for Value {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct Value_field_type_acc_type;
static Value_field_type_acc: Value_field_type_acc_type = Value_field_type_acc_type;

impl ::protobuf::reflect::FieldAccessor<Value> for Value_field_type_acc_type {
    fn name(&self) -> &'static str {
        "field_type"
    }

    fn has_field(&self, m: &Value) -> bool {
        m.has_field_type()
    }

    fn get_enum<'a>(&self, m: &Value) -> &'static ::protobuf::reflect::EnumValueDescriptor {
        use protobuf::{ProtobufEnum};
        m.get_field_type().descriptor()
    }
}

#[allow(non_camel_case_types)]
struct Value_scalar_acc_type;
static Value_scalar_acc: Value_scalar_acc_type = Value_scalar_acc_type;

impl ::protobuf::reflect::FieldAccessor<Value> for Value_scalar_acc_type {
    fn name(&self) -> &'static str {
        "scalar"
    }

    fn has_field(&self, m: &Value) -> bool {
        m.has_scalar()
    }

    fn get_message<'a>(&self, m: &'a Value) -> &'a ::protobuf::Message {
        m.get_scalar() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct Value_ranges_acc_type;
static Value_ranges_acc: Value_ranges_acc_type = Value_ranges_acc_type;

impl ::protobuf::reflect::FieldAccessor<Value> for Value_ranges_acc_type {
    fn name(&self) -> &'static str {
        "ranges"
    }

    fn has_field(&self, m: &Value) -> bool {
        m.has_ranges()
    }

    fn get_message<'a>(&self, m: &'a Value) -> &'a ::protobuf::Message {
        m.get_ranges() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct Value_set_acc_type;
static Value_set_acc: Value_set_acc_type = Value_set_acc_type;

impl ::protobuf::reflect::FieldAccessor<Value> for Value_set_acc_type {
    fn name(&self) -> &'static str {
        "set"
    }

    fn has_field(&self, m: &Value) -> bool {
        m.has_set()
    }

    fn get_message<'a>(&self, m: &'a Value) -> &'a ::protobuf::Message {
        m.get_set() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct Value_text_acc_type;
static Value_text_acc: Value_text_acc_type = Value_text_acc_type;

impl ::protobuf::reflect::FieldAccessor<Value> for Value_text_acc_type {
    fn name(&self) -> &'static str {
        "text"
    }

    fn has_field(&self, m: &Value) -> bool {
        m.has_text()
    }

    fn get_message<'a>(&self, m: &'a Value) -> &'a ::protobuf::Message {
        m.get_text() as &'a ::protobuf::Message
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct Value_Scalar {
    value: ::std::option::Option<f64>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> Value_Scalar {
    pub fn new() -> Value_Scalar {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Value_Scalar {
        static mut instance: ::protobuf::lazy::Lazy<Value_Scalar> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Value_Scalar,
        };
        unsafe {
            instance.get(|| {
                Value_Scalar {
                    value: ::std::option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // required double value = 1;

    pub fn clear_value(&mut self) {
        self.value = None;
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: f64) {
        self.value = Some(v);
    }

    pub fn get_value(&self) -> f64 {
        self.value.unwrap_or(0.)
    }
}

impl ::protobuf::Message for Value_Scalar {
    fn new() -> Value_Scalar {
        Value_Scalar::new()
    }

    fn is_initialized(&self) -> bool {
        if self.value.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.value = Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        if self.value.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.value {
            Some(v) => {
                try!(os.write_double(1, v));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<Value_Scalar>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<Value_Scalar>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&Value_Scalar_value_acc as &'static ::protobuf::reflect::FieldAccessor<Value_Scalar>) });
                ::protobuf::reflect::MessageDescriptor::new::<Value_Scalar>(
                    "Value_Scalar",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<Value_Scalar>()
    }
}

impl ::protobuf::Clear for Value_Scalar {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for Value_Scalar {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct Value_Scalar_value_acc_type;
static Value_Scalar_value_acc: Value_Scalar_value_acc_type = Value_Scalar_value_acc_type;

impl ::protobuf::reflect::FieldAccessor<Value_Scalar> for Value_Scalar_value_acc_type {
    fn name(&self) -> &'static str {
        "value"
    }

    fn has_field(&self, m: &Value_Scalar) -> bool {
        m.has_value()
    }

    fn get_f64(&self, m: &Value_Scalar) -> f64 {
        m.get_value()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct Value_Range {
    begin: ::std::option::Option<u64>,
    end: ::std::option::Option<u64>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> Value_Range {
    pub fn new() -> Value_Range {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Value_Range {
        static mut instance: ::protobuf::lazy::Lazy<Value_Range> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Value_Range,
        };
        unsafe {
            instance.get(|| {
                Value_Range {
                    begin: ::std::option::None,
                    end: ::std::option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // required uint64 begin = 1;

    pub fn clear_begin(&mut self) {
        self.begin = None;
    }

    pub fn has_begin(&self) -> bool {
        self.begin.is_some()
    }

    // Param is passed by value, moved
    pub fn set_begin(&mut self, v: u64) {
        self.begin = Some(v);
    }

    pub fn get_begin(&self) -> u64 {
        self.begin.unwrap_or(0)
    }

    // required uint64 end = 2;

    pub fn clear_end(&mut self) {
        self.end = None;
    }

    pub fn has_end(&self) -> bool {
        self.end.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end(&mut self, v: u64) {
        self.end = Some(v);
    }

    pub fn get_end(&self) -> u64 {
        self.end.unwrap_or(0)
    }
}

impl ::protobuf::Message for Value_Range {
    fn new() -> Value_Range {
        Value_Range::new()
    }

    fn is_initialized(&self) -> bool {
        if self.begin.is_none() {
            return false;
        };
        if self.end.is_none() {
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
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.begin = Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.end = Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.begin.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.end.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.begin {
            Some(v) => {
                try!(os.write_uint64(1, v));
            },
            None => {},
        };
        match self.end {
            Some(v) => {
                try!(os.write_uint64(2, v));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<Value_Range>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<Value_Range>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&Value_Range_begin_acc as &'static ::protobuf::reflect::FieldAccessor<Value_Range>) });
                fields.push(unsafe { ::std::mem::transmute(&Value_Range_end_acc as &'static ::protobuf::reflect::FieldAccessor<Value_Range>) });
                ::protobuf::reflect::MessageDescriptor::new::<Value_Range>(
                    "Value_Range",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<Value_Range>()
    }
}

impl ::protobuf::Clear for Value_Range {
    fn clear(&mut self) {
        self.clear_begin();
        self.clear_end();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for Value_Range {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct Value_Range_begin_acc_type;
static Value_Range_begin_acc: Value_Range_begin_acc_type = Value_Range_begin_acc_type;

impl ::protobuf::reflect::FieldAccessor<Value_Range> for Value_Range_begin_acc_type {
    fn name(&self) -> &'static str {
        "begin"
    }

    fn has_field(&self, m: &Value_Range) -> bool {
        m.has_begin()
    }

    fn get_u64(&self, m: &Value_Range) -> u64 {
        m.get_begin()
    }
}

#[allow(non_camel_case_types)]
struct Value_Range_end_acc_type;
static Value_Range_end_acc: Value_Range_end_acc_type = Value_Range_end_acc_type;

impl ::protobuf::reflect::FieldAccessor<Value_Range> for Value_Range_end_acc_type {
    fn name(&self) -> &'static str {
        "end"
    }

    fn has_field(&self, m: &Value_Range) -> bool {
        m.has_end()
    }

    fn get_u64(&self, m: &Value_Range) -> u64 {
        m.get_end()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct Value_Ranges {
    range: ::protobuf::RepeatedField<Value_Range>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> Value_Ranges {
    pub fn new() -> Value_Ranges {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Value_Ranges {
        static mut instance: ::protobuf::lazy::Lazy<Value_Ranges> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Value_Ranges,
        };
        unsafe {
            instance.get(|| {
                Value_Ranges {
                    range: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // repeated .mesos.Value.Range range = 1;

    pub fn clear_range(&mut self) {
        self.range.clear();
    }

    // Param is passed by value, moved
    pub fn set_range(&mut self, v: ::protobuf::RepeatedField<Value_Range>) {
        self.range = v;
    }

    // Mutable pointer to the field.
    pub fn mut_range(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Value_Range> {
        &mut self.range
    }

    pub fn get_range(&'a self) -> &'a [Value_Range] {
        self.range.as_slice()
    }
}

impl ::protobuf::Message for Value_Ranges {
    fn new() -> Value_Ranges {
        Value_Ranges::new()
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.range.push_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.range.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        for v in self.range.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(sizes[*sizes_pos]));
            *sizes_pos += 1;
            try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<Value_Ranges>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<Value_Ranges>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&Value_Ranges_range_acc as &'static ::protobuf::reflect::FieldAccessor<Value_Ranges>) });
                ::protobuf::reflect::MessageDescriptor::new::<Value_Ranges>(
                    "Value_Ranges",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<Value_Ranges>()
    }
}

impl ::protobuf::Clear for Value_Ranges {
    fn clear(&mut self) {
        self.clear_range();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for Value_Ranges {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct Value_Ranges_range_acc_type;
static Value_Ranges_range_acc: Value_Ranges_range_acc_type = Value_Ranges_range_acc_type;

impl ::protobuf::reflect::FieldAccessor<Value_Ranges> for Value_Ranges_range_acc_type {
    fn name(&self) -> &'static str {
        "range"
    }

    fn len_field(&self, m: &Value_Ranges) -> uint {
        m.get_range().len()
    }

    fn get_rep_message_item<'a>(&self, m: &'a Value_Ranges, index: uint) -> &'a ::protobuf::Message {
        &m.get_range()[index] as &'a ::protobuf::Message
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct Value_Set {
    item: ::protobuf::RepeatedField<::std::string::String>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> Value_Set {
    pub fn new() -> Value_Set {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Value_Set {
        static mut instance: ::protobuf::lazy::Lazy<Value_Set> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Value_Set,
        };
        unsafe {
            instance.get(|| {
                Value_Set {
                    item: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // repeated string item = 1;

    pub fn clear_item(&mut self) {
        self.item.clear();
    }

    // Param is passed by value, moved
    pub fn set_item(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.item = v;
    }

    // Mutable pointer to the field.
    pub fn mut_item(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.item
    }

    pub fn get_item(&'a self) -> &'a [::std::string::String] {
        self.item.as_slice()
    }
}

impl ::protobuf::Message for Value_Set {
    fn new() -> Value_Set {
        Value_Set::new()
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.item.push_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.item.iter() {
            my_size += ::protobuf::rt::string_size(1, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        for v in self.item.iter() {
            try!(os.write_string(1, v.as_slice()));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<Value_Set>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<Value_Set>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&Value_Set_item_acc as &'static ::protobuf::reflect::FieldAccessor<Value_Set>) });
                ::protobuf::reflect::MessageDescriptor::new::<Value_Set>(
                    "Value_Set",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<Value_Set>()
    }
}

impl ::protobuf::Clear for Value_Set {
    fn clear(&mut self) {
        self.clear_item();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for Value_Set {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct Value_Set_item_acc_type;
static Value_Set_item_acc: Value_Set_item_acc_type = Value_Set_item_acc_type;

impl ::protobuf::reflect::FieldAccessor<Value_Set> for Value_Set_item_acc_type {
    fn name(&self) -> &'static str {
        "item"
    }

    fn len_field(&self, m: &Value_Set) -> uint {
        m.get_item().len()
    }

    fn get_rep_str<'a>(&self, m: &'a Value_Set) -> &'a [::std::string::String] {
        m.get_item()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct Value_Text {
    value: ::protobuf::SingularField<::std::string::String>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> Value_Text {
    pub fn new() -> Value_Text {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Value_Text {
        static mut instance: ::protobuf::lazy::Lazy<Value_Text> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Value_Text,
        };
        unsafe {
            instance.get(|| {
                Value_Text {
                    value: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // required string value = 1;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&'a mut self) -> &'a mut ::std::string::String {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    pub fn get_value(&'a self) -> &'a str {
        match self.value.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }
}

impl ::protobuf::Message for Value_Text {
    fn new() -> Value_Text {
        Value_Text::new()
    }

    fn is_initialized(&self) -> bool {
        if self.value.is_none() {
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
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.value.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.value.iter() {
            my_size += ::protobuf::rt::string_size(1, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.value.as_ref() {
            Some(v) => {
                try!(os.write_string(1, v.as_slice()));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<Value_Text>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<Value_Text>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&Value_Text_value_acc as &'static ::protobuf::reflect::FieldAccessor<Value_Text>) });
                ::protobuf::reflect::MessageDescriptor::new::<Value_Text>(
                    "Value_Text",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<Value_Text>()
    }
}

impl ::protobuf::Clear for Value_Text {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for Value_Text {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct Value_Text_value_acc_type;
static Value_Text_value_acc: Value_Text_value_acc_type = Value_Text_value_acc_type;

impl ::protobuf::reflect::FieldAccessor<Value_Text> for Value_Text_value_acc_type {
    fn name(&self) -> &'static str {
        "value"
    }

    fn has_field(&self, m: &Value_Text) -> bool {
        m.has_value()
    }

    fn get_str<'a>(&self, m: &'a Value_Text) -> &'a str {
        m.get_value()
    }
}

#[deriving(Clone,PartialEq,Eq,Show)]
pub enum Value_Type {
    SCALAR = 0,
    RANGES = 1,
    SET = 2,
    TEXT = 3,
}

impl Value_Type {
    pub fn new(value: i32) -> Value_Type {
        match value {
            0 => Value_Type::SCALAR,
            1 => Value_Type::RANGES,
            2 => Value_Type::SET,
            3 => Value_Type::TEXT,
            _ => panic!()
        }
    }
}

impl ::protobuf::ProtobufEnum for Value_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn enum_descriptor_static(_: Option<Value_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Value_Type", file_descriptor_proto())
            })
        }
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct Attribute {
    name: ::protobuf::SingularField<::std::string::String>,
    field_type: ::std::option::Option<Value_Type>,
    scalar: ::protobuf::SingularPtrField<Value_Scalar>,
    ranges: ::protobuf::SingularPtrField<Value_Ranges>,
    set: ::protobuf::SingularPtrField<Value_Set>,
    text: ::protobuf::SingularPtrField<Value_Text>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> Attribute {
    pub fn new() -> Attribute {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Attribute {
        static mut instance: ::protobuf::lazy::Lazy<Attribute> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Attribute,
        };
        unsafe {
            instance.get(|| {
                Attribute {
                    name: ::protobuf::SingularField::none(),
                    field_type: ::std::option::None,
                    scalar: ::protobuf::SingularPtrField::none(),
                    ranges: ::protobuf::SingularPtrField::none(),
                    set: ::protobuf::SingularPtrField::none(),
                    text: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
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
    pub fn mut_name(&'a mut self) -> &'a mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    pub fn get_name(&'a self) -> &'a str {
        match self.name.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // required .mesos.Value.Type type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type = None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Value_Type) {
        self.field_type = Some(v);
    }

    pub fn get_field_type(&self) -> Value_Type {
        self.field_type.unwrap_or(Value_Type::SCALAR)
    }

    // optional .mesos.Value.Scalar scalar = 3;

    pub fn clear_scalar(&mut self) {
        self.scalar.clear();
    }

    pub fn has_scalar(&self) -> bool {
        self.scalar.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scalar(&mut self, v: Value_Scalar) {
        self.scalar = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_scalar(&'a mut self) -> &'a mut Value_Scalar {
        if self.scalar.is_none() {
            self.scalar.set_default();
        };
        self.scalar.as_mut().unwrap()
    }

    pub fn get_scalar(&'a self) -> &'a Value_Scalar {
        self.scalar.as_ref().unwrap_or_else(|| Value_Scalar::default_instance())
    }

    // optional .mesos.Value.Ranges ranges = 4;

    pub fn clear_ranges(&mut self) {
        self.ranges.clear();
    }

    pub fn has_ranges(&self) -> bool {
        self.ranges.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ranges(&mut self, v: Value_Ranges) {
        self.ranges = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ranges(&'a mut self) -> &'a mut Value_Ranges {
        if self.ranges.is_none() {
            self.ranges.set_default();
        };
        self.ranges.as_mut().unwrap()
    }

    pub fn get_ranges(&'a self) -> &'a Value_Ranges {
        self.ranges.as_ref().unwrap_or_else(|| Value_Ranges::default_instance())
    }

    // optional .mesos.Value.Set set = 6;

    pub fn clear_set(&mut self) {
        self.set.clear();
    }

    pub fn has_set(&self) -> bool {
        self.set.is_some()
    }

    // Param is passed by value, moved
    pub fn set_set(&mut self, v: Value_Set) {
        self.set = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_set(&'a mut self) -> &'a mut Value_Set {
        if self.set.is_none() {
            self.set.set_default();
        };
        self.set.as_mut().unwrap()
    }

    pub fn get_set(&'a self) -> &'a Value_Set {
        self.set.as_ref().unwrap_or_else(|| Value_Set::default_instance())
    }

    // optional .mesos.Value.Text text = 5;

    pub fn clear_text(&mut self) {
        self.text.clear();
    }

    pub fn has_text(&self) -> bool {
        self.text.is_some()
    }

    // Param is passed by value, moved
    pub fn set_text(&mut self, v: Value_Text) {
        self.text = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_text(&'a mut self) -> &'a mut Value_Text {
        if self.text.is_none() {
            self.text.set_default();
        };
        self.text.as_mut().unwrap()
    }

    pub fn get_text(&'a self) -> &'a Value_Text {
        self.text.as_ref().unwrap_or_else(|| Value_Text::default_instance())
    }
}

impl ::protobuf::Message for Attribute {
    fn new() -> Attribute {
        Attribute::new()
    }

    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
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
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.name.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = Value_Type::new(try!(is.read_int32()));
                    self.field_type = Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.scalar.set_default();
                    try!(is.merge_message(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.ranges.set_default();
                    try!(is.merge_message(tmp))
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.set.set_default();
                    try!(is.merge_message(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.text.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(1, value.as_slice());
        };
        for value in self.field_type.iter() {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in self.scalar.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.ranges.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.set.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.text.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.name.as_ref() {
            Some(v) => {
                try!(os.write_string(1, v.as_slice()));
            },
            None => {},
        };
        match self.field_type {
            Some(v) => {
                try!(os.write_enum(2, v as i32));
            },
            None => {},
        };
        match self.scalar.as_ref() {
            Some(v) => {
                try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.ranges.as_ref() {
            Some(v) => {
                try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.set.as_ref() {
            Some(v) => {
                try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.text.as_ref() {
            Some(v) => {
                try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<Attribute>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<Attribute>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&Attribute_name_acc as &'static ::protobuf::reflect::FieldAccessor<Attribute>) });
                fields.push(unsafe { ::std::mem::transmute(&Attribute_field_type_acc as &'static ::protobuf::reflect::FieldAccessor<Attribute>) });
                fields.push(unsafe { ::std::mem::transmute(&Attribute_scalar_acc as &'static ::protobuf::reflect::FieldAccessor<Attribute>) });
                fields.push(unsafe { ::std::mem::transmute(&Attribute_ranges_acc as &'static ::protobuf::reflect::FieldAccessor<Attribute>) });
                fields.push(unsafe { ::std::mem::transmute(&Attribute_set_acc as &'static ::protobuf::reflect::FieldAccessor<Attribute>) });
                fields.push(unsafe { ::std::mem::transmute(&Attribute_text_acc as &'static ::protobuf::reflect::FieldAccessor<Attribute>) });
                ::protobuf::reflect::MessageDescriptor::new::<Attribute>(
                    "Attribute",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<Attribute>()
    }
}

impl ::protobuf::Clear for Attribute {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_field_type();
        self.clear_scalar();
        self.clear_ranges();
        self.clear_set();
        self.clear_text();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for Attribute {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct Attribute_name_acc_type;
static Attribute_name_acc: Attribute_name_acc_type = Attribute_name_acc_type;

impl ::protobuf::reflect::FieldAccessor<Attribute> for Attribute_name_acc_type {
    fn name(&self) -> &'static str {
        "name"
    }

    fn has_field(&self, m: &Attribute) -> bool {
        m.has_name()
    }

    fn get_str<'a>(&self, m: &'a Attribute) -> &'a str {
        m.get_name()
    }
}

#[allow(non_camel_case_types)]
struct Attribute_field_type_acc_type;
static Attribute_field_type_acc: Attribute_field_type_acc_type = Attribute_field_type_acc_type;

impl ::protobuf::reflect::FieldAccessor<Attribute> for Attribute_field_type_acc_type {
    fn name(&self) -> &'static str {
        "field_type"
    }

    fn has_field(&self, m: &Attribute) -> bool {
        m.has_field_type()
    }

    fn get_enum<'a>(&self, m: &Attribute) -> &'static ::protobuf::reflect::EnumValueDescriptor {
        use protobuf::{ProtobufEnum};
        m.get_field_type().descriptor()
    }
}

#[allow(non_camel_case_types)]
struct Attribute_scalar_acc_type;
static Attribute_scalar_acc: Attribute_scalar_acc_type = Attribute_scalar_acc_type;

impl ::protobuf::reflect::FieldAccessor<Attribute> for Attribute_scalar_acc_type {
    fn name(&self) -> &'static str {
        "scalar"
    }

    fn has_field(&self, m: &Attribute) -> bool {
        m.has_scalar()
    }

    fn get_message<'a>(&self, m: &'a Attribute) -> &'a ::protobuf::Message {
        m.get_scalar() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct Attribute_ranges_acc_type;
static Attribute_ranges_acc: Attribute_ranges_acc_type = Attribute_ranges_acc_type;

impl ::protobuf::reflect::FieldAccessor<Attribute> for Attribute_ranges_acc_type {
    fn name(&self) -> &'static str {
        "ranges"
    }

    fn has_field(&self, m: &Attribute) -> bool {
        m.has_ranges()
    }

    fn get_message<'a>(&self, m: &'a Attribute) -> &'a ::protobuf::Message {
        m.get_ranges() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct Attribute_set_acc_type;
static Attribute_set_acc: Attribute_set_acc_type = Attribute_set_acc_type;

impl ::protobuf::reflect::FieldAccessor<Attribute> for Attribute_set_acc_type {
    fn name(&self) -> &'static str {
        "set"
    }

    fn has_field(&self, m: &Attribute) -> bool {
        m.has_set()
    }

    fn get_message<'a>(&self, m: &'a Attribute) -> &'a ::protobuf::Message {
        m.get_set() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct Attribute_text_acc_type;
static Attribute_text_acc: Attribute_text_acc_type = Attribute_text_acc_type;

impl ::protobuf::reflect::FieldAccessor<Attribute> for Attribute_text_acc_type {
    fn name(&self) -> &'static str {
        "text"
    }

    fn has_field(&self, m: &Attribute) -> bool {
        m.has_text()
    }

    fn get_message<'a>(&self, m: &'a Attribute) -> &'a ::protobuf::Message {
        m.get_text() as &'a ::protobuf::Message
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct Resource {
    name: ::protobuf::SingularField<::std::string::String>,
    field_type: ::std::option::Option<Value_Type>,
    scalar: ::protobuf::SingularPtrField<Value_Scalar>,
    ranges: ::protobuf::SingularPtrField<Value_Ranges>,
    set: ::protobuf::SingularPtrField<Value_Set>,
    role: ::protobuf::SingularField<::std::string::String>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> Resource {
    pub fn new() -> Resource {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Resource {
        static mut instance: ::protobuf::lazy::Lazy<Resource> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Resource,
        };
        unsafe {
            instance.get(|| {
                Resource {
                    name: ::protobuf::SingularField::none(),
                    field_type: ::std::option::None,
                    scalar: ::protobuf::SingularPtrField::none(),
                    ranges: ::protobuf::SingularPtrField::none(),
                    set: ::protobuf::SingularPtrField::none(),
                    role: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
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
    pub fn mut_name(&'a mut self) -> &'a mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    pub fn get_name(&'a self) -> &'a str {
        match self.name.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // required .mesos.Value.Type type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type = None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: Value_Type) {
        self.field_type = Some(v);
    }

    pub fn get_field_type(&self) -> Value_Type {
        self.field_type.unwrap_or(Value_Type::SCALAR)
    }

    // optional .mesos.Value.Scalar scalar = 3;

    pub fn clear_scalar(&mut self) {
        self.scalar.clear();
    }

    pub fn has_scalar(&self) -> bool {
        self.scalar.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scalar(&mut self, v: Value_Scalar) {
        self.scalar = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_scalar(&'a mut self) -> &'a mut Value_Scalar {
        if self.scalar.is_none() {
            self.scalar.set_default();
        };
        self.scalar.as_mut().unwrap()
    }

    pub fn get_scalar(&'a self) -> &'a Value_Scalar {
        self.scalar.as_ref().unwrap_or_else(|| Value_Scalar::default_instance())
    }

    // optional .mesos.Value.Ranges ranges = 4;

    pub fn clear_ranges(&mut self) {
        self.ranges.clear();
    }

    pub fn has_ranges(&self) -> bool {
        self.ranges.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ranges(&mut self, v: Value_Ranges) {
        self.ranges = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ranges(&'a mut self) -> &'a mut Value_Ranges {
        if self.ranges.is_none() {
            self.ranges.set_default();
        };
        self.ranges.as_mut().unwrap()
    }

    pub fn get_ranges(&'a self) -> &'a Value_Ranges {
        self.ranges.as_ref().unwrap_or_else(|| Value_Ranges::default_instance())
    }

    // optional .mesos.Value.Set set = 5;

    pub fn clear_set(&mut self) {
        self.set.clear();
    }

    pub fn has_set(&self) -> bool {
        self.set.is_some()
    }

    // Param is passed by value, moved
    pub fn set_set(&mut self, v: Value_Set) {
        self.set = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_set(&'a mut self) -> &'a mut Value_Set {
        if self.set.is_none() {
            self.set.set_default();
        };
        self.set.as_mut().unwrap()
    }

    pub fn get_set(&'a self) -> &'a Value_Set {
        self.set.as_ref().unwrap_or_else(|| Value_Set::default_instance())
    }

    // optional string role = 6;

    pub fn clear_role(&mut self) {
        self.role.clear();
    }

    pub fn has_role(&self) -> bool {
        self.role.is_some()
    }

    // Param is passed by value, moved
    pub fn set_role(&mut self, v: ::std::string::String) {
        self.role = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_role(&'a mut self) -> &'a mut ::std::string::String {
        if self.role.is_none() {
            self.role.set_default();
        };
        self.role.as_mut().unwrap()
    }

    pub fn get_role(&'a self) -> &'a str {
        match self.role.as_ref() {
            Some(v) => v.as_slice(),
            None => "*",
        }
    }
}

impl ::protobuf::Message for Resource {
    fn new() -> Resource {
        Resource::new()
    }

    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
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
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.name.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = Value_Type::new(try!(is.read_int32()));
                    self.field_type = Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.scalar.set_default();
                    try!(is.merge_message(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.ranges.set_default();
                    try!(is.merge_message(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.set.set_default();
                    try!(is.merge_message(tmp))
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.role.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(1, value.as_slice());
        };
        for value in self.field_type.iter() {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in self.scalar.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.ranges.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.set.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.role.iter() {
            my_size += ::protobuf::rt::string_size(6, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.name.as_ref() {
            Some(v) => {
                try!(os.write_string(1, v.as_slice()));
            },
            None => {},
        };
        match self.field_type {
            Some(v) => {
                try!(os.write_enum(2, v as i32));
            },
            None => {},
        };
        match self.scalar.as_ref() {
            Some(v) => {
                try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.ranges.as_ref() {
            Some(v) => {
                try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.set.as_ref() {
            Some(v) => {
                try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.role.as_ref() {
            Some(v) => {
                try!(os.write_string(6, v.as_slice()));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<Resource>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<Resource>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&Resource_name_acc as &'static ::protobuf::reflect::FieldAccessor<Resource>) });
                fields.push(unsafe { ::std::mem::transmute(&Resource_field_type_acc as &'static ::protobuf::reflect::FieldAccessor<Resource>) });
                fields.push(unsafe { ::std::mem::transmute(&Resource_scalar_acc as &'static ::protobuf::reflect::FieldAccessor<Resource>) });
                fields.push(unsafe { ::std::mem::transmute(&Resource_ranges_acc as &'static ::protobuf::reflect::FieldAccessor<Resource>) });
                fields.push(unsafe { ::std::mem::transmute(&Resource_set_acc as &'static ::protobuf::reflect::FieldAccessor<Resource>) });
                fields.push(unsafe { ::std::mem::transmute(&Resource_role_acc as &'static ::protobuf::reflect::FieldAccessor<Resource>) });
                ::protobuf::reflect::MessageDescriptor::new::<Resource>(
                    "Resource",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<Resource>()
    }
}

impl ::protobuf::Clear for Resource {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_field_type();
        self.clear_scalar();
        self.clear_ranges();
        self.clear_set();
        self.clear_role();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for Resource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct Resource_name_acc_type;
static Resource_name_acc: Resource_name_acc_type = Resource_name_acc_type;

impl ::protobuf::reflect::FieldAccessor<Resource> for Resource_name_acc_type {
    fn name(&self) -> &'static str {
        "name"
    }

    fn has_field(&self, m: &Resource) -> bool {
        m.has_name()
    }

    fn get_str<'a>(&self, m: &'a Resource) -> &'a str {
        m.get_name()
    }
}

#[allow(non_camel_case_types)]
struct Resource_field_type_acc_type;
static Resource_field_type_acc: Resource_field_type_acc_type = Resource_field_type_acc_type;

impl ::protobuf::reflect::FieldAccessor<Resource> for Resource_field_type_acc_type {
    fn name(&self) -> &'static str {
        "field_type"
    }

    fn has_field(&self, m: &Resource) -> bool {
        m.has_field_type()
    }

    fn get_enum<'a>(&self, m: &Resource) -> &'static ::protobuf::reflect::EnumValueDescriptor {
        use protobuf::{ProtobufEnum};
        m.get_field_type().descriptor()
    }
}

#[allow(non_camel_case_types)]
struct Resource_scalar_acc_type;
static Resource_scalar_acc: Resource_scalar_acc_type = Resource_scalar_acc_type;

impl ::protobuf::reflect::FieldAccessor<Resource> for Resource_scalar_acc_type {
    fn name(&self) -> &'static str {
        "scalar"
    }

    fn has_field(&self, m: &Resource) -> bool {
        m.has_scalar()
    }

    fn get_message<'a>(&self, m: &'a Resource) -> &'a ::protobuf::Message {
        m.get_scalar() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct Resource_ranges_acc_type;
static Resource_ranges_acc: Resource_ranges_acc_type = Resource_ranges_acc_type;

impl ::protobuf::reflect::FieldAccessor<Resource> for Resource_ranges_acc_type {
    fn name(&self) -> &'static str {
        "ranges"
    }

    fn has_field(&self, m: &Resource) -> bool {
        m.has_ranges()
    }

    fn get_message<'a>(&self, m: &'a Resource) -> &'a ::protobuf::Message {
        m.get_ranges() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct Resource_set_acc_type;
static Resource_set_acc: Resource_set_acc_type = Resource_set_acc_type;

impl ::protobuf::reflect::FieldAccessor<Resource> for Resource_set_acc_type {
    fn name(&self) -> &'static str {
        "set"
    }

    fn has_field(&self, m: &Resource) -> bool {
        m.has_set()
    }

    fn get_message<'a>(&self, m: &'a Resource) -> &'a ::protobuf::Message {
        m.get_set() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct Resource_role_acc_type;
static Resource_role_acc: Resource_role_acc_type = Resource_role_acc_type;

impl ::protobuf::reflect::FieldAccessor<Resource> for Resource_role_acc_type {
    fn name(&self) -> &'static str {
        "role"
    }

    fn has_field(&self, m: &Resource) -> bool {
        m.has_role()
    }

    fn get_str<'a>(&self, m: &'a Resource) -> &'a str {
        m.get_role()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct ResourceStatistics {
    timestamp: ::std::option::Option<f64>,
    cpus_user_time_secs: ::std::option::Option<f64>,
    cpus_system_time_secs: ::std::option::Option<f64>,
    cpus_limit: ::std::option::Option<f64>,
    cpus_nr_periods: ::std::option::Option<u32>,
    cpus_nr_throttled: ::std::option::Option<u32>,
    cpus_throttled_time_secs: ::std::option::Option<f64>,
    mem_rss_bytes: ::std::option::Option<u64>,
    mem_limit_bytes: ::std::option::Option<u64>,
    mem_file_bytes: ::std::option::Option<u64>,
    mem_anon_bytes: ::std::option::Option<u64>,
    mem_mapped_file_bytes: ::std::option::Option<u64>,
    perf: ::protobuf::SingularPtrField<PerfStatistics>,
    net_rx_packets: ::std::option::Option<u64>,
    net_rx_bytes: ::std::option::Option<u64>,
    net_rx_errors: ::std::option::Option<u64>,
    net_rx_dropped: ::std::option::Option<u64>,
    net_tx_packets: ::std::option::Option<u64>,
    net_tx_bytes: ::std::option::Option<u64>,
    net_tx_errors: ::std::option::Option<u64>,
    net_tx_dropped: ::std::option::Option<u64>,
    net_tcp_rtt_microsecs_p50: ::std::option::Option<f64>,
    net_tcp_rtt_microsecs_p90: ::std::option::Option<f64>,
    net_tcp_rtt_microsecs_p95: ::std::option::Option<f64>,
    net_tcp_rtt_microsecs_p99: ::std::option::Option<f64>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> ResourceStatistics {
    pub fn new() -> ResourceStatistics {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResourceStatistics {
        static mut instance: ::protobuf::lazy::Lazy<ResourceStatistics> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResourceStatistics,
        };
        unsafe {
            instance.get(|| {
                ResourceStatistics {
                    timestamp: ::std::option::None,
                    cpus_user_time_secs: ::std::option::None,
                    cpus_system_time_secs: ::std::option::None,
                    cpus_limit: ::std::option::None,
                    cpus_nr_periods: ::std::option::None,
                    cpus_nr_throttled: ::std::option::None,
                    cpus_throttled_time_secs: ::std::option::None,
                    mem_rss_bytes: ::std::option::None,
                    mem_limit_bytes: ::std::option::None,
                    mem_file_bytes: ::std::option::None,
                    mem_anon_bytes: ::std::option::None,
                    mem_mapped_file_bytes: ::std::option::None,
                    perf: ::protobuf::SingularPtrField::none(),
                    net_rx_packets: ::std::option::None,
                    net_rx_bytes: ::std::option::None,
                    net_rx_errors: ::std::option::None,
                    net_rx_dropped: ::std::option::None,
                    net_tx_packets: ::std::option::None,
                    net_tx_bytes: ::std::option::None,
                    net_tx_errors: ::std::option::None,
                    net_tx_dropped: ::std::option::None,
                    net_tcp_rtt_microsecs_p50: ::std::option::None,
                    net_tcp_rtt_microsecs_p90: ::std::option::None,
                    net_tcp_rtt_microsecs_p95: ::std::option::None,
                    net_tcp_rtt_microsecs_p99: ::std::option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // required double timestamp = 1;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = None;
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: f64) {
        self.timestamp = Some(v);
    }

    pub fn get_timestamp(&self) -> f64 {
        self.timestamp.unwrap_or(0.)
    }

    // optional double cpus_user_time_secs = 2;

    pub fn clear_cpus_user_time_secs(&mut self) {
        self.cpus_user_time_secs = None;
    }

    pub fn has_cpus_user_time_secs(&self) -> bool {
        self.cpus_user_time_secs.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cpus_user_time_secs(&mut self, v: f64) {
        self.cpus_user_time_secs = Some(v);
    }

    pub fn get_cpus_user_time_secs(&self) -> f64 {
        self.cpus_user_time_secs.unwrap_or(0.)
    }

    // optional double cpus_system_time_secs = 3;

    pub fn clear_cpus_system_time_secs(&mut self) {
        self.cpus_system_time_secs = None;
    }

    pub fn has_cpus_system_time_secs(&self) -> bool {
        self.cpus_system_time_secs.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cpus_system_time_secs(&mut self, v: f64) {
        self.cpus_system_time_secs = Some(v);
    }

    pub fn get_cpus_system_time_secs(&self) -> f64 {
        self.cpus_system_time_secs.unwrap_or(0.)
    }

    // optional double cpus_limit = 4;

    pub fn clear_cpus_limit(&mut self) {
        self.cpus_limit = None;
    }

    pub fn has_cpus_limit(&self) -> bool {
        self.cpus_limit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cpus_limit(&mut self, v: f64) {
        self.cpus_limit = Some(v);
    }

    pub fn get_cpus_limit(&self) -> f64 {
        self.cpus_limit.unwrap_or(0.)
    }

    // optional uint32 cpus_nr_periods = 7;

    pub fn clear_cpus_nr_periods(&mut self) {
        self.cpus_nr_periods = None;
    }

    pub fn has_cpus_nr_periods(&self) -> bool {
        self.cpus_nr_periods.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cpus_nr_periods(&mut self, v: u32) {
        self.cpus_nr_periods = Some(v);
    }

    pub fn get_cpus_nr_periods(&self) -> u32 {
        self.cpus_nr_periods.unwrap_or(0)
    }

    // optional uint32 cpus_nr_throttled = 8;

    pub fn clear_cpus_nr_throttled(&mut self) {
        self.cpus_nr_throttled = None;
    }

    pub fn has_cpus_nr_throttled(&self) -> bool {
        self.cpus_nr_throttled.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cpus_nr_throttled(&mut self, v: u32) {
        self.cpus_nr_throttled = Some(v);
    }

    pub fn get_cpus_nr_throttled(&self) -> u32 {
        self.cpus_nr_throttled.unwrap_or(0)
    }

    // optional double cpus_throttled_time_secs = 9;

    pub fn clear_cpus_throttled_time_secs(&mut self) {
        self.cpus_throttled_time_secs = None;
    }

    pub fn has_cpus_throttled_time_secs(&self) -> bool {
        self.cpus_throttled_time_secs.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cpus_throttled_time_secs(&mut self, v: f64) {
        self.cpus_throttled_time_secs = Some(v);
    }

    pub fn get_cpus_throttled_time_secs(&self) -> f64 {
        self.cpus_throttled_time_secs.unwrap_or(0.)
    }

    // optional uint64 mem_rss_bytes = 5;

    pub fn clear_mem_rss_bytes(&mut self) {
        self.mem_rss_bytes = None;
    }

    pub fn has_mem_rss_bytes(&self) -> bool {
        self.mem_rss_bytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mem_rss_bytes(&mut self, v: u64) {
        self.mem_rss_bytes = Some(v);
    }

    pub fn get_mem_rss_bytes(&self) -> u64 {
        self.mem_rss_bytes.unwrap_or(0)
    }

    // optional uint64 mem_limit_bytes = 6;

    pub fn clear_mem_limit_bytes(&mut self) {
        self.mem_limit_bytes = None;
    }

    pub fn has_mem_limit_bytes(&self) -> bool {
        self.mem_limit_bytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mem_limit_bytes(&mut self, v: u64) {
        self.mem_limit_bytes = Some(v);
    }

    pub fn get_mem_limit_bytes(&self) -> u64 {
        self.mem_limit_bytes.unwrap_or(0)
    }

    // optional uint64 mem_file_bytes = 10;

    pub fn clear_mem_file_bytes(&mut self) {
        self.mem_file_bytes = None;
    }

    pub fn has_mem_file_bytes(&self) -> bool {
        self.mem_file_bytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mem_file_bytes(&mut self, v: u64) {
        self.mem_file_bytes = Some(v);
    }

    pub fn get_mem_file_bytes(&self) -> u64 {
        self.mem_file_bytes.unwrap_or(0)
    }

    // optional uint64 mem_anon_bytes = 11;

    pub fn clear_mem_anon_bytes(&mut self) {
        self.mem_anon_bytes = None;
    }

    pub fn has_mem_anon_bytes(&self) -> bool {
        self.mem_anon_bytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mem_anon_bytes(&mut self, v: u64) {
        self.mem_anon_bytes = Some(v);
    }

    pub fn get_mem_anon_bytes(&self) -> u64 {
        self.mem_anon_bytes.unwrap_or(0)
    }

    // optional uint64 mem_mapped_file_bytes = 12;

    pub fn clear_mem_mapped_file_bytes(&mut self) {
        self.mem_mapped_file_bytes = None;
    }

    pub fn has_mem_mapped_file_bytes(&self) -> bool {
        self.mem_mapped_file_bytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mem_mapped_file_bytes(&mut self, v: u64) {
        self.mem_mapped_file_bytes = Some(v);
    }

    pub fn get_mem_mapped_file_bytes(&self) -> u64 {
        self.mem_mapped_file_bytes.unwrap_or(0)
    }

    // optional .mesos.PerfStatistics perf = 13;

    pub fn clear_perf(&mut self) {
        self.perf.clear();
    }

    pub fn has_perf(&self) -> bool {
        self.perf.is_some()
    }

    // Param is passed by value, moved
    pub fn set_perf(&mut self, v: PerfStatistics) {
        self.perf = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_perf(&'a mut self) -> &'a mut PerfStatistics {
        if self.perf.is_none() {
            self.perf.set_default();
        };
        self.perf.as_mut().unwrap()
    }

    pub fn get_perf(&'a self) -> &'a PerfStatistics {
        self.perf.as_ref().unwrap_or_else(|| PerfStatistics::default_instance())
    }

    // optional uint64 net_rx_packets = 14;

    pub fn clear_net_rx_packets(&mut self) {
        self.net_rx_packets = None;
    }

    pub fn has_net_rx_packets(&self) -> bool {
        self.net_rx_packets.is_some()
    }

    // Param is passed by value, moved
    pub fn set_net_rx_packets(&mut self, v: u64) {
        self.net_rx_packets = Some(v);
    }

    pub fn get_net_rx_packets(&self) -> u64 {
        self.net_rx_packets.unwrap_or(0)
    }

    // optional uint64 net_rx_bytes = 15;

    pub fn clear_net_rx_bytes(&mut self) {
        self.net_rx_bytes = None;
    }

    pub fn has_net_rx_bytes(&self) -> bool {
        self.net_rx_bytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_net_rx_bytes(&mut self, v: u64) {
        self.net_rx_bytes = Some(v);
    }

    pub fn get_net_rx_bytes(&self) -> u64 {
        self.net_rx_bytes.unwrap_or(0)
    }

    // optional uint64 net_rx_errors = 16;

    pub fn clear_net_rx_errors(&mut self) {
        self.net_rx_errors = None;
    }

    pub fn has_net_rx_errors(&self) -> bool {
        self.net_rx_errors.is_some()
    }

    // Param is passed by value, moved
    pub fn set_net_rx_errors(&mut self, v: u64) {
        self.net_rx_errors = Some(v);
    }

    pub fn get_net_rx_errors(&self) -> u64 {
        self.net_rx_errors.unwrap_or(0)
    }

    // optional uint64 net_rx_dropped = 17;

    pub fn clear_net_rx_dropped(&mut self) {
        self.net_rx_dropped = None;
    }

    pub fn has_net_rx_dropped(&self) -> bool {
        self.net_rx_dropped.is_some()
    }

    // Param is passed by value, moved
    pub fn set_net_rx_dropped(&mut self, v: u64) {
        self.net_rx_dropped = Some(v);
    }

    pub fn get_net_rx_dropped(&self) -> u64 {
        self.net_rx_dropped.unwrap_or(0)
    }

    // optional uint64 net_tx_packets = 18;

    pub fn clear_net_tx_packets(&mut self) {
        self.net_tx_packets = None;
    }

    pub fn has_net_tx_packets(&self) -> bool {
        self.net_tx_packets.is_some()
    }

    // Param is passed by value, moved
    pub fn set_net_tx_packets(&mut self, v: u64) {
        self.net_tx_packets = Some(v);
    }

    pub fn get_net_tx_packets(&self) -> u64 {
        self.net_tx_packets.unwrap_or(0)
    }

    // optional uint64 net_tx_bytes = 19;

    pub fn clear_net_tx_bytes(&mut self) {
        self.net_tx_bytes = None;
    }

    pub fn has_net_tx_bytes(&self) -> bool {
        self.net_tx_bytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_net_tx_bytes(&mut self, v: u64) {
        self.net_tx_bytes = Some(v);
    }

    pub fn get_net_tx_bytes(&self) -> u64 {
        self.net_tx_bytes.unwrap_or(0)
    }

    // optional uint64 net_tx_errors = 20;

    pub fn clear_net_tx_errors(&mut self) {
        self.net_tx_errors = None;
    }

    pub fn has_net_tx_errors(&self) -> bool {
        self.net_tx_errors.is_some()
    }

    // Param is passed by value, moved
    pub fn set_net_tx_errors(&mut self, v: u64) {
        self.net_tx_errors = Some(v);
    }

    pub fn get_net_tx_errors(&self) -> u64 {
        self.net_tx_errors.unwrap_or(0)
    }

    // optional uint64 net_tx_dropped = 21;

    pub fn clear_net_tx_dropped(&mut self) {
        self.net_tx_dropped = None;
    }

    pub fn has_net_tx_dropped(&self) -> bool {
        self.net_tx_dropped.is_some()
    }

    // Param is passed by value, moved
    pub fn set_net_tx_dropped(&mut self, v: u64) {
        self.net_tx_dropped = Some(v);
    }

    pub fn get_net_tx_dropped(&self) -> u64 {
        self.net_tx_dropped.unwrap_or(0)
    }

    // optional double net_tcp_rtt_microsecs_p50 = 22;

    pub fn clear_net_tcp_rtt_microsecs_p50(&mut self) {
        self.net_tcp_rtt_microsecs_p50 = None;
    }

    pub fn has_net_tcp_rtt_microsecs_p50(&self) -> bool {
        self.net_tcp_rtt_microsecs_p50.is_some()
    }

    // Param is passed by value, moved
    pub fn set_net_tcp_rtt_microsecs_p50(&mut self, v: f64) {
        self.net_tcp_rtt_microsecs_p50 = Some(v);
    }

    pub fn get_net_tcp_rtt_microsecs_p50(&self) -> f64 {
        self.net_tcp_rtt_microsecs_p50.unwrap_or(0.)
    }

    // optional double net_tcp_rtt_microsecs_p90 = 23;

    pub fn clear_net_tcp_rtt_microsecs_p90(&mut self) {
        self.net_tcp_rtt_microsecs_p90 = None;
    }

    pub fn has_net_tcp_rtt_microsecs_p90(&self) -> bool {
        self.net_tcp_rtt_microsecs_p90.is_some()
    }

    // Param is passed by value, moved
    pub fn set_net_tcp_rtt_microsecs_p90(&mut self, v: f64) {
        self.net_tcp_rtt_microsecs_p90 = Some(v);
    }

    pub fn get_net_tcp_rtt_microsecs_p90(&self) -> f64 {
        self.net_tcp_rtt_microsecs_p90.unwrap_or(0.)
    }

    // optional double net_tcp_rtt_microsecs_p95 = 24;

    pub fn clear_net_tcp_rtt_microsecs_p95(&mut self) {
        self.net_tcp_rtt_microsecs_p95 = None;
    }

    pub fn has_net_tcp_rtt_microsecs_p95(&self) -> bool {
        self.net_tcp_rtt_microsecs_p95.is_some()
    }

    // Param is passed by value, moved
    pub fn set_net_tcp_rtt_microsecs_p95(&mut self, v: f64) {
        self.net_tcp_rtt_microsecs_p95 = Some(v);
    }

    pub fn get_net_tcp_rtt_microsecs_p95(&self) -> f64 {
        self.net_tcp_rtt_microsecs_p95.unwrap_or(0.)
    }

    // optional double net_tcp_rtt_microsecs_p99 = 25;

    pub fn clear_net_tcp_rtt_microsecs_p99(&mut self) {
        self.net_tcp_rtt_microsecs_p99 = None;
    }

    pub fn has_net_tcp_rtt_microsecs_p99(&self) -> bool {
        self.net_tcp_rtt_microsecs_p99.is_some()
    }

    // Param is passed by value, moved
    pub fn set_net_tcp_rtt_microsecs_p99(&mut self, v: f64) {
        self.net_tcp_rtt_microsecs_p99 = Some(v);
    }

    pub fn get_net_tcp_rtt_microsecs_p99(&self) -> f64 {
        self.net_tcp_rtt_microsecs_p99.unwrap_or(0.)
    }
}

impl ::protobuf::Message for ResourceStatistics {
    fn new() -> ResourceStatistics {
        ResourceStatistics::new()
    }

    fn is_initialized(&self) -> bool {
        if self.timestamp.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.timestamp = Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.cpus_user_time_secs = Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.cpus_system_time_secs = Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.cpus_limit = Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.cpus_nr_periods = Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.cpus_nr_throttled = Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.cpus_throttled_time_secs = Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.mem_rss_bytes = Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.mem_limit_bytes = Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.mem_file_bytes = Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.mem_anon_bytes = Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.mem_mapped_file_bytes = Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.perf.set_default();
                    try!(is.merge_message(tmp))
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.net_rx_packets = Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.net_rx_bytes = Some(tmp);
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.net_rx_errors = Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.net_rx_dropped = Some(tmp);
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.net_tx_packets = Some(tmp);
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.net_tx_bytes = Some(tmp);
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.net_tx_errors = Some(tmp);
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.net_tx_dropped = Some(tmp);
                },
                22 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.net_tcp_rtt_microsecs_p50 = Some(tmp);
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.net_tcp_rtt_microsecs_p90 = Some(tmp);
                },
                24 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.net_tcp_rtt_microsecs_p95 = Some(tmp);
                },
                25 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.net_tcp_rtt_microsecs_p99 = Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        if self.timestamp.is_some() {
            my_size += 9;
        };
        if self.cpus_user_time_secs.is_some() {
            my_size += 9;
        };
        if self.cpus_system_time_secs.is_some() {
            my_size += 9;
        };
        if self.cpus_limit.is_some() {
            my_size += 9;
        };
        for value in self.cpus_nr_periods.iter() {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.cpus_nr_throttled.iter() {
            my_size += ::protobuf::rt::value_size(8, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.cpus_throttled_time_secs.is_some() {
            my_size += 9;
        };
        for value in self.mem_rss_bytes.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.mem_limit_bytes.iter() {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.mem_file_bytes.iter() {
            my_size += ::protobuf::rt::value_size(10, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.mem_anon_bytes.iter() {
            my_size += ::protobuf::rt::value_size(11, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.mem_mapped_file_bytes.iter() {
            my_size += ::protobuf::rt::value_size(12, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.perf.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.net_rx_packets.iter() {
            my_size += ::protobuf::rt::value_size(14, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.net_rx_bytes.iter() {
            my_size += ::protobuf::rt::value_size(15, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.net_rx_errors.iter() {
            my_size += ::protobuf::rt::value_size(16, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.net_rx_dropped.iter() {
            my_size += ::protobuf::rt::value_size(17, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.net_tx_packets.iter() {
            my_size += ::protobuf::rt::value_size(18, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.net_tx_bytes.iter() {
            my_size += ::protobuf::rt::value_size(19, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.net_tx_errors.iter() {
            my_size += ::protobuf::rt::value_size(20, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.net_tx_dropped.iter() {
            my_size += ::protobuf::rt::value_size(21, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.net_tcp_rtt_microsecs_p50.is_some() {
            my_size += 10;
        };
        if self.net_tcp_rtt_microsecs_p90.is_some() {
            my_size += 10;
        };
        if self.net_tcp_rtt_microsecs_p95.is_some() {
            my_size += 10;
        };
        if self.net_tcp_rtt_microsecs_p99.is_some() {
            my_size += 10;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.timestamp {
            Some(v) => {
                try!(os.write_double(1, v));
            },
            None => {},
        };
        match self.cpus_user_time_secs {
            Some(v) => {
                try!(os.write_double(2, v));
            },
            None => {},
        };
        match self.cpus_system_time_secs {
            Some(v) => {
                try!(os.write_double(3, v));
            },
            None => {},
        };
        match self.cpus_limit {
            Some(v) => {
                try!(os.write_double(4, v));
            },
            None => {},
        };
        match self.cpus_nr_periods {
            Some(v) => {
                try!(os.write_uint32(7, v));
            },
            None => {},
        };
        match self.cpus_nr_throttled {
            Some(v) => {
                try!(os.write_uint32(8, v));
            },
            None => {},
        };
        match self.cpus_throttled_time_secs {
            Some(v) => {
                try!(os.write_double(9, v));
            },
            None => {},
        };
        match self.mem_rss_bytes {
            Some(v) => {
                try!(os.write_uint64(5, v));
            },
            None => {},
        };
        match self.mem_limit_bytes {
            Some(v) => {
                try!(os.write_uint64(6, v));
            },
            None => {},
        };
        match self.mem_file_bytes {
            Some(v) => {
                try!(os.write_uint64(10, v));
            },
            None => {},
        };
        match self.mem_anon_bytes {
            Some(v) => {
                try!(os.write_uint64(11, v));
            },
            None => {},
        };
        match self.mem_mapped_file_bytes {
            Some(v) => {
                try!(os.write_uint64(12, v));
            },
            None => {},
        };
        match self.perf.as_ref() {
            Some(v) => {
                try!(os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.net_rx_packets {
            Some(v) => {
                try!(os.write_uint64(14, v));
            },
            None => {},
        };
        match self.net_rx_bytes {
            Some(v) => {
                try!(os.write_uint64(15, v));
            },
            None => {},
        };
        match self.net_rx_errors {
            Some(v) => {
                try!(os.write_uint64(16, v));
            },
            None => {},
        };
        match self.net_rx_dropped {
            Some(v) => {
                try!(os.write_uint64(17, v));
            },
            None => {},
        };
        match self.net_tx_packets {
            Some(v) => {
                try!(os.write_uint64(18, v));
            },
            None => {},
        };
        match self.net_tx_bytes {
            Some(v) => {
                try!(os.write_uint64(19, v));
            },
            None => {},
        };
        match self.net_tx_errors {
            Some(v) => {
                try!(os.write_uint64(20, v));
            },
            None => {},
        };
        match self.net_tx_dropped {
            Some(v) => {
                try!(os.write_uint64(21, v));
            },
            None => {},
        };
        match self.net_tcp_rtt_microsecs_p50 {
            Some(v) => {
                try!(os.write_double(22, v));
            },
            None => {},
        };
        match self.net_tcp_rtt_microsecs_p90 {
            Some(v) => {
                try!(os.write_double(23, v));
            },
            None => {},
        };
        match self.net_tcp_rtt_microsecs_p95 {
            Some(v) => {
                try!(os.write_double(24, v));
            },
            None => {},
        };
        match self.net_tcp_rtt_microsecs_p99 {
            Some(v) => {
                try!(os.write_double(25, v));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<ResourceStatistics>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<ResourceStatistics>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&ResourceStatistics_timestamp_acc as &'static ::protobuf::reflect::FieldAccessor<ResourceStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&ResourceStatistics_cpus_user_time_secs_acc as &'static ::protobuf::reflect::FieldAccessor<ResourceStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&ResourceStatistics_cpus_system_time_secs_acc as &'static ::protobuf::reflect::FieldAccessor<ResourceStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&ResourceStatistics_cpus_limit_acc as &'static ::protobuf::reflect::FieldAccessor<ResourceStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&ResourceStatistics_cpus_nr_periods_acc as &'static ::protobuf::reflect::FieldAccessor<ResourceStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&ResourceStatistics_cpus_nr_throttled_acc as &'static ::protobuf::reflect::FieldAccessor<ResourceStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&ResourceStatistics_cpus_throttled_time_secs_acc as &'static ::protobuf::reflect::FieldAccessor<ResourceStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&ResourceStatistics_mem_rss_bytes_acc as &'static ::protobuf::reflect::FieldAccessor<ResourceStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&ResourceStatistics_mem_limit_bytes_acc as &'static ::protobuf::reflect::FieldAccessor<ResourceStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&ResourceStatistics_mem_file_bytes_acc as &'static ::protobuf::reflect::FieldAccessor<ResourceStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&ResourceStatistics_mem_anon_bytes_acc as &'static ::protobuf::reflect::FieldAccessor<ResourceStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&ResourceStatistics_mem_mapped_file_bytes_acc as &'static ::protobuf::reflect::FieldAccessor<ResourceStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&ResourceStatistics_perf_acc as &'static ::protobuf::reflect::FieldAccessor<ResourceStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&ResourceStatistics_net_rx_packets_acc as &'static ::protobuf::reflect::FieldAccessor<ResourceStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&ResourceStatistics_net_rx_bytes_acc as &'static ::protobuf::reflect::FieldAccessor<ResourceStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&ResourceStatistics_net_rx_errors_acc as &'static ::protobuf::reflect::FieldAccessor<ResourceStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&ResourceStatistics_net_rx_dropped_acc as &'static ::protobuf::reflect::FieldAccessor<ResourceStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&ResourceStatistics_net_tx_packets_acc as &'static ::protobuf::reflect::FieldAccessor<ResourceStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&ResourceStatistics_net_tx_bytes_acc as &'static ::protobuf::reflect::FieldAccessor<ResourceStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&ResourceStatistics_net_tx_errors_acc as &'static ::protobuf::reflect::FieldAccessor<ResourceStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&ResourceStatistics_net_tx_dropped_acc as &'static ::protobuf::reflect::FieldAccessor<ResourceStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&ResourceStatistics_net_tcp_rtt_microsecs_p50_acc as &'static ::protobuf::reflect::FieldAccessor<ResourceStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&ResourceStatistics_net_tcp_rtt_microsecs_p90_acc as &'static ::protobuf::reflect::FieldAccessor<ResourceStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&ResourceStatistics_net_tcp_rtt_microsecs_p95_acc as &'static ::protobuf::reflect::FieldAccessor<ResourceStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&ResourceStatistics_net_tcp_rtt_microsecs_p99_acc as &'static ::protobuf::reflect::FieldAccessor<ResourceStatistics>) });
                ::protobuf::reflect::MessageDescriptor::new::<ResourceStatistics>(
                    "ResourceStatistics",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<ResourceStatistics>()
    }
}

impl ::protobuf::Clear for ResourceStatistics {
    fn clear(&mut self) {
        self.clear_timestamp();
        self.clear_cpus_user_time_secs();
        self.clear_cpus_system_time_secs();
        self.clear_cpus_limit();
        self.clear_cpus_nr_periods();
        self.clear_cpus_nr_throttled();
        self.clear_cpus_throttled_time_secs();
        self.clear_mem_rss_bytes();
        self.clear_mem_limit_bytes();
        self.clear_mem_file_bytes();
        self.clear_mem_anon_bytes();
        self.clear_mem_mapped_file_bytes();
        self.clear_perf();
        self.clear_net_rx_packets();
        self.clear_net_rx_bytes();
        self.clear_net_rx_errors();
        self.clear_net_rx_dropped();
        self.clear_net_tx_packets();
        self.clear_net_tx_bytes();
        self.clear_net_tx_errors();
        self.clear_net_tx_dropped();
        self.clear_net_tcp_rtt_microsecs_p50();
        self.clear_net_tcp_rtt_microsecs_p90();
        self.clear_net_tcp_rtt_microsecs_p95();
        self.clear_net_tcp_rtt_microsecs_p99();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for ResourceStatistics {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct ResourceStatistics_timestamp_acc_type;
static ResourceStatistics_timestamp_acc: ResourceStatistics_timestamp_acc_type = ResourceStatistics_timestamp_acc_type;

impl ::protobuf::reflect::FieldAccessor<ResourceStatistics> for ResourceStatistics_timestamp_acc_type {
    fn name(&self) -> &'static str {
        "timestamp"
    }

    fn has_field(&self, m: &ResourceStatistics) -> bool {
        m.has_timestamp()
    }

    fn get_f64(&self, m: &ResourceStatistics) -> f64 {
        m.get_timestamp()
    }
}

#[allow(non_camel_case_types)]
struct ResourceStatistics_cpus_user_time_secs_acc_type;
static ResourceStatistics_cpus_user_time_secs_acc: ResourceStatistics_cpus_user_time_secs_acc_type = ResourceStatistics_cpus_user_time_secs_acc_type;

impl ::protobuf::reflect::FieldAccessor<ResourceStatistics> for ResourceStatistics_cpus_user_time_secs_acc_type {
    fn name(&self) -> &'static str {
        "cpus_user_time_secs"
    }

    fn has_field(&self, m: &ResourceStatistics) -> bool {
        m.has_cpus_user_time_secs()
    }

    fn get_f64(&self, m: &ResourceStatistics) -> f64 {
        m.get_cpus_user_time_secs()
    }
}

#[allow(non_camel_case_types)]
struct ResourceStatistics_cpus_system_time_secs_acc_type;
static ResourceStatistics_cpus_system_time_secs_acc: ResourceStatistics_cpus_system_time_secs_acc_type = ResourceStatistics_cpus_system_time_secs_acc_type;

impl ::protobuf::reflect::FieldAccessor<ResourceStatistics> for ResourceStatistics_cpus_system_time_secs_acc_type {
    fn name(&self) -> &'static str {
        "cpus_system_time_secs"
    }

    fn has_field(&self, m: &ResourceStatistics) -> bool {
        m.has_cpus_system_time_secs()
    }

    fn get_f64(&self, m: &ResourceStatistics) -> f64 {
        m.get_cpus_system_time_secs()
    }
}

#[allow(non_camel_case_types)]
struct ResourceStatistics_cpus_limit_acc_type;
static ResourceStatistics_cpus_limit_acc: ResourceStatistics_cpus_limit_acc_type = ResourceStatistics_cpus_limit_acc_type;

impl ::protobuf::reflect::FieldAccessor<ResourceStatistics> for ResourceStatistics_cpus_limit_acc_type {
    fn name(&self) -> &'static str {
        "cpus_limit"
    }

    fn has_field(&self, m: &ResourceStatistics) -> bool {
        m.has_cpus_limit()
    }

    fn get_f64(&self, m: &ResourceStatistics) -> f64 {
        m.get_cpus_limit()
    }
}

#[allow(non_camel_case_types)]
struct ResourceStatistics_cpus_nr_periods_acc_type;
static ResourceStatistics_cpus_nr_periods_acc: ResourceStatistics_cpus_nr_periods_acc_type = ResourceStatistics_cpus_nr_periods_acc_type;

impl ::protobuf::reflect::FieldAccessor<ResourceStatistics> for ResourceStatistics_cpus_nr_periods_acc_type {
    fn name(&self) -> &'static str {
        "cpus_nr_periods"
    }

    fn has_field(&self, m: &ResourceStatistics) -> bool {
        m.has_cpus_nr_periods()
    }

    fn get_u32(&self, m: &ResourceStatistics) -> u32 {
        m.get_cpus_nr_periods()
    }
}

#[allow(non_camel_case_types)]
struct ResourceStatistics_cpus_nr_throttled_acc_type;
static ResourceStatistics_cpus_nr_throttled_acc: ResourceStatistics_cpus_nr_throttled_acc_type = ResourceStatistics_cpus_nr_throttled_acc_type;

impl ::protobuf::reflect::FieldAccessor<ResourceStatistics> for ResourceStatistics_cpus_nr_throttled_acc_type {
    fn name(&self) -> &'static str {
        "cpus_nr_throttled"
    }

    fn has_field(&self, m: &ResourceStatistics) -> bool {
        m.has_cpus_nr_throttled()
    }

    fn get_u32(&self, m: &ResourceStatistics) -> u32 {
        m.get_cpus_nr_throttled()
    }
}

#[allow(non_camel_case_types)]
struct ResourceStatistics_cpus_throttled_time_secs_acc_type;
static ResourceStatistics_cpus_throttled_time_secs_acc: ResourceStatistics_cpus_throttled_time_secs_acc_type = ResourceStatistics_cpus_throttled_time_secs_acc_type;

impl ::protobuf::reflect::FieldAccessor<ResourceStatistics> for ResourceStatistics_cpus_throttled_time_secs_acc_type {
    fn name(&self) -> &'static str {
        "cpus_throttled_time_secs"
    }

    fn has_field(&self, m: &ResourceStatistics) -> bool {
        m.has_cpus_throttled_time_secs()
    }

    fn get_f64(&self, m: &ResourceStatistics) -> f64 {
        m.get_cpus_throttled_time_secs()
    }
}

#[allow(non_camel_case_types)]
struct ResourceStatistics_mem_rss_bytes_acc_type;
static ResourceStatistics_mem_rss_bytes_acc: ResourceStatistics_mem_rss_bytes_acc_type = ResourceStatistics_mem_rss_bytes_acc_type;

impl ::protobuf::reflect::FieldAccessor<ResourceStatistics> for ResourceStatistics_mem_rss_bytes_acc_type {
    fn name(&self) -> &'static str {
        "mem_rss_bytes"
    }

    fn has_field(&self, m: &ResourceStatistics) -> bool {
        m.has_mem_rss_bytes()
    }

    fn get_u64(&self, m: &ResourceStatistics) -> u64 {
        m.get_mem_rss_bytes()
    }
}

#[allow(non_camel_case_types)]
struct ResourceStatistics_mem_limit_bytes_acc_type;
static ResourceStatistics_mem_limit_bytes_acc: ResourceStatistics_mem_limit_bytes_acc_type = ResourceStatistics_mem_limit_bytes_acc_type;

impl ::protobuf::reflect::FieldAccessor<ResourceStatistics> for ResourceStatistics_mem_limit_bytes_acc_type {
    fn name(&self) -> &'static str {
        "mem_limit_bytes"
    }

    fn has_field(&self, m: &ResourceStatistics) -> bool {
        m.has_mem_limit_bytes()
    }

    fn get_u64(&self, m: &ResourceStatistics) -> u64 {
        m.get_mem_limit_bytes()
    }
}

#[allow(non_camel_case_types)]
struct ResourceStatistics_mem_file_bytes_acc_type;
static ResourceStatistics_mem_file_bytes_acc: ResourceStatistics_mem_file_bytes_acc_type = ResourceStatistics_mem_file_bytes_acc_type;

impl ::protobuf::reflect::FieldAccessor<ResourceStatistics> for ResourceStatistics_mem_file_bytes_acc_type {
    fn name(&self) -> &'static str {
        "mem_file_bytes"
    }

    fn has_field(&self, m: &ResourceStatistics) -> bool {
        m.has_mem_file_bytes()
    }

    fn get_u64(&self, m: &ResourceStatistics) -> u64 {
        m.get_mem_file_bytes()
    }
}

#[allow(non_camel_case_types)]
struct ResourceStatistics_mem_anon_bytes_acc_type;
static ResourceStatistics_mem_anon_bytes_acc: ResourceStatistics_mem_anon_bytes_acc_type = ResourceStatistics_mem_anon_bytes_acc_type;

impl ::protobuf::reflect::FieldAccessor<ResourceStatistics> for ResourceStatistics_mem_anon_bytes_acc_type {
    fn name(&self) -> &'static str {
        "mem_anon_bytes"
    }

    fn has_field(&self, m: &ResourceStatistics) -> bool {
        m.has_mem_anon_bytes()
    }

    fn get_u64(&self, m: &ResourceStatistics) -> u64 {
        m.get_mem_anon_bytes()
    }
}

#[allow(non_camel_case_types)]
struct ResourceStatistics_mem_mapped_file_bytes_acc_type;
static ResourceStatistics_mem_mapped_file_bytes_acc: ResourceStatistics_mem_mapped_file_bytes_acc_type = ResourceStatistics_mem_mapped_file_bytes_acc_type;

impl ::protobuf::reflect::FieldAccessor<ResourceStatistics> for ResourceStatistics_mem_mapped_file_bytes_acc_type {
    fn name(&self) -> &'static str {
        "mem_mapped_file_bytes"
    }

    fn has_field(&self, m: &ResourceStatistics) -> bool {
        m.has_mem_mapped_file_bytes()
    }

    fn get_u64(&self, m: &ResourceStatistics) -> u64 {
        m.get_mem_mapped_file_bytes()
    }
}

#[allow(non_camel_case_types)]
struct ResourceStatistics_perf_acc_type;
static ResourceStatistics_perf_acc: ResourceStatistics_perf_acc_type = ResourceStatistics_perf_acc_type;

impl ::protobuf::reflect::FieldAccessor<ResourceStatistics> for ResourceStatistics_perf_acc_type {
    fn name(&self) -> &'static str {
        "perf"
    }

    fn has_field(&self, m: &ResourceStatistics) -> bool {
        m.has_perf()
    }

    fn get_message<'a>(&self, m: &'a ResourceStatistics) -> &'a ::protobuf::Message {
        m.get_perf() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct ResourceStatistics_net_rx_packets_acc_type;
static ResourceStatistics_net_rx_packets_acc: ResourceStatistics_net_rx_packets_acc_type = ResourceStatistics_net_rx_packets_acc_type;

impl ::protobuf::reflect::FieldAccessor<ResourceStatistics> for ResourceStatistics_net_rx_packets_acc_type {
    fn name(&self) -> &'static str {
        "net_rx_packets"
    }

    fn has_field(&self, m: &ResourceStatistics) -> bool {
        m.has_net_rx_packets()
    }

    fn get_u64(&self, m: &ResourceStatistics) -> u64 {
        m.get_net_rx_packets()
    }
}

#[allow(non_camel_case_types)]
struct ResourceStatistics_net_rx_bytes_acc_type;
static ResourceStatistics_net_rx_bytes_acc: ResourceStatistics_net_rx_bytes_acc_type = ResourceStatistics_net_rx_bytes_acc_type;

impl ::protobuf::reflect::FieldAccessor<ResourceStatistics> for ResourceStatistics_net_rx_bytes_acc_type {
    fn name(&self) -> &'static str {
        "net_rx_bytes"
    }

    fn has_field(&self, m: &ResourceStatistics) -> bool {
        m.has_net_rx_bytes()
    }

    fn get_u64(&self, m: &ResourceStatistics) -> u64 {
        m.get_net_rx_bytes()
    }
}

#[allow(non_camel_case_types)]
struct ResourceStatistics_net_rx_errors_acc_type;
static ResourceStatistics_net_rx_errors_acc: ResourceStatistics_net_rx_errors_acc_type = ResourceStatistics_net_rx_errors_acc_type;

impl ::protobuf::reflect::FieldAccessor<ResourceStatistics> for ResourceStatistics_net_rx_errors_acc_type {
    fn name(&self) -> &'static str {
        "net_rx_errors"
    }

    fn has_field(&self, m: &ResourceStatistics) -> bool {
        m.has_net_rx_errors()
    }

    fn get_u64(&self, m: &ResourceStatistics) -> u64 {
        m.get_net_rx_errors()
    }
}

#[allow(non_camel_case_types)]
struct ResourceStatistics_net_rx_dropped_acc_type;
static ResourceStatistics_net_rx_dropped_acc: ResourceStatistics_net_rx_dropped_acc_type = ResourceStatistics_net_rx_dropped_acc_type;

impl ::protobuf::reflect::FieldAccessor<ResourceStatistics> for ResourceStatistics_net_rx_dropped_acc_type {
    fn name(&self) -> &'static str {
        "net_rx_dropped"
    }

    fn has_field(&self, m: &ResourceStatistics) -> bool {
        m.has_net_rx_dropped()
    }

    fn get_u64(&self, m: &ResourceStatistics) -> u64 {
        m.get_net_rx_dropped()
    }
}

#[allow(non_camel_case_types)]
struct ResourceStatistics_net_tx_packets_acc_type;
static ResourceStatistics_net_tx_packets_acc: ResourceStatistics_net_tx_packets_acc_type = ResourceStatistics_net_tx_packets_acc_type;

impl ::protobuf::reflect::FieldAccessor<ResourceStatistics> for ResourceStatistics_net_tx_packets_acc_type {
    fn name(&self) -> &'static str {
        "net_tx_packets"
    }

    fn has_field(&self, m: &ResourceStatistics) -> bool {
        m.has_net_tx_packets()
    }

    fn get_u64(&self, m: &ResourceStatistics) -> u64 {
        m.get_net_tx_packets()
    }
}

#[allow(non_camel_case_types)]
struct ResourceStatistics_net_tx_bytes_acc_type;
static ResourceStatistics_net_tx_bytes_acc: ResourceStatistics_net_tx_bytes_acc_type = ResourceStatistics_net_tx_bytes_acc_type;

impl ::protobuf::reflect::FieldAccessor<ResourceStatistics> for ResourceStatistics_net_tx_bytes_acc_type {
    fn name(&self) -> &'static str {
        "net_tx_bytes"
    }

    fn has_field(&self, m: &ResourceStatistics) -> bool {
        m.has_net_tx_bytes()
    }

    fn get_u64(&self, m: &ResourceStatistics) -> u64 {
        m.get_net_tx_bytes()
    }
}

#[allow(non_camel_case_types)]
struct ResourceStatistics_net_tx_errors_acc_type;
static ResourceStatistics_net_tx_errors_acc: ResourceStatistics_net_tx_errors_acc_type = ResourceStatistics_net_tx_errors_acc_type;

impl ::protobuf::reflect::FieldAccessor<ResourceStatistics> for ResourceStatistics_net_tx_errors_acc_type {
    fn name(&self) -> &'static str {
        "net_tx_errors"
    }

    fn has_field(&self, m: &ResourceStatistics) -> bool {
        m.has_net_tx_errors()
    }

    fn get_u64(&self, m: &ResourceStatistics) -> u64 {
        m.get_net_tx_errors()
    }
}

#[allow(non_camel_case_types)]
struct ResourceStatistics_net_tx_dropped_acc_type;
static ResourceStatistics_net_tx_dropped_acc: ResourceStatistics_net_tx_dropped_acc_type = ResourceStatistics_net_tx_dropped_acc_type;

impl ::protobuf::reflect::FieldAccessor<ResourceStatistics> for ResourceStatistics_net_tx_dropped_acc_type {
    fn name(&self) -> &'static str {
        "net_tx_dropped"
    }

    fn has_field(&self, m: &ResourceStatistics) -> bool {
        m.has_net_tx_dropped()
    }

    fn get_u64(&self, m: &ResourceStatistics) -> u64 {
        m.get_net_tx_dropped()
    }
}

#[allow(non_camel_case_types)]
struct ResourceStatistics_net_tcp_rtt_microsecs_p50_acc_type;
static ResourceStatistics_net_tcp_rtt_microsecs_p50_acc: ResourceStatistics_net_tcp_rtt_microsecs_p50_acc_type = ResourceStatistics_net_tcp_rtt_microsecs_p50_acc_type;

impl ::protobuf::reflect::FieldAccessor<ResourceStatistics> for ResourceStatistics_net_tcp_rtt_microsecs_p50_acc_type {
    fn name(&self) -> &'static str {
        "net_tcp_rtt_microsecs_p50"
    }

    fn has_field(&self, m: &ResourceStatistics) -> bool {
        m.has_net_tcp_rtt_microsecs_p50()
    }

    fn get_f64(&self, m: &ResourceStatistics) -> f64 {
        m.get_net_tcp_rtt_microsecs_p50()
    }
}

#[allow(non_camel_case_types)]
struct ResourceStatistics_net_tcp_rtt_microsecs_p90_acc_type;
static ResourceStatistics_net_tcp_rtt_microsecs_p90_acc: ResourceStatistics_net_tcp_rtt_microsecs_p90_acc_type = ResourceStatistics_net_tcp_rtt_microsecs_p90_acc_type;

impl ::protobuf::reflect::FieldAccessor<ResourceStatistics> for ResourceStatistics_net_tcp_rtt_microsecs_p90_acc_type {
    fn name(&self) -> &'static str {
        "net_tcp_rtt_microsecs_p90"
    }

    fn has_field(&self, m: &ResourceStatistics) -> bool {
        m.has_net_tcp_rtt_microsecs_p90()
    }

    fn get_f64(&self, m: &ResourceStatistics) -> f64 {
        m.get_net_tcp_rtt_microsecs_p90()
    }
}

#[allow(non_camel_case_types)]
struct ResourceStatistics_net_tcp_rtt_microsecs_p95_acc_type;
static ResourceStatistics_net_tcp_rtt_microsecs_p95_acc: ResourceStatistics_net_tcp_rtt_microsecs_p95_acc_type = ResourceStatistics_net_tcp_rtt_microsecs_p95_acc_type;

impl ::protobuf::reflect::FieldAccessor<ResourceStatistics> for ResourceStatistics_net_tcp_rtt_microsecs_p95_acc_type {
    fn name(&self) -> &'static str {
        "net_tcp_rtt_microsecs_p95"
    }

    fn has_field(&self, m: &ResourceStatistics) -> bool {
        m.has_net_tcp_rtt_microsecs_p95()
    }

    fn get_f64(&self, m: &ResourceStatistics) -> f64 {
        m.get_net_tcp_rtt_microsecs_p95()
    }
}

#[allow(non_camel_case_types)]
struct ResourceStatistics_net_tcp_rtt_microsecs_p99_acc_type;
static ResourceStatistics_net_tcp_rtt_microsecs_p99_acc: ResourceStatistics_net_tcp_rtt_microsecs_p99_acc_type = ResourceStatistics_net_tcp_rtt_microsecs_p99_acc_type;

impl ::protobuf::reflect::FieldAccessor<ResourceStatistics> for ResourceStatistics_net_tcp_rtt_microsecs_p99_acc_type {
    fn name(&self) -> &'static str {
        "net_tcp_rtt_microsecs_p99"
    }

    fn has_field(&self, m: &ResourceStatistics) -> bool {
        m.has_net_tcp_rtt_microsecs_p99()
    }

    fn get_f64(&self, m: &ResourceStatistics) -> f64 {
        m.get_net_tcp_rtt_microsecs_p99()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct ResourceUsage {
    slave_id: ::protobuf::SingularPtrField<SlaveID>,
    framework_id: ::protobuf::SingularPtrField<FrameworkID>,
    executor_id: ::protobuf::SingularPtrField<ExecutorID>,
    executor_name: ::protobuf::SingularField<::std::string::String>,
    task_id: ::protobuf::SingularPtrField<TaskID>,
    statistics: ::protobuf::SingularPtrField<ResourceStatistics>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> ResourceUsage {
    pub fn new() -> ResourceUsage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResourceUsage {
        static mut instance: ::protobuf::lazy::Lazy<ResourceUsage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResourceUsage,
        };
        unsafe {
            instance.get(|| {
                ResourceUsage {
                    slave_id: ::protobuf::SingularPtrField::none(),
                    framework_id: ::protobuf::SingularPtrField::none(),
                    executor_id: ::protobuf::SingularPtrField::none(),
                    executor_name: ::protobuf::SingularField::none(),
                    task_id: ::protobuf::SingularPtrField::none(),
                    statistics: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
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
    pub fn mut_slave_id(&'a mut self) -> &'a mut SlaveID {
        if self.slave_id.is_none() {
            self.slave_id.set_default();
        };
        self.slave_id.as_mut().unwrap()
    }

    pub fn get_slave_id(&'a self) -> &'a SlaveID {
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
    pub fn mut_framework_id(&'a mut self) -> &'a mut FrameworkID {
        if self.framework_id.is_none() {
            self.framework_id.set_default();
        };
        self.framework_id.as_mut().unwrap()
    }

    pub fn get_framework_id(&'a self) -> &'a FrameworkID {
        self.framework_id.as_ref().unwrap_or_else(|| FrameworkID::default_instance())
    }

    // optional .mesos.ExecutorID executor_id = 3;

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
    pub fn mut_executor_id(&'a mut self) -> &'a mut ExecutorID {
        if self.executor_id.is_none() {
            self.executor_id.set_default();
        };
        self.executor_id.as_mut().unwrap()
    }

    pub fn get_executor_id(&'a self) -> &'a ExecutorID {
        self.executor_id.as_ref().unwrap_or_else(|| ExecutorID::default_instance())
    }

    // optional string executor_name = 4;

    pub fn clear_executor_name(&mut self) {
        self.executor_name.clear();
    }

    pub fn has_executor_name(&self) -> bool {
        self.executor_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_executor_name(&mut self, v: ::std::string::String) {
        self.executor_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_executor_name(&'a mut self) -> &'a mut ::std::string::String {
        if self.executor_name.is_none() {
            self.executor_name.set_default();
        };
        self.executor_name.as_mut().unwrap()
    }

    pub fn get_executor_name(&'a self) -> &'a str {
        match self.executor_name.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // optional .mesos.TaskID task_id = 5;

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
    pub fn mut_task_id(&'a mut self) -> &'a mut TaskID {
        if self.task_id.is_none() {
            self.task_id.set_default();
        };
        self.task_id.as_mut().unwrap()
    }

    pub fn get_task_id(&'a self) -> &'a TaskID {
        self.task_id.as_ref().unwrap_or_else(|| TaskID::default_instance())
    }

    // optional .mesos.ResourceStatistics statistics = 6;

    pub fn clear_statistics(&mut self) {
        self.statistics.clear();
    }

    pub fn has_statistics(&self) -> bool {
        self.statistics.is_some()
    }

    // Param is passed by value, moved
    pub fn set_statistics(&mut self, v: ResourceStatistics) {
        self.statistics = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_statistics(&'a mut self) -> &'a mut ResourceStatistics {
        if self.statistics.is_none() {
            self.statistics.set_default();
        };
        self.statistics.as_mut().unwrap()
    }

    pub fn get_statistics(&'a self) -> &'a ResourceStatistics {
        self.statistics.as_ref().unwrap_or_else(|| ResourceStatistics::default_instance())
    }
}

impl ::protobuf::Message for ResourceUsage {
    fn new() -> ResourceUsage {
        ResourceUsage::new()
    }

    fn is_initialized(&self) -> bool {
        if self.slave_id.is_none() {
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
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.slave_id.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.framework_id.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.executor_id.set_default();
                    try!(is.merge_message(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.executor_name.set_default();
                    try!(is.read_string_into(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.task_id.set_default();
                    try!(is.merge_message(tmp))
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.statistics.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.slave_id.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.framework_id.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.executor_id.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.executor_name.iter() {
            my_size += ::protobuf::rt::string_size(4, value.as_slice());
        };
        for value in self.task_id.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.statistics.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.slave_id.as_ref() {
            Some(v) => {
                try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.framework_id.as_ref() {
            Some(v) => {
                try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.executor_id.as_ref() {
            Some(v) => {
                try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.executor_name.as_ref() {
            Some(v) => {
                try!(os.write_string(4, v.as_slice()));
            },
            None => {},
        };
        match self.task_id.as_ref() {
            Some(v) => {
                try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.statistics.as_ref() {
            Some(v) => {
                try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<ResourceUsage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<ResourceUsage>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&ResourceUsage_slave_id_acc as &'static ::protobuf::reflect::FieldAccessor<ResourceUsage>) });
                fields.push(unsafe { ::std::mem::transmute(&ResourceUsage_framework_id_acc as &'static ::protobuf::reflect::FieldAccessor<ResourceUsage>) });
                fields.push(unsafe { ::std::mem::transmute(&ResourceUsage_executor_id_acc as &'static ::protobuf::reflect::FieldAccessor<ResourceUsage>) });
                fields.push(unsafe { ::std::mem::transmute(&ResourceUsage_executor_name_acc as &'static ::protobuf::reflect::FieldAccessor<ResourceUsage>) });
                fields.push(unsafe { ::std::mem::transmute(&ResourceUsage_task_id_acc as &'static ::protobuf::reflect::FieldAccessor<ResourceUsage>) });
                fields.push(unsafe { ::std::mem::transmute(&ResourceUsage_statistics_acc as &'static ::protobuf::reflect::FieldAccessor<ResourceUsage>) });
                ::protobuf::reflect::MessageDescriptor::new::<ResourceUsage>(
                    "ResourceUsage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<ResourceUsage>()
    }
}

impl ::protobuf::Clear for ResourceUsage {
    fn clear(&mut self) {
        self.clear_slave_id();
        self.clear_framework_id();
        self.clear_executor_id();
        self.clear_executor_name();
        self.clear_task_id();
        self.clear_statistics();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for ResourceUsage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct ResourceUsage_slave_id_acc_type;
static ResourceUsage_slave_id_acc: ResourceUsage_slave_id_acc_type = ResourceUsage_slave_id_acc_type;

impl ::protobuf::reflect::FieldAccessor<ResourceUsage> for ResourceUsage_slave_id_acc_type {
    fn name(&self) -> &'static str {
        "slave_id"
    }

    fn has_field(&self, m: &ResourceUsage) -> bool {
        m.has_slave_id()
    }

    fn get_message<'a>(&self, m: &'a ResourceUsage) -> &'a ::protobuf::Message {
        m.get_slave_id() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct ResourceUsage_framework_id_acc_type;
static ResourceUsage_framework_id_acc: ResourceUsage_framework_id_acc_type = ResourceUsage_framework_id_acc_type;

impl ::protobuf::reflect::FieldAccessor<ResourceUsage> for ResourceUsage_framework_id_acc_type {
    fn name(&self) -> &'static str {
        "framework_id"
    }

    fn has_field(&self, m: &ResourceUsage) -> bool {
        m.has_framework_id()
    }

    fn get_message<'a>(&self, m: &'a ResourceUsage) -> &'a ::protobuf::Message {
        m.get_framework_id() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct ResourceUsage_executor_id_acc_type;
static ResourceUsage_executor_id_acc: ResourceUsage_executor_id_acc_type = ResourceUsage_executor_id_acc_type;

impl ::protobuf::reflect::FieldAccessor<ResourceUsage> for ResourceUsage_executor_id_acc_type {
    fn name(&self) -> &'static str {
        "executor_id"
    }

    fn has_field(&self, m: &ResourceUsage) -> bool {
        m.has_executor_id()
    }

    fn get_message<'a>(&self, m: &'a ResourceUsage) -> &'a ::protobuf::Message {
        m.get_executor_id() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct ResourceUsage_executor_name_acc_type;
static ResourceUsage_executor_name_acc: ResourceUsage_executor_name_acc_type = ResourceUsage_executor_name_acc_type;

impl ::protobuf::reflect::FieldAccessor<ResourceUsage> for ResourceUsage_executor_name_acc_type {
    fn name(&self) -> &'static str {
        "executor_name"
    }

    fn has_field(&self, m: &ResourceUsage) -> bool {
        m.has_executor_name()
    }

    fn get_str<'a>(&self, m: &'a ResourceUsage) -> &'a str {
        m.get_executor_name()
    }
}

#[allow(non_camel_case_types)]
struct ResourceUsage_task_id_acc_type;
static ResourceUsage_task_id_acc: ResourceUsage_task_id_acc_type = ResourceUsage_task_id_acc_type;

impl ::protobuf::reflect::FieldAccessor<ResourceUsage> for ResourceUsage_task_id_acc_type {
    fn name(&self) -> &'static str {
        "task_id"
    }

    fn has_field(&self, m: &ResourceUsage) -> bool {
        m.has_task_id()
    }

    fn get_message<'a>(&self, m: &'a ResourceUsage) -> &'a ::protobuf::Message {
        m.get_task_id() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct ResourceUsage_statistics_acc_type;
static ResourceUsage_statistics_acc: ResourceUsage_statistics_acc_type = ResourceUsage_statistics_acc_type;

impl ::protobuf::reflect::FieldAccessor<ResourceUsage> for ResourceUsage_statistics_acc_type {
    fn name(&self) -> &'static str {
        "statistics"
    }

    fn has_field(&self, m: &ResourceUsage) -> bool {
        m.has_statistics()
    }

    fn get_message<'a>(&self, m: &'a ResourceUsage) -> &'a ::protobuf::Message {
        m.get_statistics() as &'a ::protobuf::Message
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct PerfStatistics {
    timestamp: ::std::option::Option<f64>,
    duration: ::std::option::Option<f64>,
    cycles: ::std::option::Option<u64>,
    stalled_cycles_frontend: ::std::option::Option<u64>,
    stalled_cycles_backend: ::std::option::Option<u64>,
    instructions: ::std::option::Option<u64>,
    cache_references: ::std::option::Option<u64>,
    cache_misses: ::std::option::Option<u64>,
    branches: ::std::option::Option<u64>,
    branch_misses: ::std::option::Option<u64>,
    bus_cycles: ::std::option::Option<u64>,
    ref_cycles: ::std::option::Option<u64>,
    cpu_clock: ::std::option::Option<f64>,
    task_clock: ::std::option::Option<f64>,
    page_faults: ::std::option::Option<u64>,
    minor_faults: ::std::option::Option<u64>,
    major_faults: ::std::option::Option<u64>,
    context_switches: ::std::option::Option<u64>,
    cpu_migrations: ::std::option::Option<u64>,
    alignment_faults: ::std::option::Option<u64>,
    emulation_faults: ::std::option::Option<u64>,
    l1_dcache_loads: ::std::option::Option<u64>,
    l1_dcache_load_misses: ::std::option::Option<u64>,
    l1_dcache_stores: ::std::option::Option<u64>,
    l1_dcache_store_misses: ::std::option::Option<u64>,
    l1_dcache_prefetches: ::std::option::Option<u64>,
    l1_dcache_prefetch_misses: ::std::option::Option<u64>,
    l1_icache_loads: ::std::option::Option<u64>,
    l1_icache_load_misses: ::std::option::Option<u64>,
    l1_icache_prefetches: ::std::option::Option<u64>,
    l1_icache_prefetch_misses: ::std::option::Option<u64>,
    llc_loads: ::std::option::Option<u64>,
    llc_load_misses: ::std::option::Option<u64>,
    llc_stores: ::std::option::Option<u64>,
    llc_store_misses: ::std::option::Option<u64>,
    llc_prefetches: ::std::option::Option<u64>,
    llc_prefetch_misses: ::std::option::Option<u64>,
    dtlb_loads: ::std::option::Option<u64>,
    dtlb_load_misses: ::std::option::Option<u64>,
    dtlb_stores: ::std::option::Option<u64>,
    dtlb_store_misses: ::std::option::Option<u64>,
    dtlb_prefetches: ::std::option::Option<u64>,
    dtlb_prefetch_misses: ::std::option::Option<u64>,
    itlb_loads: ::std::option::Option<u64>,
    itlb_load_misses: ::std::option::Option<u64>,
    branch_loads: ::std::option::Option<u64>,
    branch_load_misses: ::std::option::Option<u64>,
    node_loads: ::std::option::Option<u64>,
    node_load_misses: ::std::option::Option<u64>,
    node_stores: ::std::option::Option<u64>,
    node_store_misses: ::std::option::Option<u64>,
    node_prefetches: ::std::option::Option<u64>,
    node_prefetch_misses: ::std::option::Option<u64>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> PerfStatistics {
    pub fn new() -> PerfStatistics {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PerfStatistics {
        static mut instance: ::protobuf::lazy::Lazy<PerfStatistics> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PerfStatistics,
        };
        unsafe {
            instance.get(|| {
                PerfStatistics {
                    timestamp: ::std::option::None,
                    duration: ::std::option::None,
                    cycles: ::std::option::None,
                    stalled_cycles_frontend: ::std::option::None,
                    stalled_cycles_backend: ::std::option::None,
                    instructions: ::std::option::None,
                    cache_references: ::std::option::None,
                    cache_misses: ::std::option::None,
                    branches: ::std::option::None,
                    branch_misses: ::std::option::None,
                    bus_cycles: ::std::option::None,
                    ref_cycles: ::std::option::None,
                    cpu_clock: ::std::option::None,
                    task_clock: ::std::option::None,
                    page_faults: ::std::option::None,
                    minor_faults: ::std::option::None,
                    major_faults: ::std::option::None,
                    context_switches: ::std::option::None,
                    cpu_migrations: ::std::option::None,
                    alignment_faults: ::std::option::None,
                    emulation_faults: ::std::option::None,
                    l1_dcache_loads: ::std::option::None,
                    l1_dcache_load_misses: ::std::option::None,
                    l1_dcache_stores: ::std::option::None,
                    l1_dcache_store_misses: ::std::option::None,
                    l1_dcache_prefetches: ::std::option::None,
                    l1_dcache_prefetch_misses: ::std::option::None,
                    l1_icache_loads: ::std::option::None,
                    l1_icache_load_misses: ::std::option::None,
                    l1_icache_prefetches: ::std::option::None,
                    l1_icache_prefetch_misses: ::std::option::None,
                    llc_loads: ::std::option::None,
                    llc_load_misses: ::std::option::None,
                    llc_stores: ::std::option::None,
                    llc_store_misses: ::std::option::None,
                    llc_prefetches: ::std::option::None,
                    llc_prefetch_misses: ::std::option::None,
                    dtlb_loads: ::std::option::None,
                    dtlb_load_misses: ::std::option::None,
                    dtlb_stores: ::std::option::None,
                    dtlb_store_misses: ::std::option::None,
                    dtlb_prefetches: ::std::option::None,
                    dtlb_prefetch_misses: ::std::option::None,
                    itlb_loads: ::std::option::None,
                    itlb_load_misses: ::std::option::None,
                    branch_loads: ::std::option::None,
                    branch_load_misses: ::std::option::None,
                    node_loads: ::std::option::None,
                    node_load_misses: ::std::option::None,
                    node_stores: ::std::option::None,
                    node_store_misses: ::std::option::None,
                    node_prefetches: ::std::option::None,
                    node_prefetch_misses: ::std::option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // required double timestamp = 1;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = None;
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: f64) {
        self.timestamp = Some(v);
    }

    pub fn get_timestamp(&self) -> f64 {
        self.timestamp.unwrap_or(0.)
    }

    // required double duration = 2;

    pub fn clear_duration(&mut self) {
        self.duration = None;
    }

    pub fn has_duration(&self) -> bool {
        self.duration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_duration(&mut self, v: f64) {
        self.duration = Some(v);
    }

    pub fn get_duration(&self) -> f64 {
        self.duration.unwrap_or(0.)
    }

    // optional uint64 cycles = 3;

    pub fn clear_cycles(&mut self) {
        self.cycles = None;
    }

    pub fn has_cycles(&self) -> bool {
        self.cycles.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cycles(&mut self, v: u64) {
        self.cycles = Some(v);
    }

    pub fn get_cycles(&self) -> u64 {
        self.cycles.unwrap_or(0)
    }

    // optional uint64 stalled_cycles_frontend = 4;

    pub fn clear_stalled_cycles_frontend(&mut self) {
        self.stalled_cycles_frontend = None;
    }

    pub fn has_stalled_cycles_frontend(&self) -> bool {
        self.stalled_cycles_frontend.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stalled_cycles_frontend(&mut self, v: u64) {
        self.stalled_cycles_frontend = Some(v);
    }

    pub fn get_stalled_cycles_frontend(&self) -> u64 {
        self.stalled_cycles_frontend.unwrap_or(0)
    }

    // optional uint64 stalled_cycles_backend = 5;

    pub fn clear_stalled_cycles_backend(&mut self) {
        self.stalled_cycles_backend = None;
    }

    pub fn has_stalled_cycles_backend(&self) -> bool {
        self.stalled_cycles_backend.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stalled_cycles_backend(&mut self, v: u64) {
        self.stalled_cycles_backend = Some(v);
    }

    pub fn get_stalled_cycles_backend(&self) -> u64 {
        self.stalled_cycles_backend.unwrap_or(0)
    }

    // optional uint64 instructions = 6;

    pub fn clear_instructions(&mut self) {
        self.instructions = None;
    }

    pub fn has_instructions(&self) -> bool {
        self.instructions.is_some()
    }

    // Param is passed by value, moved
    pub fn set_instructions(&mut self, v: u64) {
        self.instructions = Some(v);
    }

    pub fn get_instructions(&self) -> u64 {
        self.instructions.unwrap_or(0)
    }

    // optional uint64 cache_references = 7;

    pub fn clear_cache_references(&mut self) {
        self.cache_references = None;
    }

    pub fn has_cache_references(&self) -> bool {
        self.cache_references.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cache_references(&mut self, v: u64) {
        self.cache_references = Some(v);
    }

    pub fn get_cache_references(&self) -> u64 {
        self.cache_references.unwrap_or(0)
    }

    // optional uint64 cache_misses = 8;

    pub fn clear_cache_misses(&mut self) {
        self.cache_misses = None;
    }

    pub fn has_cache_misses(&self) -> bool {
        self.cache_misses.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cache_misses(&mut self, v: u64) {
        self.cache_misses = Some(v);
    }

    pub fn get_cache_misses(&self) -> u64 {
        self.cache_misses.unwrap_or(0)
    }

    // optional uint64 branches = 9;

    pub fn clear_branches(&mut self) {
        self.branches = None;
    }

    pub fn has_branches(&self) -> bool {
        self.branches.is_some()
    }

    // Param is passed by value, moved
    pub fn set_branches(&mut self, v: u64) {
        self.branches = Some(v);
    }

    pub fn get_branches(&self) -> u64 {
        self.branches.unwrap_or(0)
    }

    // optional uint64 branch_misses = 10;

    pub fn clear_branch_misses(&mut self) {
        self.branch_misses = None;
    }

    pub fn has_branch_misses(&self) -> bool {
        self.branch_misses.is_some()
    }

    // Param is passed by value, moved
    pub fn set_branch_misses(&mut self, v: u64) {
        self.branch_misses = Some(v);
    }

    pub fn get_branch_misses(&self) -> u64 {
        self.branch_misses.unwrap_or(0)
    }

    // optional uint64 bus_cycles = 11;

    pub fn clear_bus_cycles(&mut self) {
        self.bus_cycles = None;
    }

    pub fn has_bus_cycles(&self) -> bool {
        self.bus_cycles.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bus_cycles(&mut self, v: u64) {
        self.bus_cycles = Some(v);
    }

    pub fn get_bus_cycles(&self) -> u64 {
        self.bus_cycles.unwrap_or(0)
    }

    // optional uint64 ref_cycles = 12;

    pub fn clear_ref_cycles(&mut self) {
        self.ref_cycles = None;
    }

    pub fn has_ref_cycles(&self) -> bool {
        self.ref_cycles.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ref_cycles(&mut self, v: u64) {
        self.ref_cycles = Some(v);
    }

    pub fn get_ref_cycles(&self) -> u64 {
        self.ref_cycles.unwrap_or(0)
    }

    // optional double cpu_clock = 13;

    pub fn clear_cpu_clock(&mut self) {
        self.cpu_clock = None;
    }

    pub fn has_cpu_clock(&self) -> bool {
        self.cpu_clock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cpu_clock(&mut self, v: f64) {
        self.cpu_clock = Some(v);
    }

    pub fn get_cpu_clock(&self) -> f64 {
        self.cpu_clock.unwrap_or(0.)
    }

    // optional double task_clock = 14;

    pub fn clear_task_clock(&mut self) {
        self.task_clock = None;
    }

    pub fn has_task_clock(&self) -> bool {
        self.task_clock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_task_clock(&mut self, v: f64) {
        self.task_clock = Some(v);
    }

    pub fn get_task_clock(&self) -> f64 {
        self.task_clock.unwrap_or(0.)
    }

    // optional uint64 page_faults = 15;

    pub fn clear_page_faults(&mut self) {
        self.page_faults = None;
    }

    pub fn has_page_faults(&self) -> bool {
        self.page_faults.is_some()
    }

    // Param is passed by value, moved
    pub fn set_page_faults(&mut self, v: u64) {
        self.page_faults = Some(v);
    }

    pub fn get_page_faults(&self) -> u64 {
        self.page_faults.unwrap_or(0)
    }

    // optional uint64 minor_faults = 16;

    pub fn clear_minor_faults(&mut self) {
        self.minor_faults = None;
    }

    pub fn has_minor_faults(&self) -> bool {
        self.minor_faults.is_some()
    }

    // Param is passed by value, moved
    pub fn set_minor_faults(&mut self, v: u64) {
        self.minor_faults = Some(v);
    }

    pub fn get_minor_faults(&self) -> u64 {
        self.minor_faults.unwrap_or(0)
    }

    // optional uint64 major_faults = 17;

    pub fn clear_major_faults(&mut self) {
        self.major_faults = None;
    }

    pub fn has_major_faults(&self) -> bool {
        self.major_faults.is_some()
    }

    // Param is passed by value, moved
    pub fn set_major_faults(&mut self, v: u64) {
        self.major_faults = Some(v);
    }

    pub fn get_major_faults(&self) -> u64 {
        self.major_faults.unwrap_or(0)
    }

    // optional uint64 context_switches = 18;

    pub fn clear_context_switches(&mut self) {
        self.context_switches = None;
    }

    pub fn has_context_switches(&self) -> bool {
        self.context_switches.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context_switches(&mut self, v: u64) {
        self.context_switches = Some(v);
    }

    pub fn get_context_switches(&self) -> u64 {
        self.context_switches.unwrap_or(0)
    }

    // optional uint64 cpu_migrations = 19;

    pub fn clear_cpu_migrations(&mut self) {
        self.cpu_migrations = None;
    }

    pub fn has_cpu_migrations(&self) -> bool {
        self.cpu_migrations.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cpu_migrations(&mut self, v: u64) {
        self.cpu_migrations = Some(v);
    }

    pub fn get_cpu_migrations(&self) -> u64 {
        self.cpu_migrations.unwrap_or(0)
    }

    // optional uint64 alignment_faults = 20;

    pub fn clear_alignment_faults(&mut self) {
        self.alignment_faults = None;
    }

    pub fn has_alignment_faults(&self) -> bool {
        self.alignment_faults.is_some()
    }

    // Param is passed by value, moved
    pub fn set_alignment_faults(&mut self, v: u64) {
        self.alignment_faults = Some(v);
    }

    pub fn get_alignment_faults(&self) -> u64 {
        self.alignment_faults.unwrap_or(0)
    }

    // optional uint64 emulation_faults = 21;

    pub fn clear_emulation_faults(&mut self) {
        self.emulation_faults = None;
    }

    pub fn has_emulation_faults(&self) -> bool {
        self.emulation_faults.is_some()
    }

    // Param is passed by value, moved
    pub fn set_emulation_faults(&mut self, v: u64) {
        self.emulation_faults = Some(v);
    }

    pub fn get_emulation_faults(&self) -> u64 {
        self.emulation_faults.unwrap_or(0)
    }

    // optional uint64 l1_dcache_loads = 22;

    pub fn clear_l1_dcache_loads(&mut self) {
        self.l1_dcache_loads = None;
    }

    pub fn has_l1_dcache_loads(&self) -> bool {
        self.l1_dcache_loads.is_some()
    }

    // Param is passed by value, moved
    pub fn set_l1_dcache_loads(&mut self, v: u64) {
        self.l1_dcache_loads = Some(v);
    }

    pub fn get_l1_dcache_loads(&self) -> u64 {
        self.l1_dcache_loads.unwrap_or(0)
    }

    // optional uint64 l1_dcache_load_misses = 23;

    pub fn clear_l1_dcache_load_misses(&mut self) {
        self.l1_dcache_load_misses = None;
    }

    pub fn has_l1_dcache_load_misses(&self) -> bool {
        self.l1_dcache_load_misses.is_some()
    }

    // Param is passed by value, moved
    pub fn set_l1_dcache_load_misses(&mut self, v: u64) {
        self.l1_dcache_load_misses = Some(v);
    }

    pub fn get_l1_dcache_load_misses(&self) -> u64 {
        self.l1_dcache_load_misses.unwrap_or(0)
    }

    // optional uint64 l1_dcache_stores = 24;

    pub fn clear_l1_dcache_stores(&mut self) {
        self.l1_dcache_stores = None;
    }

    pub fn has_l1_dcache_stores(&self) -> bool {
        self.l1_dcache_stores.is_some()
    }

    // Param is passed by value, moved
    pub fn set_l1_dcache_stores(&mut self, v: u64) {
        self.l1_dcache_stores = Some(v);
    }

    pub fn get_l1_dcache_stores(&self) -> u64 {
        self.l1_dcache_stores.unwrap_or(0)
    }

    // optional uint64 l1_dcache_store_misses = 25;

    pub fn clear_l1_dcache_store_misses(&mut self) {
        self.l1_dcache_store_misses = None;
    }

    pub fn has_l1_dcache_store_misses(&self) -> bool {
        self.l1_dcache_store_misses.is_some()
    }

    // Param is passed by value, moved
    pub fn set_l1_dcache_store_misses(&mut self, v: u64) {
        self.l1_dcache_store_misses = Some(v);
    }

    pub fn get_l1_dcache_store_misses(&self) -> u64 {
        self.l1_dcache_store_misses.unwrap_or(0)
    }

    // optional uint64 l1_dcache_prefetches = 26;

    pub fn clear_l1_dcache_prefetches(&mut self) {
        self.l1_dcache_prefetches = None;
    }

    pub fn has_l1_dcache_prefetches(&self) -> bool {
        self.l1_dcache_prefetches.is_some()
    }

    // Param is passed by value, moved
    pub fn set_l1_dcache_prefetches(&mut self, v: u64) {
        self.l1_dcache_prefetches = Some(v);
    }

    pub fn get_l1_dcache_prefetches(&self) -> u64 {
        self.l1_dcache_prefetches.unwrap_or(0)
    }

    // optional uint64 l1_dcache_prefetch_misses = 27;

    pub fn clear_l1_dcache_prefetch_misses(&mut self) {
        self.l1_dcache_prefetch_misses = None;
    }

    pub fn has_l1_dcache_prefetch_misses(&self) -> bool {
        self.l1_dcache_prefetch_misses.is_some()
    }

    // Param is passed by value, moved
    pub fn set_l1_dcache_prefetch_misses(&mut self, v: u64) {
        self.l1_dcache_prefetch_misses = Some(v);
    }

    pub fn get_l1_dcache_prefetch_misses(&self) -> u64 {
        self.l1_dcache_prefetch_misses.unwrap_or(0)
    }

    // optional uint64 l1_icache_loads = 28;

    pub fn clear_l1_icache_loads(&mut self) {
        self.l1_icache_loads = None;
    }

    pub fn has_l1_icache_loads(&self) -> bool {
        self.l1_icache_loads.is_some()
    }

    // Param is passed by value, moved
    pub fn set_l1_icache_loads(&mut self, v: u64) {
        self.l1_icache_loads = Some(v);
    }

    pub fn get_l1_icache_loads(&self) -> u64 {
        self.l1_icache_loads.unwrap_or(0)
    }

    // optional uint64 l1_icache_load_misses = 29;

    pub fn clear_l1_icache_load_misses(&mut self) {
        self.l1_icache_load_misses = None;
    }

    pub fn has_l1_icache_load_misses(&self) -> bool {
        self.l1_icache_load_misses.is_some()
    }

    // Param is passed by value, moved
    pub fn set_l1_icache_load_misses(&mut self, v: u64) {
        self.l1_icache_load_misses = Some(v);
    }

    pub fn get_l1_icache_load_misses(&self) -> u64 {
        self.l1_icache_load_misses.unwrap_or(0)
    }

    // optional uint64 l1_icache_prefetches = 30;

    pub fn clear_l1_icache_prefetches(&mut self) {
        self.l1_icache_prefetches = None;
    }

    pub fn has_l1_icache_prefetches(&self) -> bool {
        self.l1_icache_prefetches.is_some()
    }

    // Param is passed by value, moved
    pub fn set_l1_icache_prefetches(&mut self, v: u64) {
        self.l1_icache_prefetches = Some(v);
    }

    pub fn get_l1_icache_prefetches(&self) -> u64 {
        self.l1_icache_prefetches.unwrap_or(0)
    }

    // optional uint64 l1_icache_prefetch_misses = 31;

    pub fn clear_l1_icache_prefetch_misses(&mut self) {
        self.l1_icache_prefetch_misses = None;
    }

    pub fn has_l1_icache_prefetch_misses(&self) -> bool {
        self.l1_icache_prefetch_misses.is_some()
    }

    // Param is passed by value, moved
    pub fn set_l1_icache_prefetch_misses(&mut self, v: u64) {
        self.l1_icache_prefetch_misses = Some(v);
    }

    pub fn get_l1_icache_prefetch_misses(&self) -> u64 {
        self.l1_icache_prefetch_misses.unwrap_or(0)
    }

    // optional uint64 llc_loads = 32;

    pub fn clear_llc_loads(&mut self) {
        self.llc_loads = None;
    }

    pub fn has_llc_loads(&self) -> bool {
        self.llc_loads.is_some()
    }

    // Param is passed by value, moved
    pub fn set_llc_loads(&mut self, v: u64) {
        self.llc_loads = Some(v);
    }

    pub fn get_llc_loads(&self) -> u64 {
        self.llc_loads.unwrap_or(0)
    }

    // optional uint64 llc_load_misses = 33;

    pub fn clear_llc_load_misses(&mut self) {
        self.llc_load_misses = None;
    }

    pub fn has_llc_load_misses(&self) -> bool {
        self.llc_load_misses.is_some()
    }

    // Param is passed by value, moved
    pub fn set_llc_load_misses(&mut self, v: u64) {
        self.llc_load_misses = Some(v);
    }

    pub fn get_llc_load_misses(&self) -> u64 {
        self.llc_load_misses.unwrap_or(0)
    }

    // optional uint64 llc_stores = 34;

    pub fn clear_llc_stores(&mut self) {
        self.llc_stores = None;
    }

    pub fn has_llc_stores(&self) -> bool {
        self.llc_stores.is_some()
    }

    // Param is passed by value, moved
    pub fn set_llc_stores(&mut self, v: u64) {
        self.llc_stores = Some(v);
    }

    pub fn get_llc_stores(&self) -> u64 {
        self.llc_stores.unwrap_or(0)
    }

    // optional uint64 llc_store_misses = 35;

    pub fn clear_llc_store_misses(&mut self) {
        self.llc_store_misses = None;
    }

    pub fn has_llc_store_misses(&self) -> bool {
        self.llc_store_misses.is_some()
    }

    // Param is passed by value, moved
    pub fn set_llc_store_misses(&mut self, v: u64) {
        self.llc_store_misses = Some(v);
    }

    pub fn get_llc_store_misses(&self) -> u64 {
        self.llc_store_misses.unwrap_or(0)
    }

    // optional uint64 llc_prefetches = 36;

    pub fn clear_llc_prefetches(&mut self) {
        self.llc_prefetches = None;
    }

    pub fn has_llc_prefetches(&self) -> bool {
        self.llc_prefetches.is_some()
    }

    // Param is passed by value, moved
    pub fn set_llc_prefetches(&mut self, v: u64) {
        self.llc_prefetches = Some(v);
    }

    pub fn get_llc_prefetches(&self) -> u64 {
        self.llc_prefetches.unwrap_or(0)
    }

    // optional uint64 llc_prefetch_misses = 37;

    pub fn clear_llc_prefetch_misses(&mut self) {
        self.llc_prefetch_misses = None;
    }

    pub fn has_llc_prefetch_misses(&self) -> bool {
        self.llc_prefetch_misses.is_some()
    }

    // Param is passed by value, moved
    pub fn set_llc_prefetch_misses(&mut self, v: u64) {
        self.llc_prefetch_misses = Some(v);
    }

    pub fn get_llc_prefetch_misses(&self) -> u64 {
        self.llc_prefetch_misses.unwrap_or(0)
    }

    // optional uint64 dtlb_loads = 38;

    pub fn clear_dtlb_loads(&mut self) {
        self.dtlb_loads = None;
    }

    pub fn has_dtlb_loads(&self) -> bool {
        self.dtlb_loads.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dtlb_loads(&mut self, v: u64) {
        self.dtlb_loads = Some(v);
    }

    pub fn get_dtlb_loads(&self) -> u64 {
        self.dtlb_loads.unwrap_or(0)
    }

    // optional uint64 dtlb_load_misses = 39;

    pub fn clear_dtlb_load_misses(&mut self) {
        self.dtlb_load_misses = None;
    }

    pub fn has_dtlb_load_misses(&self) -> bool {
        self.dtlb_load_misses.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dtlb_load_misses(&mut self, v: u64) {
        self.dtlb_load_misses = Some(v);
    }

    pub fn get_dtlb_load_misses(&self) -> u64 {
        self.dtlb_load_misses.unwrap_or(0)
    }

    // optional uint64 dtlb_stores = 40;

    pub fn clear_dtlb_stores(&mut self) {
        self.dtlb_stores = None;
    }

    pub fn has_dtlb_stores(&self) -> bool {
        self.dtlb_stores.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dtlb_stores(&mut self, v: u64) {
        self.dtlb_stores = Some(v);
    }

    pub fn get_dtlb_stores(&self) -> u64 {
        self.dtlb_stores.unwrap_or(0)
    }

    // optional uint64 dtlb_store_misses = 41;

    pub fn clear_dtlb_store_misses(&mut self) {
        self.dtlb_store_misses = None;
    }

    pub fn has_dtlb_store_misses(&self) -> bool {
        self.dtlb_store_misses.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dtlb_store_misses(&mut self, v: u64) {
        self.dtlb_store_misses = Some(v);
    }

    pub fn get_dtlb_store_misses(&self) -> u64 {
        self.dtlb_store_misses.unwrap_or(0)
    }

    // optional uint64 dtlb_prefetches = 42;

    pub fn clear_dtlb_prefetches(&mut self) {
        self.dtlb_prefetches = None;
    }

    pub fn has_dtlb_prefetches(&self) -> bool {
        self.dtlb_prefetches.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dtlb_prefetches(&mut self, v: u64) {
        self.dtlb_prefetches = Some(v);
    }

    pub fn get_dtlb_prefetches(&self) -> u64 {
        self.dtlb_prefetches.unwrap_or(0)
    }

    // optional uint64 dtlb_prefetch_misses = 43;

    pub fn clear_dtlb_prefetch_misses(&mut self) {
        self.dtlb_prefetch_misses = None;
    }

    pub fn has_dtlb_prefetch_misses(&self) -> bool {
        self.dtlb_prefetch_misses.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dtlb_prefetch_misses(&mut self, v: u64) {
        self.dtlb_prefetch_misses = Some(v);
    }

    pub fn get_dtlb_prefetch_misses(&self) -> u64 {
        self.dtlb_prefetch_misses.unwrap_or(0)
    }

    // optional uint64 itlb_loads = 44;

    pub fn clear_itlb_loads(&mut self) {
        self.itlb_loads = None;
    }

    pub fn has_itlb_loads(&self) -> bool {
        self.itlb_loads.is_some()
    }

    // Param is passed by value, moved
    pub fn set_itlb_loads(&mut self, v: u64) {
        self.itlb_loads = Some(v);
    }

    pub fn get_itlb_loads(&self) -> u64 {
        self.itlb_loads.unwrap_or(0)
    }

    // optional uint64 itlb_load_misses = 45;

    pub fn clear_itlb_load_misses(&mut self) {
        self.itlb_load_misses = None;
    }

    pub fn has_itlb_load_misses(&self) -> bool {
        self.itlb_load_misses.is_some()
    }

    // Param is passed by value, moved
    pub fn set_itlb_load_misses(&mut self, v: u64) {
        self.itlb_load_misses = Some(v);
    }

    pub fn get_itlb_load_misses(&self) -> u64 {
        self.itlb_load_misses.unwrap_or(0)
    }

    // optional uint64 branch_loads = 46;

    pub fn clear_branch_loads(&mut self) {
        self.branch_loads = None;
    }

    pub fn has_branch_loads(&self) -> bool {
        self.branch_loads.is_some()
    }

    // Param is passed by value, moved
    pub fn set_branch_loads(&mut self, v: u64) {
        self.branch_loads = Some(v);
    }

    pub fn get_branch_loads(&self) -> u64 {
        self.branch_loads.unwrap_or(0)
    }

    // optional uint64 branch_load_misses = 47;

    pub fn clear_branch_load_misses(&mut self) {
        self.branch_load_misses = None;
    }

    pub fn has_branch_load_misses(&self) -> bool {
        self.branch_load_misses.is_some()
    }

    // Param is passed by value, moved
    pub fn set_branch_load_misses(&mut self, v: u64) {
        self.branch_load_misses = Some(v);
    }

    pub fn get_branch_load_misses(&self) -> u64 {
        self.branch_load_misses.unwrap_or(0)
    }

    // optional uint64 node_loads = 48;

    pub fn clear_node_loads(&mut self) {
        self.node_loads = None;
    }

    pub fn has_node_loads(&self) -> bool {
        self.node_loads.is_some()
    }

    // Param is passed by value, moved
    pub fn set_node_loads(&mut self, v: u64) {
        self.node_loads = Some(v);
    }

    pub fn get_node_loads(&self) -> u64 {
        self.node_loads.unwrap_or(0)
    }

    // optional uint64 node_load_misses = 49;

    pub fn clear_node_load_misses(&mut self) {
        self.node_load_misses = None;
    }

    pub fn has_node_load_misses(&self) -> bool {
        self.node_load_misses.is_some()
    }

    // Param is passed by value, moved
    pub fn set_node_load_misses(&mut self, v: u64) {
        self.node_load_misses = Some(v);
    }

    pub fn get_node_load_misses(&self) -> u64 {
        self.node_load_misses.unwrap_or(0)
    }

    // optional uint64 node_stores = 50;

    pub fn clear_node_stores(&mut self) {
        self.node_stores = None;
    }

    pub fn has_node_stores(&self) -> bool {
        self.node_stores.is_some()
    }

    // Param is passed by value, moved
    pub fn set_node_stores(&mut self, v: u64) {
        self.node_stores = Some(v);
    }

    pub fn get_node_stores(&self) -> u64 {
        self.node_stores.unwrap_or(0)
    }

    // optional uint64 node_store_misses = 51;

    pub fn clear_node_store_misses(&mut self) {
        self.node_store_misses = None;
    }

    pub fn has_node_store_misses(&self) -> bool {
        self.node_store_misses.is_some()
    }

    // Param is passed by value, moved
    pub fn set_node_store_misses(&mut self, v: u64) {
        self.node_store_misses = Some(v);
    }

    pub fn get_node_store_misses(&self) -> u64 {
        self.node_store_misses.unwrap_or(0)
    }

    // optional uint64 node_prefetches = 52;

    pub fn clear_node_prefetches(&mut self) {
        self.node_prefetches = None;
    }

    pub fn has_node_prefetches(&self) -> bool {
        self.node_prefetches.is_some()
    }

    // Param is passed by value, moved
    pub fn set_node_prefetches(&mut self, v: u64) {
        self.node_prefetches = Some(v);
    }

    pub fn get_node_prefetches(&self) -> u64 {
        self.node_prefetches.unwrap_or(0)
    }

    // optional uint64 node_prefetch_misses = 53;

    pub fn clear_node_prefetch_misses(&mut self) {
        self.node_prefetch_misses = None;
    }

    pub fn has_node_prefetch_misses(&self) -> bool {
        self.node_prefetch_misses.is_some()
    }

    // Param is passed by value, moved
    pub fn set_node_prefetch_misses(&mut self, v: u64) {
        self.node_prefetch_misses = Some(v);
    }

    pub fn get_node_prefetch_misses(&self) -> u64 {
        self.node_prefetch_misses.unwrap_or(0)
    }
}

impl ::protobuf::Message for PerfStatistics {
    fn new() -> PerfStatistics {
        PerfStatistics::new()
    }

    fn is_initialized(&self) -> bool {
        if self.timestamp.is_none() {
            return false;
        };
        if self.duration.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.timestamp = Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.duration = Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.cycles = Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.stalled_cycles_frontend = Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.stalled_cycles_backend = Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.instructions = Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.cache_references = Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.cache_misses = Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.branches = Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.branch_misses = Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.bus_cycles = Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.ref_cycles = Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.cpu_clock = Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.task_clock = Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.page_faults = Some(tmp);
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.minor_faults = Some(tmp);
                },
                17 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.major_faults = Some(tmp);
                },
                18 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.context_switches = Some(tmp);
                },
                19 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.cpu_migrations = Some(tmp);
                },
                20 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.alignment_faults = Some(tmp);
                },
                21 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.emulation_faults = Some(tmp);
                },
                22 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.l1_dcache_loads = Some(tmp);
                },
                23 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.l1_dcache_load_misses = Some(tmp);
                },
                24 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.l1_dcache_stores = Some(tmp);
                },
                25 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.l1_dcache_store_misses = Some(tmp);
                },
                26 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.l1_dcache_prefetches = Some(tmp);
                },
                27 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.l1_dcache_prefetch_misses = Some(tmp);
                },
                28 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.l1_icache_loads = Some(tmp);
                },
                29 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.l1_icache_load_misses = Some(tmp);
                },
                30 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.l1_icache_prefetches = Some(tmp);
                },
                31 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.l1_icache_prefetch_misses = Some(tmp);
                },
                32 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.llc_loads = Some(tmp);
                },
                33 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.llc_load_misses = Some(tmp);
                },
                34 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.llc_stores = Some(tmp);
                },
                35 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.llc_store_misses = Some(tmp);
                },
                36 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.llc_prefetches = Some(tmp);
                },
                37 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.llc_prefetch_misses = Some(tmp);
                },
                38 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.dtlb_loads = Some(tmp);
                },
                39 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.dtlb_load_misses = Some(tmp);
                },
                40 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.dtlb_stores = Some(tmp);
                },
                41 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.dtlb_store_misses = Some(tmp);
                },
                42 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.dtlb_prefetches = Some(tmp);
                },
                43 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.dtlb_prefetch_misses = Some(tmp);
                },
                44 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.itlb_loads = Some(tmp);
                },
                45 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.itlb_load_misses = Some(tmp);
                },
                46 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.branch_loads = Some(tmp);
                },
                47 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.branch_load_misses = Some(tmp);
                },
                48 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.node_loads = Some(tmp);
                },
                49 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.node_load_misses = Some(tmp);
                },
                50 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.node_stores = Some(tmp);
                },
                51 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.node_store_misses = Some(tmp);
                },
                52 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.node_prefetches = Some(tmp);
                },
                53 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.node_prefetch_misses = Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        if self.timestamp.is_some() {
            my_size += 9;
        };
        if self.duration.is_some() {
            my_size += 9;
        };
        for value in self.cycles.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.stalled_cycles_frontend.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.stalled_cycles_backend.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.instructions.iter() {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.cache_references.iter() {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.cache_misses.iter() {
            my_size += ::protobuf::rt::value_size(8, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.branches.iter() {
            my_size += ::protobuf::rt::value_size(9, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.branch_misses.iter() {
            my_size += ::protobuf::rt::value_size(10, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.bus_cycles.iter() {
            my_size += ::protobuf::rt::value_size(11, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.ref_cycles.iter() {
            my_size += ::protobuf::rt::value_size(12, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.cpu_clock.is_some() {
            my_size += 9;
        };
        if self.task_clock.is_some() {
            my_size += 9;
        };
        for value in self.page_faults.iter() {
            my_size += ::protobuf::rt::value_size(15, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.minor_faults.iter() {
            my_size += ::protobuf::rt::value_size(16, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.major_faults.iter() {
            my_size += ::protobuf::rt::value_size(17, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.context_switches.iter() {
            my_size += ::protobuf::rt::value_size(18, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.cpu_migrations.iter() {
            my_size += ::protobuf::rt::value_size(19, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.alignment_faults.iter() {
            my_size += ::protobuf::rt::value_size(20, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.emulation_faults.iter() {
            my_size += ::protobuf::rt::value_size(21, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.l1_dcache_loads.iter() {
            my_size += ::protobuf::rt::value_size(22, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.l1_dcache_load_misses.iter() {
            my_size += ::protobuf::rt::value_size(23, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.l1_dcache_stores.iter() {
            my_size += ::protobuf::rt::value_size(24, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.l1_dcache_store_misses.iter() {
            my_size += ::protobuf::rt::value_size(25, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.l1_dcache_prefetches.iter() {
            my_size += ::protobuf::rt::value_size(26, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.l1_dcache_prefetch_misses.iter() {
            my_size += ::protobuf::rt::value_size(27, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.l1_icache_loads.iter() {
            my_size += ::protobuf::rt::value_size(28, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.l1_icache_load_misses.iter() {
            my_size += ::protobuf::rt::value_size(29, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.l1_icache_prefetches.iter() {
            my_size += ::protobuf::rt::value_size(30, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.l1_icache_prefetch_misses.iter() {
            my_size += ::protobuf::rt::value_size(31, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.llc_loads.iter() {
            my_size += ::protobuf::rt::value_size(32, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.llc_load_misses.iter() {
            my_size += ::protobuf::rt::value_size(33, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.llc_stores.iter() {
            my_size += ::protobuf::rt::value_size(34, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.llc_store_misses.iter() {
            my_size += ::protobuf::rt::value_size(35, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.llc_prefetches.iter() {
            my_size += ::protobuf::rt::value_size(36, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.llc_prefetch_misses.iter() {
            my_size += ::protobuf::rt::value_size(37, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.dtlb_loads.iter() {
            my_size += ::protobuf::rt::value_size(38, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.dtlb_load_misses.iter() {
            my_size += ::protobuf::rt::value_size(39, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.dtlb_stores.iter() {
            my_size += ::protobuf::rt::value_size(40, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.dtlb_store_misses.iter() {
            my_size += ::protobuf::rt::value_size(41, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.dtlb_prefetches.iter() {
            my_size += ::protobuf::rt::value_size(42, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.dtlb_prefetch_misses.iter() {
            my_size += ::protobuf::rt::value_size(43, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.itlb_loads.iter() {
            my_size += ::protobuf::rt::value_size(44, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.itlb_load_misses.iter() {
            my_size += ::protobuf::rt::value_size(45, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.branch_loads.iter() {
            my_size += ::protobuf::rt::value_size(46, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.branch_load_misses.iter() {
            my_size += ::protobuf::rt::value_size(47, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.node_loads.iter() {
            my_size += ::protobuf::rt::value_size(48, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.node_load_misses.iter() {
            my_size += ::protobuf::rt::value_size(49, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.node_stores.iter() {
            my_size += ::protobuf::rt::value_size(50, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.node_store_misses.iter() {
            my_size += ::protobuf::rt::value_size(51, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.node_prefetches.iter() {
            my_size += ::protobuf::rt::value_size(52, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.node_prefetch_misses.iter() {
            my_size += ::protobuf::rt::value_size(53, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.timestamp {
            Some(v) => {
                try!(os.write_double(1, v));
            },
            None => {},
        };
        match self.duration {
            Some(v) => {
                try!(os.write_double(2, v));
            },
            None => {},
        };
        match self.cycles {
            Some(v) => {
                try!(os.write_uint64(3, v));
            },
            None => {},
        };
        match self.stalled_cycles_frontend {
            Some(v) => {
                try!(os.write_uint64(4, v));
            },
            None => {},
        };
        match self.stalled_cycles_backend {
            Some(v) => {
                try!(os.write_uint64(5, v));
            },
            None => {},
        };
        match self.instructions {
            Some(v) => {
                try!(os.write_uint64(6, v));
            },
            None => {},
        };
        match self.cache_references {
            Some(v) => {
                try!(os.write_uint64(7, v));
            },
            None => {},
        };
        match self.cache_misses {
            Some(v) => {
                try!(os.write_uint64(8, v));
            },
            None => {},
        };
        match self.branches {
            Some(v) => {
                try!(os.write_uint64(9, v));
            },
            None => {},
        };
        match self.branch_misses {
            Some(v) => {
                try!(os.write_uint64(10, v));
            },
            None => {},
        };
        match self.bus_cycles {
            Some(v) => {
                try!(os.write_uint64(11, v));
            },
            None => {},
        };
        match self.ref_cycles {
            Some(v) => {
                try!(os.write_uint64(12, v));
            },
            None => {},
        };
        match self.cpu_clock {
            Some(v) => {
                try!(os.write_double(13, v));
            },
            None => {},
        };
        match self.task_clock {
            Some(v) => {
                try!(os.write_double(14, v));
            },
            None => {},
        };
        match self.page_faults {
            Some(v) => {
                try!(os.write_uint64(15, v));
            },
            None => {},
        };
        match self.minor_faults {
            Some(v) => {
                try!(os.write_uint64(16, v));
            },
            None => {},
        };
        match self.major_faults {
            Some(v) => {
                try!(os.write_uint64(17, v));
            },
            None => {},
        };
        match self.context_switches {
            Some(v) => {
                try!(os.write_uint64(18, v));
            },
            None => {},
        };
        match self.cpu_migrations {
            Some(v) => {
                try!(os.write_uint64(19, v));
            },
            None => {},
        };
        match self.alignment_faults {
            Some(v) => {
                try!(os.write_uint64(20, v));
            },
            None => {},
        };
        match self.emulation_faults {
            Some(v) => {
                try!(os.write_uint64(21, v));
            },
            None => {},
        };
        match self.l1_dcache_loads {
            Some(v) => {
                try!(os.write_uint64(22, v));
            },
            None => {},
        };
        match self.l1_dcache_load_misses {
            Some(v) => {
                try!(os.write_uint64(23, v));
            },
            None => {},
        };
        match self.l1_dcache_stores {
            Some(v) => {
                try!(os.write_uint64(24, v));
            },
            None => {},
        };
        match self.l1_dcache_store_misses {
            Some(v) => {
                try!(os.write_uint64(25, v));
            },
            None => {},
        };
        match self.l1_dcache_prefetches {
            Some(v) => {
                try!(os.write_uint64(26, v));
            },
            None => {},
        };
        match self.l1_dcache_prefetch_misses {
            Some(v) => {
                try!(os.write_uint64(27, v));
            },
            None => {},
        };
        match self.l1_icache_loads {
            Some(v) => {
                try!(os.write_uint64(28, v));
            },
            None => {},
        };
        match self.l1_icache_load_misses {
            Some(v) => {
                try!(os.write_uint64(29, v));
            },
            None => {},
        };
        match self.l1_icache_prefetches {
            Some(v) => {
                try!(os.write_uint64(30, v));
            },
            None => {},
        };
        match self.l1_icache_prefetch_misses {
            Some(v) => {
                try!(os.write_uint64(31, v));
            },
            None => {},
        };
        match self.llc_loads {
            Some(v) => {
                try!(os.write_uint64(32, v));
            },
            None => {},
        };
        match self.llc_load_misses {
            Some(v) => {
                try!(os.write_uint64(33, v));
            },
            None => {},
        };
        match self.llc_stores {
            Some(v) => {
                try!(os.write_uint64(34, v));
            },
            None => {},
        };
        match self.llc_store_misses {
            Some(v) => {
                try!(os.write_uint64(35, v));
            },
            None => {},
        };
        match self.llc_prefetches {
            Some(v) => {
                try!(os.write_uint64(36, v));
            },
            None => {},
        };
        match self.llc_prefetch_misses {
            Some(v) => {
                try!(os.write_uint64(37, v));
            },
            None => {},
        };
        match self.dtlb_loads {
            Some(v) => {
                try!(os.write_uint64(38, v));
            },
            None => {},
        };
        match self.dtlb_load_misses {
            Some(v) => {
                try!(os.write_uint64(39, v));
            },
            None => {},
        };
        match self.dtlb_stores {
            Some(v) => {
                try!(os.write_uint64(40, v));
            },
            None => {},
        };
        match self.dtlb_store_misses {
            Some(v) => {
                try!(os.write_uint64(41, v));
            },
            None => {},
        };
        match self.dtlb_prefetches {
            Some(v) => {
                try!(os.write_uint64(42, v));
            },
            None => {},
        };
        match self.dtlb_prefetch_misses {
            Some(v) => {
                try!(os.write_uint64(43, v));
            },
            None => {},
        };
        match self.itlb_loads {
            Some(v) => {
                try!(os.write_uint64(44, v));
            },
            None => {},
        };
        match self.itlb_load_misses {
            Some(v) => {
                try!(os.write_uint64(45, v));
            },
            None => {},
        };
        match self.branch_loads {
            Some(v) => {
                try!(os.write_uint64(46, v));
            },
            None => {},
        };
        match self.branch_load_misses {
            Some(v) => {
                try!(os.write_uint64(47, v));
            },
            None => {},
        };
        match self.node_loads {
            Some(v) => {
                try!(os.write_uint64(48, v));
            },
            None => {},
        };
        match self.node_load_misses {
            Some(v) => {
                try!(os.write_uint64(49, v));
            },
            None => {},
        };
        match self.node_stores {
            Some(v) => {
                try!(os.write_uint64(50, v));
            },
            None => {},
        };
        match self.node_store_misses {
            Some(v) => {
                try!(os.write_uint64(51, v));
            },
            None => {},
        };
        match self.node_prefetches {
            Some(v) => {
                try!(os.write_uint64(52, v));
            },
            None => {},
        };
        match self.node_prefetch_misses {
            Some(v) => {
                try!(os.write_uint64(53, v));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<PerfStatistics>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<PerfStatistics>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_timestamp_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_duration_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_cycles_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_stalled_cycles_frontend_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_stalled_cycles_backend_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_instructions_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_cache_references_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_cache_misses_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_branches_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_branch_misses_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_bus_cycles_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_ref_cycles_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_cpu_clock_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_task_clock_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_page_faults_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_minor_faults_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_major_faults_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_context_switches_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_cpu_migrations_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_alignment_faults_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_emulation_faults_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_l1_dcache_loads_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_l1_dcache_load_misses_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_l1_dcache_stores_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_l1_dcache_store_misses_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_l1_dcache_prefetches_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_l1_dcache_prefetch_misses_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_l1_icache_loads_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_l1_icache_load_misses_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_l1_icache_prefetches_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_l1_icache_prefetch_misses_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_llc_loads_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_llc_load_misses_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_llc_stores_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_llc_store_misses_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_llc_prefetches_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_llc_prefetch_misses_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_dtlb_loads_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_dtlb_load_misses_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_dtlb_stores_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_dtlb_store_misses_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_dtlb_prefetches_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_dtlb_prefetch_misses_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_itlb_loads_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_itlb_load_misses_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_branch_loads_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_branch_load_misses_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_node_loads_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_node_load_misses_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_node_stores_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_node_store_misses_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_node_prefetches_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                fields.push(unsafe { ::std::mem::transmute(&PerfStatistics_node_prefetch_misses_acc as &'static ::protobuf::reflect::FieldAccessor<PerfStatistics>) });
                ::protobuf::reflect::MessageDescriptor::new::<PerfStatistics>(
                    "PerfStatistics",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<PerfStatistics>()
    }
}

impl ::protobuf::Clear for PerfStatistics {
    fn clear(&mut self) {
        self.clear_timestamp();
        self.clear_duration();
        self.clear_cycles();
        self.clear_stalled_cycles_frontend();
        self.clear_stalled_cycles_backend();
        self.clear_instructions();
        self.clear_cache_references();
        self.clear_cache_misses();
        self.clear_branches();
        self.clear_branch_misses();
        self.clear_bus_cycles();
        self.clear_ref_cycles();
        self.clear_cpu_clock();
        self.clear_task_clock();
        self.clear_page_faults();
        self.clear_minor_faults();
        self.clear_major_faults();
        self.clear_context_switches();
        self.clear_cpu_migrations();
        self.clear_alignment_faults();
        self.clear_emulation_faults();
        self.clear_l1_dcache_loads();
        self.clear_l1_dcache_load_misses();
        self.clear_l1_dcache_stores();
        self.clear_l1_dcache_store_misses();
        self.clear_l1_dcache_prefetches();
        self.clear_l1_dcache_prefetch_misses();
        self.clear_l1_icache_loads();
        self.clear_l1_icache_load_misses();
        self.clear_l1_icache_prefetches();
        self.clear_l1_icache_prefetch_misses();
        self.clear_llc_loads();
        self.clear_llc_load_misses();
        self.clear_llc_stores();
        self.clear_llc_store_misses();
        self.clear_llc_prefetches();
        self.clear_llc_prefetch_misses();
        self.clear_dtlb_loads();
        self.clear_dtlb_load_misses();
        self.clear_dtlb_stores();
        self.clear_dtlb_store_misses();
        self.clear_dtlb_prefetches();
        self.clear_dtlb_prefetch_misses();
        self.clear_itlb_loads();
        self.clear_itlb_load_misses();
        self.clear_branch_loads();
        self.clear_branch_load_misses();
        self.clear_node_loads();
        self.clear_node_load_misses();
        self.clear_node_stores();
        self.clear_node_store_misses();
        self.clear_node_prefetches();
        self.clear_node_prefetch_misses();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for PerfStatistics {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct PerfStatistics_timestamp_acc_type;
static PerfStatistics_timestamp_acc: PerfStatistics_timestamp_acc_type = PerfStatistics_timestamp_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_timestamp_acc_type {
    fn name(&self) -> &'static str {
        "timestamp"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_timestamp()
    }

    fn get_f64(&self, m: &PerfStatistics) -> f64 {
        m.get_timestamp()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_duration_acc_type;
static PerfStatistics_duration_acc: PerfStatistics_duration_acc_type = PerfStatistics_duration_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_duration_acc_type {
    fn name(&self) -> &'static str {
        "duration"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_duration()
    }

    fn get_f64(&self, m: &PerfStatistics) -> f64 {
        m.get_duration()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_cycles_acc_type;
static PerfStatistics_cycles_acc: PerfStatistics_cycles_acc_type = PerfStatistics_cycles_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_cycles_acc_type {
    fn name(&self) -> &'static str {
        "cycles"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_cycles()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_cycles()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_stalled_cycles_frontend_acc_type;
static PerfStatistics_stalled_cycles_frontend_acc: PerfStatistics_stalled_cycles_frontend_acc_type = PerfStatistics_stalled_cycles_frontend_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_stalled_cycles_frontend_acc_type {
    fn name(&self) -> &'static str {
        "stalled_cycles_frontend"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_stalled_cycles_frontend()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_stalled_cycles_frontend()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_stalled_cycles_backend_acc_type;
static PerfStatistics_stalled_cycles_backend_acc: PerfStatistics_stalled_cycles_backend_acc_type = PerfStatistics_stalled_cycles_backend_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_stalled_cycles_backend_acc_type {
    fn name(&self) -> &'static str {
        "stalled_cycles_backend"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_stalled_cycles_backend()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_stalled_cycles_backend()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_instructions_acc_type;
static PerfStatistics_instructions_acc: PerfStatistics_instructions_acc_type = PerfStatistics_instructions_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_instructions_acc_type {
    fn name(&self) -> &'static str {
        "instructions"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_instructions()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_instructions()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_cache_references_acc_type;
static PerfStatistics_cache_references_acc: PerfStatistics_cache_references_acc_type = PerfStatistics_cache_references_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_cache_references_acc_type {
    fn name(&self) -> &'static str {
        "cache_references"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_cache_references()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_cache_references()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_cache_misses_acc_type;
static PerfStatistics_cache_misses_acc: PerfStatistics_cache_misses_acc_type = PerfStatistics_cache_misses_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_cache_misses_acc_type {
    fn name(&self) -> &'static str {
        "cache_misses"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_cache_misses()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_cache_misses()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_branches_acc_type;
static PerfStatistics_branches_acc: PerfStatistics_branches_acc_type = PerfStatistics_branches_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_branches_acc_type {
    fn name(&self) -> &'static str {
        "branches"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_branches()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_branches()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_branch_misses_acc_type;
static PerfStatistics_branch_misses_acc: PerfStatistics_branch_misses_acc_type = PerfStatistics_branch_misses_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_branch_misses_acc_type {
    fn name(&self) -> &'static str {
        "branch_misses"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_branch_misses()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_branch_misses()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_bus_cycles_acc_type;
static PerfStatistics_bus_cycles_acc: PerfStatistics_bus_cycles_acc_type = PerfStatistics_bus_cycles_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_bus_cycles_acc_type {
    fn name(&self) -> &'static str {
        "bus_cycles"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_bus_cycles()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_bus_cycles()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_ref_cycles_acc_type;
static PerfStatistics_ref_cycles_acc: PerfStatistics_ref_cycles_acc_type = PerfStatistics_ref_cycles_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_ref_cycles_acc_type {
    fn name(&self) -> &'static str {
        "ref_cycles"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_ref_cycles()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_ref_cycles()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_cpu_clock_acc_type;
static PerfStatistics_cpu_clock_acc: PerfStatistics_cpu_clock_acc_type = PerfStatistics_cpu_clock_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_cpu_clock_acc_type {
    fn name(&self) -> &'static str {
        "cpu_clock"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_cpu_clock()
    }

    fn get_f64(&self, m: &PerfStatistics) -> f64 {
        m.get_cpu_clock()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_task_clock_acc_type;
static PerfStatistics_task_clock_acc: PerfStatistics_task_clock_acc_type = PerfStatistics_task_clock_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_task_clock_acc_type {
    fn name(&self) -> &'static str {
        "task_clock"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_task_clock()
    }

    fn get_f64(&self, m: &PerfStatistics) -> f64 {
        m.get_task_clock()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_page_faults_acc_type;
static PerfStatistics_page_faults_acc: PerfStatistics_page_faults_acc_type = PerfStatistics_page_faults_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_page_faults_acc_type {
    fn name(&self) -> &'static str {
        "page_faults"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_page_faults()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_page_faults()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_minor_faults_acc_type;
static PerfStatistics_minor_faults_acc: PerfStatistics_minor_faults_acc_type = PerfStatistics_minor_faults_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_minor_faults_acc_type {
    fn name(&self) -> &'static str {
        "minor_faults"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_minor_faults()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_minor_faults()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_major_faults_acc_type;
static PerfStatistics_major_faults_acc: PerfStatistics_major_faults_acc_type = PerfStatistics_major_faults_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_major_faults_acc_type {
    fn name(&self) -> &'static str {
        "major_faults"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_major_faults()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_major_faults()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_context_switches_acc_type;
static PerfStatistics_context_switches_acc: PerfStatistics_context_switches_acc_type = PerfStatistics_context_switches_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_context_switches_acc_type {
    fn name(&self) -> &'static str {
        "context_switches"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_context_switches()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_context_switches()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_cpu_migrations_acc_type;
static PerfStatistics_cpu_migrations_acc: PerfStatistics_cpu_migrations_acc_type = PerfStatistics_cpu_migrations_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_cpu_migrations_acc_type {
    fn name(&self) -> &'static str {
        "cpu_migrations"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_cpu_migrations()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_cpu_migrations()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_alignment_faults_acc_type;
static PerfStatistics_alignment_faults_acc: PerfStatistics_alignment_faults_acc_type = PerfStatistics_alignment_faults_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_alignment_faults_acc_type {
    fn name(&self) -> &'static str {
        "alignment_faults"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_alignment_faults()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_alignment_faults()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_emulation_faults_acc_type;
static PerfStatistics_emulation_faults_acc: PerfStatistics_emulation_faults_acc_type = PerfStatistics_emulation_faults_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_emulation_faults_acc_type {
    fn name(&self) -> &'static str {
        "emulation_faults"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_emulation_faults()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_emulation_faults()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_l1_dcache_loads_acc_type;
static PerfStatistics_l1_dcache_loads_acc: PerfStatistics_l1_dcache_loads_acc_type = PerfStatistics_l1_dcache_loads_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_l1_dcache_loads_acc_type {
    fn name(&self) -> &'static str {
        "l1_dcache_loads"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_l1_dcache_loads()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_l1_dcache_loads()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_l1_dcache_load_misses_acc_type;
static PerfStatistics_l1_dcache_load_misses_acc: PerfStatistics_l1_dcache_load_misses_acc_type = PerfStatistics_l1_dcache_load_misses_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_l1_dcache_load_misses_acc_type {
    fn name(&self) -> &'static str {
        "l1_dcache_load_misses"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_l1_dcache_load_misses()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_l1_dcache_load_misses()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_l1_dcache_stores_acc_type;
static PerfStatistics_l1_dcache_stores_acc: PerfStatistics_l1_dcache_stores_acc_type = PerfStatistics_l1_dcache_stores_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_l1_dcache_stores_acc_type {
    fn name(&self) -> &'static str {
        "l1_dcache_stores"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_l1_dcache_stores()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_l1_dcache_stores()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_l1_dcache_store_misses_acc_type;
static PerfStatistics_l1_dcache_store_misses_acc: PerfStatistics_l1_dcache_store_misses_acc_type = PerfStatistics_l1_dcache_store_misses_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_l1_dcache_store_misses_acc_type {
    fn name(&self) -> &'static str {
        "l1_dcache_store_misses"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_l1_dcache_store_misses()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_l1_dcache_store_misses()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_l1_dcache_prefetches_acc_type;
static PerfStatistics_l1_dcache_prefetches_acc: PerfStatistics_l1_dcache_prefetches_acc_type = PerfStatistics_l1_dcache_prefetches_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_l1_dcache_prefetches_acc_type {
    fn name(&self) -> &'static str {
        "l1_dcache_prefetches"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_l1_dcache_prefetches()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_l1_dcache_prefetches()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_l1_dcache_prefetch_misses_acc_type;
static PerfStatistics_l1_dcache_prefetch_misses_acc: PerfStatistics_l1_dcache_prefetch_misses_acc_type = PerfStatistics_l1_dcache_prefetch_misses_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_l1_dcache_prefetch_misses_acc_type {
    fn name(&self) -> &'static str {
        "l1_dcache_prefetch_misses"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_l1_dcache_prefetch_misses()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_l1_dcache_prefetch_misses()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_l1_icache_loads_acc_type;
static PerfStatistics_l1_icache_loads_acc: PerfStatistics_l1_icache_loads_acc_type = PerfStatistics_l1_icache_loads_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_l1_icache_loads_acc_type {
    fn name(&self) -> &'static str {
        "l1_icache_loads"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_l1_icache_loads()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_l1_icache_loads()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_l1_icache_load_misses_acc_type;
static PerfStatistics_l1_icache_load_misses_acc: PerfStatistics_l1_icache_load_misses_acc_type = PerfStatistics_l1_icache_load_misses_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_l1_icache_load_misses_acc_type {
    fn name(&self) -> &'static str {
        "l1_icache_load_misses"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_l1_icache_load_misses()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_l1_icache_load_misses()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_l1_icache_prefetches_acc_type;
static PerfStatistics_l1_icache_prefetches_acc: PerfStatistics_l1_icache_prefetches_acc_type = PerfStatistics_l1_icache_prefetches_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_l1_icache_prefetches_acc_type {
    fn name(&self) -> &'static str {
        "l1_icache_prefetches"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_l1_icache_prefetches()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_l1_icache_prefetches()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_l1_icache_prefetch_misses_acc_type;
static PerfStatistics_l1_icache_prefetch_misses_acc: PerfStatistics_l1_icache_prefetch_misses_acc_type = PerfStatistics_l1_icache_prefetch_misses_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_l1_icache_prefetch_misses_acc_type {
    fn name(&self) -> &'static str {
        "l1_icache_prefetch_misses"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_l1_icache_prefetch_misses()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_l1_icache_prefetch_misses()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_llc_loads_acc_type;
static PerfStatistics_llc_loads_acc: PerfStatistics_llc_loads_acc_type = PerfStatistics_llc_loads_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_llc_loads_acc_type {
    fn name(&self) -> &'static str {
        "llc_loads"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_llc_loads()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_llc_loads()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_llc_load_misses_acc_type;
static PerfStatistics_llc_load_misses_acc: PerfStatistics_llc_load_misses_acc_type = PerfStatistics_llc_load_misses_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_llc_load_misses_acc_type {
    fn name(&self) -> &'static str {
        "llc_load_misses"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_llc_load_misses()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_llc_load_misses()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_llc_stores_acc_type;
static PerfStatistics_llc_stores_acc: PerfStatistics_llc_stores_acc_type = PerfStatistics_llc_stores_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_llc_stores_acc_type {
    fn name(&self) -> &'static str {
        "llc_stores"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_llc_stores()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_llc_stores()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_llc_store_misses_acc_type;
static PerfStatistics_llc_store_misses_acc: PerfStatistics_llc_store_misses_acc_type = PerfStatistics_llc_store_misses_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_llc_store_misses_acc_type {
    fn name(&self) -> &'static str {
        "llc_store_misses"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_llc_store_misses()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_llc_store_misses()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_llc_prefetches_acc_type;
static PerfStatistics_llc_prefetches_acc: PerfStatistics_llc_prefetches_acc_type = PerfStatistics_llc_prefetches_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_llc_prefetches_acc_type {
    fn name(&self) -> &'static str {
        "llc_prefetches"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_llc_prefetches()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_llc_prefetches()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_llc_prefetch_misses_acc_type;
static PerfStatistics_llc_prefetch_misses_acc: PerfStatistics_llc_prefetch_misses_acc_type = PerfStatistics_llc_prefetch_misses_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_llc_prefetch_misses_acc_type {
    fn name(&self) -> &'static str {
        "llc_prefetch_misses"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_llc_prefetch_misses()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_llc_prefetch_misses()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_dtlb_loads_acc_type;
static PerfStatistics_dtlb_loads_acc: PerfStatistics_dtlb_loads_acc_type = PerfStatistics_dtlb_loads_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_dtlb_loads_acc_type {
    fn name(&self) -> &'static str {
        "dtlb_loads"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_dtlb_loads()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_dtlb_loads()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_dtlb_load_misses_acc_type;
static PerfStatistics_dtlb_load_misses_acc: PerfStatistics_dtlb_load_misses_acc_type = PerfStatistics_dtlb_load_misses_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_dtlb_load_misses_acc_type {
    fn name(&self) -> &'static str {
        "dtlb_load_misses"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_dtlb_load_misses()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_dtlb_load_misses()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_dtlb_stores_acc_type;
static PerfStatistics_dtlb_stores_acc: PerfStatistics_dtlb_stores_acc_type = PerfStatistics_dtlb_stores_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_dtlb_stores_acc_type {
    fn name(&self) -> &'static str {
        "dtlb_stores"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_dtlb_stores()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_dtlb_stores()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_dtlb_store_misses_acc_type;
static PerfStatistics_dtlb_store_misses_acc: PerfStatistics_dtlb_store_misses_acc_type = PerfStatistics_dtlb_store_misses_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_dtlb_store_misses_acc_type {
    fn name(&self) -> &'static str {
        "dtlb_store_misses"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_dtlb_store_misses()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_dtlb_store_misses()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_dtlb_prefetches_acc_type;
static PerfStatistics_dtlb_prefetches_acc: PerfStatistics_dtlb_prefetches_acc_type = PerfStatistics_dtlb_prefetches_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_dtlb_prefetches_acc_type {
    fn name(&self) -> &'static str {
        "dtlb_prefetches"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_dtlb_prefetches()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_dtlb_prefetches()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_dtlb_prefetch_misses_acc_type;
static PerfStatistics_dtlb_prefetch_misses_acc: PerfStatistics_dtlb_prefetch_misses_acc_type = PerfStatistics_dtlb_prefetch_misses_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_dtlb_prefetch_misses_acc_type {
    fn name(&self) -> &'static str {
        "dtlb_prefetch_misses"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_dtlb_prefetch_misses()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_dtlb_prefetch_misses()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_itlb_loads_acc_type;
static PerfStatistics_itlb_loads_acc: PerfStatistics_itlb_loads_acc_type = PerfStatistics_itlb_loads_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_itlb_loads_acc_type {
    fn name(&self) -> &'static str {
        "itlb_loads"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_itlb_loads()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_itlb_loads()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_itlb_load_misses_acc_type;
static PerfStatistics_itlb_load_misses_acc: PerfStatistics_itlb_load_misses_acc_type = PerfStatistics_itlb_load_misses_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_itlb_load_misses_acc_type {
    fn name(&self) -> &'static str {
        "itlb_load_misses"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_itlb_load_misses()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_itlb_load_misses()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_branch_loads_acc_type;
static PerfStatistics_branch_loads_acc: PerfStatistics_branch_loads_acc_type = PerfStatistics_branch_loads_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_branch_loads_acc_type {
    fn name(&self) -> &'static str {
        "branch_loads"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_branch_loads()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_branch_loads()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_branch_load_misses_acc_type;
static PerfStatistics_branch_load_misses_acc: PerfStatistics_branch_load_misses_acc_type = PerfStatistics_branch_load_misses_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_branch_load_misses_acc_type {
    fn name(&self) -> &'static str {
        "branch_load_misses"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_branch_load_misses()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_branch_load_misses()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_node_loads_acc_type;
static PerfStatistics_node_loads_acc: PerfStatistics_node_loads_acc_type = PerfStatistics_node_loads_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_node_loads_acc_type {
    fn name(&self) -> &'static str {
        "node_loads"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_node_loads()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_node_loads()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_node_load_misses_acc_type;
static PerfStatistics_node_load_misses_acc: PerfStatistics_node_load_misses_acc_type = PerfStatistics_node_load_misses_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_node_load_misses_acc_type {
    fn name(&self) -> &'static str {
        "node_load_misses"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_node_load_misses()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_node_load_misses()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_node_stores_acc_type;
static PerfStatistics_node_stores_acc: PerfStatistics_node_stores_acc_type = PerfStatistics_node_stores_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_node_stores_acc_type {
    fn name(&self) -> &'static str {
        "node_stores"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_node_stores()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_node_stores()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_node_store_misses_acc_type;
static PerfStatistics_node_store_misses_acc: PerfStatistics_node_store_misses_acc_type = PerfStatistics_node_store_misses_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_node_store_misses_acc_type {
    fn name(&self) -> &'static str {
        "node_store_misses"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_node_store_misses()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_node_store_misses()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_node_prefetches_acc_type;
static PerfStatistics_node_prefetches_acc: PerfStatistics_node_prefetches_acc_type = PerfStatistics_node_prefetches_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_node_prefetches_acc_type {
    fn name(&self) -> &'static str {
        "node_prefetches"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_node_prefetches()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_node_prefetches()
    }
}

#[allow(non_camel_case_types)]
struct PerfStatistics_node_prefetch_misses_acc_type;
static PerfStatistics_node_prefetch_misses_acc: PerfStatistics_node_prefetch_misses_acc_type = PerfStatistics_node_prefetch_misses_acc_type;

impl ::protobuf::reflect::FieldAccessor<PerfStatistics> for PerfStatistics_node_prefetch_misses_acc_type {
    fn name(&self) -> &'static str {
        "node_prefetch_misses"
    }

    fn has_field(&self, m: &PerfStatistics) -> bool {
        m.has_node_prefetch_misses()
    }

    fn get_u64(&self, m: &PerfStatistics) -> u64 {
        m.get_node_prefetch_misses()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct Request {
    slave_id: ::protobuf::SingularPtrField<SlaveID>,
    resources: ::protobuf::RepeatedField<Resource>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> Request {
    pub fn new() -> Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Request {
        static mut instance: ::protobuf::lazy::Lazy<Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Request,
        };
        unsafe {
            instance.get(|| {
                Request {
                    slave_id: ::protobuf::SingularPtrField::none(),
                    resources: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // optional .mesos.SlaveID slave_id = 1;

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
    pub fn mut_slave_id(&'a mut self) -> &'a mut SlaveID {
        if self.slave_id.is_none() {
            self.slave_id.set_default();
        };
        self.slave_id.as_mut().unwrap()
    }

    pub fn get_slave_id(&'a self) -> &'a SlaveID {
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
    pub fn mut_resources(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Resource> {
        &mut self.resources
    }

    pub fn get_resources(&'a self) -> &'a [Resource] {
        self.resources.as_slice()
    }
}

impl ::protobuf::Message for Request {
    fn new() -> Request {
        Request::new()
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.slave_id.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.resources.push_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.slave_id.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.resources.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.slave_id.as_ref() {
            Some(v) => {
                try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        for v in self.resources.iter() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(sizes[*sizes_pos]));
            *sizes_pos += 1;
            try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<Request>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&Request_slave_id_acc as &'static ::protobuf::reflect::FieldAccessor<Request>) });
                fields.push(unsafe { ::std::mem::transmute(&Request_resources_acc as &'static ::protobuf::reflect::FieldAccessor<Request>) });
                ::protobuf::reflect::MessageDescriptor::new::<Request>(
                    "Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<Request>()
    }
}

impl ::protobuf::Clear for Request {
    fn clear(&mut self) {
        self.clear_slave_id();
        self.clear_resources();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct Request_slave_id_acc_type;
static Request_slave_id_acc: Request_slave_id_acc_type = Request_slave_id_acc_type;

impl ::protobuf::reflect::FieldAccessor<Request> for Request_slave_id_acc_type {
    fn name(&self) -> &'static str {
        "slave_id"
    }

    fn has_field(&self, m: &Request) -> bool {
        m.has_slave_id()
    }

    fn get_message<'a>(&self, m: &'a Request) -> &'a ::protobuf::Message {
        m.get_slave_id() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct Request_resources_acc_type;
static Request_resources_acc: Request_resources_acc_type = Request_resources_acc_type;

impl ::protobuf::reflect::FieldAccessor<Request> for Request_resources_acc_type {
    fn name(&self) -> &'static str {
        "resources"
    }

    fn len_field(&self, m: &Request) -> uint {
        m.get_resources().len()
    }

    fn get_rep_message_item<'a>(&self, m: &'a Request, index: uint) -> &'a ::protobuf::Message {
        &m.get_resources()[index] as &'a ::protobuf::Message
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct Offer {
    id: ::protobuf::SingularPtrField<OfferID>,
    framework_id: ::protobuf::SingularPtrField<FrameworkID>,
    slave_id: ::protobuf::SingularPtrField<SlaveID>,
    hostname: ::protobuf::SingularField<::std::string::String>,
    resources: ::protobuf::RepeatedField<Resource>,
    attributes: ::protobuf::RepeatedField<Attribute>,
    executor_ids: ::protobuf::RepeatedField<ExecutorID>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> Offer {
    pub fn new() -> Offer {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Offer {
        static mut instance: ::protobuf::lazy::Lazy<Offer> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Offer,
        };
        unsafe {
            instance.get(|| {
                Offer {
                    id: ::protobuf::SingularPtrField::none(),
                    framework_id: ::protobuf::SingularPtrField::none(),
                    slave_id: ::protobuf::SingularPtrField::none(),
                    hostname: ::protobuf::SingularField::none(),
                    resources: ::protobuf::RepeatedField::new(),
                    attributes: ::protobuf::RepeatedField::new(),
                    executor_ids: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // required .mesos.OfferID id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: OfferID) {
        self.id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&'a mut self) -> &'a mut OfferID {
        if self.id.is_none() {
            self.id.set_default();
        };
        self.id.as_mut().unwrap()
    }

    pub fn get_id(&'a self) -> &'a OfferID {
        self.id.as_ref().unwrap_or_else(|| OfferID::default_instance())
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
    pub fn mut_framework_id(&'a mut self) -> &'a mut FrameworkID {
        if self.framework_id.is_none() {
            self.framework_id.set_default();
        };
        self.framework_id.as_mut().unwrap()
    }

    pub fn get_framework_id(&'a self) -> &'a FrameworkID {
        self.framework_id.as_ref().unwrap_or_else(|| FrameworkID::default_instance())
    }

    // required .mesos.SlaveID slave_id = 3;

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
    pub fn mut_slave_id(&'a mut self) -> &'a mut SlaveID {
        if self.slave_id.is_none() {
            self.slave_id.set_default();
        };
        self.slave_id.as_mut().unwrap()
    }

    pub fn get_slave_id(&'a self) -> &'a SlaveID {
        self.slave_id.as_ref().unwrap_or_else(|| SlaveID::default_instance())
    }

    // required string hostname = 4;

    pub fn clear_hostname(&mut self) {
        self.hostname.clear();
    }

    pub fn has_hostname(&self) -> bool {
        self.hostname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hostname(&mut self, v: ::std::string::String) {
        self.hostname = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hostname(&'a mut self) -> &'a mut ::std::string::String {
        if self.hostname.is_none() {
            self.hostname.set_default();
        };
        self.hostname.as_mut().unwrap()
    }

    pub fn get_hostname(&'a self) -> &'a str {
        match self.hostname.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // repeated .mesos.Resource resources = 5;

    pub fn clear_resources(&mut self) {
        self.resources.clear();
    }

    // Param is passed by value, moved
    pub fn set_resources(&mut self, v: ::protobuf::RepeatedField<Resource>) {
        self.resources = v;
    }

    // Mutable pointer to the field.
    pub fn mut_resources(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Resource> {
        &mut self.resources
    }

    pub fn get_resources(&'a self) -> &'a [Resource] {
        self.resources.as_slice()
    }

    // repeated .mesos.Attribute attributes = 7;

    pub fn clear_attributes(&mut self) {
        self.attributes.clear();
    }

    // Param is passed by value, moved
    pub fn set_attributes(&mut self, v: ::protobuf::RepeatedField<Attribute>) {
        self.attributes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_attributes(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Attribute> {
        &mut self.attributes
    }

    pub fn get_attributes(&'a self) -> &'a [Attribute] {
        self.attributes.as_slice()
    }

    // repeated .mesos.ExecutorID executor_ids = 6;

    pub fn clear_executor_ids(&mut self) {
        self.executor_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_executor_ids(&mut self, v: ::protobuf::RepeatedField<ExecutorID>) {
        self.executor_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_executor_ids(&'a mut self) -> &'a mut ::protobuf::RepeatedField<ExecutorID> {
        &mut self.executor_ids
    }

    pub fn get_executor_ids(&'a self) -> &'a [ExecutorID] {
        self.executor_ids.as_slice()
    }
}

impl ::protobuf::Message for Offer {
    fn new() -> Offer {
        Offer::new()
    }

    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        };
        if self.framework_id.is_none() {
            return false;
        };
        if self.slave_id.is_none() {
            return false;
        };
        if self.hostname.is_none() {
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
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.id.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.framework_id.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.slave_id.set_default();
                    try!(is.merge_message(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.hostname.set_default();
                    try!(is.read_string_into(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.resources.push_default();
                    try!(is.merge_message(tmp))
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.attributes.push_default();
                    try!(is.merge_message(tmp))
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.executor_ids.push_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.id.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.framework_id.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.slave_id.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.hostname.iter() {
            my_size += ::protobuf::rt::string_size(4, value.as_slice());
        };
        for value in self.resources.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.attributes.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.executor_ids.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.id.as_ref() {
            Some(v) => {
                try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.framework_id.as_ref() {
            Some(v) => {
                try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.slave_id.as_ref() {
            Some(v) => {
                try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.hostname.as_ref() {
            Some(v) => {
                try!(os.write_string(4, v.as_slice()));
            },
            None => {},
        };
        for v in self.resources.iter() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(sizes[*sizes_pos]));
            *sizes_pos += 1;
            try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
        };
        for v in self.attributes.iter() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(sizes[*sizes_pos]));
            *sizes_pos += 1;
            try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
        };
        for v in self.executor_ids.iter() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(sizes[*sizes_pos]));
            *sizes_pos += 1;
            try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<Offer>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<Offer>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&Offer_id_acc as &'static ::protobuf::reflect::FieldAccessor<Offer>) });
                fields.push(unsafe { ::std::mem::transmute(&Offer_framework_id_acc as &'static ::protobuf::reflect::FieldAccessor<Offer>) });
                fields.push(unsafe { ::std::mem::transmute(&Offer_slave_id_acc as &'static ::protobuf::reflect::FieldAccessor<Offer>) });
                fields.push(unsafe { ::std::mem::transmute(&Offer_hostname_acc as &'static ::protobuf::reflect::FieldAccessor<Offer>) });
                fields.push(unsafe { ::std::mem::transmute(&Offer_resources_acc as &'static ::protobuf::reflect::FieldAccessor<Offer>) });
                fields.push(unsafe { ::std::mem::transmute(&Offer_attributes_acc as &'static ::protobuf::reflect::FieldAccessor<Offer>) });
                fields.push(unsafe { ::std::mem::transmute(&Offer_executor_ids_acc as &'static ::protobuf::reflect::FieldAccessor<Offer>) });
                ::protobuf::reflect::MessageDescriptor::new::<Offer>(
                    "Offer",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<Offer>()
    }
}

impl ::protobuf::Clear for Offer {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_framework_id();
        self.clear_slave_id();
        self.clear_hostname();
        self.clear_resources();
        self.clear_attributes();
        self.clear_executor_ids();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for Offer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct Offer_id_acc_type;
static Offer_id_acc: Offer_id_acc_type = Offer_id_acc_type;

impl ::protobuf::reflect::FieldAccessor<Offer> for Offer_id_acc_type {
    fn name(&self) -> &'static str {
        "id"
    }

    fn has_field(&self, m: &Offer) -> bool {
        m.has_id()
    }

    fn get_message<'a>(&self, m: &'a Offer) -> &'a ::protobuf::Message {
        m.get_id() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct Offer_framework_id_acc_type;
static Offer_framework_id_acc: Offer_framework_id_acc_type = Offer_framework_id_acc_type;

impl ::protobuf::reflect::FieldAccessor<Offer> for Offer_framework_id_acc_type {
    fn name(&self) -> &'static str {
        "framework_id"
    }

    fn has_field(&self, m: &Offer) -> bool {
        m.has_framework_id()
    }

    fn get_message<'a>(&self, m: &'a Offer) -> &'a ::protobuf::Message {
        m.get_framework_id() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct Offer_slave_id_acc_type;
static Offer_slave_id_acc: Offer_slave_id_acc_type = Offer_slave_id_acc_type;

impl ::protobuf::reflect::FieldAccessor<Offer> for Offer_slave_id_acc_type {
    fn name(&self) -> &'static str {
        "slave_id"
    }

    fn has_field(&self, m: &Offer) -> bool {
        m.has_slave_id()
    }

    fn get_message<'a>(&self, m: &'a Offer) -> &'a ::protobuf::Message {
        m.get_slave_id() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct Offer_hostname_acc_type;
static Offer_hostname_acc: Offer_hostname_acc_type = Offer_hostname_acc_type;

impl ::protobuf::reflect::FieldAccessor<Offer> for Offer_hostname_acc_type {
    fn name(&self) -> &'static str {
        "hostname"
    }

    fn has_field(&self, m: &Offer) -> bool {
        m.has_hostname()
    }

    fn get_str<'a>(&self, m: &'a Offer) -> &'a str {
        m.get_hostname()
    }
}

#[allow(non_camel_case_types)]
struct Offer_resources_acc_type;
static Offer_resources_acc: Offer_resources_acc_type = Offer_resources_acc_type;

impl ::protobuf::reflect::FieldAccessor<Offer> for Offer_resources_acc_type {
    fn name(&self) -> &'static str {
        "resources"
    }

    fn len_field(&self, m: &Offer) -> uint {
        m.get_resources().len()
    }

    fn get_rep_message_item<'a>(&self, m: &'a Offer, index: uint) -> &'a ::protobuf::Message {
        &m.get_resources()[index] as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct Offer_attributes_acc_type;
static Offer_attributes_acc: Offer_attributes_acc_type = Offer_attributes_acc_type;

impl ::protobuf::reflect::FieldAccessor<Offer> for Offer_attributes_acc_type {
    fn name(&self) -> &'static str {
        "attributes"
    }

    fn len_field(&self, m: &Offer) -> uint {
        m.get_attributes().len()
    }

    fn get_rep_message_item<'a>(&self, m: &'a Offer, index: uint) -> &'a ::protobuf::Message {
        &m.get_attributes()[index] as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct Offer_executor_ids_acc_type;
static Offer_executor_ids_acc: Offer_executor_ids_acc_type = Offer_executor_ids_acc_type;

impl ::protobuf::reflect::FieldAccessor<Offer> for Offer_executor_ids_acc_type {
    fn name(&self) -> &'static str {
        "executor_ids"
    }

    fn len_field(&self, m: &Offer) -> uint {
        m.get_executor_ids().len()
    }

    fn get_rep_message_item<'a>(&self, m: &'a Offer, index: uint) -> &'a ::protobuf::Message {
        &m.get_executor_ids()[index] as &'a ::protobuf::Message
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct TaskInfo {
    name: ::protobuf::SingularField<::std::string::String>,
    task_id: ::protobuf::SingularPtrField<TaskID>,
    slave_id: ::protobuf::SingularPtrField<SlaveID>,
    resources: ::protobuf::RepeatedField<Resource>,
    executor: ::protobuf::SingularPtrField<ExecutorInfo>,
    command: ::protobuf::SingularPtrField<CommandInfo>,
    container: ::protobuf::SingularPtrField<ContainerInfo>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    health_check: ::protobuf::SingularPtrField<HealthCheck>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> TaskInfo {
    pub fn new() -> TaskInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TaskInfo {
        static mut instance: ::protobuf::lazy::Lazy<TaskInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TaskInfo,
        };
        unsafe {
            instance.get(|| {
                TaskInfo {
                    name: ::protobuf::SingularField::none(),
                    task_id: ::protobuf::SingularPtrField::none(),
                    slave_id: ::protobuf::SingularPtrField::none(),
                    resources: ::protobuf::RepeatedField::new(),
                    executor: ::protobuf::SingularPtrField::none(),
                    command: ::protobuf::SingularPtrField::none(),
                    container: ::protobuf::SingularPtrField::none(),
                    data: ::protobuf::SingularField::none(),
                    health_check: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
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
    pub fn mut_name(&'a mut self) -> &'a mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    pub fn get_name(&'a self) -> &'a str {
        match self.name.as_ref() {
            Some(v) => v.as_slice(),
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
    pub fn mut_task_id(&'a mut self) -> &'a mut TaskID {
        if self.task_id.is_none() {
            self.task_id.set_default();
        };
        self.task_id.as_mut().unwrap()
    }

    pub fn get_task_id(&'a self) -> &'a TaskID {
        self.task_id.as_ref().unwrap_or_else(|| TaskID::default_instance())
    }

    // required .mesos.SlaveID slave_id = 3;

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
    pub fn mut_slave_id(&'a mut self) -> &'a mut SlaveID {
        if self.slave_id.is_none() {
            self.slave_id.set_default();
        };
        self.slave_id.as_mut().unwrap()
    }

    pub fn get_slave_id(&'a self) -> &'a SlaveID {
        self.slave_id.as_ref().unwrap_or_else(|| SlaveID::default_instance())
    }

    // repeated .mesos.Resource resources = 4;

    pub fn clear_resources(&mut self) {
        self.resources.clear();
    }

    // Param is passed by value, moved
    pub fn set_resources(&mut self, v: ::protobuf::RepeatedField<Resource>) {
        self.resources = v;
    }

    // Mutable pointer to the field.
    pub fn mut_resources(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Resource> {
        &mut self.resources
    }

    pub fn get_resources(&'a self) -> &'a [Resource] {
        self.resources.as_slice()
    }

    // optional .mesos.ExecutorInfo executor = 5;

    pub fn clear_executor(&mut self) {
        self.executor.clear();
    }

    pub fn has_executor(&self) -> bool {
        self.executor.is_some()
    }

    // Param is passed by value, moved
    pub fn set_executor(&mut self, v: ExecutorInfo) {
        self.executor = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_executor(&'a mut self) -> &'a mut ExecutorInfo {
        if self.executor.is_none() {
            self.executor.set_default();
        };
        self.executor.as_mut().unwrap()
    }

    pub fn get_executor(&'a self) -> &'a ExecutorInfo {
        self.executor.as_ref().unwrap_or_else(|| ExecutorInfo::default_instance())
    }

    // optional .mesos.CommandInfo command = 7;

    pub fn clear_command(&mut self) {
        self.command.clear();
    }

    pub fn has_command(&self) -> bool {
        self.command.is_some()
    }

    // Param is passed by value, moved
    pub fn set_command(&mut self, v: CommandInfo) {
        self.command = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_command(&'a mut self) -> &'a mut CommandInfo {
        if self.command.is_none() {
            self.command.set_default();
        };
        self.command.as_mut().unwrap()
    }

    pub fn get_command(&'a self) -> &'a CommandInfo {
        self.command.as_ref().unwrap_or_else(|| CommandInfo::default_instance())
    }

    // optional .mesos.ContainerInfo container = 9;

    pub fn clear_container(&mut self) {
        self.container.clear();
    }

    pub fn has_container(&self) -> bool {
        self.container.is_some()
    }

    // Param is passed by value, moved
    pub fn set_container(&mut self, v: ContainerInfo) {
        self.container = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_container(&'a mut self) -> &'a mut ContainerInfo {
        if self.container.is_none() {
            self.container.set_default();
        };
        self.container.as_mut().unwrap()
    }

    pub fn get_container(&'a self) -> &'a ContainerInfo {
        self.container.as_ref().unwrap_or_else(|| ContainerInfo::default_instance())
    }

    // optional bytes data = 6;

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
    pub fn mut_data(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        };
        self.data.as_mut().unwrap()
    }

    pub fn get_data(&'a self) -> &'a [u8] {
        match self.data.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }

    // optional .mesos.HealthCheck health_check = 8;

    pub fn clear_health_check(&mut self) {
        self.health_check.clear();
    }

    pub fn has_health_check(&self) -> bool {
        self.health_check.is_some()
    }

    // Param is passed by value, moved
    pub fn set_health_check(&mut self, v: HealthCheck) {
        self.health_check = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_health_check(&'a mut self) -> &'a mut HealthCheck {
        if self.health_check.is_none() {
            self.health_check.set_default();
        };
        self.health_check.as_mut().unwrap()
    }

    pub fn get_health_check(&'a self) -> &'a HealthCheck {
        self.health_check.as_ref().unwrap_or_else(|| HealthCheck::default_instance())
    }
}

impl ::protobuf::Message for TaskInfo {
    fn new() -> TaskInfo {
        TaskInfo::new()
    }

    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        if self.task_id.is_none() {
            return false;
        };
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
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.name.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.task_id.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.slave_id.set_default();
                    try!(is.merge_message(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.resources.push_default();
                    try!(is.merge_message(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.executor.set_default();
                    try!(is.merge_message(tmp))
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.command.set_default();
                    try!(is.merge_message(tmp))
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.container.set_default();
                    try!(is.merge_message(tmp))
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.data.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.health_check.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(1, value.as_slice());
        };
        for value in self.task_id.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.slave_id.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.resources.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.executor.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.command.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.container.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.data.iter() {
            my_size += ::protobuf::rt::bytes_size(6, value.as_slice());
        };
        for value in self.health_check.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.name.as_ref() {
            Some(v) => {
                try!(os.write_string(1, v.as_slice()));
            },
            None => {},
        };
        match self.task_id.as_ref() {
            Some(v) => {
                try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.slave_id.as_ref() {
            Some(v) => {
                try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        for v in self.resources.iter() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(sizes[*sizes_pos]));
            *sizes_pos += 1;
            try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
        };
        match self.executor.as_ref() {
            Some(v) => {
                try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.command.as_ref() {
            Some(v) => {
                try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.container.as_ref() {
            Some(v) => {
                try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.data.as_ref() {
            Some(v) => {
                try!(os.write_bytes(6, v.as_slice()));
            },
            None => {},
        };
        match self.health_check.as_ref() {
            Some(v) => {
                try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<TaskInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<TaskInfo>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&TaskInfo_name_acc as &'static ::protobuf::reflect::FieldAccessor<TaskInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&TaskInfo_task_id_acc as &'static ::protobuf::reflect::FieldAccessor<TaskInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&TaskInfo_slave_id_acc as &'static ::protobuf::reflect::FieldAccessor<TaskInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&TaskInfo_resources_acc as &'static ::protobuf::reflect::FieldAccessor<TaskInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&TaskInfo_executor_acc as &'static ::protobuf::reflect::FieldAccessor<TaskInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&TaskInfo_command_acc as &'static ::protobuf::reflect::FieldAccessor<TaskInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&TaskInfo_container_acc as &'static ::protobuf::reflect::FieldAccessor<TaskInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&TaskInfo_data_acc as &'static ::protobuf::reflect::FieldAccessor<TaskInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&TaskInfo_health_check_acc as &'static ::protobuf::reflect::FieldAccessor<TaskInfo>) });
                ::protobuf::reflect::MessageDescriptor::new::<TaskInfo>(
                    "TaskInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<TaskInfo>()
    }
}

impl ::protobuf::Clear for TaskInfo {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_task_id();
        self.clear_slave_id();
        self.clear_resources();
        self.clear_executor();
        self.clear_command();
        self.clear_container();
        self.clear_data();
        self.clear_health_check();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for TaskInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct TaskInfo_name_acc_type;
static TaskInfo_name_acc: TaskInfo_name_acc_type = TaskInfo_name_acc_type;

impl ::protobuf::reflect::FieldAccessor<TaskInfo> for TaskInfo_name_acc_type {
    fn name(&self) -> &'static str {
        "name"
    }

    fn has_field(&self, m: &TaskInfo) -> bool {
        m.has_name()
    }

    fn get_str<'a>(&self, m: &'a TaskInfo) -> &'a str {
        m.get_name()
    }
}

#[allow(non_camel_case_types)]
struct TaskInfo_task_id_acc_type;
static TaskInfo_task_id_acc: TaskInfo_task_id_acc_type = TaskInfo_task_id_acc_type;

impl ::protobuf::reflect::FieldAccessor<TaskInfo> for TaskInfo_task_id_acc_type {
    fn name(&self) -> &'static str {
        "task_id"
    }

    fn has_field(&self, m: &TaskInfo) -> bool {
        m.has_task_id()
    }

    fn get_message<'a>(&self, m: &'a TaskInfo) -> &'a ::protobuf::Message {
        m.get_task_id() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct TaskInfo_slave_id_acc_type;
static TaskInfo_slave_id_acc: TaskInfo_slave_id_acc_type = TaskInfo_slave_id_acc_type;

impl ::protobuf::reflect::FieldAccessor<TaskInfo> for TaskInfo_slave_id_acc_type {
    fn name(&self) -> &'static str {
        "slave_id"
    }

    fn has_field(&self, m: &TaskInfo) -> bool {
        m.has_slave_id()
    }

    fn get_message<'a>(&self, m: &'a TaskInfo) -> &'a ::protobuf::Message {
        m.get_slave_id() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct TaskInfo_resources_acc_type;
static TaskInfo_resources_acc: TaskInfo_resources_acc_type = TaskInfo_resources_acc_type;

impl ::protobuf::reflect::FieldAccessor<TaskInfo> for TaskInfo_resources_acc_type {
    fn name(&self) -> &'static str {
        "resources"
    }

    fn len_field(&self, m: &TaskInfo) -> uint {
        m.get_resources().len()
    }

    fn get_rep_message_item<'a>(&self, m: &'a TaskInfo, index: uint) -> &'a ::protobuf::Message {
        &m.get_resources()[index] as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct TaskInfo_executor_acc_type;
static TaskInfo_executor_acc: TaskInfo_executor_acc_type = TaskInfo_executor_acc_type;

impl ::protobuf::reflect::FieldAccessor<TaskInfo> for TaskInfo_executor_acc_type {
    fn name(&self) -> &'static str {
        "executor"
    }

    fn has_field(&self, m: &TaskInfo) -> bool {
        m.has_executor()
    }

    fn get_message<'a>(&self, m: &'a TaskInfo) -> &'a ::protobuf::Message {
        m.get_executor() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct TaskInfo_command_acc_type;
static TaskInfo_command_acc: TaskInfo_command_acc_type = TaskInfo_command_acc_type;

impl ::protobuf::reflect::FieldAccessor<TaskInfo> for TaskInfo_command_acc_type {
    fn name(&self) -> &'static str {
        "command"
    }

    fn has_field(&self, m: &TaskInfo) -> bool {
        m.has_command()
    }

    fn get_message<'a>(&self, m: &'a TaskInfo) -> &'a ::protobuf::Message {
        m.get_command() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct TaskInfo_container_acc_type;
static TaskInfo_container_acc: TaskInfo_container_acc_type = TaskInfo_container_acc_type;

impl ::protobuf::reflect::FieldAccessor<TaskInfo> for TaskInfo_container_acc_type {
    fn name(&self) -> &'static str {
        "container"
    }

    fn has_field(&self, m: &TaskInfo) -> bool {
        m.has_container()
    }

    fn get_message<'a>(&self, m: &'a TaskInfo) -> &'a ::protobuf::Message {
        m.get_container() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct TaskInfo_data_acc_type;
static TaskInfo_data_acc: TaskInfo_data_acc_type = TaskInfo_data_acc_type;

impl ::protobuf::reflect::FieldAccessor<TaskInfo> for TaskInfo_data_acc_type {
    fn name(&self) -> &'static str {
        "data"
    }

    fn has_field(&self, m: &TaskInfo) -> bool {
        m.has_data()
    }

    fn get_bytes<'a>(&self, m: &'a TaskInfo) -> &'a [u8] {
        m.get_data()
    }
}

#[allow(non_camel_case_types)]
struct TaskInfo_health_check_acc_type;
static TaskInfo_health_check_acc: TaskInfo_health_check_acc_type = TaskInfo_health_check_acc_type;

impl ::protobuf::reflect::FieldAccessor<TaskInfo> for TaskInfo_health_check_acc_type {
    fn name(&self) -> &'static str {
        "health_check"
    }

    fn has_field(&self, m: &TaskInfo) -> bool {
        m.has_health_check()
    }

    fn get_message<'a>(&self, m: &'a TaskInfo) -> &'a ::protobuf::Message {
        m.get_health_check() as &'a ::protobuf::Message
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct TaskStatus {
    task_id: ::protobuf::SingularPtrField<TaskID>,
    state: ::std::option::Option<TaskState>,
    message: ::protobuf::SingularField<::std::string::String>,
    source: ::std::option::Option<TaskStatus_Source>,
    reason: ::std::option::Option<TaskStatus_Reason>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    slave_id: ::protobuf::SingularPtrField<SlaveID>,
    executor_id: ::protobuf::SingularPtrField<ExecutorID>,
    timestamp: ::std::option::Option<f64>,
    healthy: ::std::option::Option<bool>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> TaskStatus {
    pub fn new() -> TaskStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TaskStatus {
        static mut instance: ::protobuf::lazy::Lazy<TaskStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TaskStatus,
        };
        unsafe {
            instance.get(|| {
                TaskStatus {
                    task_id: ::protobuf::SingularPtrField::none(),
                    state: ::std::option::None,
                    message: ::protobuf::SingularField::none(),
                    source: ::std::option::None,
                    reason: ::std::option::None,
                    data: ::protobuf::SingularField::none(),
                    slave_id: ::protobuf::SingularPtrField::none(),
                    executor_id: ::protobuf::SingularPtrField::none(),
                    timestamp: ::std::option::None,
                    healthy: ::std::option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
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
    pub fn mut_task_id(&'a mut self) -> &'a mut TaskID {
        if self.task_id.is_none() {
            self.task_id.set_default();
        };
        self.task_id.as_mut().unwrap()
    }

    pub fn get_task_id(&'a self) -> &'a TaskID {
        self.task_id.as_ref().unwrap_or_else(|| TaskID::default_instance())
    }

    // required .mesos.TaskState state = 2;

    pub fn clear_state(&mut self) {
        self.state = None;
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: TaskState) {
        self.state = Some(v);
    }

    pub fn get_state(&self) -> TaskState {
        self.state.unwrap_or(TaskState::TASK_STAGING)
    }

    // optional string message = 4;

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
    pub fn mut_message(&'a mut self) -> &'a mut ::std::string::String {
        if self.message.is_none() {
            self.message.set_default();
        };
        self.message.as_mut().unwrap()
    }

    pub fn get_message(&'a self) -> &'a str {
        match self.message.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // optional .mesos.TaskStatus.Source source = 9;

    pub fn clear_source(&mut self) {
        self.source = None;
    }

    pub fn has_source(&self) -> bool {
        self.source.is_some()
    }

    // Param is passed by value, moved
    pub fn set_source(&mut self, v: TaskStatus_Source) {
        self.source = Some(v);
    }

    pub fn get_source(&self) -> TaskStatus_Source {
        self.source.unwrap_or(TaskStatus_Source::SOURCE_MASTER)
    }

    // optional .mesos.TaskStatus.Reason reason = 10;

    pub fn clear_reason(&mut self) {
        self.reason = None;
    }

    pub fn has_reason(&self) -> bool {
        self.reason.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reason(&mut self, v: TaskStatus_Reason) {
        self.reason = Some(v);
    }

    pub fn get_reason(&self) -> TaskStatus_Reason {
        self.reason.unwrap_or(TaskStatus_Reason::REASON_COMMAND_EXECUTOR_FAILED)
    }

    // optional bytes data = 3;

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
    pub fn mut_data(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        };
        self.data.as_mut().unwrap()
    }

    pub fn get_data(&'a self) -> &'a [u8] {
        match self.data.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }

    // optional .mesos.SlaveID slave_id = 5;

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
    pub fn mut_slave_id(&'a mut self) -> &'a mut SlaveID {
        if self.slave_id.is_none() {
            self.slave_id.set_default();
        };
        self.slave_id.as_mut().unwrap()
    }

    pub fn get_slave_id(&'a self) -> &'a SlaveID {
        self.slave_id.as_ref().unwrap_or_else(|| SlaveID::default_instance())
    }

    // optional .mesos.ExecutorID executor_id = 7;

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
    pub fn mut_executor_id(&'a mut self) -> &'a mut ExecutorID {
        if self.executor_id.is_none() {
            self.executor_id.set_default();
        };
        self.executor_id.as_mut().unwrap()
    }

    pub fn get_executor_id(&'a self) -> &'a ExecutorID {
        self.executor_id.as_ref().unwrap_or_else(|| ExecutorID::default_instance())
    }

    // optional double timestamp = 6;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = None;
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: f64) {
        self.timestamp = Some(v);
    }

    pub fn get_timestamp(&self) -> f64 {
        self.timestamp.unwrap_or(0.)
    }

    // optional bool healthy = 8;

    pub fn clear_healthy(&mut self) {
        self.healthy = None;
    }

    pub fn has_healthy(&self) -> bool {
        self.healthy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_healthy(&mut self, v: bool) {
        self.healthy = Some(v);
    }

    pub fn get_healthy(&self) -> bool {
        self.healthy.unwrap_or(false)
    }
}

impl ::protobuf::Message for TaskStatus {
    fn new() -> TaskStatus {
        TaskStatus::new()
    }

    fn is_initialized(&self) -> bool {
        if self.task_id.is_none() {
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
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.task_id.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = TaskState::new(try!(is.read_int32()));
                    self.state = Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.message.set_default();
                    try!(is.read_string_into(tmp))
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = TaskStatus_Source::new(try!(is.read_int32()));
                    self.source = Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = TaskStatus_Reason::new(try!(is.read_int32()));
                    self.reason = Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.data.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.slave_id.set_default();
                    try!(is.merge_message(tmp))
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.executor_id.set_default();
                    try!(is.merge_message(tmp))
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.timestamp = Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.healthy = Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.task_id.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.state.iter() {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in self.message.iter() {
            my_size += ::protobuf::rt::string_size(4, value.as_slice());
        };
        for value in self.source.iter() {
            my_size += ::protobuf::rt::enum_size(9, *value);
        };
        for value in self.reason.iter() {
            my_size += ::protobuf::rt::enum_size(10, *value);
        };
        for value in self.data.iter() {
            my_size += ::protobuf::rt::bytes_size(3, value.as_slice());
        };
        for value in self.slave_id.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.executor_id.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.timestamp.is_some() {
            my_size += 9;
        };
        if self.healthy.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.task_id.as_ref() {
            Some(v) => {
                try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.state {
            Some(v) => {
                try!(os.write_enum(2, v as i32));
            },
            None => {},
        };
        match self.message.as_ref() {
            Some(v) => {
                try!(os.write_string(4, v.as_slice()));
            },
            None => {},
        };
        match self.source {
            Some(v) => {
                try!(os.write_enum(9, v as i32));
            },
            None => {},
        };
        match self.reason {
            Some(v) => {
                try!(os.write_enum(10, v as i32));
            },
            None => {},
        };
        match self.data.as_ref() {
            Some(v) => {
                try!(os.write_bytes(3, v.as_slice()));
            },
            None => {},
        };
        match self.slave_id.as_ref() {
            Some(v) => {
                try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.executor_id.as_ref() {
            Some(v) => {
                try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.timestamp {
            Some(v) => {
                try!(os.write_double(6, v));
            },
            None => {},
        };
        match self.healthy {
            Some(v) => {
                try!(os.write_bool(8, v));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<TaskStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<TaskStatus>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&TaskStatus_task_id_acc as &'static ::protobuf::reflect::FieldAccessor<TaskStatus>) });
                fields.push(unsafe { ::std::mem::transmute(&TaskStatus_state_acc as &'static ::protobuf::reflect::FieldAccessor<TaskStatus>) });
                fields.push(unsafe { ::std::mem::transmute(&TaskStatus_message_acc as &'static ::protobuf::reflect::FieldAccessor<TaskStatus>) });
                fields.push(unsafe { ::std::mem::transmute(&TaskStatus_source_acc as &'static ::protobuf::reflect::FieldAccessor<TaskStatus>) });
                fields.push(unsafe { ::std::mem::transmute(&TaskStatus_reason_acc as &'static ::protobuf::reflect::FieldAccessor<TaskStatus>) });
                fields.push(unsafe { ::std::mem::transmute(&TaskStatus_data_acc as &'static ::protobuf::reflect::FieldAccessor<TaskStatus>) });
                fields.push(unsafe { ::std::mem::transmute(&TaskStatus_slave_id_acc as &'static ::protobuf::reflect::FieldAccessor<TaskStatus>) });
                fields.push(unsafe { ::std::mem::transmute(&TaskStatus_executor_id_acc as &'static ::protobuf::reflect::FieldAccessor<TaskStatus>) });
                fields.push(unsafe { ::std::mem::transmute(&TaskStatus_timestamp_acc as &'static ::protobuf::reflect::FieldAccessor<TaskStatus>) });
                fields.push(unsafe { ::std::mem::transmute(&TaskStatus_healthy_acc as &'static ::protobuf::reflect::FieldAccessor<TaskStatus>) });
                ::protobuf::reflect::MessageDescriptor::new::<TaskStatus>(
                    "TaskStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<TaskStatus>()
    }
}

impl ::protobuf::Clear for TaskStatus {
    fn clear(&mut self) {
        self.clear_task_id();
        self.clear_state();
        self.clear_message();
        self.clear_source();
        self.clear_reason();
        self.clear_data();
        self.clear_slave_id();
        self.clear_executor_id();
        self.clear_timestamp();
        self.clear_healthy();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for TaskStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct TaskStatus_task_id_acc_type;
static TaskStatus_task_id_acc: TaskStatus_task_id_acc_type = TaskStatus_task_id_acc_type;

impl ::protobuf::reflect::FieldAccessor<TaskStatus> for TaskStatus_task_id_acc_type {
    fn name(&self) -> &'static str {
        "task_id"
    }

    fn has_field(&self, m: &TaskStatus) -> bool {
        m.has_task_id()
    }

    fn get_message<'a>(&self, m: &'a TaskStatus) -> &'a ::protobuf::Message {
        m.get_task_id() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct TaskStatus_state_acc_type;
static TaskStatus_state_acc: TaskStatus_state_acc_type = TaskStatus_state_acc_type;

impl ::protobuf::reflect::FieldAccessor<TaskStatus> for TaskStatus_state_acc_type {
    fn name(&self) -> &'static str {
        "state"
    }

    fn has_field(&self, m: &TaskStatus) -> bool {
        m.has_state()
    }

    fn get_enum<'a>(&self, m: &TaskStatus) -> &'static ::protobuf::reflect::EnumValueDescriptor {
        use protobuf::{ProtobufEnum};
        m.get_state().descriptor()
    }
}

#[allow(non_camel_case_types)]
struct TaskStatus_message_acc_type;
static TaskStatus_message_acc: TaskStatus_message_acc_type = TaskStatus_message_acc_type;

impl ::protobuf::reflect::FieldAccessor<TaskStatus> for TaskStatus_message_acc_type {
    fn name(&self) -> &'static str {
        "message"
    }

    fn has_field(&self, m: &TaskStatus) -> bool {
        m.has_message()
    }

    fn get_str<'a>(&self, m: &'a TaskStatus) -> &'a str {
        m.get_message()
    }
}

#[allow(non_camel_case_types)]
struct TaskStatus_source_acc_type;
static TaskStatus_source_acc: TaskStatus_source_acc_type = TaskStatus_source_acc_type;

impl ::protobuf::reflect::FieldAccessor<TaskStatus> for TaskStatus_source_acc_type {
    fn name(&self) -> &'static str {
        "source"
    }

    fn has_field(&self, m: &TaskStatus) -> bool {
        m.has_source()
    }

    fn get_enum<'a>(&self, m: &TaskStatus) -> &'static ::protobuf::reflect::EnumValueDescriptor {
        use protobuf::{ProtobufEnum};
        m.get_source().descriptor()
    }
}

#[allow(non_camel_case_types)]
struct TaskStatus_reason_acc_type;
static TaskStatus_reason_acc: TaskStatus_reason_acc_type = TaskStatus_reason_acc_type;

impl ::protobuf::reflect::FieldAccessor<TaskStatus> for TaskStatus_reason_acc_type {
    fn name(&self) -> &'static str {
        "reason"
    }

    fn has_field(&self, m: &TaskStatus) -> bool {
        m.has_reason()
    }

    fn get_enum<'a>(&self, m: &TaskStatus) -> &'static ::protobuf::reflect::EnumValueDescriptor {
        use protobuf::{ProtobufEnum};
        m.get_reason().descriptor()
    }
}

#[allow(non_camel_case_types)]
struct TaskStatus_data_acc_type;
static TaskStatus_data_acc: TaskStatus_data_acc_type = TaskStatus_data_acc_type;

impl ::protobuf::reflect::FieldAccessor<TaskStatus> for TaskStatus_data_acc_type {
    fn name(&self) -> &'static str {
        "data"
    }

    fn has_field(&self, m: &TaskStatus) -> bool {
        m.has_data()
    }

    fn get_bytes<'a>(&self, m: &'a TaskStatus) -> &'a [u8] {
        m.get_data()
    }
}

#[allow(non_camel_case_types)]
struct TaskStatus_slave_id_acc_type;
static TaskStatus_slave_id_acc: TaskStatus_slave_id_acc_type = TaskStatus_slave_id_acc_type;

impl ::protobuf::reflect::FieldAccessor<TaskStatus> for TaskStatus_slave_id_acc_type {
    fn name(&self) -> &'static str {
        "slave_id"
    }

    fn has_field(&self, m: &TaskStatus) -> bool {
        m.has_slave_id()
    }

    fn get_message<'a>(&self, m: &'a TaskStatus) -> &'a ::protobuf::Message {
        m.get_slave_id() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct TaskStatus_executor_id_acc_type;
static TaskStatus_executor_id_acc: TaskStatus_executor_id_acc_type = TaskStatus_executor_id_acc_type;

impl ::protobuf::reflect::FieldAccessor<TaskStatus> for TaskStatus_executor_id_acc_type {
    fn name(&self) -> &'static str {
        "executor_id"
    }

    fn has_field(&self, m: &TaskStatus) -> bool {
        m.has_executor_id()
    }

    fn get_message<'a>(&self, m: &'a TaskStatus) -> &'a ::protobuf::Message {
        m.get_executor_id() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct TaskStatus_timestamp_acc_type;
static TaskStatus_timestamp_acc: TaskStatus_timestamp_acc_type = TaskStatus_timestamp_acc_type;

impl ::protobuf::reflect::FieldAccessor<TaskStatus> for TaskStatus_timestamp_acc_type {
    fn name(&self) -> &'static str {
        "timestamp"
    }

    fn has_field(&self, m: &TaskStatus) -> bool {
        m.has_timestamp()
    }

    fn get_f64(&self, m: &TaskStatus) -> f64 {
        m.get_timestamp()
    }
}

#[allow(non_camel_case_types)]
struct TaskStatus_healthy_acc_type;
static TaskStatus_healthy_acc: TaskStatus_healthy_acc_type = TaskStatus_healthy_acc_type;

impl ::protobuf::reflect::FieldAccessor<TaskStatus> for TaskStatus_healthy_acc_type {
    fn name(&self) -> &'static str {
        "healthy"
    }

    fn has_field(&self, m: &TaskStatus) -> bool {
        m.has_healthy()
    }

    fn get_bool(&self, m: &TaskStatus) -> bool {
        m.get_healthy()
    }
}

#[deriving(Clone,PartialEq,Eq,Show)]
pub enum TaskStatus_Source {
    SOURCE_MASTER = 0,
    SOURCE_SLAVE = 1,
    SOURCE_EXECUTOR = 2,
}

impl TaskStatus_Source {
    pub fn new(value: i32) -> TaskStatus_Source {
        match value {
            0 => TaskStatus_Source::SOURCE_MASTER,
            1 => TaskStatus_Source::SOURCE_SLAVE,
            2 => TaskStatus_Source::SOURCE_EXECUTOR,
            _ => panic!()
        }
    }
}

impl ::protobuf::ProtobufEnum for TaskStatus_Source {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn enum_descriptor_static(_: Option<TaskStatus_Source>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("TaskStatus_Source", file_descriptor_proto())
            })
        }
    }
}

#[deriving(Clone,PartialEq,Eq,Show)]
pub enum TaskStatus_Reason {
    REASON_COMMAND_EXECUTOR_FAILED = 0,
    REASON_EXECUTOR_TERMINATED = 1,
    REASON_EXECUTOR_UNREGISTERED = 2,
    REASON_FRAMEWORK_REMOVED = 3,
    REASON_GC_ERROR = 4,
    REASON_INVALID_FRAMEWORKID = 5,
    REASON_INVALID_OFFERS = 6,
    REASON_MASTER_DISCONNECTED = 7,
    REASON_MEMORY_LIMIT = 8,
    REASON_RECONCILIATION = 9,
    REASON_SLAVE_DISCONNECTED = 10,
    REASON_SLAVE_REMOVED = 11,
    REASON_SLAVE_RESTARTED = 12,
    REASON_SLAVE_UNKNOWN = 13,
    REASON_TASK_INVALID = 14,
    REASON_TASK_UNAUTHORIZED = 15,
    REASON_TASK_UNKNOWN = 16,
}

impl TaskStatus_Reason {
    pub fn new(value: i32) -> TaskStatus_Reason {
        match value {
            0 => TaskStatus_Reason::REASON_COMMAND_EXECUTOR_FAILED,
            1 => TaskStatus_Reason::REASON_EXECUTOR_TERMINATED,
            2 => TaskStatus_Reason::REASON_EXECUTOR_UNREGISTERED,
            3 => TaskStatus_Reason::REASON_FRAMEWORK_REMOVED,
            4 => TaskStatus_Reason::REASON_GC_ERROR,
            5 => TaskStatus_Reason::REASON_INVALID_FRAMEWORKID,
            6 => TaskStatus_Reason::REASON_INVALID_OFFERS,
            7 => TaskStatus_Reason::REASON_MASTER_DISCONNECTED,
            8 => TaskStatus_Reason::REASON_MEMORY_LIMIT,
            9 => TaskStatus_Reason::REASON_RECONCILIATION,
            10 => TaskStatus_Reason::REASON_SLAVE_DISCONNECTED,
            11 => TaskStatus_Reason::REASON_SLAVE_REMOVED,
            12 => TaskStatus_Reason::REASON_SLAVE_RESTARTED,
            13 => TaskStatus_Reason::REASON_SLAVE_UNKNOWN,
            14 => TaskStatus_Reason::REASON_TASK_INVALID,
            15 => TaskStatus_Reason::REASON_TASK_UNAUTHORIZED,
            16 => TaskStatus_Reason::REASON_TASK_UNKNOWN,
            _ => panic!()
        }
    }
}

impl ::protobuf::ProtobufEnum for TaskStatus_Reason {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn enum_descriptor_static(_: Option<TaskStatus_Reason>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("TaskStatus_Reason", file_descriptor_proto())
            })
        }
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct Filters {
    refuse_seconds: ::std::option::Option<f64>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> Filters {
    pub fn new() -> Filters {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Filters {
        static mut instance: ::protobuf::lazy::Lazy<Filters> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Filters,
        };
        unsafe {
            instance.get(|| {
                Filters {
                    refuse_seconds: ::std::option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // optional double refuse_seconds = 1;

    pub fn clear_refuse_seconds(&mut self) {
        self.refuse_seconds = None;
    }

    pub fn has_refuse_seconds(&self) -> bool {
        self.refuse_seconds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_refuse_seconds(&mut self, v: f64) {
        self.refuse_seconds = Some(v);
    }

    pub fn get_refuse_seconds(&self) -> f64 {
        self.refuse_seconds.unwrap_or(5f64)
    }
}

impl ::protobuf::Message for Filters {
    fn new() -> Filters {
        Filters::new()
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.refuse_seconds = Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        if self.refuse_seconds.is_some() {
            my_size += 9;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.refuse_seconds {
            Some(v) => {
                try!(os.write_double(1, v));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<Filters>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<Filters>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&Filters_refuse_seconds_acc as &'static ::protobuf::reflect::FieldAccessor<Filters>) });
                ::protobuf::reflect::MessageDescriptor::new::<Filters>(
                    "Filters",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<Filters>()
    }
}

impl ::protobuf::Clear for Filters {
    fn clear(&mut self) {
        self.clear_refuse_seconds();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for Filters {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct Filters_refuse_seconds_acc_type;
static Filters_refuse_seconds_acc: Filters_refuse_seconds_acc_type = Filters_refuse_seconds_acc_type;

impl ::protobuf::reflect::FieldAccessor<Filters> for Filters_refuse_seconds_acc_type {
    fn name(&self) -> &'static str {
        "refuse_seconds"
    }

    fn has_field(&self, m: &Filters) -> bool {
        m.has_refuse_seconds()
    }

    fn get_f64(&self, m: &Filters) -> f64 {
        m.get_refuse_seconds()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct Environment {
    variables: ::protobuf::RepeatedField<Environment_Variable>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> Environment {
    pub fn new() -> Environment {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Environment {
        static mut instance: ::protobuf::lazy::Lazy<Environment> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Environment,
        };
        unsafe {
            instance.get(|| {
                Environment {
                    variables: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // repeated .mesos.Environment.Variable variables = 1;

    pub fn clear_variables(&mut self) {
        self.variables.clear();
    }

    // Param is passed by value, moved
    pub fn set_variables(&mut self, v: ::protobuf::RepeatedField<Environment_Variable>) {
        self.variables = v;
    }

    // Mutable pointer to the field.
    pub fn mut_variables(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Environment_Variable> {
        &mut self.variables
    }

    pub fn get_variables(&'a self) -> &'a [Environment_Variable] {
        self.variables.as_slice()
    }
}

impl ::protobuf::Message for Environment {
    fn new() -> Environment {
        Environment::new()
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.variables.push_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.variables.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        for v in self.variables.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(sizes[*sizes_pos]));
            *sizes_pos += 1;
            try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<Environment>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<Environment>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&Environment_variables_acc as &'static ::protobuf::reflect::FieldAccessor<Environment>) });
                ::protobuf::reflect::MessageDescriptor::new::<Environment>(
                    "Environment",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<Environment>()
    }
}

impl ::protobuf::Clear for Environment {
    fn clear(&mut self) {
        self.clear_variables();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for Environment {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct Environment_variables_acc_type;
static Environment_variables_acc: Environment_variables_acc_type = Environment_variables_acc_type;

impl ::protobuf::reflect::FieldAccessor<Environment> for Environment_variables_acc_type {
    fn name(&self) -> &'static str {
        "variables"
    }

    fn len_field(&self, m: &Environment) -> uint {
        m.get_variables().len()
    }

    fn get_rep_message_item<'a>(&self, m: &'a Environment, index: uint) -> &'a ::protobuf::Message {
        &m.get_variables()[index] as &'a ::protobuf::Message
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct Environment_Variable {
    name: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::string::String>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> Environment_Variable {
    pub fn new() -> Environment_Variable {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Environment_Variable {
        static mut instance: ::protobuf::lazy::Lazy<Environment_Variable> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Environment_Variable,
        };
        unsafe {
            instance.get(|| {
                Environment_Variable {
                    name: ::protobuf::SingularField::none(),
                    value: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
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
    pub fn mut_name(&'a mut self) -> &'a mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    pub fn get_name(&'a self) -> &'a str {
        match self.name.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // required string value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&'a mut self) -> &'a mut ::std::string::String {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    pub fn get_value(&'a self) -> &'a str {
        match self.value.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }
}

impl ::protobuf::Message for Environment_Variable {
    fn new() -> Environment_Variable {
        Environment_Variable::new()
    }

    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        if self.value.is_none() {
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
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.name.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.value.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(1, value.as_slice());
        };
        for value in self.value.iter() {
            my_size += ::protobuf::rt::string_size(2, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.name.as_ref() {
            Some(v) => {
                try!(os.write_string(1, v.as_slice()));
            },
            None => {},
        };
        match self.value.as_ref() {
            Some(v) => {
                try!(os.write_string(2, v.as_slice()));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<Environment_Variable>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<Environment_Variable>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&Environment_Variable_name_acc as &'static ::protobuf::reflect::FieldAccessor<Environment_Variable>) });
                fields.push(unsafe { ::std::mem::transmute(&Environment_Variable_value_acc as &'static ::protobuf::reflect::FieldAccessor<Environment_Variable>) });
                ::protobuf::reflect::MessageDescriptor::new::<Environment_Variable>(
                    "Environment_Variable",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<Environment_Variable>()
    }
}

impl ::protobuf::Clear for Environment_Variable {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for Environment_Variable {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct Environment_Variable_name_acc_type;
static Environment_Variable_name_acc: Environment_Variable_name_acc_type = Environment_Variable_name_acc_type;

impl ::protobuf::reflect::FieldAccessor<Environment_Variable> for Environment_Variable_name_acc_type {
    fn name(&self) -> &'static str {
        "name"
    }

    fn has_field(&self, m: &Environment_Variable) -> bool {
        m.has_name()
    }

    fn get_str<'a>(&self, m: &'a Environment_Variable) -> &'a str {
        m.get_name()
    }
}

#[allow(non_camel_case_types)]
struct Environment_Variable_value_acc_type;
static Environment_Variable_value_acc: Environment_Variable_value_acc_type = Environment_Variable_value_acc_type;

impl ::protobuf::reflect::FieldAccessor<Environment_Variable> for Environment_Variable_value_acc_type {
    fn name(&self) -> &'static str {
        "value"
    }

    fn has_field(&self, m: &Environment_Variable) -> bool {
        m.has_value()
    }

    fn get_str<'a>(&self, m: &'a Environment_Variable) -> &'a str {
        m.get_value()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct Parameter {
    key: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularField<::std::string::String>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> Parameter {
    pub fn new() -> Parameter {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Parameter {
        static mut instance: ::protobuf::lazy::Lazy<Parameter> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Parameter,
        };
        unsafe {
            instance.get(|| {
                Parameter {
                    key: ::protobuf::SingularField::none(),
                    value: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // required string key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&'a mut self) -> &'a mut ::std::string::String {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    pub fn get_key(&'a self) -> &'a str {
        match self.key.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // required string value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::string::String) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&'a mut self) -> &'a mut ::std::string::String {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    pub fn get_value(&'a self) -> &'a str {
        match self.value.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }
}

impl ::protobuf::Message for Parameter {
    fn new() -> Parameter {
        Parameter::new()
    }

    fn is_initialized(&self) -> bool {
        if self.key.is_none() {
            return false;
        };
        if self.value.is_none() {
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
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.key.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.value.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.key.iter() {
            my_size += ::protobuf::rt::string_size(1, value.as_slice());
        };
        for value in self.value.iter() {
            my_size += ::protobuf::rt::string_size(2, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.key.as_ref() {
            Some(v) => {
                try!(os.write_string(1, v.as_slice()));
            },
            None => {},
        };
        match self.value.as_ref() {
            Some(v) => {
                try!(os.write_string(2, v.as_slice()));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<Parameter>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<Parameter>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&Parameter_key_acc as &'static ::protobuf::reflect::FieldAccessor<Parameter>) });
                fields.push(unsafe { ::std::mem::transmute(&Parameter_value_acc as &'static ::protobuf::reflect::FieldAccessor<Parameter>) });
                ::protobuf::reflect::MessageDescriptor::new::<Parameter>(
                    "Parameter",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<Parameter>()
    }
}

impl ::protobuf::Clear for Parameter {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for Parameter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct Parameter_key_acc_type;
static Parameter_key_acc: Parameter_key_acc_type = Parameter_key_acc_type;

impl ::protobuf::reflect::FieldAccessor<Parameter> for Parameter_key_acc_type {
    fn name(&self) -> &'static str {
        "key"
    }

    fn has_field(&self, m: &Parameter) -> bool {
        m.has_key()
    }

    fn get_str<'a>(&self, m: &'a Parameter) -> &'a str {
        m.get_key()
    }
}

#[allow(non_camel_case_types)]
struct Parameter_value_acc_type;
static Parameter_value_acc: Parameter_value_acc_type = Parameter_value_acc_type;

impl ::protobuf::reflect::FieldAccessor<Parameter> for Parameter_value_acc_type {
    fn name(&self) -> &'static str {
        "value"
    }

    fn has_field(&self, m: &Parameter) -> bool {
        m.has_value()
    }

    fn get_str<'a>(&self, m: &'a Parameter) -> &'a str {
        m.get_value()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct Parameters {
    parameter: ::protobuf::RepeatedField<Parameter>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> Parameters {
    pub fn new() -> Parameters {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Parameters {
        static mut instance: ::protobuf::lazy::Lazy<Parameters> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Parameters,
        };
        unsafe {
            instance.get(|| {
                Parameters {
                    parameter: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // repeated .mesos.Parameter parameter = 1;

    pub fn clear_parameter(&mut self) {
        self.parameter.clear();
    }

    // Param is passed by value, moved
    pub fn set_parameter(&mut self, v: ::protobuf::RepeatedField<Parameter>) {
        self.parameter = v;
    }

    // Mutable pointer to the field.
    pub fn mut_parameter(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Parameter> {
        &mut self.parameter
    }

    pub fn get_parameter(&'a self) -> &'a [Parameter] {
        self.parameter.as_slice()
    }
}

impl ::protobuf::Message for Parameters {
    fn new() -> Parameters {
        Parameters::new()
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.parameter.push_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.parameter.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        for v in self.parameter.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(sizes[*sizes_pos]));
            *sizes_pos += 1;
            try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<Parameters>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<Parameters>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&Parameters_parameter_acc as &'static ::protobuf::reflect::FieldAccessor<Parameters>) });
                ::protobuf::reflect::MessageDescriptor::new::<Parameters>(
                    "Parameters",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<Parameters>()
    }
}

impl ::protobuf::Clear for Parameters {
    fn clear(&mut self) {
        self.clear_parameter();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for Parameters {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct Parameters_parameter_acc_type;
static Parameters_parameter_acc: Parameters_parameter_acc_type = Parameters_parameter_acc_type;

impl ::protobuf::reflect::FieldAccessor<Parameters> for Parameters_parameter_acc_type {
    fn name(&self) -> &'static str {
        "parameter"
    }

    fn len_field(&self, m: &Parameters) -> uint {
        m.get_parameter().len()
    }

    fn get_rep_message_item<'a>(&self, m: &'a Parameters, index: uint) -> &'a ::protobuf::Message {
        &m.get_parameter()[index] as &'a ::protobuf::Message
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct Credential {
    principal: ::protobuf::SingularField<::std::string::String>,
    secret: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> Credential {
    pub fn new() -> Credential {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Credential {
        static mut instance: ::protobuf::lazy::Lazy<Credential> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Credential,
        };
        unsafe {
            instance.get(|| {
                Credential {
                    principal: ::protobuf::SingularField::none(),
                    secret: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // required string principal = 1;

    pub fn clear_principal(&mut self) {
        self.principal.clear();
    }

    pub fn has_principal(&self) -> bool {
        self.principal.is_some()
    }

    // Param is passed by value, moved
    pub fn set_principal(&mut self, v: ::std::string::String) {
        self.principal = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_principal(&'a mut self) -> &'a mut ::std::string::String {
        if self.principal.is_none() {
            self.principal.set_default();
        };
        self.principal.as_mut().unwrap()
    }

    pub fn get_principal(&'a self) -> &'a str {
        match self.principal.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // optional bytes secret = 2;

    pub fn clear_secret(&mut self) {
        self.secret.clear();
    }

    pub fn has_secret(&self) -> bool {
        self.secret.is_some()
    }

    // Param is passed by value, moved
    pub fn set_secret(&mut self, v: ::std::vec::Vec<u8>) {
        self.secret = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_secret(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.secret.is_none() {
            self.secret.set_default();
        };
        self.secret.as_mut().unwrap()
    }

    pub fn get_secret(&'a self) -> &'a [u8] {
        match self.secret.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }
}

impl ::protobuf::Message for Credential {
    fn new() -> Credential {
        Credential::new()
    }

    fn is_initialized(&self) -> bool {
        if self.principal.is_none() {
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
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.principal.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.secret.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.principal.iter() {
            my_size += ::protobuf::rt::string_size(1, value.as_slice());
        };
        for value in self.secret.iter() {
            my_size += ::protobuf::rt::bytes_size(2, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.principal.as_ref() {
            Some(v) => {
                try!(os.write_string(1, v.as_slice()));
            },
            None => {},
        };
        match self.secret.as_ref() {
            Some(v) => {
                try!(os.write_bytes(2, v.as_slice()));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<Credential>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<Credential>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&Credential_principal_acc as &'static ::protobuf::reflect::FieldAccessor<Credential>) });
                fields.push(unsafe { ::std::mem::transmute(&Credential_secret_acc as &'static ::protobuf::reflect::FieldAccessor<Credential>) });
                ::protobuf::reflect::MessageDescriptor::new::<Credential>(
                    "Credential",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<Credential>()
    }
}

impl ::protobuf::Clear for Credential {
    fn clear(&mut self) {
        self.clear_principal();
        self.clear_secret();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for Credential {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct Credential_principal_acc_type;
static Credential_principal_acc: Credential_principal_acc_type = Credential_principal_acc_type;

impl ::protobuf::reflect::FieldAccessor<Credential> for Credential_principal_acc_type {
    fn name(&self) -> &'static str {
        "principal"
    }

    fn has_field(&self, m: &Credential) -> bool {
        m.has_principal()
    }

    fn get_str<'a>(&self, m: &'a Credential) -> &'a str {
        m.get_principal()
    }
}

#[allow(non_camel_case_types)]
struct Credential_secret_acc_type;
static Credential_secret_acc: Credential_secret_acc_type = Credential_secret_acc_type;

impl ::protobuf::reflect::FieldAccessor<Credential> for Credential_secret_acc_type {
    fn name(&self) -> &'static str {
        "secret"
    }

    fn has_field(&self, m: &Credential) -> bool {
        m.has_secret()
    }

    fn get_bytes<'a>(&self, m: &'a Credential) -> &'a [u8] {
        m.get_secret()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct Credentials {
    credentials: ::protobuf::RepeatedField<Credential>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> Credentials {
    pub fn new() -> Credentials {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Credentials {
        static mut instance: ::protobuf::lazy::Lazy<Credentials> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Credentials,
        };
        unsafe {
            instance.get(|| {
                Credentials {
                    credentials: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // repeated .mesos.Credential credentials = 1;

    pub fn clear_credentials(&mut self) {
        self.credentials.clear();
    }

    // Param is passed by value, moved
    pub fn set_credentials(&mut self, v: ::protobuf::RepeatedField<Credential>) {
        self.credentials = v;
    }

    // Mutable pointer to the field.
    pub fn mut_credentials(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Credential> {
        &mut self.credentials
    }

    pub fn get_credentials(&'a self) -> &'a [Credential] {
        self.credentials.as_slice()
    }
}

impl ::protobuf::Message for Credentials {
    fn new() -> Credentials {
        Credentials::new()
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.credentials.push_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.credentials.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        for v in self.credentials.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(sizes[*sizes_pos]));
            *sizes_pos += 1;
            try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<Credentials>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<Credentials>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&Credentials_credentials_acc as &'static ::protobuf::reflect::FieldAccessor<Credentials>) });
                ::protobuf::reflect::MessageDescriptor::new::<Credentials>(
                    "Credentials",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<Credentials>()
    }
}

impl ::protobuf::Clear for Credentials {
    fn clear(&mut self) {
        self.clear_credentials();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for Credentials {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct Credentials_credentials_acc_type;
static Credentials_credentials_acc: Credentials_credentials_acc_type = Credentials_credentials_acc_type;

impl ::protobuf::reflect::FieldAccessor<Credentials> for Credentials_credentials_acc_type {
    fn name(&self) -> &'static str {
        "credentials"
    }

    fn len_field(&self, m: &Credentials) -> uint {
        m.get_credentials().len()
    }

    fn get_rep_message_item<'a>(&self, m: &'a Credentials, index: uint) -> &'a ::protobuf::Message {
        &m.get_credentials()[index] as &'a ::protobuf::Message
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct ACL {
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> ACL {
    pub fn new() -> ACL {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ACL {
        static mut instance: ::protobuf::lazy::Lazy<ACL> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ACL,
        };
        unsafe {
            instance.get(|| {
                ACL {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }
}

impl ::protobuf::Message for ACL {
    fn new() -> ACL {
        ACL::new()
    }

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
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<ACL>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<ACL>> = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ACL>(
                    "ACL",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<ACL>()
    }
}

impl ::protobuf::Clear for ACL {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for ACL {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[deriving(Clone,PartialEq,Default)]
pub struct ACL_Entity {
    field_type: ::std::option::Option<ACL_Entity_Type>,
    values: ::protobuf::RepeatedField<::std::string::String>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> ACL_Entity {
    pub fn new() -> ACL_Entity {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ACL_Entity {
        static mut instance: ::protobuf::lazy::Lazy<ACL_Entity> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ACL_Entity,
        };
        unsafe {
            instance.get(|| {
                ACL_Entity {
                    field_type: ::std::option::None,
                    values: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // optional .mesos.ACL.Entity.Type type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ACL_Entity_Type) {
        self.field_type = Some(v);
    }

    pub fn get_field_type(&self) -> ACL_Entity_Type {
        self.field_type.unwrap_or(ACL_Entity_Type::SOME)
    }

    // repeated string values = 2;

    pub fn clear_values(&mut self) {
        self.values.clear();
    }

    // Param is passed by value, moved
    pub fn set_values(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.values = v;
    }

    // Mutable pointer to the field.
    pub fn mut_values(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.values
    }

    pub fn get_values(&'a self) -> &'a [::std::string::String] {
        self.values.as_slice()
    }
}

impl ::protobuf::Message for ACL_Entity {
    fn new() -> ACL_Entity {
        ACL_Entity::new()
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = ACL_Entity_Type::new(try!(is.read_int32()));
                    self.field_type = Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.values.push_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.field_type.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.values.iter() {
            my_size += ::protobuf::rt::string_size(2, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.field_type {
            Some(v) => {
                try!(os.write_enum(1, v as i32));
            },
            None => {},
        };
        for v in self.values.iter() {
            try!(os.write_string(2, v.as_slice()));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<ACL_Entity>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<ACL_Entity>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&ACL_Entity_field_type_acc as &'static ::protobuf::reflect::FieldAccessor<ACL_Entity>) });
                fields.push(unsafe { ::std::mem::transmute(&ACL_Entity_values_acc as &'static ::protobuf::reflect::FieldAccessor<ACL_Entity>) });
                ::protobuf::reflect::MessageDescriptor::new::<ACL_Entity>(
                    "ACL_Entity",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<ACL_Entity>()
    }
}

impl ::protobuf::Clear for ACL_Entity {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_values();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for ACL_Entity {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct ACL_Entity_field_type_acc_type;
static ACL_Entity_field_type_acc: ACL_Entity_field_type_acc_type = ACL_Entity_field_type_acc_type;

impl ::protobuf::reflect::FieldAccessor<ACL_Entity> for ACL_Entity_field_type_acc_type {
    fn name(&self) -> &'static str {
        "field_type"
    }

    fn has_field(&self, m: &ACL_Entity) -> bool {
        m.has_field_type()
    }

    fn get_enum<'a>(&self, m: &ACL_Entity) -> &'static ::protobuf::reflect::EnumValueDescriptor {
        use protobuf::{ProtobufEnum};
        m.get_field_type().descriptor()
    }
}

#[allow(non_camel_case_types)]
struct ACL_Entity_values_acc_type;
static ACL_Entity_values_acc: ACL_Entity_values_acc_type = ACL_Entity_values_acc_type;

impl ::protobuf::reflect::FieldAccessor<ACL_Entity> for ACL_Entity_values_acc_type {
    fn name(&self) -> &'static str {
        "values"
    }

    fn len_field(&self, m: &ACL_Entity) -> uint {
        m.get_values().len()
    }

    fn get_rep_str<'a>(&self, m: &'a ACL_Entity) -> &'a [::std::string::String] {
        m.get_values()
    }
}

#[deriving(Clone,PartialEq,Eq,Show)]
pub enum ACL_Entity_Type {
    SOME = 0,
    ANY = 1,
    NONE = 2,
}

impl ACL_Entity_Type {
    pub fn new(value: i32) -> ACL_Entity_Type {
        match value {
            0 => ACL_Entity_Type::SOME,
            1 => ACL_Entity_Type::ANY,
            2 => ACL_Entity_Type::NONE,
            _ => panic!()
        }
    }
}

impl ::protobuf::ProtobufEnum for ACL_Entity_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn enum_descriptor_static(_: Option<ACL_Entity_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ACL_Entity_Type", file_descriptor_proto())
            })
        }
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct ACL_RegisterFramework {
    principals: ::protobuf::SingularPtrField<ACL_Entity>,
    roles: ::protobuf::SingularPtrField<ACL_Entity>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> ACL_RegisterFramework {
    pub fn new() -> ACL_RegisterFramework {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ACL_RegisterFramework {
        static mut instance: ::protobuf::lazy::Lazy<ACL_RegisterFramework> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ACL_RegisterFramework,
        };
        unsafe {
            instance.get(|| {
                ACL_RegisterFramework {
                    principals: ::protobuf::SingularPtrField::none(),
                    roles: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // required .mesos.ACL.Entity principals = 1;

    pub fn clear_principals(&mut self) {
        self.principals.clear();
    }

    pub fn has_principals(&self) -> bool {
        self.principals.is_some()
    }

    // Param is passed by value, moved
    pub fn set_principals(&mut self, v: ACL_Entity) {
        self.principals = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_principals(&'a mut self) -> &'a mut ACL_Entity {
        if self.principals.is_none() {
            self.principals.set_default();
        };
        self.principals.as_mut().unwrap()
    }

    pub fn get_principals(&'a self) -> &'a ACL_Entity {
        self.principals.as_ref().unwrap_or_else(|| ACL_Entity::default_instance())
    }

    // required .mesos.ACL.Entity roles = 2;

    pub fn clear_roles(&mut self) {
        self.roles.clear();
    }

    pub fn has_roles(&self) -> bool {
        self.roles.is_some()
    }

    // Param is passed by value, moved
    pub fn set_roles(&mut self, v: ACL_Entity) {
        self.roles = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_roles(&'a mut self) -> &'a mut ACL_Entity {
        if self.roles.is_none() {
            self.roles.set_default();
        };
        self.roles.as_mut().unwrap()
    }

    pub fn get_roles(&'a self) -> &'a ACL_Entity {
        self.roles.as_ref().unwrap_or_else(|| ACL_Entity::default_instance())
    }
}

impl ::protobuf::Message for ACL_RegisterFramework {
    fn new() -> ACL_RegisterFramework {
        ACL_RegisterFramework::new()
    }

    fn is_initialized(&self) -> bool {
        if self.principals.is_none() {
            return false;
        };
        if self.roles.is_none() {
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
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.principals.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.roles.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.principals.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.roles.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.principals.as_ref() {
            Some(v) => {
                try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.roles.as_ref() {
            Some(v) => {
                try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<ACL_RegisterFramework>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<ACL_RegisterFramework>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&ACL_RegisterFramework_principals_acc as &'static ::protobuf::reflect::FieldAccessor<ACL_RegisterFramework>) });
                fields.push(unsafe { ::std::mem::transmute(&ACL_RegisterFramework_roles_acc as &'static ::protobuf::reflect::FieldAccessor<ACL_RegisterFramework>) });
                ::protobuf::reflect::MessageDescriptor::new::<ACL_RegisterFramework>(
                    "ACL_RegisterFramework",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<ACL_RegisterFramework>()
    }
}

impl ::protobuf::Clear for ACL_RegisterFramework {
    fn clear(&mut self) {
        self.clear_principals();
        self.clear_roles();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for ACL_RegisterFramework {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct ACL_RegisterFramework_principals_acc_type;
static ACL_RegisterFramework_principals_acc: ACL_RegisterFramework_principals_acc_type = ACL_RegisterFramework_principals_acc_type;

impl ::protobuf::reflect::FieldAccessor<ACL_RegisterFramework> for ACL_RegisterFramework_principals_acc_type {
    fn name(&self) -> &'static str {
        "principals"
    }

    fn has_field(&self, m: &ACL_RegisterFramework) -> bool {
        m.has_principals()
    }

    fn get_message<'a>(&self, m: &'a ACL_RegisterFramework) -> &'a ::protobuf::Message {
        m.get_principals() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct ACL_RegisterFramework_roles_acc_type;
static ACL_RegisterFramework_roles_acc: ACL_RegisterFramework_roles_acc_type = ACL_RegisterFramework_roles_acc_type;

impl ::protobuf::reflect::FieldAccessor<ACL_RegisterFramework> for ACL_RegisterFramework_roles_acc_type {
    fn name(&self) -> &'static str {
        "roles"
    }

    fn has_field(&self, m: &ACL_RegisterFramework) -> bool {
        m.has_roles()
    }

    fn get_message<'a>(&self, m: &'a ACL_RegisterFramework) -> &'a ::protobuf::Message {
        m.get_roles() as &'a ::protobuf::Message
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct ACL_RunTask {
    principals: ::protobuf::SingularPtrField<ACL_Entity>,
    users: ::protobuf::SingularPtrField<ACL_Entity>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> ACL_RunTask {
    pub fn new() -> ACL_RunTask {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ACL_RunTask {
        static mut instance: ::protobuf::lazy::Lazy<ACL_RunTask> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ACL_RunTask,
        };
        unsafe {
            instance.get(|| {
                ACL_RunTask {
                    principals: ::protobuf::SingularPtrField::none(),
                    users: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // required .mesos.ACL.Entity principals = 1;

    pub fn clear_principals(&mut self) {
        self.principals.clear();
    }

    pub fn has_principals(&self) -> bool {
        self.principals.is_some()
    }

    // Param is passed by value, moved
    pub fn set_principals(&mut self, v: ACL_Entity) {
        self.principals = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_principals(&'a mut self) -> &'a mut ACL_Entity {
        if self.principals.is_none() {
            self.principals.set_default();
        };
        self.principals.as_mut().unwrap()
    }

    pub fn get_principals(&'a self) -> &'a ACL_Entity {
        self.principals.as_ref().unwrap_or_else(|| ACL_Entity::default_instance())
    }

    // required .mesos.ACL.Entity users = 2;

    pub fn clear_users(&mut self) {
        self.users.clear();
    }

    pub fn has_users(&self) -> bool {
        self.users.is_some()
    }

    // Param is passed by value, moved
    pub fn set_users(&mut self, v: ACL_Entity) {
        self.users = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_users(&'a mut self) -> &'a mut ACL_Entity {
        if self.users.is_none() {
            self.users.set_default();
        };
        self.users.as_mut().unwrap()
    }

    pub fn get_users(&'a self) -> &'a ACL_Entity {
        self.users.as_ref().unwrap_or_else(|| ACL_Entity::default_instance())
    }
}

impl ::protobuf::Message for ACL_RunTask {
    fn new() -> ACL_RunTask {
        ACL_RunTask::new()
    }

    fn is_initialized(&self) -> bool {
        if self.principals.is_none() {
            return false;
        };
        if self.users.is_none() {
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
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.principals.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.users.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.principals.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.users.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.principals.as_ref() {
            Some(v) => {
                try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.users.as_ref() {
            Some(v) => {
                try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<ACL_RunTask>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<ACL_RunTask>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&ACL_RunTask_principals_acc as &'static ::protobuf::reflect::FieldAccessor<ACL_RunTask>) });
                fields.push(unsafe { ::std::mem::transmute(&ACL_RunTask_users_acc as &'static ::protobuf::reflect::FieldAccessor<ACL_RunTask>) });
                ::protobuf::reflect::MessageDescriptor::new::<ACL_RunTask>(
                    "ACL_RunTask",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<ACL_RunTask>()
    }
}

impl ::protobuf::Clear for ACL_RunTask {
    fn clear(&mut self) {
        self.clear_principals();
        self.clear_users();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for ACL_RunTask {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct ACL_RunTask_principals_acc_type;
static ACL_RunTask_principals_acc: ACL_RunTask_principals_acc_type = ACL_RunTask_principals_acc_type;

impl ::protobuf::reflect::FieldAccessor<ACL_RunTask> for ACL_RunTask_principals_acc_type {
    fn name(&self) -> &'static str {
        "principals"
    }

    fn has_field(&self, m: &ACL_RunTask) -> bool {
        m.has_principals()
    }

    fn get_message<'a>(&self, m: &'a ACL_RunTask) -> &'a ::protobuf::Message {
        m.get_principals() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct ACL_RunTask_users_acc_type;
static ACL_RunTask_users_acc: ACL_RunTask_users_acc_type = ACL_RunTask_users_acc_type;

impl ::protobuf::reflect::FieldAccessor<ACL_RunTask> for ACL_RunTask_users_acc_type {
    fn name(&self) -> &'static str {
        "users"
    }

    fn has_field(&self, m: &ACL_RunTask) -> bool {
        m.has_users()
    }

    fn get_message<'a>(&self, m: &'a ACL_RunTask) -> &'a ::protobuf::Message {
        m.get_users() as &'a ::protobuf::Message
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct ACL_ShutdownFramework {
    principals: ::protobuf::SingularPtrField<ACL_Entity>,
    framework_principals: ::protobuf::SingularPtrField<ACL_Entity>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> ACL_ShutdownFramework {
    pub fn new() -> ACL_ShutdownFramework {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ACL_ShutdownFramework {
        static mut instance: ::protobuf::lazy::Lazy<ACL_ShutdownFramework> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ACL_ShutdownFramework,
        };
        unsafe {
            instance.get(|| {
                ACL_ShutdownFramework {
                    principals: ::protobuf::SingularPtrField::none(),
                    framework_principals: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // required .mesos.ACL.Entity principals = 1;

    pub fn clear_principals(&mut self) {
        self.principals.clear();
    }

    pub fn has_principals(&self) -> bool {
        self.principals.is_some()
    }

    // Param is passed by value, moved
    pub fn set_principals(&mut self, v: ACL_Entity) {
        self.principals = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_principals(&'a mut self) -> &'a mut ACL_Entity {
        if self.principals.is_none() {
            self.principals.set_default();
        };
        self.principals.as_mut().unwrap()
    }

    pub fn get_principals(&'a self) -> &'a ACL_Entity {
        self.principals.as_ref().unwrap_or_else(|| ACL_Entity::default_instance())
    }

    // required .mesos.ACL.Entity framework_principals = 2;

    pub fn clear_framework_principals(&mut self) {
        self.framework_principals.clear();
    }

    pub fn has_framework_principals(&self) -> bool {
        self.framework_principals.is_some()
    }

    // Param is passed by value, moved
    pub fn set_framework_principals(&mut self, v: ACL_Entity) {
        self.framework_principals = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_framework_principals(&'a mut self) -> &'a mut ACL_Entity {
        if self.framework_principals.is_none() {
            self.framework_principals.set_default();
        };
        self.framework_principals.as_mut().unwrap()
    }

    pub fn get_framework_principals(&'a self) -> &'a ACL_Entity {
        self.framework_principals.as_ref().unwrap_or_else(|| ACL_Entity::default_instance())
    }
}

impl ::protobuf::Message for ACL_ShutdownFramework {
    fn new() -> ACL_ShutdownFramework {
        ACL_ShutdownFramework::new()
    }

    fn is_initialized(&self) -> bool {
        if self.principals.is_none() {
            return false;
        };
        if self.framework_principals.is_none() {
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
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.principals.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.framework_principals.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.principals.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.framework_principals.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.principals.as_ref() {
            Some(v) => {
                try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        match self.framework_principals.as_ref() {
            Some(v) => {
                try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<ACL_ShutdownFramework>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<ACL_ShutdownFramework>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&ACL_ShutdownFramework_principals_acc as &'static ::protobuf::reflect::FieldAccessor<ACL_ShutdownFramework>) });
                fields.push(unsafe { ::std::mem::transmute(&ACL_ShutdownFramework_framework_principals_acc as &'static ::protobuf::reflect::FieldAccessor<ACL_ShutdownFramework>) });
                ::protobuf::reflect::MessageDescriptor::new::<ACL_ShutdownFramework>(
                    "ACL_ShutdownFramework",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<ACL_ShutdownFramework>()
    }
}

impl ::protobuf::Clear for ACL_ShutdownFramework {
    fn clear(&mut self) {
        self.clear_principals();
        self.clear_framework_principals();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for ACL_ShutdownFramework {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct ACL_ShutdownFramework_principals_acc_type;
static ACL_ShutdownFramework_principals_acc: ACL_ShutdownFramework_principals_acc_type = ACL_ShutdownFramework_principals_acc_type;

impl ::protobuf::reflect::FieldAccessor<ACL_ShutdownFramework> for ACL_ShutdownFramework_principals_acc_type {
    fn name(&self) -> &'static str {
        "principals"
    }

    fn has_field(&self, m: &ACL_ShutdownFramework) -> bool {
        m.has_principals()
    }

    fn get_message<'a>(&self, m: &'a ACL_ShutdownFramework) -> &'a ::protobuf::Message {
        m.get_principals() as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct ACL_ShutdownFramework_framework_principals_acc_type;
static ACL_ShutdownFramework_framework_principals_acc: ACL_ShutdownFramework_framework_principals_acc_type = ACL_ShutdownFramework_framework_principals_acc_type;

impl ::protobuf::reflect::FieldAccessor<ACL_ShutdownFramework> for ACL_ShutdownFramework_framework_principals_acc_type {
    fn name(&self) -> &'static str {
        "framework_principals"
    }

    fn has_field(&self, m: &ACL_ShutdownFramework) -> bool {
        m.has_framework_principals()
    }

    fn get_message<'a>(&self, m: &'a ACL_ShutdownFramework) -> &'a ::protobuf::Message {
        m.get_framework_principals() as &'a ::protobuf::Message
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct ACLs {
    permissive: ::std::option::Option<bool>,
    register_frameworks: ::protobuf::RepeatedField<ACL_RegisterFramework>,
    run_tasks: ::protobuf::RepeatedField<ACL_RunTask>,
    shutdown_frameworks: ::protobuf::RepeatedField<ACL_ShutdownFramework>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> ACLs {
    pub fn new() -> ACLs {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ACLs {
        static mut instance: ::protobuf::lazy::Lazy<ACLs> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ACLs,
        };
        unsafe {
            instance.get(|| {
                ACLs {
                    permissive: ::std::option::None,
                    register_frameworks: ::protobuf::RepeatedField::new(),
                    run_tasks: ::protobuf::RepeatedField::new(),
                    shutdown_frameworks: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // optional bool permissive = 1;

    pub fn clear_permissive(&mut self) {
        self.permissive = None;
    }

    pub fn has_permissive(&self) -> bool {
        self.permissive.is_some()
    }

    // Param is passed by value, moved
    pub fn set_permissive(&mut self, v: bool) {
        self.permissive = Some(v);
    }

    pub fn get_permissive(&self) -> bool {
        self.permissive.unwrap_or(true)
    }

    // repeated .mesos.ACL.RegisterFramework register_frameworks = 2;

    pub fn clear_register_frameworks(&mut self) {
        self.register_frameworks.clear();
    }

    // Param is passed by value, moved
    pub fn set_register_frameworks(&mut self, v: ::protobuf::RepeatedField<ACL_RegisterFramework>) {
        self.register_frameworks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_register_frameworks(&'a mut self) -> &'a mut ::protobuf::RepeatedField<ACL_RegisterFramework> {
        &mut self.register_frameworks
    }

    pub fn get_register_frameworks(&'a self) -> &'a [ACL_RegisterFramework] {
        self.register_frameworks.as_slice()
    }

    // repeated .mesos.ACL.RunTask run_tasks = 3;

    pub fn clear_run_tasks(&mut self) {
        self.run_tasks.clear();
    }

    // Param is passed by value, moved
    pub fn set_run_tasks(&mut self, v: ::protobuf::RepeatedField<ACL_RunTask>) {
        self.run_tasks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_run_tasks(&'a mut self) -> &'a mut ::protobuf::RepeatedField<ACL_RunTask> {
        &mut self.run_tasks
    }

    pub fn get_run_tasks(&'a self) -> &'a [ACL_RunTask] {
        self.run_tasks.as_slice()
    }

    // repeated .mesos.ACL.ShutdownFramework shutdown_frameworks = 4;

    pub fn clear_shutdown_frameworks(&mut self) {
        self.shutdown_frameworks.clear();
    }

    // Param is passed by value, moved
    pub fn set_shutdown_frameworks(&mut self, v: ::protobuf::RepeatedField<ACL_ShutdownFramework>) {
        self.shutdown_frameworks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_shutdown_frameworks(&'a mut self) -> &'a mut ::protobuf::RepeatedField<ACL_ShutdownFramework> {
        &mut self.shutdown_frameworks
    }

    pub fn get_shutdown_frameworks(&'a self) -> &'a [ACL_ShutdownFramework] {
        self.shutdown_frameworks.as_slice()
    }
}

impl ::protobuf::Message for ACLs {
    fn new() -> ACLs {
        ACLs::new()
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.permissive = Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.register_frameworks.push_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.run_tasks.push_default();
                    try!(is.merge_message(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.shutdown_frameworks.push_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        if self.permissive.is_some() {
            my_size += 2;
        };
        for value in self.register_frameworks.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.run_tasks.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.shutdown_frameworks.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.permissive {
            Some(v) => {
                try!(os.write_bool(1, v));
            },
            None => {},
        };
        for v in self.register_frameworks.iter() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(sizes[*sizes_pos]));
            *sizes_pos += 1;
            try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
        };
        for v in self.run_tasks.iter() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(sizes[*sizes_pos]));
            *sizes_pos += 1;
            try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
        };
        for v in self.shutdown_frameworks.iter() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(sizes[*sizes_pos]));
            *sizes_pos += 1;
            try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<ACLs>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<ACLs>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&ACLs_permissive_acc as &'static ::protobuf::reflect::FieldAccessor<ACLs>) });
                fields.push(unsafe { ::std::mem::transmute(&ACLs_register_frameworks_acc as &'static ::protobuf::reflect::FieldAccessor<ACLs>) });
                fields.push(unsafe { ::std::mem::transmute(&ACLs_run_tasks_acc as &'static ::protobuf::reflect::FieldAccessor<ACLs>) });
                fields.push(unsafe { ::std::mem::transmute(&ACLs_shutdown_frameworks_acc as &'static ::protobuf::reflect::FieldAccessor<ACLs>) });
                ::protobuf::reflect::MessageDescriptor::new::<ACLs>(
                    "ACLs",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<ACLs>()
    }
}

impl ::protobuf::Clear for ACLs {
    fn clear(&mut self) {
        self.clear_permissive();
        self.clear_register_frameworks();
        self.clear_run_tasks();
        self.clear_shutdown_frameworks();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for ACLs {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct ACLs_permissive_acc_type;
static ACLs_permissive_acc: ACLs_permissive_acc_type = ACLs_permissive_acc_type;

impl ::protobuf::reflect::FieldAccessor<ACLs> for ACLs_permissive_acc_type {
    fn name(&self) -> &'static str {
        "permissive"
    }

    fn has_field(&self, m: &ACLs) -> bool {
        m.has_permissive()
    }

    fn get_bool(&self, m: &ACLs) -> bool {
        m.get_permissive()
    }
}

#[allow(non_camel_case_types)]
struct ACLs_register_frameworks_acc_type;
static ACLs_register_frameworks_acc: ACLs_register_frameworks_acc_type = ACLs_register_frameworks_acc_type;

impl ::protobuf::reflect::FieldAccessor<ACLs> for ACLs_register_frameworks_acc_type {
    fn name(&self) -> &'static str {
        "register_frameworks"
    }

    fn len_field(&self, m: &ACLs) -> uint {
        m.get_register_frameworks().len()
    }

    fn get_rep_message_item<'a>(&self, m: &'a ACLs, index: uint) -> &'a ::protobuf::Message {
        &m.get_register_frameworks()[index] as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct ACLs_run_tasks_acc_type;
static ACLs_run_tasks_acc: ACLs_run_tasks_acc_type = ACLs_run_tasks_acc_type;

impl ::protobuf::reflect::FieldAccessor<ACLs> for ACLs_run_tasks_acc_type {
    fn name(&self) -> &'static str {
        "run_tasks"
    }

    fn len_field(&self, m: &ACLs) -> uint {
        m.get_run_tasks().len()
    }

    fn get_rep_message_item<'a>(&self, m: &'a ACLs, index: uint) -> &'a ::protobuf::Message {
        &m.get_run_tasks()[index] as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct ACLs_shutdown_frameworks_acc_type;
static ACLs_shutdown_frameworks_acc: ACLs_shutdown_frameworks_acc_type = ACLs_shutdown_frameworks_acc_type;

impl ::protobuf::reflect::FieldAccessor<ACLs> for ACLs_shutdown_frameworks_acc_type {
    fn name(&self) -> &'static str {
        "shutdown_frameworks"
    }

    fn len_field(&self, m: &ACLs) -> uint {
        m.get_shutdown_frameworks().len()
    }

    fn get_rep_message_item<'a>(&self, m: &'a ACLs, index: uint) -> &'a ::protobuf::Message {
        &m.get_shutdown_frameworks()[index] as &'a ::protobuf::Message
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct RateLimit {
    qps: ::std::option::Option<f64>,
    principal: ::protobuf::SingularField<::std::string::String>,
    capacity: ::std::option::Option<u64>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> RateLimit {
    pub fn new() -> RateLimit {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RateLimit {
        static mut instance: ::protobuf::lazy::Lazy<RateLimit> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RateLimit,
        };
        unsafe {
            instance.get(|| {
                RateLimit {
                    qps: ::std::option::None,
                    principal: ::protobuf::SingularField::none(),
                    capacity: ::std::option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // optional double qps = 1;

    pub fn clear_qps(&mut self) {
        self.qps = None;
    }

    pub fn has_qps(&self) -> bool {
        self.qps.is_some()
    }

    // Param is passed by value, moved
    pub fn set_qps(&mut self, v: f64) {
        self.qps = Some(v);
    }

    pub fn get_qps(&self) -> f64 {
        self.qps.unwrap_or(0.)
    }

    // required string principal = 2;

    pub fn clear_principal(&mut self) {
        self.principal.clear();
    }

    pub fn has_principal(&self) -> bool {
        self.principal.is_some()
    }

    // Param is passed by value, moved
    pub fn set_principal(&mut self, v: ::std::string::String) {
        self.principal = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_principal(&'a mut self) -> &'a mut ::std::string::String {
        if self.principal.is_none() {
            self.principal.set_default();
        };
        self.principal.as_mut().unwrap()
    }

    pub fn get_principal(&'a self) -> &'a str {
        match self.principal.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // optional uint64 capacity = 3;

    pub fn clear_capacity(&mut self) {
        self.capacity = None;
    }

    pub fn has_capacity(&self) -> bool {
        self.capacity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_capacity(&mut self, v: u64) {
        self.capacity = Some(v);
    }

    pub fn get_capacity(&self) -> u64 {
        self.capacity.unwrap_or(0)
    }
}

impl ::protobuf::Message for RateLimit {
    fn new() -> RateLimit {
        RateLimit::new()
    }

    fn is_initialized(&self) -> bool {
        if self.principal.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.qps = Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.principal.set_default();
                    try!(is.read_string_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.capacity = Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        if self.qps.is_some() {
            my_size += 9;
        };
        for value in self.principal.iter() {
            my_size += ::protobuf::rt::string_size(2, value.as_slice());
        };
        for value in self.capacity.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.qps {
            Some(v) => {
                try!(os.write_double(1, v));
            },
            None => {},
        };
        match self.principal.as_ref() {
            Some(v) => {
                try!(os.write_string(2, v.as_slice()));
            },
            None => {},
        };
        match self.capacity {
            Some(v) => {
                try!(os.write_uint64(3, v));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<RateLimit>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<RateLimit>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&RateLimit_qps_acc as &'static ::protobuf::reflect::FieldAccessor<RateLimit>) });
                fields.push(unsafe { ::std::mem::transmute(&RateLimit_principal_acc as &'static ::protobuf::reflect::FieldAccessor<RateLimit>) });
                fields.push(unsafe { ::std::mem::transmute(&RateLimit_capacity_acc as &'static ::protobuf::reflect::FieldAccessor<RateLimit>) });
                ::protobuf::reflect::MessageDescriptor::new::<RateLimit>(
                    "RateLimit",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<RateLimit>()
    }
}

impl ::protobuf::Clear for RateLimit {
    fn clear(&mut self) {
        self.clear_qps();
        self.clear_principal();
        self.clear_capacity();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for RateLimit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct RateLimit_qps_acc_type;
static RateLimit_qps_acc: RateLimit_qps_acc_type = RateLimit_qps_acc_type;

impl ::protobuf::reflect::FieldAccessor<RateLimit> for RateLimit_qps_acc_type {
    fn name(&self) -> &'static str {
        "qps"
    }

    fn has_field(&self, m: &RateLimit) -> bool {
        m.has_qps()
    }

    fn get_f64(&self, m: &RateLimit) -> f64 {
        m.get_qps()
    }
}

#[allow(non_camel_case_types)]
struct RateLimit_principal_acc_type;
static RateLimit_principal_acc: RateLimit_principal_acc_type = RateLimit_principal_acc_type;

impl ::protobuf::reflect::FieldAccessor<RateLimit> for RateLimit_principal_acc_type {
    fn name(&self) -> &'static str {
        "principal"
    }

    fn has_field(&self, m: &RateLimit) -> bool {
        m.has_principal()
    }

    fn get_str<'a>(&self, m: &'a RateLimit) -> &'a str {
        m.get_principal()
    }
}

#[allow(non_camel_case_types)]
struct RateLimit_capacity_acc_type;
static RateLimit_capacity_acc: RateLimit_capacity_acc_type = RateLimit_capacity_acc_type;

impl ::protobuf::reflect::FieldAccessor<RateLimit> for RateLimit_capacity_acc_type {
    fn name(&self) -> &'static str {
        "capacity"
    }

    fn has_field(&self, m: &RateLimit) -> bool {
        m.has_capacity()
    }

    fn get_u64(&self, m: &RateLimit) -> u64 {
        m.get_capacity()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct RateLimits {
    limits: ::protobuf::RepeatedField<RateLimit>,
    aggregate_default_qps: ::std::option::Option<f64>,
    aggregate_default_capacity: ::std::option::Option<u64>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> RateLimits {
    pub fn new() -> RateLimits {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RateLimits {
        static mut instance: ::protobuf::lazy::Lazy<RateLimits> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RateLimits,
        };
        unsafe {
            instance.get(|| {
                RateLimits {
                    limits: ::protobuf::RepeatedField::new(),
                    aggregate_default_qps: ::std::option::None,
                    aggregate_default_capacity: ::std::option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // repeated .mesos.RateLimit limits = 1;

    pub fn clear_limits(&mut self) {
        self.limits.clear();
    }

    // Param is passed by value, moved
    pub fn set_limits(&mut self, v: ::protobuf::RepeatedField<RateLimit>) {
        self.limits = v;
    }

    // Mutable pointer to the field.
    pub fn mut_limits(&'a mut self) -> &'a mut ::protobuf::RepeatedField<RateLimit> {
        &mut self.limits
    }

    pub fn get_limits(&'a self) -> &'a [RateLimit] {
        self.limits.as_slice()
    }

    // optional double aggregate_default_qps = 2;

    pub fn clear_aggregate_default_qps(&mut self) {
        self.aggregate_default_qps = None;
    }

    pub fn has_aggregate_default_qps(&self) -> bool {
        self.aggregate_default_qps.is_some()
    }

    // Param is passed by value, moved
    pub fn set_aggregate_default_qps(&mut self, v: f64) {
        self.aggregate_default_qps = Some(v);
    }

    pub fn get_aggregate_default_qps(&self) -> f64 {
        self.aggregate_default_qps.unwrap_or(0.)
    }

    // optional uint64 aggregate_default_capacity = 3;

    pub fn clear_aggregate_default_capacity(&mut self) {
        self.aggregate_default_capacity = None;
    }

    pub fn has_aggregate_default_capacity(&self) -> bool {
        self.aggregate_default_capacity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_aggregate_default_capacity(&mut self, v: u64) {
        self.aggregate_default_capacity = Some(v);
    }

    pub fn get_aggregate_default_capacity(&self) -> u64 {
        self.aggregate_default_capacity.unwrap_or(0)
    }
}

impl ::protobuf::Message for RateLimits {
    fn new() -> RateLimits {
        RateLimits::new()
    }

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.limits.push_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_double());
                    self.aggregate_default_qps = Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.aggregate_default_capacity = Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.limits.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.aggregate_default_qps.is_some() {
            my_size += 9;
        };
        for value in self.aggregate_default_capacity.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        for v in self.limits.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(sizes[*sizes_pos]));
            *sizes_pos += 1;
            try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
        };
        match self.aggregate_default_qps {
            Some(v) => {
                try!(os.write_double(2, v));
            },
            None => {},
        };
        match self.aggregate_default_capacity {
            Some(v) => {
                try!(os.write_uint64(3, v));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<RateLimits>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<RateLimits>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&RateLimits_limits_acc as &'static ::protobuf::reflect::FieldAccessor<RateLimits>) });
                fields.push(unsafe { ::std::mem::transmute(&RateLimits_aggregate_default_qps_acc as &'static ::protobuf::reflect::FieldAccessor<RateLimits>) });
                fields.push(unsafe { ::std::mem::transmute(&RateLimits_aggregate_default_capacity_acc as &'static ::protobuf::reflect::FieldAccessor<RateLimits>) });
                ::protobuf::reflect::MessageDescriptor::new::<RateLimits>(
                    "RateLimits",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<RateLimits>()
    }
}

impl ::protobuf::Clear for RateLimits {
    fn clear(&mut self) {
        self.clear_limits();
        self.clear_aggregate_default_qps();
        self.clear_aggregate_default_capacity();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for RateLimits {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct RateLimits_limits_acc_type;
static RateLimits_limits_acc: RateLimits_limits_acc_type = RateLimits_limits_acc_type;

impl ::protobuf::reflect::FieldAccessor<RateLimits> for RateLimits_limits_acc_type {
    fn name(&self) -> &'static str {
        "limits"
    }

    fn len_field(&self, m: &RateLimits) -> uint {
        m.get_limits().len()
    }

    fn get_rep_message_item<'a>(&self, m: &'a RateLimits, index: uint) -> &'a ::protobuf::Message {
        &m.get_limits()[index] as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct RateLimits_aggregate_default_qps_acc_type;
static RateLimits_aggregate_default_qps_acc: RateLimits_aggregate_default_qps_acc_type = RateLimits_aggregate_default_qps_acc_type;

impl ::protobuf::reflect::FieldAccessor<RateLimits> for RateLimits_aggregate_default_qps_acc_type {
    fn name(&self) -> &'static str {
        "aggregate_default_qps"
    }

    fn has_field(&self, m: &RateLimits) -> bool {
        m.has_aggregate_default_qps()
    }

    fn get_f64(&self, m: &RateLimits) -> f64 {
        m.get_aggregate_default_qps()
    }
}

#[allow(non_camel_case_types)]
struct RateLimits_aggregate_default_capacity_acc_type;
static RateLimits_aggregate_default_capacity_acc: RateLimits_aggregate_default_capacity_acc_type = RateLimits_aggregate_default_capacity_acc_type;

impl ::protobuf::reflect::FieldAccessor<RateLimits> for RateLimits_aggregate_default_capacity_acc_type {
    fn name(&self) -> &'static str {
        "aggregate_default_capacity"
    }

    fn has_field(&self, m: &RateLimits) -> bool {
        m.has_aggregate_default_capacity()
    }

    fn get_u64(&self, m: &RateLimits) -> u64 {
        m.get_aggregate_default_capacity()
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct Volume {
    container_path: ::protobuf::SingularField<::std::string::String>,
    host_path: ::protobuf::SingularField<::std::string::String>,
    mode: ::std::option::Option<Volume_Mode>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> Volume {
    pub fn new() -> Volume {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Volume {
        static mut instance: ::protobuf::lazy::Lazy<Volume> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Volume,
        };
        unsafe {
            instance.get(|| {
                Volume {
                    container_path: ::protobuf::SingularField::none(),
                    host_path: ::protobuf::SingularField::none(),
                    mode: ::std::option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // required string container_path = 1;

    pub fn clear_container_path(&mut self) {
        self.container_path.clear();
    }

    pub fn has_container_path(&self) -> bool {
        self.container_path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_container_path(&mut self, v: ::std::string::String) {
        self.container_path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_container_path(&'a mut self) -> &'a mut ::std::string::String {
        if self.container_path.is_none() {
            self.container_path.set_default();
        };
        self.container_path.as_mut().unwrap()
    }

    pub fn get_container_path(&'a self) -> &'a str {
        match self.container_path.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // optional string host_path = 2;

    pub fn clear_host_path(&mut self) {
        self.host_path.clear();
    }

    pub fn has_host_path(&self) -> bool {
        self.host_path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_host_path(&mut self, v: ::std::string::String) {
        self.host_path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_host_path(&'a mut self) -> &'a mut ::std::string::String {
        if self.host_path.is_none() {
            self.host_path.set_default();
        };
        self.host_path.as_mut().unwrap()
    }

    pub fn get_host_path(&'a self) -> &'a str {
        match self.host_path.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // required .mesos.Volume.Mode mode = 3;

    pub fn clear_mode(&mut self) {
        self.mode = None;
    }

    pub fn has_mode(&self) -> bool {
        self.mode.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mode(&mut self, v: Volume_Mode) {
        self.mode = Some(v);
    }

    pub fn get_mode(&self) -> Volume_Mode {
        self.mode.unwrap_or(Volume_Mode::RW)
    }
}

impl ::protobuf::Message for Volume {
    fn new() -> Volume {
        Volume::new()
    }

    fn is_initialized(&self) -> bool {
        if self.container_path.is_none() {
            return false;
        };
        if self.mode.is_none() {
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
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.container_path.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.host_path.set_default();
                    try!(is.read_string_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = Volume_Mode::new(try!(is.read_int32()));
                    self.mode = Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.container_path.iter() {
            my_size += ::protobuf::rt::string_size(1, value.as_slice());
        };
        for value in self.host_path.iter() {
            my_size += ::protobuf::rt::string_size(2, value.as_slice());
        };
        for value in self.mode.iter() {
            my_size += ::protobuf::rt::enum_size(3, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.container_path.as_ref() {
            Some(v) => {
                try!(os.write_string(1, v.as_slice()));
            },
            None => {},
        };
        match self.host_path.as_ref() {
            Some(v) => {
                try!(os.write_string(2, v.as_slice()));
            },
            None => {},
        };
        match self.mode {
            Some(v) => {
                try!(os.write_enum(3, v as i32));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<Volume>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<Volume>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&Volume_container_path_acc as &'static ::protobuf::reflect::FieldAccessor<Volume>) });
                fields.push(unsafe { ::std::mem::transmute(&Volume_host_path_acc as &'static ::protobuf::reflect::FieldAccessor<Volume>) });
                fields.push(unsafe { ::std::mem::transmute(&Volume_mode_acc as &'static ::protobuf::reflect::FieldAccessor<Volume>) });
                ::protobuf::reflect::MessageDescriptor::new::<Volume>(
                    "Volume",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<Volume>()
    }
}

impl ::protobuf::Clear for Volume {
    fn clear(&mut self) {
        self.clear_container_path();
        self.clear_host_path();
        self.clear_mode();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for Volume {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct Volume_container_path_acc_type;
static Volume_container_path_acc: Volume_container_path_acc_type = Volume_container_path_acc_type;

impl ::protobuf::reflect::FieldAccessor<Volume> for Volume_container_path_acc_type {
    fn name(&self) -> &'static str {
        "container_path"
    }

    fn has_field(&self, m: &Volume) -> bool {
        m.has_container_path()
    }

    fn get_str<'a>(&self, m: &'a Volume) -> &'a str {
        m.get_container_path()
    }
}

#[allow(non_camel_case_types)]
struct Volume_host_path_acc_type;
static Volume_host_path_acc: Volume_host_path_acc_type = Volume_host_path_acc_type;

impl ::protobuf::reflect::FieldAccessor<Volume> for Volume_host_path_acc_type {
    fn name(&self) -> &'static str {
        "host_path"
    }

    fn has_field(&self, m: &Volume) -> bool {
        m.has_host_path()
    }

    fn get_str<'a>(&self, m: &'a Volume) -> &'a str {
        m.get_host_path()
    }
}

#[allow(non_camel_case_types)]
struct Volume_mode_acc_type;
static Volume_mode_acc: Volume_mode_acc_type = Volume_mode_acc_type;

impl ::protobuf::reflect::FieldAccessor<Volume> for Volume_mode_acc_type {
    fn name(&self) -> &'static str {
        "mode"
    }

    fn has_field(&self, m: &Volume) -> bool {
        m.has_mode()
    }

    fn get_enum<'a>(&self, m: &Volume) -> &'static ::protobuf::reflect::EnumValueDescriptor {
        use protobuf::{ProtobufEnum};
        m.get_mode().descriptor()
    }
}

#[deriving(Clone,PartialEq,Eq,Show)]
pub enum Volume_Mode {
    RW = 1,
    RO = 2,
}

impl Volume_Mode {
    pub fn new(value: i32) -> Volume_Mode {
        match value {
            1 => Volume_Mode::RW,
            2 => Volume_Mode::RO,
            _ => panic!()
        }
    }
}

impl ::protobuf::ProtobufEnum for Volume_Mode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn enum_descriptor_static(_: Option<Volume_Mode>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Volume_Mode", file_descriptor_proto())
            })
        }
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct ContainerInfo {
    field_type: ::std::option::Option<ContainerInfo_Type>,
    volumes: ::protobuf::RepeatedField<Volume>,
    hostname: ::protobuf::SingularField<::std::string::String>,
    docker: ::protobuf::SingularPtrField<ContainerInfo_DockerInfo>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> ContainerInfo {
    pub fn new() -> ContainerInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ContainerInfo {
        static mut instance: ::protobuf::lazy::Lazy<ContainerInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ContainerInfo,
        };
        unsafe {
            instance.get(|| {
                ContainerInfo {
                    field_type: ::std::option::None,
                    volumes: ::protobuf::RepeatedField::new(),
                    hostname: ::protobuf::SingularField::none(),
                    docker: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // required .mesos.ContainerInfo.Type type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ContainerInfo_Type) {
        self.field_type = Some(v);
    }

    pub fn get_field_type(&self) -> ContainerInfo_Type {
        self.field_type.unwrap_or(ContainerInfo_Type::DOCKER)
    }

    // repeated .mesos.Volume volumes = 2;

    pub fn clear_volumes(&mut self) {
        self.volumes.clear();
    }

    // Param is passed by value, moved
    pub fn set_volumes(&mut self, v: ::protobuf::RepeatedField<Volume>) {
        self.volumes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_volumes(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Volume> {
        &mut self.volumes
    }

    pub fn get_volumes(&'a self) -> &'a [Volume] {
        self.volumes.as_slice()
    }

    // optional string hostname = 4;

    pub fn clear_hostname(&mut self) {
        self.hostname.clear();
    }

    pub fn has_hostname(&self) -> bool {
        self.hostname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hostname(&mut self, v: ::std::string::String) {
        self.hostname = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hostname(&'a mut self) -> &'a mut ::std::string::String {
        if self.hostname.is_none() {
            self.hostname.set_default();
        };
        self.hostname.as_mut().unwrap()
    }

    pub fn get_hostname(&'a self) -> &'a str {
        match self.hostname.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // optional .mesos.ContainerInfo.DockerInfo docker = 3;

    pub fn clear_docker(&mut self) {
        self.docker.clear();
    }

    pub fn has_docker(&self) -> bool {
        self.docker.is_some()
    }

    // Param is passed by value, moved
    pub fn set_docker(&mut self, v: ContainerInfo_DockerInfo) {
        self.docker = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_docker(&'a mut self) -> &'a mut ContainerInfo_DockerInfo {
        if self.docker.is_none() {
            self.docker.set_default();
        };
        self.docker.as_mut().unwrap()
    }

    pub fn get_docker(&'a self) -> &'a ContainerInfo_DockerInfo {
        self.docker.as_ref().unwrap_or_else(|| ContainerInfo_DockerInfo::default_instance())
    }
}

impl ::protobuf::Message for ContainerInfo {
    fn new() -> ContainerInfo {
        ContainerInfo::new()
    }

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
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = ContainerInfo_Type::new(try!(is.read_int32()));
                    self.field_type = Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.volumes.push_default();
                    try!(is.merge_message(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.hostname.set_default();
                    try!(is.read_string_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.docker.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.field_type.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.volumes.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.hostname.iter() {
            my_size += ::protobuf::rt::string_size(4, value.as_slice());
        };
        for value in self.docker.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.field_type {
            Some(v) => {
                try!(os.write_enum(1, v as i32));
            },
            None => {},
        };
        for v in self.volumes.iter() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(sizes[*sizes_pos]));
            *sizes_pos += 1;
            try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
        };
        match self.hostname.as_ref() {
            Some(v) => {
                try!(os.write_string(4, v.as_slice()));
            },
            None => {},
        };
        match self.docker.as_ref() {
            Some(v) => {
                try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
                try!(os.write_raw_varint32(sizes[*sizes_pos]));
                *sizes_pos += 1;
                try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<ContainerInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<ContainerInfo>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&ContainerInfo_field_type_acc as &'static ::protobuf::reflect::FieldAccessor<ContainerInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&ContainerInfo_volumes_acc as &'static ::protobuf::reflect::FieldAccessor<ContainerInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&ContainerInfo_hostname_acc as &'static ::protobuf::reflect::FieldAccessor<ContainerInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&ContainerInfo_docker_acc as &'static ::protobuf::reflect::FieldAccessor<ContainerInfo>) });
                ::protobuf::reflect::MessageDescriptor::new::<ContainerInfo>(
                    "ContainerInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<ContainerInfo>()
    }
}

impl ::protobuf::Clear for ContainerInfo {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_volumes();
        self.clear_hostname();
        self.clear_docker();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for ContainerInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct ContainerInfo_field_type_acc_type;
static ContainerInfo_field_type_acc: ContainerInfo_field_type_acc_type = ContainerInfo_field_type_acc_type;

impl ::protobuf::reflect::FieldAccessor<ContainerInfo> for ContainerInfo_field_type_acc_type {
    fn name(&self) -> &'static str {
        "field_type"
    }

    fn has_field(&self, m: &ContainerInfo) -> bool {
        m.has_field_type()
    }

    fn get_enum<'a>(&self, m: &ContainerInfo) -> &'static ::protobuf::reflect::EnumValueDescriptor {
        use protobuf::{ProtobufEnum};
        m.get_field_type().descriptor()
    }
}

#[allow(non_camel_case_types)]
struct ContainerInfo_volumes_acc_type;
static ContainerInfo_volumes_acc: ContainerInfo_volumes_acc_type = ContainerInfo_volumes_acc_type;

impl ::protobuf::reflect::FieldAccessor<ContainerInfo> for ContainerInfo_volumes_acc_type {
    fn name(&self) -> &'static str {
        "volumes"
    }

    fn len_field(&self, m: &ContainerInfo) -> uint {
        m.get_volumes().len()
    }

    fn get_rep_message_item<'a>(&self, m: &'a ContainerInfo, index: uint) -> &'a ::protobuf::Message {
        &m.get_volumes()[index] as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct ContainerInfo_hostname_acc_type;
static ContainerInfo_hostname_acc: ContainerInfo_hostname_acc_type = ContainerInfo_hostname_acc_type;

impl ::protobuf::reflect::FieldAccessor<ContainerInfo> for ContainerInfo_hostname_acc_type {
    fn name(&self) -> &'static str {
        "hostname"
    }

    fn has_field(&self, m: &ContainerInfo) -> bool {
        m.has_hostname()
    }

    fn get_str<'a>(&self, m: &'a ContainerInfo) -> &'a str {
        m.get_hostname()
    }
}

#[allow(non_camel_case_types)]
struct ContainerInfo_docker_acc_type;
static ContainerInfo_docker_acc: ContainerInfo_docker_acc_type = ContainerInfo_docker_acc_type;

impl ::protobuf::reflect::FieldAccessor<ContainerInfo> for ContainerInfo_docker_acc_type {
    fn name(&self) -> &'static str {
        "docker"
    }

    fn has_field(&self, m: &ContainerInfo) -> bool {
        m.has_docker()
    }

    fn get_message<'a>(&self, m: &'a ContainerInfo) -> &'a ::protobuf::Message {
        m.get_docker() as &'a ::protobuf::Message
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct ContainerInfo_DockerInfo {
    image: ::protobuf::SingularField<::std::string::String>,
    network: ::std::option::Option<ContainerInfo_DockerInfo_Network>,
    port_mappings: ::protobuf::RepeatedField<ContainerInfo_DockerInfo_PortMapping>,
    privileged: ::std::option::Option<bool>,
    parameters: ::protobuf::RepeatedField<Parameter>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> ContainerInfo_DockerInfo {
    pub fn new() -> ContainerInfo_DockerInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ContainerInfo_DockerInfo {
        static mut instance: ::protobuf::lazy::Lazy<ContainerInfo_DockerInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ContainerInfo_DockerInfo,
        };
        unsafe {
            instance.get(|| {
                ContainerInfo_DockerInfo {
                    image: ::protobuf::SingularField::none(),
                    network: ::std::option::None,
                    port_mappings: ::protobuf::RepeatedField::new(),
                    privileged: ::std::option::None,
                    parameters: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // required string image = 1;

    pub fn clear_image(&mut self) {
        self.image.clear();
    }

    pub fn has_image(&self) -> bool {
        self.image.is_some()
    }

    // Param is passed by value, moved
    pub fn set_image(&mut self, v: ::std::string::String) {
        self.image = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_image(&'a mut self) -> &'a mut ::std::string::String {
        if self.image.is_none() {
            self.image.set_default();
        };
        self.image.as_mut().unwrap()
    }

    pub fn get_image(&'a self) -> &'a str {
        match self.image.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // optional .mesos.ContainerInfo.DockerInfo.Network network = 2;

    pub fn clear_network(&mut self) {
        self.network = None;
    }

    pub fn has_network(&self) -> bool {
        self.network.is_some()
    }

    // Param is passed by value, moved
    pub fn set_network(&mut self, v: ContainerInfo_DockerInfo_Network) {
        self.network = Some(v);
    }

    pub fn get_network(&self) -> ContainerInfo_DockerInfo_Network {
        self.network.unwrap_or(ContainerInfo_DockerInfo_Network::HOST)
    }

    // repeated .mesos.ContainerInfo.DockerInfo.PortMapping port_mappings = 3;

    pub fn clear_port_mappings(&mut self) {
        self.port_mappings.clear();
    }

    // Param is passed by value, moved
    pub fn set_port_mappings(&mut self, v: ::protobuf::RepeatedField<ContainerInfo_DockerInfo_PortMapping>) {
        self.port_mappings = v;
    }

    // Mutable pointer to the field.
    pub fn mut_port_mappings(&'a mut self) -> &'a mut ::protobuf::RepeatedField<ContainerInfo_DockerInfo_PortMapping> {
        &mut self.port_mappings
    }

    pub fn get_port_mappings(&'a self) -> &'a [ContainerInfo_DockerInfo_PortMapping] {
        self.port_mappings.as_slice()
    }

    // optional bool privileged = 4;

    pub fn clear_privileged(&mut self) {
        self.privileged = None;
    }

    pub fn has_privileged(&self) -> bool {
        self.privileged.is_some()
    }

    // Param is passed by value, moved
    pub fn set_privileged(&mut self, v: bool) {
        self.privileged = Some(v);
    }

    pub fn get_privileged(&self) -> bool {
        self.privileged.unwrap_or(false)
    }

    // repeated .mesos.Parameter parameters = 5;

    pub fn clear_parameters(&mut self) {
        self.parameters.clear();
    }

    // Param is passed by value, moved
    pub fn set_parameters(&mut self, v: ::protobuf::RepeatedField<Parameter>) {
        self.parameters = v;
    }

    // Mutable pointer to the field.
    pub fn mut_parameters(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Parameter> {
        &mut self.parameters
    }

    pub fn get_parameters(&'a self) -> &'a [Parameter] {
        self.parameters.as_slice()
    }
}

impl ::protobuf::Message for ContainerInfo_DockerInfo {
    fn new() -> ContainerInfo_DockerInfo {
        ContainerInfo_DockerInfo::new()
    }

    fn is_initialized(&self) -> bool {
        if self.image.is_none() {
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
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.image.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = ContainerInfo_DockerInfo_Network::new(try!(is.read_int32()));
                    self.network = Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.port_mappings.push_default();
                    try!(is.merge_message(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.privileged = Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.parameters.push_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.image.iter() {
            my_size += ::protobuf::rt::string_size(1, value.as_slice());
        };
        for value in self.network.iter() {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in self.port_mappings.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.privileged.is_some() {
            my_size += 2;
        };
        for value in self.parameters.iter() {
            let len = value.compute_sizes(sizes);
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.image.as_ref() {
            Some(v) => {
                try!(os.write_string(1, v.as_slice()));
            },
            None => {},
        };
        match self.network {
            Some(v) => {
                try!(os.write_enum(2, v as i32));
            },
            None => {},
        };
        for v in self.port_mappings.iter() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(sizes[*sizes_pos]));
            *sizes_pos += 1;
            try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
        };
        match self.privileged {
            Some(v) => {
                try!(os.write_bool(4, v));
            },
            None => {},
        };
        for v in self.parameters.iter() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(sizes[*sizes_pos]));
            *sizes_pos += 1;
            try!(v.write_to_with_computed_sizes(os, sizes.as_slice(), sizes_pos));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<ContainerInfo_DockerInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<ContainerInfo_DockerInfo>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&ContainerInfo_DockerInfo_image_acc as &'static ::protobuf::reflect::FieldAccessor<ContainerInfo_DockerInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&ContainerInfo_DockerInfo_network_acc as &'static ::protobuf::reflect::FieldAccessor<ContainerInfo_DockerInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&ContainerInfo_DockerInfo_port_mappings_acc as &'static ::protobuf::reflect::FieldAccessor<ContainerInfo_DockerInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&ContainerInfo_DockerInfo_privileged_acc as &'static ::protobuf::reflect::FieldAccessor<ContainerInfo_DockerInfo>) });
                fields.push(unsafe { ::std::mem::transmute(&ContainerInfo_DockerInfo_parameters_acc as &'static ::protobuf::reflect::FieldAccessor<ContainerInfo_DockerInfo>) });
                ::protobuf::reflect::MessageDescriptor::new::<ContainerInfo_DockerInfo>(
                    "ContainerInfo_DockerInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<ContainerInfo_DockerInfo>()
    }
}

impl ::protobuf::Clear for ContainerInfo_DockerInfo {
    fn clear(&mut self) {
        self.clear_image();
        self.clear_network();
        self.clear_port_mappings();
        self.clear_privileged();
        self.clear_parameters();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for ContainerInfo_DockerInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct ContainerInfo_DockerInfo_image_acc_type;
static ContainerInfo_DockerInfo_image_acc: ContainerInfo_DockerInfo_image_acc_type = ContainerInfo_DockerInfo_image_acc_type;

impl ::protobuf::reflect::FieldAccessor<ContainerInfo_DockerInfo> for ContainerInfo_DockerInfo_image_acc_type {
    fn name(&self) -> &'static str {
        "image"
    }

    fn has_field(&self, m: &ContainerInfo_DockerInfo) -> bool {
        m.has_image()
    }

    fn get_str<'a>(&self, m: &'a ContainerInfo_DockerInfo) -> &'a str {
        m.get_image()
    }
}

#[allow(non_camel_case_types)]
struct ContainerInfo_DockerInfo_network_acc_type;
static ContainerInfo_DockerInfo_network_acc: ContainerInfo_DockerInfo_network_acc_type = ContainerInfo_DockerInfo_network_acc_type;

impl ::protobuf::reflect::FieldAccessor<ContainerInfo_DockerInfo> for ContainerInfo_DockerInfo_network_acc_type {
    fn name(&self) -> &'static str {
        "network"
    }

    fn has_field(&self, m: &ContainerInfo_DockerInfo) -> bool {
        m.has_network()
    }

    fn get_enum<'a>(&self, m: &ContainerInfo_DockerInfo) -> &'static ::protobuf::reflect::EnumValueDescriptor {
        use protobuf::{ProtobufEnum};
        m.get_network().descriptor()
    }
}

#[allow(non_camel_case_types)]
struct ContainerInfo_DockerInfo_port_mappings_acc_type;
static ContainerInfo_DockerInfo_port_mappings_acc: ContainerInfo_DockerInfo_port_mappings_acc_type = ContainerInfo_DockerInfo_port_mappings_acc_type;

impl ::protobuf::reflect::FieldAccessor<ContainerInfo_DockerInfo> for ContainerInfo_DockerInfo_port_mappings_acc_type {
    fn name(&self) -> &'static str {
        "port_mappings"
    }

    fn len_field(&self, m: &ContainerInfo_DockerInfo) -> uint {
        m.get_port_mappings().len()
    }

    fn get_rep_message_item<'a>(&self, m: &'a ContainerInfo_DockerInfo, index: uint) -> &'a ::protobuf::Message {
        &m.get_port_mappings()[index] as &'a ::protobuf::Message
    }
}

#[allow(non_camel_case_types)]
struct ContainerInfo_DockerInfo_privileged_acc_type;
static ContainerInfo_DockerInfo_privileged_acc: ContainerInfo_DockerInfo_privileged_acc_type = ContainerInfo_DockerInfo_privileged_acc_type;

impl ::protobuf::reflect::FieldAccessor<ContainerInfo_DockerInfo> for ContainerInfo_DockerInfo_privileged_acc_type {
    fn name(&self) -> &'static str {
        "privileged"
    }

    fn has_field(&self, m: &ContainerInfo_DockerInfo) -> bool {
        m.has_privileged()
    }

    fn get_bool(&self, m: &ContainerInfo_DockerInfo) -> bool {
        m.get_privileged()
    }
}

#[allow(non_camel_case_types)]
struct ContainerInfo_DockerInfo_parameters_acc_type;
static ContainerInfo_DockerInfo_parameters_acc: ContainerInfo_DockerInfo_parameters_acc_type = ContainerInfo_DockerInfo_parameters_acc_type;

impl ::protobuf::reflect::FieldAccessor<ContainerInfo_DockerInfo> for ContainerInfo_DockerInfo_parameters_acc_type {
    fn name(&self) -> &'static str {
        "parameters"
    }

    fn len_field(&self, m: &ContainerInfo_DockerInfo) -> uint {
        m.get_parameters().len()
    }

    fn get_rep_message_item<'a>(&self, m: &'a ContainerInfo_DockerInfo, index: uint) -> &'a ::protobuf::Message {
        &m.get_parameters()[index] as &'a ::protobuf::Message
    }
}

#[deriving(Clone,PartialEq,Default)]
pub struct ContainerInfo_DockerInfo_PortMapping {
    host_port: ::std::option::Option<u32>,
    container_port: ::std::option::Option<u32>,
    protocol: ::protobuf::SingularField<::std::string::String>,
    unknown_fields: ::protobuf::UnknownFields,
}

impl<'a> ContainerInfo_DockerInfo_PortMapping {
    pub fn new() -> ContainerInfo_DockerInfo_PortMapping {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ContainerInfo_DockerInfo_PortMapping {
        static mut instance: ::protobuf::lazy::Lazy<ContainerInfo_DockerInfo_PortMapping> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ContainerInfo_DockerInfo_PortMapping,
        };
        unsafe {
            instance.get(|| {
                ContainerInfo_DockerInfo_PortMapping {
                    host_port: ::std::option::None,
                    container_port: ::std::option::None,
                    protocol: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                }
            })
        }
    }

    // required uint32 host_port = 1;

    pub fn clear_host_port(&mut self) {
        self.host_port = None;
    }

    pub fn has_host_port(&self) -> bool {
        self.host_port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_host_port(&mut self, v: u32) {
        self.host_port = Some(v);
    }

    pub fn get_host_port(&self) -> u32 {
        self.host_port.unwrap_or(0)
    }

    // required uint32 container_port = 2;

    pub fn clear_container_port(&mut self) {
        self.container_port = None;
    }

    pub fn has_container_port(&self) -> bool {
        self.container_port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_container_port(&mut self, v: u32) {
        self.container_port = Some(v);
    }

    pub fn get_container_port(&self) -> u32 {
        self.container_port.unwrap_or(0)
    }

    // optional string protocol = 3;

    pub fn clear_protocol(&mut self) {
        self.protocol.clear();
    }

    pub fn has_protocol(&self) -> bool {
        self.protocol.is_some()
    }

    // Param is passed by value, moved
    pub fn set_protocol(&mut self, v: ::std::string::String) {
        self.protocol = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_protocol(&'a mut self) -> &'a mut ::std::string::String {
        if self.protocol.is_none() {
            self.protocol.set_default();
        };
        self.protocol.as_mut().unwrap()
    }

    pub fn get_protocol(&'a self) -> &'a str {
        match self.protocol.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }
}

impl ::protobuf::Message for ContainerInfo_DockerInfo_PortMapping {
    fn new() -> ContainerInfo_DockerInfo_PortMapping {
        ContainerInfo_DockerInfo_PortMapping::new()
    }

    fn is_initialized(&self) -> bool {
        if self.host_port.is_none() {
            return false;
        };
        if self.container_port.is_none() {
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
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.host_port = Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.container_port = Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.protocol.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_sizes(&self, sizes: &mut ::std::vec::Vec<u32>) -> u32 {
        use protobuf::{Message};
        let pos = sizes.len();
        sizes.push(0);
        let mut my_size = 0;
        for value in self.host_port.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.container_port.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.protocol.iter() {
            my_size += ::protobuf::rt::string_size(3, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        sizes[pos] = my_size;
        // value is returned for convenience
        my_size
    }

    #[allow(unused_variables)]
    fn write_to_with_computed_sizes(&self, os: &mut ::protobuf::CodedOutputStream, sizes: &[u32], sizes_pos: &mut uint) -> ::protobuf::ProtobufResult<()> {
        use protobuf::{Message};
        match self.host_port {
            Some(v) => {
                try!(os.write_uint32(1, v));
            },
            None => {},
        };
        match self.container_port {
            Some(v) => {
                try!(os.write_uint32(2, v));
            },
            None => {},
        };
        match self.protocol.as_ref() {
            Some(v) => {
                try!(os.write_string(3, v.as_slice()));
            },
            None => {},
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Ok(())
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<ContainerInfo_DockerInfo_PortMapping>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields: ::std::vec::Vec<&'static ::protobuf::reflect::FieldAccessor<ContainerInfo_DockerInfo_PortMapping>> = ::std::vec::Vec::new();
                fields.push(unsafe { ::std::mem::transmute(&ContainerInfo_DockerInfo_PortMapping_host_port_acc as &'static ::protobuf::reflect::FieldAccessor<ContainerInfo_DockerInfo_PortMapping>) });
                fields.push(unsafe { ::std::mem::transmute(&ContainerInfo_DockerInfo_PortMapping_container_port_acc as &'static ::protobuf::reflect::FieldAccessor<ContainerInfo_DockerInfo_PortMapping>) });
                fields.push(unsafe { ::std::mem::transmute(&ContainerInfo_DockerInfo_PortMapping_protocol_acc as &'static ::protobuf::reflect::FieldAccessor<ContainerInfo_DockerInfo_PortMapping>) });
                ::protobuf::reflect::MessageDescriptor::new::<ContainerInfo_DockerInfo_PortMapping>(
                    "ContainerInfo_DockerInfo_PortMapping",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn type_id(&self) -> ::std::intrinsics::TypeId {
        ::std::intrinsics::TypeId::of::<ContainerInfo_DockerInfo_PortMapping>()
    }
}

impl ::protobuf::Clear for ContainerInfo_DockerInfo_PortMapping {
    fn clear(&mut self) {
        self.clear_host_port();
        self.clear_container_port();
        self.clear_protocol();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Show for ContainerInfo_DockerInfo_PortMapping {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use protobuf::{Message};
        self.fmt_impl(f)
    }
}


#[allow(non_camel_case_types)]
struct ContainerInfo_DockerInfo_PortMapping_host_port_acc_type;
static ContainerInfo_DockerInfo_PortMapping_host_port_acc: ContainerInfo_DockerInfo_PortMapping_host_port_acc_type = ContainerInfo_DockerInfo_PortMapping_host_port_acc_type;

impl ::protobuf::reflect::FieldAccessor<ContainerInfo_DockerInfo_PortMapping> for ContainerInfo_DockerInfo_PortMapping_host_port_acc_type {
    fn name(&self) -> &'static str {
        "host_port"
    }

    fn has_field(&self, m: &ContainerInfo_DockerInfo_PortMapping) -> bool {
        m.has_host_port()
    }

    fn get_u32(&self, m: &ContainerInfo_DockerInfo_PortMapping) -> u32 {
        m.get_host_port()
    }
}

#[allow(non_camel_case_types)]
struct ContainerInfo_DockerInfo_PortMapping_container_port_acc_type;
static ContainerInfo_DockerInfo_PortMapping_container_port_acc: ContainerInfo_DockerInfo_PortMapping_container_port_acc_type = ContainerInfo_DockerInfo_PortMapping_container_port_acc_type;

impl ::protobuf::reflect::FieldAccessor<ContainerInfo_DockerInfo_PortMapping> for ContainerInfo_DockerInfo_PortMapping_container_port_acc_type {
    fn name(&self) -> &'static str {
        "container_port"
    }

    fn has_field(&self, m: &ContainerInfo_DockerInfo_PortMapping) -> bool {
        m.has_container_port()
    }

    fn get_u32(&self, m: &ContainerInfo_DockerInfo_PortMapping) -> u32 {
        m.get_container_port()
    }
}

#[allow(non_camel_case_types)]
struct ContainerInfo_DockerInfo_PortMapping_protocol_acc_type;
static ContainerInfo_DockerInfo_PortMapping_protocol_acc: ContainerInfo_DockerInfo_PortMapping_protocol_acc_type = ContainerInfo_DockerInfo_PortMapping_protocol_acc_type;

impl ::protobuf::reflect::FieldAccessor<ContainerInfo_DockerInfo_PortMapping> for ContainerInfo_DockerInfo_PortMapping_protocol_acc_type {
    fn name(&self) -> &'static str {
        "protocol"
    }

    fn has_field(&self, m: &ContainerInfo_DockerInfo_PortMapping) -> bool {
        m.has_protocol()
    }

    fn get_str<'a>(&self, m: &'a ContainerInfo_DockerInfo_PortMapping) -> &'a str {
        m.get_protocol()
    }
}

#[deriving(Clone,PartialEq,Eq,Show)]
pub enum ContainerInfo_DockerInfo_Network {
    HOST = 1,
    BRIDGE = 2,
    NONE = 3,
}

impl ContainerInfo_DockerInfo_Network {
    pub fn new(value: i32) -> ContainerInfo_DockerInfo_Network {
        match value {
            1 => ContainerInfo_DockerInfo_Network::HOST,
            2 => ContainerInfo_DockerInfo_Network::BRIDGE,
            3 => ContainerInfo_DockerInfo_Network::NONE,
            _ => panic!()
        }
    }
}

impl ::protobuf::ProtobufEnum for ContainerInfo_DockerInfo_Network {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn enum_descriptor_static(_: Option<ContainerInfo_DockerInfo_Network>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ContainerInfo_DockerInfo_Network", file_descriptor_proto())
            })
        }
    }
}

#[deriving(Clone,PartialEq,Eq,Show)]
pub enum ContainerInfo_Type {
    DOCKER = 1,
    MESOS = 2,
}

impl ContainerInfo_Type {
    pub fn new(value: i32) -> ContainerInfo_Type {
        match value {
            1 => ContainerInfo_Type::DOCKER,
            2 => ContainerInfo_Type::MESOS,
            _ => panic!()
        }
    }
}

impl ::protobuf::ProtobufEnum for ContainerInfo_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn enum_descriptor_static(_: Option<ContainerInfo_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ContainerInfo_Type", file_descriptor_proto())
            })
        }
    }
}

#[deriving(Clone,PartialEq,Eq,Show)]
pub enum Status {
    DRIVER_NOT_STARTED = 1,
    DRIVER_RUNNING = 2,
    DRIVER_ABORTED = 3,
    DRIVER_STOPPED = 4,
}

impl Status {
    pub fn new(value: i32) -> Status {
        match value {
            1 => Status::DRIVER_NOT_STARTED,
            2 => Status::DRIVER_RUNNING,
            3 => Status::DRIVER_ABORTED,
            4 => Status::DRIVER_STOPPED,
            _ => panic!()
        }
    }
}

impl ::protobuf::ProtobufEnum for Status {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn enum_descriptor_static(_: Option<Status>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Status", file_descriptor_proto())
            })
        }
    }
}

#[deriving(Clone,PartialEq,Eq,Show)]
pub enum TaskState {
    TASK_STAGING = 6,
    TASK_STARTING = 0,
    TASK_RUNNING = 1,
    TASK_FINISHED = 2,
    TASK_FAILED = 3,
    TASK_KILLED = 4,
    TASK_LOST = 5,
    TASK_ERROR = 7,
}

impl TaskState {
    pub fn new(value: i32) -> TaskState {
        match value {
            6 => TaskState::TASK_STAGING,
            0 => TaskState::TASK_STARTING,
            1 => TaskState::TASK_RUNNING,
            2 => TaskState::TASK_FINISHED,
            3 => TaskState::TASK_FAILED,
            4 => TaskState::TASK_KILLED,
            5 => TaskState::TASK_LOST,
            7 => TaskState::TASK_ERROR,
            _ => panic!()
        }
    }
}

impl ::protobuf::ProtobufEnum for TaskState {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn enum_descriptor_static(_: Option<TaskState>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("TaskState", file_descriptor_proto())
            })
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x15, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x73, 0x2f, 0x6d, 0x65, 0x73, 0x6f,
    0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x05, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x22, 0x1c,
    0x0a, 0x0b, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x49, 0x44, 0x12, 0x0d, 0x0a,
    0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x22, 0x18, 0x0a, 0x07,
    0x4f, 0x66, 0x66, 0x65, 0x72, 0x49, 0x44, 0x12, 0x0d, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x22, 0x18, 0x0a, 0x07, 0x53, 0x6c, 0x61, 0x76, 0x65, 0x49,
    0x44, 0x12, 0x0d, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09,
    0x22, 0x17, 0x0a, 0x06, 0x54, 0x61, 0x73, 0x6b, 0x49, 0x44, 0x12, 0x0d, 0x0a, 0x05, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x22, 0x1b, 0x0a, 0x0a, 0x45, 0x78, 0x65,
    0x63, 0x75, 0x74, 0x6f, 0x72, 0x49, 0x44, 0x12, 0x0d, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x22, 0x1c, 0x0a, 0x0b, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x69,
    0x6e, 0x65, 0x72, 0x49, 0x44, 0x12, 0x0d, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x09, 0x22, 0xcc, 0x01, 0x0a, 0x0d, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f,
    0x72, 0x6b, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x0c, 0x0a, 0x04, 0x75, 0x73, 0x65, 0x72, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x09, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x02,
    0x28, 0x09, 0x12, 0x1e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12,
    0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b,
    0x49, 0x44, 0x12, 0x1b, 0x0a, 0x10, 0x66, 0x61, 0x69, 0x6c, 0x6f, 0x76, 0x65, 0x72, 0x5f, 0x74,
    0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x01, 0x3a, 0x01, 0x30, 0x12,
    0x19, 0x0a, 0x0a, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x18, 0x05, 0x20,
    0x01, 0x28, 0x08, 0x3a, 0x05, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x12, 0x0f, 0x0a, 0x04, 0x72, 0x6f,
    0x6c, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x3a, 0x01, 0x2a, 0x12, 0x10, 0x0a, 0x08, 0x68,
    0x6f, 0x73, 0x74, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x12, 0x11, 0x0a,
    0x09, 0x70, 0x72, 0x69, 0x6e, 0x63, 0x69, 0x70, 0x61, 0x6c, 0x18, 0x08, 0x20, 0x01, 0x28, 0x09,
    0x12, 0x11, 0x0a, 0x09, 0x77, 0x65, 0x62, 0x75, 0x69, 0x5f, 0x75, 0x72, 0x6c, 0x18, 0x09, 0x20,
    0x01, 0x28, 0x09, 0x22, 0xab, 0x02, 0x0a, 0x0b, 0x48, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x43, 0x68,
    0x65, 0x63, 0x6b, 0x12, 0x25, 0x0a, 0x04, 0x68, 0x74, 0x74, 0x70, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x17, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x48, 0x65, 0x61, 0x6c, 0x74, 0x68,
    0x43, 0x68, 0x65, 0x63, 0x6b, 0x2e, 0x48, 0x54, 0x54, 0x50, 0x12, 0x19, 0x0a, 0x0d, 0x64, 0x65,
    0x6c, 0x61, 0x79, 0x5f, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x01, 0x3a, 0x02, 0x31, 0x35, 0x12, 0x1c, 0x0a, 0x10, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61,
    0x6c, 0x5f, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x01, 0x3a,
    0x02, 0x31, 0x30, 0x12, 0x1b, 0x0a, 0x0f, 0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x5f, 0x73,
    0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x01, 0x3a, 0x02, 0x32, 0x30,
    0x12, 0x1f, 0x0a, 0x14, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x63, 0x75, 0x74, 0x69, 0x76, 0x65, 0x5f,
    0x66, 0x61, 0x69, 0x6c, 0x75, 0x72, 0x65, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x3a, 0x01,
    0x33, 0x12, 0x20, 0x0a, 0x14, 0x67, 0x72, 0x61, 0x63, 0x65, 0x5f, 0x70, 0x65, 0x72, 0x69, 0x6f,
    0x64, 0x5f, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x18, 0x06, 0x20, 0x01, 0x28, 0x01, 0x3a,
    0x02, 0x31, 0x30, 0x12, 0x23, 0x0a, 0x07, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x18, 0x07,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x43, 0x6f, 0x6d,
    0x6d, 0x61, 0x6e, 0x64, 0x49, 0x6e, 0x66, 0x6f, 0x1a, 0x37, 0x0a, 0x04, 0x48, 0x54, 0x54, 0x50,
    0x12, 0x0c, 0x0a, 0x04, 0x70, 0x6f, 0x72, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x0f,
    0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x3a, 0x01, 0x2f, 0x12,
    0x10, 0x0a, 0x08, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x65, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28,
    0x0d, 0x22, 0xc8, 0x02, 0x0a, 0x0b, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x49, 0x6e, 0x66,
    0x6f, 0x12, 0x33, 0x0a, 0x09, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x43, 0x6f, 0x6d,
    0x6d, 0x61, 0x6e, 0x64, 0x49, 0x6e, 0x66, 0x6f, 0x2e, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e,
    0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x24, 0x0a, 0x04, 0x75, 0x72, 0x69, 0x73, 0x18, 0x01,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x43, 0x6f, 0x6d,
    0x6d, 0x61, 0x6e, 0x64, 0x49, 0x6e, 0x66, 0x6f, 0x2e, 0x55, 0x52, 0x49, 0x12, 0x27, 0x0a, 0x0b,
    0x65, 0x6e, 0x76, 0x69, 0x72, 0x6f, 0x6e, 0x6d, 0x65, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x12, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x45, 0x6e, 0x76, 0x69, 0x72, 0x6f,
    0x6e, 0x6d, 0x65, 0x6e, 0x74, 0x12, 0x13, 0x0a, 0x05, 0x73, 0x68, 0x65, 0x6c, 0x6c, 0x18, 0x06,
    0x20, 0x01, 0x28, 0x08, 0x3a, 0x04, 0x74, 0x72, 0x75, 0x65, 0x12, 0x0d, 0x0a, 0x05, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x12, 0x11, 0x0a, 0x09, 0x61, 0x72, 0x67,
    0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x18, 0x07, 0x20, 0x03, 0x28, 0x09, 0x12, 0x0c, 0x0a, 0x04,
    0x75, 0x73, 0x65, 0x72, 0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x1a, 0x3f, 0x0a, 0x03, 0x55, 0x52,
    0x49, 0x12, 0x0d, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09,
    0x12, 0x12, 0x0a, 0x0a, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x08, 0x12, 0x15, 0x0a, 0x07, 0x65, 0x78, 0x74, 0x72, 0x61, 0x63, 0x74, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x08, 0x3a, 0x04, 0x74, 0x72, 0x75, 0x65, 0x1a, 0x2f, 0x0a, 0x0d, 0x43,
    0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x0d, 0x0a, 0x05,
    0x69, 0x6d, 0x61, 0x67, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0f, 0x0a, 0x07, 0x6f,
    0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x09, 0x22, 0xfe, 0x01, 0x0a,
    0x0c, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x26, 0x0a,
    0x0b, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x45, 0x78, 0x65, 0x63, 0x75,
    0x74, 0x6f, 0x72, 0x49, 0x44, 0x12, 0x28, 0x0a, 0x0c, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f,
    0x72, 0x6b, 0x5f, 0x69, 0x64, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x6d, 0x65,
    0x73, 0x6f, 0x73, 0x2e, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x49, 0x44, 0x12,
    0x23, 0x0a, 0x07, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x18, 0x07, 0x20, 0x02, 0x28, 0x0b,
    0x32, 0x12, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64,
    0x49, 0x6e, 0x66, 0x6f, 0x12, 0x27, 0x0a, 0x09, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65,
    0x72, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e,
    0x43, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x22, 0x0a,
    0x09, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x73, 0x18, 0x05, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x0f, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63,
    0x65, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x09, 0x20, 0x01, 0x28, 0x09, 0x12,
    0x0e, 0x0a, 0x06, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x09, 0x12,
    0x0c, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x57, 0x0a,
    0x0a, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x0a, 0x0a, 0x02, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0a, 0x0a, 0x02, 0x69, 0x70, 0x18, 0x02, 0x20,
    0x02, 0x28, 0x0d, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x6f, 0x72, 0x74, 0x18, 0x03, 0x20, 0x02, 0x28,
    0x0d, 0x3a, 0x04, 0x35, 0x30, 0x35, 0x30, 0x12, 0x0b, 0x0a, 0x03, 0x70, 0x69, 0x64, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x09, 0x12, 0x10, 0x0a, 0x08, 0x68, 0x6f, 0x73, 0x74, 0x6e, 0x61, 0x6d, 0x65,
    0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x22, 0xb2, 0x01, 0x0a, 0x09, 0x53, 0x6c, 0x61, 0x76, 0x65,
    0x49, 0x6e, 0x66, 0x6f, 0x12, 0x10, 0x0a, 0x08, 0x68, 0x6f, 0x73, 0x74, 0x6e, 0x61, 0x6d, 0x65,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x12, 0x0a, 0x04, 0x70, 0x6f, 0x72, 0x74, 0x18, 0x08,
    0x20, 0x01, 0x28, 0x05, 0x3a, 0x04, 0x35, 0x30, 0x35, 0x31, 0x12, 0x22, 0x0a, 0x09, 0x72, 0x65,
    0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0f, 0x2e,
    0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x12, 0x24,
    0x0a, 0x0a, 0x61, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x18, 0x05, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x10, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x41, 0x74, 0x74, 0x72, 0x69,
    0x62, 0x75, 0x74, 0x65, 0x12, 0x1a, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x53, 0x6c, 0x61, 0x76, 0x65, 0x49, 0x44,
    0x12, 0x19, 0x0a, 0x0a, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x18, 0x07,
    0x20, 0x01, 0x28, 0x08, 0x3a, 0x05, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x22, 0xfc, 0x02, 0x0a, 0x05,
    0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x1f, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x0e, 0x32, 0x11, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x56, 0x61, 0x6c, 0x75,
    0x65, 0x2e, 0x54, 0x79, 0x70, 0x65, 0x12, 0x23, 0x0a, 0x06, 0x73, 0x63, 0x61, 0x6c, 0x61, 0x72,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x56,
    0x61, 0x6c, 0x75, 0x65, 0x2e, 0x53, 0x63, 0x61, 0x6c, 0x61, 0x72, 0x12, 0x23, 0x0a, 0x06, 0x72,
    0x61, 0x6e, 0x67, 0x65, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x6d, 0x65,
    0x73, 0x6f, 0x73, 0x2e, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x2e, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x73,
    0x12, 0x1d, 0x0a, 0x03, 0x73, 0x65, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e,
    0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x2e, 0x53, 0x65, 0x74, 0x12,
    0x1f, 0x0a, 0x04, 0x74, 0x65, 0x78, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e,
    0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x2e, 0x54, 0x65, 0x78, 0x74,
    0x1a, 0x17, 0x0a, 0x06, 0x53, 0x63, 0x61, 0x6c, 0x61, 0x72, 0x12, 0x0d, 0x0a, 0x05, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x01, 0x1a, 0x23, 0x0a, 0x05, 0x52, 0x61, 0x6e,
    0x67, 0x65, 0x12, 0x0d, 0x0a, 0x05, 0x62, 0x65, 0x67, 0x69, 0x6e, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x04, 0x12, 0x0b, 0x0a, 0x03, 0x65, 0x6e, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x1a, 0x2b,
    0x0a, 0x06, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x12, 0x21, 0x0a, 0x05, 0x72, 0x61, 0x6e, 0x67,
    0x65, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e,
    0x56, 0x61, 0x6c, 0x75, 0x65, 0x2e, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x1a, 0x13, 0x0a, 0x03, 0x53,
    0x65, 0x74, 0x12, 0x0c, 0x0a, 0x04, 0x69, 0x74, 0x65, 0x6d, 0x18, 0x01, 0x20, 0x03, 0x28, 0x09,
    0x1a, 0x15, 0x0a, 0x04, 0x54, 0x65, 0x78, 0x74, 0x12, 0x0d, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x22, 0x31, 0x0a, 0x04, 0x54, 0x79, 0x70, 0x65, 0x12,
    0x0a, 0x0a, 0x06, 0x53, 0x43, 0x41, 0x4c, 0x41, 0x52, 0x10, 0x00, 0x12, 0x0a, 0x0a, 0x06, 0x52,
    0x41, 0x4e, 0x47, 0x45, 0x53, 0x10, 0x01, 0x12, 0x07, 0x0a, 0x03, 0x53, 0x45, 0x54, 0x10, 0x02,
    0x12, 0x08, 0x0a, 0x04, 0x54, 0x45, 0x58, 0x54, 0x10, 0x03, 0x22, 0xc4, 0x01, 0x0a, 0x09, 0x41,
    0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x1f, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02,
    0x20, 0x02, 0x28, 0x0e, 0x32, 0x11, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x56, 0x61, 0x6c,
    0x75, 0x65, 0x2e, 0x54, 0x79, 0x70, 0x65, 0x12, 0x23, 0x0a, 0x06, 0x73, 0x63, 0x61, 0x6c, 0x61,
    0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e,
    0x56, 0x61, 0x6c, 0x75, 0x65, 0x2e, 0x53, 0x63, 0x61, 0x6c, 0x61, 0x72, 0x12, 0x23, 0x0a, 0x06,
    0x72, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x6d,
    0x65, 0x73, 0x6f, 0x73, 0x2e, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x2e, 0x52, 0x61, 0x6e, 0x67, 0x65,
    0x73, 0x12, 0x1d, 0x0a, 0x03, 0x73, 0x65, 0x74, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10,
    0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x2e, 0x53, 0x65, 0x74,
    0x12, 0x1f, 0x0a, 0x04, 0x74, 0x65, 0x78, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11,
    0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x2e, 0x54, 0x65, 0x78,
    0x74, 0x22, 0xb3, 0x01, 0x0a, 0x08, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x12, 0x0c,
    0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x1f, 0x0a, 0x04,
    0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x11, 0x2e, 0x6d, 0x65, 0x73,
    0x6f, 0x73, 0x2e, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x2e, 0x54, 0x79, 0x70, 0x65, 0x12, 0x23, 0x0a,
    0x06, 0x73, 0x63, 0x61, 0x6c, 0x61, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e,
    0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x2e, 0x53, 0x63, 0x61, 0x6c,
    0x61, 0x72, 0x12, 0x23, 0x0a, 0x06, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x13, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x56, 0x61, 0x6c, 0x75, 0x65,
    0x2e, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x12, 0x1d, 0x0a, 0x03, 0x73, 0x65, 0x74, 0x18, 0x05,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x56, 0x61, 0x6c,
    0x75, 0x65, 0x2e, 0x53, 0x65, 0x74, 0x12, 0x0f, 0x0a, 0x04, 0x72, 0x6f, 0x6c, 0x65, 0x18, 0x06,
    0x20, 0x01, 0x28, 0x09, 0x3a, 0x01, 0x2a, 0x22, 0xb7, 0x05, 0x0a, 0x12, 0x52, 0x65, 0x73, 0x6f,
    0x75, 0x72, 0x63, 0x65, 0x53, 0x74, 0x61, 0x74, 0x69, 0x73, 0x74, 0x69, 0x63, 0x73, 0x12, 0x11,
    0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x01, 0x12, 0x1b, 0x0a, 0x13, 0x63, 0x70, 0x75, 0x73, 0x5f, 0x75, 0x73, 0x65, 0x72, 0x5f, 0x74,
    0x69, 0x6d, 0x65, 0x5f, 0x73, 0x65, 0x63, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x01, 0x12, 0x1d,
    0x0a, 0x15, 0x63, 0x70, 0x75, 0x73, 0x5f, 0x73, 0x79, 0x73, 0x74, 0x65, 0x6d, 0x5f, 0x74, 0x69,
    0x6d, 0x65, 0x5f, 0x73, 0x65, 0x63, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x01, 0x12, 0x12, 0x0a,
    0x0a, 0x63, 0x70, 0x75, 0x73, 0x5f, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x01, 0x12, 0x17, 0x0a, 0x0f, 0x63, 0x70, 0x75, 0x73, 0x5f, 0x6e, 0x72, 0x5f, 0x70, 0x65, 0x72,
    0x69, 0x6f, 0x64, 0x73, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x19, 0x0a, 0x11, 0x63, 0x70,
    0x75, 0x73, 0x5f, 0x6e, 0x72, 0x5f, 0x74, 0x68, 0x72, 0x6f, 0x74, 0x74, 0x6c, 0x65, 0x64, 0x18,
    0x08, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x20, 0x0a, 0x18, 0x63, 0x70, 0x75, 0x73, 0x5f, 0x74, 0x68,
    0x72, 0x6f, 0x74, 0x74, 0x6c, 0x65, 0x64, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x5f, 0x73, 0x65, 0x63,
    0x73, 0x18, 0x09, 0x20, 0x01, 0x28, 0x01, 0x12, 0x15, 0x0a, 0x0d, 0x6d, 0x65, 0x6d, 0x5f, 0x72,
    0x73, 0x73, 0x5f, 0x62, 0x79, 0x74, 0x65, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x04, 0x12, 0x17,
    0x0a, 0x0f, 0x6d, 0x65, 0x6d, 0x5f, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x5f, 0x62, 0x79, 0x74, 0x65,
    0x73, 0x18, 0x06, 0x20, 0x01, 0x28, 0x04, 0x12, 0x16, 0x0a, 0x0e, 0x6d, 0x65, 0x6d, 0x5f, 0x66,
    0x69, 0x6c, 0x65, 0x5f, 0x62, 0x79, 0x74, 0x65, 0x73, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x04, 0x12,
    0x16, 0x0a, 0x0e, 0x6d, 0x65, 0x6d, 0x5f, 0x61, 0x6e, 0x6f, 0x6e, 0x5f, 0x62, 0x79, 0x74, 0x65,
    0x73, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x04, 0x12, 0x1d, 0x0a, 0x15, 0x6d, 0x65, 0x6d, 0x5f, 0x6d,
    0x61, 0x70, 0x70, 0x65, 0x64, 0x5f, 0x66, 0x69, 0x6c, 0x65, 0x5f, 0x62, 0x79, 0x74, 0x65, 0x73,
    0x18, 0x0c, 0x20, 0x01, 0x28, 0x04, 0x12, 0x23, 0x0a, 0x04, 0x70, 0x65, 0x72, 0x66, 0x18, 0x0d,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x50, 0x65, 0x72,
    0x66, 0x53, 0x74, 0x61, 0x74, 0x69, 0x73, 0x74, 0x69, 0x63, 0x73, 0x12, 0x16, 0x0a, 0x0e, 0x6e,
    0x65, 0x74, 0x5f, 0x72, 0x78, 0x5f, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x18, 0x0e, 0x20,
    0x01, 0x28, 0x04, 0x12, 0x14, 0x0a, 0x0c, 0x6e, 0x65, 0x74, 0x5f, 0x72, 0x78, 0x5f, 0x62, 0x79,
    0x74, 0x65, 0x73, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x04, 0x12, 0x15, 0x0a, 0x0d, 0x6e, 0x65, 0x74,
    0x5f, 0x72, 0x78, 0x5f, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x73, 0x18, 0x10, 0x20, 0x01, 0x28, 0x04,
    0x12, 0x16, 0x0a, 0x0e, 0x6e, 0x65, 0x74, 0x5f, 0x72, 0x78, 0x5f, 0x64, 0x72, 0x6f, 0x70, 0x70,
    0x65, 0x64, 0x18, 0x11, 0x20, 0x01, 0x28, 0x04, 0x12, 0x16, 0x0a, 0x0e, 0x6e, 0x65, 0x74, 0x5f,
    0x74, 0x78, 0x5f, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x18, 0x12, 0x20, 0x01, 0x28, 0x04,
    0x12, 0x14, 0x0a, 0x0c, 0x6e, 0x65, 0x74, 0x5f, 0x74, 0x78, 0x5f, 0x62, 0x79, 0x74, 0x65, 0x73,
    0x18, 0x13, 0x20, 0x01, 0x28, 0x04, 0x12, 0x15, 0x0a, 0x0d, 0x6e, 0x65, 0x74, 0x5f, 0x74, 0x78,
    0x5f, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x73, 0x18, 0x14, 0x20, 0x01, 0x28, 0x04, 0x12, 0x16, 0x0a,
    0x0e, 0x6e, 0x65, 0x74, 0x5f, 0x74, 0x78, 0x5f, 0x64, 0x72, 0x6f, 0x70, 0x70, 0x65, 0x64, 0x18,
    0x15, 0x20, 0x01, 0x28, 0x04, 0x12, 0x21, 0x0a, 0x19, 0x6e, 0x65, 0x74, 0x5f, 0x74, 0x63, 0x70,
    0x5f, 0x72, 0x74, 0x74, 0x5f, 0x6d, 0x69, 0x63, 0x72, 0x6f, 0x73, 0x65, 0x63, 0x73, 0x5f, 0x70,
    0x35, 0x30, 0x18, 0x16, 0x20, 0x01, 0x28, 0x01, 0x12, 0x21, 0x0a, 0x19, 0x6e, 0x65, 0x74, 0x5f,
    0x74, 0x63, 0x70, 0x5f, 0x72, 0x74, 0x74, 0x5f, 0x6d, 0x69, 0x63, 0x72, 0x6f, 0x73, 0x65, 0x63,
    0x73, 0x5f, 0x70, 0x39, 0x30, 0x18, 0x17, 0x20, 0x01, 0x28, 0x01, 0x12, 0x21, 0x0a, 0x19, 0x6e,
    0x65, 0x74, 0x5f, 0x74, 0x63, 0x70, 0x5f, 0x72, 0x74, 0x74, 0x5f, 0x6d, 0x69, 0x63, 0x72, 0x6f,
    0x73, 0x65, 0x63, 0x73, 0x5f, 0x70, 0x39, 0x35, 0x18, 0x18, 0x20, 0x01, 0x28, 0x01, 0x12, 0x21,
    0x0a, 0x19, 0x6e, 0x65, 0x74, 0x5f, 0x74, 0x63, 0x70, 0x5f, 0x72, 0x74, 0x74, 0x5f, 0x6d, 0x69,
    0x63, 0x72, 0x6f, 0x73, 0x65, 0x63, 0x73, 0x5f, 0x70, 0x39, 0x39, 0x18, 0x19, 0x20, 0x01, 0x28,
    0x01, 0x22, 0xe9, 0x01, 0x0a, 0x0d, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x55, 0x73,
    0x61, 0x67, 0x65, 0x12, 0x20, 0x0a, 0x08, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x5f, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x53, 0x6c,
    0x61, 0x76, 0x65, 0x49, 0x44, 0x12, 0x28, 0x0a, 0x0c, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f,
    0x72, 0x6b, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x6d, 0x65,
    0x73, 0x6f, 0x73, 0x2e, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x49, 0x44, 0x12,
    0x26, 0x0a, 0x0b, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x45, 0x78, 0x65,
    0x63, 0x75, 0x74, 0x6f, 0x72, 0x49, 0x44, 0x12, 0x15, 0x0a, 0x0d, 0x65, 0x78, 0x65, 0x63, 0x75,
    0x74, 0x6f, 0x72, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x12, 0x1e,
    0x0a, 0x07, 0x74, 0x61, 0x73, 0x6b, 0x5f, 0x69, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x0d, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x54, 0x61, 0x73, 0x6b, 0x49, 0x44, 0x12, 0x2d,
    0x0a, 0x0a, 0x73, 0x74, 0x61, 0x74, 0x69, 0x73, 0x74, 0x69, 0x63, 0x73, 0x18, 0x06, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x19, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x52, 0x65, 0x73, 0x6f, 0x75,
    0x72, 0x63, 0x65, 0x53, 0x74, 0x61, 0x74, 0x69, 0x73, 0x74, 0x69, 0x63, 0x73, 0x22, 0xb0, 0x0a,
    0x0a, 0x0e, 0x50, 0x65, 0x72, 0x66, 0x53, 0x74, 0x61, 0x74, 0x69, 0x73, 0x74, 0x69, 0x63, 0x73,
    0x12, 0x11, 0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x01, 0x12, 0x10, 0x0a, 0x08, 0x64, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x01, 0x12, 0x0e, 0x0a, 0x06, 0x63, 0x79, 0x63, 0x6c, 0x65, 0x73, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x04, 0x12, 0x1f, 0x0a, 0x17, 0x73, 0x74, 0x61, 0x6c, 0x6c, 0x65, 0x64,
    0x5f, 0x63, 0x79, 0x63, 0x6c, 0x65, 0x73, 0x5f, 0x66, 0x72, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x64,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x04, 0x12, 0x1e, 0x0a, 0x16, 0x73, 0x74, 0x61, 0x6c, 0x6c, 0x65,
    0x64, 0x5f, 0x63, 0x79, 0x63, 0x6c, 0x65, 0x73, 0x5f, 0x62, 0x61, 0x63, 0x6b, 0x65, 0x6e, 0x64,
    0x18, 0x05, 0x20, 0x01, 0x28, 0x04, 0x12, 0x14, 0x0a, 0x0c, 0x69, 0x6e, 0x73, 0x74, 0x72, 0x75,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x06, 0x20, 0x01, 0x28, 0x04, 0x12, 0x18, 0x0a, 0x10,
    0x63, 0x61, 0x63, 0x68, 0x65, 0x5f, 0x72, 0x65, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63, 0x65, 0x73,
    0x18, 0x07, 0x20, 0x01, 0x28, 0x04, 0x12, 0x14, 0x0a, 0x0c, 0x63, 0x61, 0x63, 0x68, 0x65, 0x5f,
    0x6d, 0x69, 0x73, 0x73, 0x65, 0x73, 0x18, 0x08, 0x20, 0x01, 0x28, 0x04, 0x12, 0x10, 0x0a, 0x08,
    0x62, 0x72, 0x61, 0x6e, 0x63, 0x68, 0x65, 0x73, 0x18, 0x09, 0x20, 0x01, 0x28, 0x04, 0x12, 0x15,
    0x0a, 0x0d, 0x62, 0x72, 0x61, 0x6e, 0x63, 0x68, 0x5f, 0x6d, 0x69, 0x73, 0x73, 0x65, 0x73, 0x18,
    0x0a, 0x20, 0x01, 0x28, 0x04, 0x12, 0x12, 0x0a, 0x0a, 0x62, 0x75, 0x73, 0x5f, 0x63, 0x79, 0x63,
    0x6c, 0x65, 0x73, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x04, 0x12, 0x12, 0x0a, 0x0a, 0x72, 0x65, 0x66,
    0x5f, 0x63, 0x79, 0x63, 0x6c, 0x65, 0x73, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x04, 0x12, 0x11, 0x0a,
    0x09, 0x63, 0x70, 0x75, 0x5f, 0x63, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x01,
    0x12, 0x12, 0x0a, 0x0a, 0x74, 0x61, 0x73, 0x6b, 0x5f, 0x63, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x0e,
    0x20, 0x01, 0x28, 0x01, 0x12, 0x13, 0x0a, 0x0b, 0x70, 0x61, 0x67, 0x65, 0x5f, 0x66, 0x61, 0x75,
    0x6c, 0x74, 0x73, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x04, 0x12, 0x14, 0x0a, 0x0c, 0x6d, 0x69, 0x6e,
    0x6f, 0x72, 0x5f, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x73, 0x18, 0x10, 0x20, 0x01, 0x28, 0x04, 0x12,
    0x14, 0x0a, 0x0c, 0x6d, 0x61, 0x6a, 0x6f, 0x72, 0x5f, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x73, 0x18,
    0x11, 0x20, 0x01, 0x28, 0x04, 0x12, 0x18, 0x0a, 0x10, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74,
    0x5f, 0x73, 0x77, 0x69, 0x74, 0x63, 0x68, 0x65, 0x73, 0x18, 0x12, 0x20, 0x01, 0x28, 0x04, 0x12,
    0x16, 0x0a, 0x0e, 0x63, 0x70, 0x75, 0x5f, 0x6d, 0x69, 0x67, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x73, 0x18, 0x13, 0x20, 0x01, 0x28, 0x04, 0x12, 0x18, 0x0a, 0x10, 0x61, 0x6c, 0x69, 0x67, 0x6e,
    0x6d, 0x65, 0x6e, 0x74, 0x5f, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x73, 0x18, 0x14, 0x20, 0x01, 0x28,
    0x04, 0x12, 0x18, 0x0a, 0x10, 0x65, 0x6d, 0x75, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x66,
    0x61, 0x75, 0x6c, 0x74, 0x73, 0x18, 0x15, 0x20, 0x01, 0x28, 0x04, 0x12, 0x17, 0x0a, 0x0f, 0x6c,
    0x31, 0x5f, 0x64, 0x63, 0x61, 0x63, 0x68, 0x65, 0x5f, 0x6c, 0x6f, 0x61, 0x64, 0x73, 0x18, 0x16,
    0x20, 0x01, 0x28, 0x04, 0x12, 0x1d, 0x0a, 0x15, 0x6c, 0x31, 0x5f, 0x64, 0x63, 0x61, 0x63, 0x68,
    0x65, 0x5f, 0x6c, 0x6f, 0x61, 0x64, 0x5f, 0x6d, 0x69, 0x73, 0x73, 0x65, 0x73, 0x18, 0x17, 0x20,
    0x01, 0x28, 0x04, 0x12, 0x18, 0x0a, 0x10, 0x6c, 0x31, 0x5f, 0x64, 0x63, 0x61, 0x63, 0x68, 0x65,
    0x5f, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x73, 0x18, 0x18, 0x20, 0x01, 0x28, 0x04, 0x12, 0x1e, 0x0a,
    0x16, 0x6c, 0x31, 0x5f, 0x64, 0x63, 0x61, 0x63, 0x68, 0x65, 0x5f, 0x73, 0x74, 0x6f, 0x72, 0x65,
    0x5f, 0x6d, 0x69, 0x73, 0x73, 0x65, 0x73, 0x18, 0x19, 0x20, 0x01, 0x28, 0x04, 0x12, 0x1c, 0x0a,
    0x14, 0x6c, 0x31, 0x5f, 0x64, 0x63, 0x61, 0x63, 0x68, 0x65, 0x5f, 0x70, 0x72, 0x65, 0x66, 0x65,
    0x74, 0x63, 0x68, 0x65, 0x73, 0x18, 0x1a, 0x20, 0x01, 0x28, 0x04, 0x12, 0x21, 0x0a, 0x19, 0x6c,
    0x31, 0x5f, 0x64, 0x63, 0x61, 0x63, 0x68, 0x65, 0x5f, 0x70, 0x72, 0x65, 0x66, 0x65, 0x74, 0x63,
    0x68, 0x5f, 0x6d, 0x69, 0x73, 0x73, 0x65, 0x73, 0x18, 0x1b, 0x20, 0x01, 0x28, 0x04, 0x12, 0x17,
    0x0a, 0x0f, 0x6c, 0x31, 0x5f, 0x69, 0x63, 0x61, 0x63, 0x68, 0x65, 0x5f, 0x6c, 0x6f, 0x61, 0x64,
    0x73, 0x18, 0x1c, 0x20, 0x01, 0x28, 0x04, 0x12, 0x1d, 0x0a, 0x15, 0x6c, 0x31, 0x5f, 0x69, 0x63,
    0x61, 0x63, 0x68, 0x65, 0x5f, 0x6c, 0x6f, 0x61, 0x64, 0x5f, 0x6d, 0x69, 0x73, 0x73, 0x65, 0x73,
    0x18, 0x1d, 0x20, 0x01, 0x28, 0x04, 0x12, 0x1c, 0x0a, 0x14, 0x6c, 0x31, 0x5f, 0x69, 0x63, 0x61,
    0x63, 0x68, 0x65, 0x5f, 0x70, 0x72, 0x65, 0x66, 0x65, 0x74, 0x63, 0x68, 0x65, 0x73, 0x18, 0x1e,
    0x20, 0x01, 0x28, 0x04, 0x12, 0x21, 0x0a, 0x19, 0x6c, 0x31, 0x5f, 0x69, 0x63, 0x61, 0x63, 0x68,
    0x65, 0x5f, 0x70, 0x72, 0x65, 0x66, 0x65, 0x74, 0x63, 0x68, 0x5f, 0x6d, 0x69, 0x73, 0x73, 0x65,
    0x73, 0x18, 0x1f, 0x20, 0x01, 0x28, 0x04, 0x12, 0x11, 0x0a, 0x09, 0x6c, 0x6c, 0x63, 0x5f, 0x6c,
    0x6f, 0x61, 0x64, 0x73, 0x18, 0x20, 0x20, 0x01, 0x28, 0x04, 0x12, 0x17, 0x0a, 0x0f, 0x6c, 0x6c,
    0x63, 0x5f, 0x6c, 0x6f, 0x61, 0x64, 0x5f, 0x6d, 0x69, 0x73, 0x73, 0x65, 0x73, 0x18, 0x21, 0x20,
    0x01, 0x28, 0x04, 0x12, 0x12, 0x0a, 0x0a, 0x6c, 0x6c, 0x63, 0x5f, 0x73, 0x74, 0x6f, 0x72, 0x65,
    0x73, 0x18, 0x22, 0x20, 0x01, 0x28, 0x04, 0x12, 0x18, 0x0a, 0x10, 0x6c, 0x6c, 0x63, 0x5f, 0x73,
    0x74, 0x6f, 0x72, 0x65, 0x5f, 0x6d, 0x69, 0x73, 0x73, 0x65, 0x73, 0x18, 0x23, 0x20, 0x01, 0x28,
    0x04, 0x12, 0x16, 0x0a, 0x0e, 0x6c, 0x6c, 0x63, 0x5f, 0x70, 0x72, 0x65, 0x66, 0x65, 0x74, 0x63,
    0x68, 0x65, 0x73, 0x18, 0x24, 0x20, 0x01, 0x28, 0x04, 0x12, 0x1b, 0x0a, 0x13, 0x6c, 0x6c, 0x63,
    0x5f, 0x70, 0x72, 0x65, 0x66, 0x65, 0x74, 0x63, 0x68, 0x5f, 0x6d, 0x69, 0x73, 0x73, 0x65, 0x73,
    0x18, 0x25, 0x20, 0x01, 0x28, 0x04, 0x12, 0x12, 0x0a, 0x0a, 0x64, 0x74, 0x6c, 0x62, 0x5f, 0x6c,
    0x6f, 0x61, 0x64, 0x73, 0x18, 0x26, 0x20, 0x01, 0x28, 0x04, 0x12, 0x18, 0x0a, 0x10, 0x64, 0x74,
    0x6c, 0x62, 0x5f, 0x6c, 0x6f, 0x61, 0x64, 0x5f, 0x6d, 0x69, 0x73, 0x73, 0x65, 0x73, 0x18, 0x27,
    0x20, 0x01, 0x28, 0x04, 0x12, 0x13, 0x0a, 0x0b, 0x64, 0x74, 0x6c, 0x62, 0x5f, 0x73, 0x74, 0x6f,
    0x72, 0x65, 0x73, 0x18, 0x28, 0x20, 0x01, 0x28, 0x04, 0x12, 0x19, 0x0a, 0x11, 0x64, 0x74, 0x6c,
    0x62, 0x5f, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x5f, 0x6d, 0x69, 0x73, 0x73, 0x65, 0x73, 0x18, 0x29,
    0x20, 0x01, 0x28, 0x04, 0x12, 0x17, 0x0a, 0x0f, 0x64, 0x74, 0x6c, 0x62, 0x5f, 0x70, 0x72, 0x65,
    0x66, 0x65, 0x74, 0x63, 0x68, 0x65, 0x73, 0x18, 0x2a, 0x20, 0x01, 0x28, 0x04, 0x12, 0x1c, 0x0a,
    0x14, 0x64, 0x74, 0x6c, 0x62, 0x5f, 0x70, 0x72, 0x65, 0x66, 0x65, 0x74, 0x63, 0x68, 0x5f, 0x6d,
    0x69, 0x73, 0x73, 0x65, 0x73, 0x18, 0x2b, 0x20, 0x01, 0x28, 0x04, 0x12, 0x12, 0x0a, 0x0a, 0x69,
    0x74, 0x6c, 0x62, 0x5f, 0x6c, 0x6f, 0x61, 0x64, 0x73, 0x18, 0x2c, 0x20, 0x01, 0x28, 0x04, 0x12,
    0x18, 0x0a, 0x10, 0x69, 0x74, 0x6c, 0x62, 0x5f, 0x6c, 0x6f, 0x61, 0x64, 0x5f, 0x6d, 0x69, 0x73,
    0x73, 0x65, 0x73, 0x18, 0x2d, 0x20, 0x01, 0x28, 0x04, 0x12, 0x14, 0x0a, 0x0c, 0x62, 0x72, 0x61,
    0x6e, 0x63, 0x68, 0x5f, 0x6c, 0x6f, 0x61, 0x64, 0x73, 0x18, 0x2e, 0x20, 0x01, 0x28, 0x04, 0x12,
    0x1a, 0x0a, 0x12, 0x62, 0x72, 0x61, 0x6e, 0x63, 0x68, 0x5f, 0x6c, 0x6f, 0x61, 0x64, 0x5f, 0x6d,
    0x69, 0x73, 0x73, 0x65, 0x73, 0x18, 0x2f, 0x20, 0x01, 0x28, 0x04, 0x12, 0x12, 0x0a, 0x0a, 0x6e,
    0x6f, 0x64, 0x65, 0x5f, 0x6c, 0x6f, 0x61, 0x64, 0x73, 0x18, 0x30, 0x20, 0x01, 0x28, 0x04, 0x12,
    0x18, 0x0a, 0x10, 0x6e, 0x6f, 0x64, 0x65, 0x5f, 0x6c, 0x6f, 0x61, 0x64, 0x5f, 0x6d, 0x69, 0x73,
    0x73, 0x65, 0x73, 0x18, 0x31, 0x20, 0x01, 0x28, 0x04, 0x12, 0x13, 0x0a, 0x0b, 0x6e, 0x6f, 0x64,
    0x65, 0x5f, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x73, 0x18, 0x32, 0x20, 0x01, 0x28, 0x04, 0x12, 0x19,
    0x0a, 0x11, 0x6e, 0x6f, 0x64, 0x65, 0x5f, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x5f, 0x6d, 0x69, 0x73,
    0x73, 0x65, 0x73, 0x18, 0x33, 0x20, 0x01, 0x28, 0x04, 0x12, 0x17, 0x0a, 0x0f, 0x6e, 0x6f, 0x64,
    0x65, 0x5f, 0x70, 0x72, 0x65, 0x66, 0x65, 0x74, 0x63, 0x68, 0x65, 0x73, 0x18, 0x34, 0x20, 0x01,
    0x28, 0x04, 0x12, 0x1c, 0x0a, 0x14, 0x6e, 0x6f, 0x64, 0x65, 0x5f, 0x70, 0x72, 0x65, 0x66, 0x65,
    0x74, 0x63, 0x68, 0x5f, 0x6d, 0x69, 0x73, 0x73, 0x65, 0x73, 0x18, 0x35, 0x20, 0x01, 0x28, 0x04,
    0x22, 0x4f, 0x0a, 0x07, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x20, 0x0a, 0x08, 0x73,
    0x6c, 0x61, 0x76, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e,
    0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x53, 0x6c, 0x61, 0x76, 0x65, 0x49, 0x44, 0x12, 0x22, 0x0a,
    0x09, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x0f, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63,
    0x65, 0x22, 0xf4, 0x01, 0x0a, 0x05, 0x4f, 0x66, 0x66, 0x65, 0x72, 0x12, 0x1a, 0x0a, 0x02, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e,
    0x4f, 0x66, 0x66, 0x65, 0x72, 0x49, 0x44, 0x12, 0x28, 0x0a, 0x0c, 0x66, 0x72, 0x61, 0x6d, 0x65,
    0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x12, 0x2e,
    0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x49,
    0x44, 0x12, 0x20, 0x0a, 0x08, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20,
    0x02, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x53, 0x6c, 0x61, 0x76,
    0x65, 0x49, 0x44, 0x12, 0x10, 0x0a, 0x08, 0x68, 0x6f, 0x73, 0x74, 0x6e, 0x61, 0x6d, 0x65, 0x18,
    0x04, 0x20, 0x02, 0x28, 0x09, 0x12, 0x22, 0x0a, 0x09, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63,
    0x65, 0x73, 0x18, 0x05, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73,
    0x2e, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x12, 0x24, 0x0a, 0x0a, 0x61, 0x74, 0x74,
    0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x18, 0x07, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x10, 0x2e,
    0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x12,
    0x27, 0x0a, 0x0c, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x5f, 0x69, 0x64, 0x73, 0x18,
    0x06, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x45, 0x78,
    0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x49, 0x44, 0x22, 0xab, 0x02, 0x0a, 0x08, 0x54, 0x61, 0x73,
    0x6b, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x09, 0x12, 0x1e, 0x0a, 0x07, 0x74, 0x61, 0x73, 0x6b, 0x5f, 0x69, 0x64, 0x18, 0x02,
    0x20, 0x02, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x54, 0x61, 0x73,
    0x6b, 0x49, 0x44, 0x12, 0x20, 0x0a, 0x08, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x5f, 0x69, 0x64, 0x18,
    0x03, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x53, 0x6c,
    0x61, 0x76, 0x65, 0x49, 0x44, 0x12, 0x22, 0x0a, 0x09, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63,
    0x65, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73,
    0x2e, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x12, 0x25, 0x0a, 0x08, 0x65, 0x78, 0x65,
    0x63, 0x75, 0x74, 0x6f, 0x72, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x6d, 0x65,
    0x73, 0x6f, 0x73, 0x2e, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x49, 0x6e, 0x66, 0x6f,
    0x12, 0x23, 0x0a, 0x07, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x12, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e,
    0x64, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x27, 0x0a, 0x09, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e,
    0x65, 0x72, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73,
    0x2e, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x0c,
    0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x28, 0x0a, 0x0c,
    0x68, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x5f, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x18, 0x08, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x12, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x48, 0x65, 0x61, 0x6c, 0x74,
    0x68, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x22, 0xe4, 0x06, 0x0a, 0x0a, 0x54, 0x61, 0x73, 0x6b, 0x53,
    0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x1e, 0x0a, 0x07, 0x74, 0x61, 0x73, 0x6b, 0x5f, 0x69, 0x64,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x54,
    0x61, 0x73, 0x6b, 0x49, 0x44, 0x12, 0x1f, 0x0a, 0x05, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x02,
    0x20, 0x02, 0x28, 0x0e, 0x32, 0x10, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x54, 0x61, 0x73,
    0x6b, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x0f, 0x0a, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x12, 0x28, 0x0a, 0x06, 0x73, 0x6f, 0x75, 0x72, 0x63,
    0x65, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x18, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e,
    0x54, 0x61, 0x73, 0x6b, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x53, 0x6f, 0x75, 0x72, 0x63,
    0x65, 0x12, 0x28, 0x0a, 0x06, 0x72, 0x65, 0x61, 0x73, 0x6f, 0x6e, 0x18, 0x0a, 0x20, 0x01, 0x28,
    0x0e, 0x32, 0x18, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x54, 0x61, 0x73, 0x6b, 0x53, 0x74,
    0x61, 0x74, 0x75, 0x73, 0x2e, 0x52, 0x65, 0x61, 0x73, 0x6f, 0x6e, 0x12, 0x0c, 0x0a, 0x04, 0x64,
    0x61, 0x74, 0x61, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x20, 0x0a, 0x08, 0x73, 0x6c, 0x61,
    0x76, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6d, 0x65,
    0x73, 0x6f, 0x73, 0x2e, 0x53, 0x6c, 0x61, 0x76, 0x65, 0x49, 0x44, 0x12, 0x26, 0x0a, 0x0b, 0x65,
    0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x11, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f,
    0x72, 0x49, 0x44, 0x12, 0x11, 0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70,
    0x18, 0x06, 0x20, 0x01, 0x28, 0x01, 0x12, 0x0f, 0x0a, 0x07, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68,
    0x79, 0x18, 0x08, 0x20, 0x01, 0x28, 0x08, 0x22, 0x42, 0x0a, 0x06, 0x53, 0x6f, 0x75, 0x72, 0x63,
    0x65, 0x12, 0x11, 0x0a, 0x0d, 0x53, 0x4f, 0x55, 0x52, 0x43, 0x45, 0x5f, 0x4d, 0x41, 0x53, 0x54,
    0x45, 0x52, 0x10, 0x00, 0x12, 0x10, 0x0a, 0x0c, 0x53, 0x4f, 0x55, 0x52, 0x43, 0x45, 0x5f, 0x53,
    0x4c, 0x41, 0x56, 0x45, 0x10, 0x01, 0x12, 0x13, 0x0a, 0x0f, 0x53, 0x4f, 0x55, 0x52, 0x43, 0x45,
    0x5f, 0x45, 0x58, 0x45, 0x43, 0x55, 0x54, 0x4f, 0x52, 0x10, 0x02, 0x22, 0xef, 0x03, 0x0a, 0x06,
    0x52, 0x65, 0x61, 0x73, 0x6f, 0x6e, 0x12, 0x22, 0x0a, 0x1e, 0x52, 0x45, 0x41, 0x53, 0x4f, 0x4e,
    0x5f, 0x43, 0x4f, 0x4d, 0x4d, 0x41, 0x4e, 0x44, 0x5f, 0x45, 0x58, 0x45, 0x43, 0x55, 0x54, 0x4f,
    0x52, 0x5f, 0x46, 0x41, 0x49, 0x4c, 0x45, 0x44, 0x10, 0x00, 0x12, 0x1e, 0x0a, 0x1a, 0x52, 0x45,
    0x41, 0x53, 0x4f, 0x4e, 0x5f, 0x45, 0x58, 0x45, 0x43, 0x55, 0x54, 0x4f, 0x52, 0x5f, 0x54, 0x45,
    0x52, 0x4d, 0x49, 0x4e, 0x41, 0x54, 0x45, 0x44, 0x10, 0x01, 0x12, 0x20, 0x0a, 0x1c, 0x52, 0x45,
    0x41, 0x53, 0x4f, 0x4e, 0x5f, 0x45, 0x58, 0x45, 0x43, 0x55, 0x54, 0x4f, 0x52, 0x5f, 0x55, 0x4e,
    0x52, 0x45, 0x47, 0x49, 0x53, 0x54, 0x45, 0x52, 0x45, 0x44, 0x10, 0x02, 0x12, 0x1c, 0x0a, 0x18,
    0x52, 0x45, 0x41, 0x53, 0x4f, 0x4e, 0x5f, 0x46, 0x52, 0x41, 0x4d, 0x45, 0x57, 0x4f, 0x52, 0x4b,
    0x5f, 0x52, 0x45, 0x4d, 0x4f, 0x56, 0x45, 0x44, 0x10, 0x03, 0x12, 0x13, 0x0a, 0x0f, 0x52, 0x45,
    0x41, 0x53, 0x4f, 0x4e, 0x5f, 0x47, 0x43, 0x5f, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x10, 0x04, 0x12,
    0x1e, 0x0a, 0x1a, 0x52, 0x45, 0x41, 0x53, 0x4f, 0x4e, 0x5f, 0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49,
    0x44, 0x5f, 0x46, 0x52, 0x41, 0x4d, 0x45, 0x57, 0x4f, 0x52, 0x4b, 0x49, 0x44, 0x10, 0x05, 0x12,
    0x19, 0x0a, 0x15, 0x52, 0x45, 0x41, 0x53, 0x4f, 0x4e, 0x5f, 0x49, 0x4e, 0x56, 0x41, 0x4c, 0x49,
    0x44, 0x5f, 0x4f, 0x46, 0x46, 0x45, 0x52, 0x53, 0x10, 0x06, 0x12, 0x1e, 0x0a, 0x1a, 0x52, 0x45,
    0x41, 0x53, 0x4f, 0x4e, 0x5f, 0x4d, 0x41, 0x53, 0x54, 0x45, 0x52, 0x5f, 0x44, 0x49, 0x53, 0x43,
    0x4f, 0x4e, 0x4e, 0x45, 0x43, 0x54, 0x45, 0x44, 0x10, 0x07, 0x12, 0x17, 0x0a, 0x13, 0x52, 0x45,
    0x41, 0x53, 0x4f, 0x4e, 0x5f, 0x4d, 0x45, 0x4d, 0x4f, 0x52, 0x59, 0x5f, 0x4c, 0x49, 0x4d, 0x49,
    0x54, 0x10, 0x08, 0x12, 0x19, 0x0a, 0x15, 0x52, 0x45, 0x41, 0x53, 0x4f, 0x4e, 0x5f, 0x52, 0x45,
    0x43, 0x4f, 0x4e, 0x43, 0x49, 0x4c, 0x49, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x10, 0x09, 0x12, 0x1d,
    0x0a, 0x19, 0x52, 0x45, 0x41, 0x53, 0x4f, 0x4e, 0x5f, 0x53, 0x4c, 0x41, 0x56, 0x45, 0x5f, 0x44,
    0x49, 0x53, 0x43, 0x4f, 0x4e, 0x4e, 0x45, 0x43, 0x54, 0x45, 0x44, 0x10, 0x0a, 0x12, 0x18, 0x0a,
    0x14, 0x52, 0x45, 0x41, 0x53, 0x4f, 0x4e, 0x5f, 0x53, 0x4c, 0x41, 0x56, 0x45, 0x5f, 0x52, 0x45,
    0x4d, 0x4f, 0x56, 0x45, 0x44, 0x10, 0x0b, 0x12, 0x1a, 0x0a, 0x16, 0x52, 0x45, 0x41, 0x53, 0x4f,
    0x4e, 0x5f, 0x53, 0x4c, 0x41, 0x56, 0x45, 0x5f, 0x52, 0x45, 0x53, 0x54, 0x41, 0x52, 0x54, 0x45,
    0x44, 0x10, 0x0c, 0x12, 0x18, 0x0a, 0x14, 0x52, 0x45, 0x41, 0x53, 0x4f, 0x4e, 0x5f, 0x53, 0x4c,
    0x41, 0x56, 0x45, 0x5f, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x10, 0x0d, 0x12, 0x17, 0x0a,
    0x13, 0x52, 0x45, 0x41, 0x53, 0x4f, 0x4e, 0x5f, 0x54, 0x41, 0x53, 0x4b, 0x5f, 0x49, 0x4e, 0x56,
    0x41, 0x4c, 0x49, 0x44, 0x10, 0x0e, 0x12, 0x1c, 0x0a, 0x18, 0x52, 0x45, 0x41, 0x53, 0x4f, 0x4e,
    0x5f, 0x54, 0x41, 0x53, 0x4b, 0x5f, 0x55, 0x4e, 0x41, 0x55, 0x54, 0x48, 0x4f, 0x52, 0x49, 0x5a,
    0x45, 0x44, 0x10, 0x0f, 0x12, 0x17, 0x0a, 0x13, 0x52, 0x45, 0x41, 0x53, 0x4f, 0x4e, 0x5f, 0x54,
    0x41, 0x53, 0x4b, 0x5f, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x10, 0x10, 0x22, 0x24, 0x0a,
    0x07, 0x46, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x12, 0x19, 0x0a, 0x0e, 0x72, 0x65, 0x66, 0x75,
    0x73, 0x65, 0x5f, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x01,
    0x3a, 0x01, 0x35, 0x22, 0x66, 0x0a, 0x0b, 0x45, 0x6e, 0x76, 0x69, 0x72, 0x6f, 0x6e, 0x6d, 0x65,
    0x6e, 0x74, 0x12, 0x2e, 0x0a, 0x09, 0x76, 0x61, 0x72, 0x69, 0x61, 0x62, 0x6c, 0x65, 0x73, 0x18,
    0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x45, 0x6e,
    0x76, 0x69, 0x72, 0x6f, 0x6e, 0x6d, 0x65, 0x6e, 0x74, 0x2e, 0x56, 0x61, 0x72, 0x69, 0x61, 0x62,
    0x6c, 0x65, 0x1a, 0x27, 0x0a, 0x08, 0x56, 0x61, 0x72, 0x69, 0x61, 0x62, 0x6c, 0x65, 0x12, 0x0c,
    0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0d, 0x0a, 0x05,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x22, 0x27, 0x0a, 0x09, 0x50,
    0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0d, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02,
    0x20, 0x02, 0x28, 0x09, 0x22, 0x31, 0x0a, 0x0a, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65,
    0x72, 0x73, 0x12, 0x23, 0x0a, 0x09, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x18,
    0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x50, 0x61,
    0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x22, 0x2f, 0x0a, 0x0a, 0x43, 0x72, 0x65, 0x64, 0x65,
    0x6e, 0x74, 0x69, 0x61, 0x6c, 0x12, 0x11, 0x0a, 0x09, 0x70, 0x72, 0x69, 0x6e, 0x63, 0x69, 0x70,
    0x61, 0x6c, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0e, 0x0a, 0x06, 0x73, 0x65, 0x63, 0x72,
    0x65, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x35, 0x0a, 0x0b, 0x43, 0x72, 0x65, 0x64,
    0x65, 0x6e, 0x74, 0x69, 0x61, 0x6c, 0x73, 0x12, 0x26, 0x0a, 0x0b, 0x63, 0x72, 0x65, 0x64, 0x65,
    0x6e, 0x74, 0x69, 0x61, 0x6c, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6d,
    0x65, 0x73, 0x6f, 0x73, 0x2e, 0x43, 0x72, 0x65, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x61, 0x6c, 0x22,
    0x8f, 0x03, 0x0a, 0x03, 0x41, 0x43, 0x4c, 0x1a, 0x69, 0x0a, 0x06, 0x45, 0x6e, 0x74, 0x69, 0x74,
    0x79, 0x12, 0x2a, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x16, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x41, 0x43, 0x4c, 0x2e, 0x45, 0x6e, 0x74, 0x69,
    0x74, 0x79, 0x2e, 0x54, 0x79, 0x70, 0x65, 0x3a, 0x04, 0x53, 0x4f, 0x4d, 0x45, 0x12, 0x0e, 0x0a,
    0x06, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x09, 0x22, 0x23, 0x0a,
    0x04, 0x54, 0x79, 0x70, 0x65, 0x12, 0x08, 0x0a, 0x04, 0x53, 0x4f, 0x4d, 0x45, 0x10, 0x00, 0x12,
    0x07, 0x0a, 0x03, 0x41, 0x4e, 0x59, 0x10, 0x01, 0x12, 0x08, 0x0a, 0x04, 0x4e, 0x4f, 0x4e, 0x45,
    0x10, 0x02, 0x1a, 0x5c, 0x0a, 0x11, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x46, 0x72,
    0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x12, 0x25, 0x0a, 0x0a, 0x70, 0x72, 0x69, 0x6e, 0x63,
    0x69, 0x70, 0x61, 0x6c, 0x73, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6d, 0x65,
    0x73, 0x6f, 0x73, 0x2e, 0x41, 0x43, 0x4c, 0x2e, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x12, 0x20,
    0x0a, 0x05, 0x72, 0x6f, 0x6c, 0x65, 0x73, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x11, 0x2e,
    0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x41, 0x43, 0x4c, 0x2e, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79,
    0x1a, 0x52, 0x0a, 0x07, 0x52, 0x75, 0x6e, 0x54, 0x61, 0x73, 0x6b, 0x12, 0x25, 0x0a, 0x0a, 0x70,
    0x72, 0x69, 0x6e, 0x63, 0x69, 0x70, 0x61, 0x6c, 0x73, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32,
    0x11, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x41, 0x43, 0x4c, 0x2e, 0x45, 0x6e, 0x74, 0x69,
    0x74, 0x79, 0x12, 0x20, 0x0a, 0x05, 0x75, 0x73, 0x65, 0x72, 0x73, 0x18, 0x02, 0x20, 0x02, 0x28,
    0x0b, 0x32, 0x11, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x41, 0x43, 0x4c, 0x2e, 0x45, 0x6e,
    0x74, 0x69, 0x74, 0x79, 0x1a, 0x6b, 0x0a, 0x11, 0x53, 0x68, 0x75, 0x74, 0x64, 0x6f, 0x77, 0x6e,
    0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x12, 0x25, 0x0a, 0x0a, 0x70, 0x72, 0x69,
    0x6e, 0x63, 0x69, 0x70, 0x61, 0x6c, 0x73, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x11, 0x2e,
    0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x41, 0x43, 0x4c, 0x2e, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79,
    0x12, 0x2f, 0x0a, 0x14, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x5f, 0x70, 0x72,
    0x69, 0x6e, 0x63, 0x69, 0x70, 0x61, 0x6c, 0x73, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x11,
    0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x41, 0x43, 0x4c, 0x2e, 0x45, 0x6e, 0x74, 0x69, 0x74,
    0x79, 0x22, 0xbd, 0x01, 0x0a, 0x04, 0x41, 0x43, 0x4c, 0x73, 0x12, 0x18, 0x0a, 0x0a, 0x70, 0x65,
    0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x76, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x3a, 0x04,
    0x74, 0x72, 0x75, 0x65, 0x12, 0x39, 0x0a, 0x13, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72,
    0x5f, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x1c, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x41, 0x43, 0x4c, 0x2e, 0x52, 0x65,
    0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x12,
    0x25, 0x0a, 0x09, 0x72, 0x75, 0x6e, 0x5f, 0x74, 0x61, 0x73, 0x6b, 0x73, 0x18, 0x03, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x12, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x41, 0x43, 0x4c, 0x2e, 0x52,
    0x75, 0x6e, 0x54, 0x61, 0x73, 0x6b, 0x12, 0x39, 0x0a, 0x13, 0x73, 0x68, 0x75, 0x74, 0x64, 0x6f,
    0x77, 0x6e, 0x5f, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x73, 0x18, 0x04, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x41, 0x43, 0x4c, 0x2e,
    0x53, 0x68, 0x75, 0x74, 0x64, 0x6f, 0x77, 0x6e, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72,
    0x6b, 0x22, 0x3d, 0x0a, 0x09, 0x52, 0x61, 0x74, 0x65, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x12, 0x0b,
    0x0a, 0x03, 0x71, 0x70, 0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x01, 0x12, 0x11, 0x0a, 0x09, 0x70,
    0x72, 0x69, 0x6e, 0x63, 0x69, 0x70, 0x61, 0x6c, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x12, 0x10,
    0x0a, 0x08, 0x63, 0x61, 0x70, 0x61, 0x63, 0x69, 0x74, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04,
    0x22, 0x71, 0x0a, 0x0a, 0x52, 0x61, 0x74, 0x65, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x73, 0x12, 0x20,
    0x0a, 0x06, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x10,
    0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x52, 0x61, 0x74, 0x65, 0x4c, 0x69, 0x6d, 0x69, 0x74,
    0x12, 0x1d, 0x0a, 0x15, 0x61, 0x67, 0x67, 0x72, 0x65, 0x67, 0x61, 0x74, 0x65, 0x5f, 0x64, 0x65,
    0x66, 0x61, 0x75, 0x6c, 0x74, 0x5f, 0x71, 0x70, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x01, 0x12,
    0x22, 0x0a, 0x1a, 0x61, 0x67, 0x67, 0x72, 0x65, 0x67, 0x61, 0x74, 0x65, 0x5f, 0x64, 0x65, 0x66,
    0x61, 0x75, 0x6c, 0x74, 0x5f, 0x63, 0x61, 0x70, 0x61, 0x63, 0x69, 0x74, 0x79, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x04, 0x22, 0x6d, 0x0a, 0x06, 0x56, 0x6f, 0x6c, 0x75, 0x6d, 0x65, 0x12, 0x16, 0x0a,
    0x0e, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x5f, 0x70, 0x61, 0x74, 0x68, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x11, 0x0a, 0x09, 0x68, 0x6f, 0x73, 0x74, 0x5f, 0x70, 0x61,
    0x74, 0x68, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x12, 0x20, 0x0a, 0x04, 0x6d, 0x6f, 0x64, 0x65,
    0x18, 0x03, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x12, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x56,
    0x6f, 0x6c, 0x75, 0x6d, 0x65, 0x2e, 0x4d, 0x6f, 0x64, 0x65, 0x22, 0x16, 0x0a, 0x04, 0x4d, 0x6f,
    0x64, 0x65, 0x12, 0x06, 0x0a, 0x02, 0x52, 0x57, 0x10, 0x01, 0x12, 0x06, 0x0a, 0x02, 0x52, 0x4f,
    0x10, 0x02, 0x22, 0x94, 0x04, 0x0a, 0x0d, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65, 0x72,
    0x49, 0x6e, 0x66, 0x6f, 0x12, 0x27, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x0e, 0x32, 0x19, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x43, 0x6f, 0x6e, 0x74, 0x61,
    0x69, 0x6e, 0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x2e, 0x54, 0x79, 0x70, 0x65, 0x12, 0x1e, 0x0a,
    0x07, 0x76, 0x6f, 0x6c, 0x75, 0x6d, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0d,
    0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x56, 0x6f, 0x6c, 0x75, 0x6d, 0x65, 0x12, 0x10, 0x0a,
    0x08, 0x68, 0x6f, 0x73, 0x74, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x12,
    0x2f, 0x0a, 0x06, 0x64, 0x6f, 0x63, 0x6b, 0x65, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x1f, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65,
    0x72, 0x49, 0x6e, 0x66, 0x6f, 0x2e, 0x44, 0x6f, 0x63, 0x6b, 0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f,
    0x1a, 0xd7, 0x02, 0x0a, 0x0a, 0x44, 0x6f, 0x63, 0x6b, 0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x12,
    0x0d, 0x0a, 0x05, 0x69, 0x6d, 0x61, 0x67, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x3e,
    0x0a, 0x07, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x27, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65,
    0x72, 0x49, 0x6e, 0x66, 0x6f, 0x2e, 0x44, 0x6f, 0x63, 0x6b, 0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f,
    0x2e, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x3a, 0x04, 0x48, 0x4f, 0x53, 0x54, 0x12, 0x42,
    0x0a, 0x0d, 0x70, 0x6f, 0x72, 0x74, 0x5f, 0x6d, 0x61, 0x70, 0x70, 0x69, 0x6e, 0x67, 0x73, 0x18,
    0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x2b, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x43, 0x6f,
    0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x2e, 0x44, 0x6f, 0x63, 0x6b,
    0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x2e, 0x50, 0x6f, 0x72, 0x74, 0x4d, 0x61, 0x70, 0x70, 0x69,
    0x6e, 0x67, 0x12, 0x19, 0x0a, 0x0a, 0x70, 0x72, 0x69, 0x76, 0x69, 0x6c, 0x65, 0x67, 0x65, 0x64,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x3a, 0x05, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x12, 0x24, 0x0a,
    0x0a, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x18, 0x05, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x10, 0x2e, 0x6d, 0x65, 0x73, 0x6f, 0x73, 0x2e, 0x50, 0x61, 0x72, 0x61, 0x6d, 0x65,
    0x74, 0x65, 0x72, 0x1a, 0x4a, 0x0a, 0x0b, 0x50, 0x6f, 0x72, 0x74, 0x4d, 0x61, 0x70, 0x70, 0x69,
    0x6e, 0x67, 0x12, 0x11, 0x0a, 0x09, 0x68, 0x6f, 0x73, 0x74, 0x5f, 0x70, 0x6f, 0x72, 0x74, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x16, 0x0a, 0x0e, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e,
    0x65, 0x72, 0x5f, 0x70, 0x6f, 0x72, 0x74, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x10, 0x0a,
    0x08, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x22,
    0x29, 0x0a, 0x07, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x12, 0x08, 0x0a, 0x04, 0x48, 0x4f,
    0x53, 0x54, 0x10, 0x01, 0x12, 0x0a, 0x0a, 0x06, 0x42, 0x52, 0x49, 0x44, 0x47, 0x45, 0x10, 0x02,
    0x12, 0x08, 0x0a, 0x04, 0x4e, 0x4f, 0x4e, 0x45, 0x10, 0x03, 0x22, 0x1d, 0x0a, 0x04, 0x54, 0x79,
    0x70, 0x65, 0x12, 0x0a, 0x0a, 0x06, 0x44, 0x4f, 0x43, 0x4b, 0x45, 0x52, 0x10, 0x01, 0x12, 0x09,
    0x0a, 0x05, 0x4d, 0x45, 0x53, 0x4f, 0x53, 0x10, 0x02, 0x2a, 0x5c, 0x0a, 0x06, 0x53, 0x74, 0x61,
    0x74, 0x75, 0x73, 0x12, 0x16, 0x0a, 0x12, 0x44, 0x52, 0x49, 0x56, 0x45, 0x52, 0x5f, 0x4e, 0x4f,
    0x54, 0x5f, 0x53, 0x54, 0x41, 0x52, 0x54, 0x45, 0x44, 0x10, 0x01, 0x12, 0x12, 0x0a, 0x0e, 0x44,
    0x52, 0x49, 0x56, 0x45, 0x52, 0x5f, 0x52, 0x55, 0x4e, 0x4e, 0x49, 0x4e, 0x47, 0x10, 0x02, 0x12,
    0x12, 0x0a, 0x0e, 0x44, 0x52, 0x49, 0x56, 0x45, 0x52, 0x5f, 0x41, 0x42, 0x4f, 0x52, 0x54, 0x45,
    0x44, 0x10, 0x03, 0x12, 0x12, 0x0a, 0x0e, 0x44, 0x52, 0x49, 0x56, 0x45, 0x52, 0x5f, 0x53, 0x54,
    0x4f, 0x50, 0x50, 0x45, 0x44, 0x10, 0x04, 0x2a, 0x96, 0x01, 0x0a, 0x09, 0x54, 0x61, 0x73, 0x6b,
    0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x10, 0x0a, 0x0c, 0x54, 0x41, 0x53, 0x4b, 0x5f, 0x53, 0x54,
    0x41, 0x47, 0x49, 0x4e, 0x47, 0x10, 0x06, 0x12, 0x11, 0x0a, 0x0d, 0x54, 0x41, 0x53, 0x4b, 0x5f,
    0x53, 0x54, 0x41, 0x52, 0x54, 0x49, 0x4e, 0x47, 0x10, 0x00, 0x12, 0x10, 0x0a, 0x0c, 0x54, 0x41,
    0x53, 0x4b, 0x5f, 0x52, 0x55, 0x4e, 0x4e, 0x49, 0x4e, 0x47, 0x10, 0x01, 0x12, 0x11, 0x0a, 0x0d,
    0x54, 0x41, 0x53, 0x4b, 0x5f, 0x46, 0x49, 0x4e, 0x49, 0x53, 0x48, 0x45, 0x44, 0x10, 0x02, 0x12,
    0x0f, 0x0a, 0x0b, 0x54, 0x41, 0x53, 0x4b, 0x5f, 0x46, 0x41, 0x49, 0x4c, 0x45, 0x44, 0x10, 0x03,
    0x12, 0x0f, 0x0a, 0x0b, 0x54, 0x41, 0x53, 0x4b, 0x5f, 0x4b, 0x49, 0x4c, 0x4c, 0x45, 0x44, 0x10,
    0x04, 0x12, 0x0d, 0x0a, 0x09, 0x54, 0x41, 0x53, 0x4b, 0x5f, 0x4c, 0x4f, 0x53, 0x54, 0x10, 0x05,
    0x12, 0x0e, 0x0a, 0x0a, 0x54, 0x41, 0x53, 0x4b, 0x5f, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x10, 0x07,
    0x42, 0x1a, 0x0a, 0x10, 0x6f, 0x72, 0x67, 0x2e, 0x61, 0x70, 0x61, 0x63, 0x68, 0x65, 0x2e, 0x6d,
    0x65, 0x73, 0x6f, 0x73, 0x42, 0x06, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0x4a, 0xa0, 0xa8, 0x02,
    0x0a, 0x07, 0x12, 0x05, 0x12, 0x00, 0xa2, 0x07, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03,
    0x12, 0x08, 0x0d, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x14, 0x00, 0x29, 0x0a, 0x0b, 0x0a,
    0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x14, 0x00, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x12, 0x03, 0x14, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x14, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x14, 0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x07,
    0x12, 0x03, 0x14, 0x16, 0x28, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x15, 0x00, 0x27, 0x0a,
    0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x01, 0x12, 0x03, 0x15, 0x00, 0x27, 0x0a, 0x0c, 0x0a, 0x05,
    0x08, 0xe7, 0x07, 0x01, 0x02, 0x12, 0x03, 0x15, 0x07, 0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7,
    0x07, 0x01, 0x02, 0x00, 0x12, 0x03, 0x15, 0x07, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07,
    0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x15, 0x07, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07,
    0x01, 0x07, 0x12, 0x03, 0x15, 0x1e, 0x26, 0x0a, 0x70, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x1c,
    0x00, 0x21, 0x01, 0x1a, 0x64, 0x2a, 0x0a, 0x20, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x20, 0x69,
    0x73, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x63, 0x61,
    0x74, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x20, 0x6f, 0x66, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x73, 0x63, 0x68, 0x65, 0x64, 0x75, 0x6c, 0x65, 0x72, 0x20, 0x61, 0x6e,
    0x64, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x0a, 0x20, 0x64, 0x72, 0x69, 0x76,
    0x65, 0x72, 0x20, 0x61, 0x66, 0x74, 0x65, 0x72, 0x20, 0x66, 0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x20, 0x63, 0x61, 0x6c, 0x6c, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01,
    0x12, 0x03, 0x1c, 0x05, 0x0b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1d,
    0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1d, 0x02, 0x14,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x1d, 0x17, 0x18, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x1e, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1e, 0x02, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x01, 0x02, 0x12, 0x03, 0x1e, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12,
    0x03, 0x1f, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1f,
    0x02, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x1f, 0x13, 0x14,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x03, 0x20, 0x02, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x20, 0x02, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x20, 0x13, 0x14, 0x0a, 0x87, 0x01, 0x0a, 0x02, 0x04, 0x00,
    0x12, 0x04, 0x28, 0x00, 0x2a, 0x01, 0x1a, 0x7b, 0x2a, 0x0a, 0x20, 0x41, 0x20, 0x75, 0x6e, 0x69,
    0x71, 0x75, 0x65, 0x20, 0x49, 0x44, 0x20, 0x61, 0x73, 0x73, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x20,
    0x74, 0x6f, 0x20, 0x61, 0x20, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x20,
    0x41, 0x20, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x20, 0x63, 0x61, 0x6e, 0x20,
    0x72, 0x65, 0x75, 0x73, 0x65, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x49, 0x44, 0x0a, 0x20, 0x69,
    0x6e, 0x20, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x64, 0x6f, 0x20, 0x66, 0x61,
    0x69, 0x6c, 0x6f, 0x76, 0x65, 0x72, 0x20, 0x28, 0x73, 0x65, 0x65, 0x20, 0x4d, 0x65, 0x73, 0x6f,
    0x73, 0x53, 0x63, 0x68, 0x65, 0x64, 0x75, 0x6c, 0x65, 0x72, 0x44, 0x72, 0x69, 0x76, 0x65, 0x72,
    0x29, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x28, 0x08, 0x13, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x29, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x29, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x29, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x29, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x29, 0x1a, 0x1b, 0x0a, 0x31, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x30, 0x00, 0x32, 0x01,
    0x1a, 0x25, 0x2a, 0x0a, 0x20, 0x41, 0x20, 0x75, 0x6e, 0x69, 0x71, 0x75, 0x65, 0x20, 0x49, 0x44,
    0x20, 0x61, 0x73, 0x73, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x6e, 0x20,
    0x6f, 0x66, 0x66, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03,
    0x30, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x31, 0x02, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x31, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x31, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x31, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x31, 0x1a, 0x1b, 0x0a, 0xd0, 0x01, 0x0a, 0x02, 0x04, 0x02, 0x12,
    0x04, 0x3a, 0x00, 0x3c, 0x01, 0x1a, 0xc3, 0x01, 0x2a, 0x0a, 0x20, 0x41, 0x20, 0x75, 0x6e, 0x69,
    0x71, 0x75, 0x65, 0x20, 0x49, 0x44, 0x20, 0x61, 0x73, 0x73, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x20,
    0x74, 0x6f, 0x20, 0x61, 0x20, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x2e, 0x20, 0x43, 0x75, 0x72, 0x72,
    0x65, 0x6e, 0x74, 0x6c, 0x79, 0x2c, 0x20, 0x61, 0x20, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x20, 0x67,
    0x65, 0x74, 0x73, 0x20, 0x61, 0x20, 0x6e, 0x65, 0x77, 0x20, 0x49, 0x44, 0x0a, 0x20, 0x77, 0x68,
    0x65, 0x6e, 0x65, 0x76, 0x65, 0x72, 0x20, 0x69, 0x74, 0x20, 0x28, 0x72, 0x65, 0x29, 0x72, 0x65,
    0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x73, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x4d, 0x65, 0x73,
    0x6f, 0x73, 0x2e, 0x20, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x20, 0x77, 0x72,
    0x69, 0x74, 0x65, 0x72, 0x73, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x6e, 0x27, 0x74, 0x0a,
    0x20, 0x61, 0x73, 0x73, 0x75, 0x6d, 0x65, 0x20, 0x61, 0x6e, 0x79, 0x20, 0x62, 0x69, 0x6e, 0x64,
    0x69, 0x6e, 0x67, 0x20, 0x62, 0x65, 0x74, 0x77, 0x65, 0x65, 0x6e, 0x20, 0x61, 0x20, 0x73, 0x6c,
    0x61, 0x76, 0x65, 0x20, 0x49, 0x44, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x61,
    0x20, 0x68, 0x6f, 0x73, 0x74, 0x6e, 0x61, 0x6d, 0x65, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x02, 0x01, 0x12, 0x03, 0x3a, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12,
    0x03, 0x3b, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x3b,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x3b, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3b, 0x12, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3b, 0x1a, 0x1b, 0x0a, 0x99, 0x02, 0x0a,
    0x02, 0x04, 0x03, 0x12, 0x04, 0x45, 0x00, 0x47, 0x01, 0x1a, 0x8c, 0x02, 0x2a, 0x0a, 0x20, 0x41,
    0x20, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x20, 0x67, 0x65, 0x6e, 0x65, 0x72,
    0x61, 0x74, 0x65, 0x64, 0x20, 0x49, 0x44, 0x20, 0x74, 0x6f, 0x20, 0x64, 0x69, 0x73, 0x74, 0x69,
    0x6e, 0x67, 0x75, 0x69, 0x73, 0x68, 0x20, 0x61, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x2e, 0x20, 0x54,
    0x68, 0x65, 0x20, 0x49, 0x44, 0x20, 0x6d, 0x75, 0x73, 0x74, 0x20, 0x72, 0x65, 0x6d, 0x61, 0x69,
    0x6e, 0x0a, 0x20, 0x75, 0x6e, 0x69, 0x71, 0x75, 0x65, 0x20, 0x77, 0x68, 0x69, 0x6c, 0x65, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x20, 0x69, 0x73, 0x20, 0x61, 0x63, 0x74, 0x69,
    0x76, 0x65, 0x2e, 0x20, 0x48, 0x6f, 0x77, 0x65, 0x76, 0x65, 0x72, 0x2c, 0x20, 0x61, 0x20, 0x66,
    0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x72, 0x65, 0x75,
    0x73, 0x65, 0x20, 0x61, 0x6e, 0x0a, 0x20, 0x49, 0x44, 0x20, 0x5f, 0x6f, 0x6e, 0x6c, 0x79, 0x5f,
    0x20, 0x69, 0x66, 0x20, 0x61, 0x20, 0x70, 0x72, 0x65, 0x76, 0x69, 0x6f, 0x75, 0x73, 0x20, 0x74,
    0x61, 0x73, 0x6b, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x61, 0x6d,
    0x65, 0x20, 0x49, 0x44, 0x20, 0x68, 0x61, 0x73, 0x20, 0x72, 0x65, 0x61, 0x63, 0x68, 0x65, 0x64,
    0x20, 0x61, 0x0a, 0x20, 0x74, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x61, 0x6c, 0x20, 0x73, 0x74, 0x61,
    0x74, 0x65, 0x20, 0x28, 0x65, 0x2e, 0x67, 0x2e, 0x2c, 0x20, 0x54, 0x41, 0x53, 0x4b, 0x5f, 0x46,
    0x49, 0x4e, 0x49, 0x53, 0x48, 0x45, 0x44, 0x2c, 0x20, 0x54, 0x41, 0x53, 0x4b, 0x5f, 0x4c, 0x4f,
    0x53, 0x54, 0x2c, 0x20, 0x54, 0x41, 0x53, 0x4b, 0x5f, 0x4b, 0x49, 0x4c, 0x4c, 0x45, 0x44, 0x2c,
    0x20, 0x65, 0x74, 0x63, 0x2e, 0x29, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12,
    0x03, 0x45, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x46, 0x02,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x46, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x46, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x46, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x46, 0x1a, 0x1b, 0x0a, 0x95, 0x01, 0x0a, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x4f, 0x00, 0x51, 0x01, 0x1a, 0x88, 0x01, 0x2a, 0x0a, 0x20, 0x41, 0x20, 0x66, 0x72,
    0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x20, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x61, 0x74, 0x65,
    0x64, 0x20, 0x49, 0x44, 0x20, 0x74, 0x6f, 0x20, 0x64, 0x69, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x75,
    0x69, 0x73, 0x68, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x2e,
    0x20, 0x4f, 0x6e, 0x6c, 0x79, 0x20, 0x6f, 0x6e, 0x65, 0x0a, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75,
    0x74, 0x6f, 0x72, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x61, 0x6d,
    0x65, 0x20, 0x49, 0x44, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x61, 0x63, 0x74, 0x69,
    0x76, 0x65, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x61, 0x6d, 0x65, 0x20, 0x73,
    0x6c, 0x61, 0x76, 0x65, 0x20, 0x61, 0x74, 0x20, 0x61, 0x0a, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x2e,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x4f, 0x08, 0x12, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x50, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x50, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x50, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x50, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x50,
    0x1a, 0x1b, 0x0a, 0xf7, 0x01, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x5a, 0x00, 0x5c, 0x01, 0x1a,
    0xea, 0x01, 0x2a, 0x0a, 0x20, 0x41, 0x20, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x20, 0x67, 0x65, 0x6e,
    0x65, 0x72, 0x61, 0x74, 0x65, 0x64, 0x20, 0x49, 0x44, 0x20, 0x74, 0x6f, 0x20, 0x64, 0x69, 0x73,
    0x74, 0x69, 0x6e, 0x67, 0x75, 0x69, 0x73, 0x68, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61,
    0x69, 0x6e, 0x65, 0x72, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x49, 0x44, 0x20, 0x6d, 0x75, 0x73,
    0x74, 0x20, 0x62, 0x65, 0x20, 0x75, 0x6e, 0x69, 0x71, 0x75, 0x65, 0x0a, 0x20, 0x62, 0x65, 0x74,
    0x77, 0x65, 0x65, 0x6e, 0x20, 0x61, 0x6e, 0x79, 0x20, 0x61, 0x63, 0x74, 0x69, 0x76, 0x65, 0x20,
    0x6f, 0x72, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x20, 0x63, 0x6f, 0x6e,
    0x74, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x73, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73,
    0x6c, 0x61, 0x76, 0x65, 0x2e, 0x20, 0x49, 0x6e, 0x20, 0x70, 0x61, 0x72, 0x74, 0x69, 0x63, 0x75,
    0x6c, 0x61, 0x72, 0x2c, 0x0a, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x73,
    0x20, 0x66, 0x6f, 0x72, 0x20, 0x64, 0x69, 0x66, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x72,
    0x75, 0x6e, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x61, 0x6d, 0x65, 0x20,
    0x28, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x2c, 0x20, 0x65, 0x78, 0x65, 0x63,
    0x75, 0x74, 0x6f, 0x72, 0x29, 0x20, 0x70, 0x61, 0x69, 0x72, 0x20, 0x6d, 0x75, 0x73, 0x74, 0x20,
    0x62, 0x65, 0x0a, 0x20, 0x75, 0x6e, 0x69, 0x71, 0x75, 0x65, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x05, 0x01, 0x12, 0x03, 0x5a, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00,
    0x12, 0x03, 0x5b, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x5b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x5b, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5b, 0x12, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5b, 0x1a, 0x1b, 0x0a, 0xe9, 0x0b,
    0x0a, 0x02, 0x04, 0x06, 0x12, 0x05, 0x79, 0x00, 0x83, 0x01, 0x01, 0x1a, 0xdb, 0x0b, 0x2a, 0x0a,
    0x20, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x73, 0x20, 0x61, 0x20, 0x66, 0x72, 0x61,
    0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x75, 0x73, 0x65, 0x72,
    0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x69, 0x73, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x74,
    0x6f, 0x20, 0x64, 0x65, 0x74, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x65, 0x20, 0x74, 0x68, 0x65, 0x0a,
    0x20, 0x55, 0x6e, 0x69, 0x78, 0x20, 0x75, 0x73, 0x65, 0x72, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20,
    0x61, 0x6e, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x2f, 0x74, 0x61, 0x73, 0x6b,
    0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x6c, 0x61, 0x75, 0x6e, 0x63,
    0x68, 0x65, 0x64, 0x20, 0x61, 0x73, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x75,
    0x73, 0x65, 0x72, 0x0a, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x69, 0x73, 0x20, 0x73, 0x65,
    0x74, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x6d, 0x70, 0x74, 0x79, 0x20, 0x73, 0x74,
    0x72, 0x69, 0x6e, 0x67, 0x20, 0x4d, 0x65, 0x73, 0x6f, 0x73, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20,
    0x61, 0x75, 0x74, 0x6f, 0x6d, 0x61, 0x67, 0x69, 0x63, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x73, 0x65,
    0x74, 0x20, 0x69, 0x74, 0x0a, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x75, 0x72,
    0x72, 0x65, 0x6e, 0x74, 0x20, 0x75, 0x73, 0x65, 0x72, 0x2e, 0x20, 0x4e, 0x6f, 0x74, 0x65, 0x20,
    0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x49, 0x44, 0x20, 0x69, 0x73, 0x20, 0x6f,
    0x6e, 0x6c, 0x79, 0x20, 0x61, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x61, 0x66,
    0x74, 0x65, 0x72, 0x20, 0x61, 0x0a, 0x20, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b,
    0x20, 0x68, 0x61, 0x73, 0x20, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x65, 0x64, 0x2c,
    0x20, 0x68, 0x6f, 0x77, 0x65, 0x76, 0x65, 0x72, 0x2c, 0x20, 0x69, 0x74, 0x20, 0x69, 0x73, 0x20,
    0x69, 0x6e, 0x63, 0x6c, 0x75, 0x64, 0x65, 0x64, 0x20, 0x68, 0x65, 0x72, 0x65, 0x20, 0x69, 0x6e,
    0x20, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x0a, 0x20, 0x66, 0x61, 0x63, 0x69, 0x6c,
    0x69, 0x74, 0x61, 0x74, 0x65, 0x20, 0x73, 0x63, 0x68, 0x65, 0x64, 0x75, 0x6c, 0x65, 0x72, 0x20,
    0x66, 0x61, 0x69, 0x6c, 0x6f, 0x76, 0x65, 0x72, 0x20, 0x28, 0x69, 0x2e, 0x65, 0x2e, 0x2c, 0x20,
    0x69, 0x66, 0x20, 0x69, 0x74, 0x20, 0x69, 0x73, 0x20, 0x73, 0x65, 0x74, 0x20, 0x74, 0x68, 0x65,
    0x6e, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x4d, 0x65, 0x73, 0x6f, 0x73, 0x53, 0x63, 0x68, 0x65,
    0x64, 0x75, 0x6c, 0x65, 0x72, 0x44, 0x72, 0x69, 0x76, 0x65, 0x72, 0x20, 0x65, 0x78, 0x70, 0x65,
    0x63, 0x74, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x63, 0x68, 0x65, 0x64, 0x75, 0x6c, 0x65,
    0x72, 0x20, 0x69, 0x73, 0x20, 0x70, 0x65, 0x72, 0x66, 0x6f, 0x72, 0x6d, 0x69, 0x6e, 0x67, 0x20,
    0x66, 0x61, 0x69, 0x6c, 0x6f, 0x76, 0x65, 0x72, 0x29, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x65, 0x20,
    0x61, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x20, 0x74,
    0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x20, 0x77,
    0x69, 0x6c, 0x6c, 0x20, 0x77, 0x61, 0x69, 0x74, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x73, 0x63, 0x68, 0x65, 0x64, 0x75, 0x6c, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x0a, 0x20, 0x66,
    0x61, 0x69, 0x6c, 0x6f, 0x76, 0x65, 0x72, 0x20, 0x62, 0x65, 0x66, 0x6f, 0x72, 0x65, 0x20, 0x72,
    0x65, 0x6d, 0x6f, 0x76, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x72, 0x61, 0x6d,
    0x65, 0x77, 0x6f, 0x72, 0x6b, 0x20, 0x69, 0x73, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69,
    0x65, 0x64, 0x20, 0x62, 0x79, 0x0a, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x6f, 0x76, 0x65, 0x72, 0x5f,
    0x74, 0x69, 0x6d, 0x65, 0x6f, 0x75, 0x74, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x63, 0x68, 0x65, 0x63,
    0x6b, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x20, 0x69, 0x73, 0x20, 0x73, 0x65, 0x74, 0x2c, 0x20, 0x66,
    0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x20, 0x70, 0x69, 0x64, 0x2c, 0x20, 0x65, 0x78,
    0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x0a, 0x20, 0x70, 0x69, 0x64, 0x73, 0x20, 0x61, 0x6e, 0x64,
    0x20, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73, 0x20,
    0x61, 0x72, 0x65, 0x20, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x65, 0x64,
    0x20, 0x74, 0x6f, 0x20, 0x64, 0x69, 0x73, 0x6b, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x73, 0x6c, 0x61, 0x76, 0x65, 0x73, 0x2e, 0x0a, 0x20, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x70, 0x6f,
    0x69, 0x6e, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x73, 0x20, 0x61, 0x20,
    0x72, 0x65, 0x73, 0x74, 0x61, 0x72, 0x74, 0x65, 0x64, 0x20, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x20,
    0x74, 0x6f, 0x20, 0x72, 0x65, 0x63, 0x6f, 0x6e, 0x6e, 0x65, 0x63, 0x74, 0x20, 0x77, 0x69, 0x74,
    0x68, 0x20, 0x6f, 0x6c, 0x64, 0x0a, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x73,
    0x20, 0x61, 0x6e, 0x64, 0x20, 0x72, 0x65, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x20, 0x73, 0x74, 0x61,
    0x74, 0x75, 0x73, 0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73, 0x2c, 0x20, 0x61, 0x74, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x64, 0x69, 0x73, 0x6b,
    0x20, 0x49, 0x2f, 0x4f, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x65, 0x20, 0x72, 0x6f, 0x6c, 0x65, 0x20,
    0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x69, 0x73, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f,
    0x20, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x20, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b,
    0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x6c, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x0a, 0x20, 0x64, 0x65, 0x63, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2c, 0x20, 0x64, 0x65, 0x70,
    0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x6c,
    0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x70, 0x6f, 0x6c, 0x69, 0x63, 0x79, 0x20,
    0x62, 0x65, 0x69, 0x6e, 0x67, 0x20, 0x75, 0x73, 0x65, 0x64, 0x2e, 0x0a, 0x20, 0x49, 0x66, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x68, 0x6f, 0x73, 0x74, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x66, 0x69, 0x65,
    0x6c, 0x64, 0x20, 0x69, 0x73, 0x20, 0x73, 0x65, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x6e, 0x20,
    0x65, 0x6d, 0x70, 0x74, 0x79, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x4d, 0x65, 0x73,
    0x6f, 0x73, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x0a, 0x20, 0x61, 0x75, 0x74, 0x6f, 0x6d, 0x61, 0x67,
    0x69, 0x63, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x73, 0x65, 0x74, 0x20, 0x69, 0x74, 0x20, 0x74, 0x6f,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x68, 0x6f, 0x73,
    0x74, 0x6e, 0x61, 0x6d, 0x65, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x65, 0x20, 0x70, 0x72, 0x69, 0x6e,
    0x63, 0x69, 0x70, 0x61, 0x6c, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x73, 0x68, 0x6f, 0x75,
    0x6c, 0x64, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x72, 0x65,
    0x64, 0x65, 0x6e, 0x74, 0x69, 0x61, 0x6c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x72, 0x61, 0x6d,
    0x65, 0x77, 0x6f, 0x72, 0x6b, 0x20, 0x75, 0x73, 0x65, 0x73, 0x0a, 0x20, 0x69, 0x6e, 0x20, 0x61,
    0x75, 0x74, 0x68, 0x65, 0x6e, 0x74, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x20, 0x54,
    0x68, 0x69, 0x73, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x69, 0x73, 0x20, 0x75, 0x73, 0x65,
    0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x20,
    0x41, 0x50, 0x49, 0x20, 0x72, 0x61, 0x74, 0x65, 0x0a, 0x20, 0x65, 0x78, 0x70, 0x6f, 0x72, 0x74,
    0x69, 0x6e, 0x67, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x69, 0x6e, 0x67,
    0x20, 0x61, 0x6e, 0x64, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x73,
    0x65, 0x74, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x20, 0x69, 0x66, 0x20, 0x61, 0x75, 0x74, 0x68, 0x65,
    0x6e, 0x74, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x73, 0x0a, 0x20, 0x6e, 0x6f,
    0x74, 0x20, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65,
    0x73, 0x65, 0x20, 0x66, 0x65, 0x61, 0x74, 0x75, 0x72, 0x65, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20,
    0x64, 0x65, 0x73, 0x69, 0x72, 0x65, 0x64, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x65, 0x20, 0x77, 0x65,
    0x62, 0x75, 0x69, 0x5f, 0x75, 0x72, 0x6c, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x61, 0x6c,
    0x6c, 0x6f, 0x77, 0x73, 0x20, 0x61, 0x20, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b,
    0x20, 0x74, 0x6f, 0x20, 0x61, 0x64, 0x76, 0x65, 0x72, 0x74, 0x69, 0x73, 0x65, 0x20, 0x69, 0x74,
    0x73, 0x20, 0x77, 0x65, 0x62, 0x20, 0x55, 0x49, 0x2c, 0x20, 0x73, 0x6f, 0x0a, 0x20, 0x74, 0x68,
    0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4d, 0x65, 0x73, 0x6f, 0x73, 0x20, 0x77, 0x65, 0x62,
    0x20, 0x55, 0x49, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x6c, 0x69, 0x6e, 0x6b, 0x20, 0x74, 0x6f, 0x20,
    0x69, 0x74, 0x2e, 0x20, 0x49, 0x74, 0x20, 0x69, 0x73, 0x20, 0x65, 0x78, 0x70, 0x65, 0x63, 0x74,
    0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x61, 0x20, 0x66, 0x75, 0x6c, 0x6c, 0x0a,
    0x20, 0x55, 0x52, 0x4c, 0x2c, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c,
    0x65, 0x20, 0x68, 0x74, 0x74, 0x70, 0x3a, 0x2f, 0x2f, 0x6d, 0x79, 0x2d, 0x73, 0x63, 0x68, 0x65,
    0x64, 0x75, 0x6c, 0x65, 0x72, 0x2e, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x2e, 0x63, 0x6f,
    0x6d, 0x3a, 0x38, 0x30, 0x38, 0x30, 0x2f, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01,
    0x12, 0x03, 0x79, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x7a,
    0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x7a, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x7a, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x7a, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x7a, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06,
    0x02, 0x01, 0x12, 0x03, 0x7b, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x7b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x7b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x7b, 0x12,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x7b, 0x19, 0x1a, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x03, 0x7c, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x02, 0x04, 0x12, 0x03, 0x7c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x02, 0x06, 0x12, 0x03, 0x7c, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x7c, 0x17, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x7c, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x03, 0x12, 0x03, 0x7d, 0x02,
    0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x04, 0x12, 0x03, 0x7d, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x05, 0x12, 0x03, 0x7d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x03, 0x01, 0x12, 0x03, 0x7d, 0x12, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x03, 0x03, 0x12, 0x03, 0x7d, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x03, 0x08, 0x12, 0x03, 0x7d, 0x27, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x07,
    0x12, 0x03, 0x7d, 0x32, 0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x04, 0x12, 0x03, 0x7e,
    0x02, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x04, 0x12, 0x03, 0x7e, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x05, 0x12, 0x03, 0x7e, 0x0b, 0x0f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x01, 0x12, 0x03, 0x7e, 0x10, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x04, 0x03, 0x12, 0x03, 0x7e, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x04, 0x08, 0x12, 0x03, 0x7e, 0x1f, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04,
    0x07, 0x12, 0x03, 0x7e, 0x2a, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x05, 0x12, 0x03,
    0x7f, 0x02, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x04, 0x12, 0x03, 0x7f, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x05, 0x12, 0x03, 0x7f, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x01, 0x12, 0x03, 0x7f, 0x12, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x05, 0x03, 0x12, 0x03, 0x7f, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x05, 0x08, 0x12, 0x03, 0x7f, 0x1b, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x05, 0x07, 0x12, 0x03, 0x7f, 0x26, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x06, 0x12,
    0x04, 0x80, 0x01, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x04, 0x12, 0x04,
    0x80, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x05, 0x12, 0x04, 0x80,
    0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x01, 0x12, 0x04, 0x80, 0x01,
    0x12, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x03, 0x12, 0x04, 0x80, 0x01, 0x1d,
    0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x07, 0x12, 0x04, 0x81, 0x01, 0x02, 0x20, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x07, 0x04, 0x12, 0x04, 0x81, 0x01, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x07, 0x05, 0x12, 0x04, 0x81, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x07, 0x01, 0x12, 0x04, 0x81, 0x01, 0x12, 0x1b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x07, 0x03, 0x12, 0x04, 0x81, 0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x06, 0x02, 0x08, 0x12, 0x04, 0x82, 0x01, 0x02, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x08, 0x04, 0x12, 0x04, 0x82, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x08,
    0x05, 0x12, 0x04, 0x82, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x08, 0x01,
    0x12, 0x04, 0x82, 0x01, 0x12, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x08, 0x03, 0x12,
    0x04, 0x82, 0x01, 0x1e, 0x1f, 0x0a, 0x8c, 0x02, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x06, 0x8c, 0x01,
    0x00, 0xc0, 0x01, 0x01, 0x1a, 0xfd, 0x01, 0x2a, 0x0a, 0x20, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69,
    0x62, 0x65, 0x73, 0x20, 0x61, 0x20, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x20, 0x63, 0x68, 0x65,
    0x63, 0x6b, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x20, 0x6f, 0x72,
    0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x20, 0x28, 0x6f, 0x72, 0x20, 0x61, 0x6e,
    0x79, 0x20, 0x61, 0x72, 0x62, 0x69, 0x74, 0x72, 0x61, 0x72, 0x79, 0x0a, 0x20, 0x70, 0x72, 0x6f,
    0x63, 0x65, 0x73, 0x73, 0x2f, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x29, 0x2e, 0x20, 0x41,
    0x20, 0x22, 0x73, 0x74, 0x72, 0x61, 0x74, 0x65, 0x67, 0x79, 0x22, 0x20, 0x69, 0x73, 0x20, 0x70,
    0x69, 0x63, 0x6b, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x79,
    0x69, 0x6e, 0x67, 0x20, 0x6f, 0x6e, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20,
    0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x2c,
    0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x6c, 0x79, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20,
    0x27, 0x68, 0x74, 0x74, 0x70, 0x27, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x27, 0x63, 0x6f, 0x6d, 0x6d,
    0x61, 0x6e, 0x64, 0x27, 0x20, 0x61, 0x72, 0x65, 0x0a, 0x20, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72,
    0x74, 0x65, 0x64, 0x2e, 0x20, 0x53, 0x70, 0x65, 0x63, 0x69, 0x66, 0x79, 0x69, 0x6e, 0x67, 0x20,
    0x6d, 0x6f, 0x72, 0x65, 0x20, 0x74, 0x68, 0x61, 0x6e, 0x20, 0x6f, 0x6e, 0x65, 0x20, 0x73, 0x74,
    0x72, 0x61, 0x74, 0x65, 0x67, 0x79, 0x20, 0x69, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x72, 0x72,
    0x6f, 0x72, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x04, 0x8c, 0x01, 0x08,
    0x13, 0x0a, 0x31, 0x0a, 0x04, 0x04, 0x07, 0x03, 0x00, 0x12, 0x06, 0x8e, 0x01, 0x02, 0x9f, 0x01,
    0x03, 0x1a, 0x21, 0x20, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x73, 0x20, 0x61, 0x6e,
    0x20, 0x48, 0x54, 0x54, 0x50, 0x20, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x20, 0x63, 0x68, 0x65,
    0x63, 0x6b, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x03, 0x00, 0x01, 0x12, 0x04, 0x8e,
    0x01, 0x0a, 0x0e, 0x0a, 0x30, 0x0a, 0x06, 0x04, 0x07, 0x03, 0x00, 0x02, 0x00, 0x12, 0x04, 0x90,
    0x01, 0x04, 0x1d, 0x1a, 0x20, 0x20, 0x50, 0x6f, 0x72, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x73, 0x65,
    0x6e, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x48, 0x54, 0x54, 0x50, 0x20, 0x72, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x2e, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x90, 0x01, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x00,
    0x05, 0x12, 0x04, 0x90, 0x01, 0x0d, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x04, 0x90, 0x01, 0x14, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x04, 0x90, 0x01, 0x1b, 0x1c, 0x0a, 0x24, 0x0a, 0x06, 0x04, 0x07, 0x03,
    0x00, 0x02, 0x01, 0x12, 0x04, 0x93, 0x01, 0x04, 0x2d, 0x1a, 0x14, 0x20, 0x48, 0x54, 0x54, 0x50,
    0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x70, 0x61, 0x74, 0x68, 0x2e, 0x0a, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x93, 0x01, 0x04, 0x0c,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x04, 0x93, 0x01, 0x0d,
    0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0x93, 0x01,
    0x14, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0x93,
    0x01, 0x1b, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x01, 0x08, 0x12, 0x04,
    0x93, 0x01, 0x1d, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x01, 0x07, 0x12,
    0x04, 0x93, 0x01, 0x28, 0x2b, 0x0a, 0x7a, 0x0a, 0x06, 0x04, 0x07, 0x03, 0x00, 0x02, 0x02, 0x12,
    0x04, 0x9b, 0x01, 0x04, 0x21, 0x1a, 0x6a, 0x20, 0x45, 0x78, 0x70, 0x65, 0x63, 0x74, 0x65, 0x64,
    0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x65, 0x73, 0x2e, 0x20, 0x4e, 0x6f, 0x74, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x79, 0x69,
    0x6e, 0x67, 0x20, 0x61, 0x6e, 0x79, 0x20, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x65, 0x73, 0x20,
    0x69, 0x6d, 0x70, 0x6c, 0x69, 0x65, 0x73, 0x0a, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x61, 0x6e,
    0x79, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x65, 0x64, 0x20, 0x73, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x63, 0x63, 0x65, 0x70, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x2e,
    0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0x9b, 0x01,
    0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x02, 0x05, 0x12, 0x04, 0x9b,
    0x01, 0x0d, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04,
    0x9b, 0x01, 0x14, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12,
    0x04, 0x9b, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x04, 0xa1,
    0x01, 0x02, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa1, 0x01,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x06, 0x12, 0x04, 0xa1, 0x01, 0x0b,
    0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa1, 0x01, 0x10, 0x14,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa1, 0x01, 0x17, 0x18, 0x0a,
    0x48, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x04, 0xb0, 0x01, 0x02, 0x35, 0x1a, 0x3a, 0x20,
    0x41, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x20, 0x74,
    0x6f, 0x20, 0x77, 0x61, 0x69, 0x74, 0x20, 0x75, 0x6e, 0x74, 0x69, 0x6c, 0x20, 0x73, 0x74, 0x61,
    0x72, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68,
    0x20, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xb0, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01,
    0x05, 0x12, 0x04, 0xb0, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xb0, 0x01, 0x12, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xb0, 0x01, 0x22, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x08, 0x12, 0x04,
    0xb0, 0x01, 0x24, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x07, 0x12, 0x04, 0xb0,
    0x01, 0x2f, 0x33, 0x0a, 0x2f, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x02, 0x12, 0x04, 0xb3, 0x01, 0x02,
    0x38, 0x1a, 0x21, 0x20, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6c, 0x20, 0x62, 0x65, 0x74,
    0x77, 0x65, 0x65, 0x6e, 0x20, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x20, 0x63, 0x68, 0x65, 0x63,
    0x6b, 0x73, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x04, 0x12, 0x04, 0xb3,
    0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x05, 0x12, 0x04, 0xb3, 0x01,
    0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x01, 0x12, 0x04, 0xb3, 0x01, 0x12,
    0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x03, 0x12, 0x04, 0xb3, 0x01, 0x25, 0x26,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x08, 0x12, 0x04, 0xb3, 0x01, 0x27, 0x37, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x07, 0x12, 0x04, 0xb3, 0x01, 0x32, 0x36, 0x0a, 0x48,
    0x0a, 0x04, 0x04, 0x07, 0x02, 0x03, 0x12, 0x04, 0xb6, 0x01, 0x02, 0x37, 0x1a, 0x3a, 0x20, 0x41,
    0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x20, 0x74, 0x6f,
    0x20, 0x77, 0x61, 0x69, 0x74, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x68, 0x65,
    0x61, 0x6c, 0x74, 0x68, 0x20, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x20, 0x74, 0x6f, 0x20, 0x63, 0x6f,
    0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03,
    0x04, 0x12, 0x04, 0xb6, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x05,
    0x12, 0x04, 0xb6, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x01, 0x12,
    0x04, 0xb6, 0x01, 0x12, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x03, 0x12, 0x04,
    0xb6, 0x01, 0x24, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x08, 0x12, 0x04, 0xb6,
    0x01, 0x26, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x07, 0x12, 0x04, 0xb6, 0x01,
    0x31, 0x35, 0x0a, 0x4a, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x04, 0x12, 0x04, 0xb9, 0x01, 0x02, 0x39,
    0x1a, 0x3c, 0x20, 0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x63, 0x6f, 0x6e,
    0x73, 0x65, 0x63, 0x75, 0x74, 0x69, 0x76, 0x65, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x75, 0x72, 0x65,
    0x73, 0x20, 0x75, 0x6e, 0x74, 0x69, 0x6c, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x69, 0x64, 0x65, 0x72,
    0x65, 0x64, 0x20, 0x75, 0x6e, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x79, 0x2e, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x04, 0x12, 0x04, 0xb9, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x04, 0x05, 0x12, 0x04, 0xb9, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x04, 0x01, 0x12, 0x04, 0xb9, 0x01, 0x12, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x04, 0x03, 0x12, 0x04, 0xb9, 0x01, 0x29, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x04, 0x08, 0x12, 0x04, 0xb9, 0x01, 0x2b, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x04, 0x07, 0x12, 0x04, 0xb9, 0x01, 0x36, 0x37, 0x0a, 0x4a, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x05,
    0x12, 0x04, 0xbc, 0x01, 0x02, 0x3c, 0x1a, 0x3c, 0x20, 0x41, 0x6d, 0x6f, 0x75, 0x6e, 0x74, 0x20,
    0x6f, 0x66, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x6c, 0x6c, 0x6f, 0x77,
    0x20, 0x66, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x20, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x20, 0x63,
    0x68, 0x65, 0x63, 0x6b, 0x73, 0x20, 0x73, 0x69, 0x6e, 0x63, 0x65, 0x20, 0x6c, 0x61, 0x75, 0x6e,
    0x63, 0x68, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x05, 0x04, 0x12, 0x04, 0xbc,
    0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x05, 0x05, 0x12, 0x04, 0xbc, 0x01,
    0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x05, 0x01, 0x12, 0x04, 0xbc, 0x01, 0x12,
    0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x05, 0x03, 0x12, 0x04, 0xbc, 0x01, 0x29, 0x2a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x05, 0x08, 0x12, 0x04, 0xbc, 0x01, 0x2b, 0x3b, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x05, 0x07, 0x12, 0x04, 0xbc, 0x01, 0x36, 0x3a, 0x0a, 0x25,
    0x0a, 0x04, 0x04, 0x07, 0x02, 0x06, 0x12, 0x04, 0xbf, 0x01, 0x02, 0x23, 0x1a, 0x17, 0x20, 0x43,
    0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x20, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x20, 0x63, 0x68,
    0x65, 0x63, 0x6b, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x06, 0x04, 0x12, 0x04,
    0xbf, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x06, 0x06, 0x12, 0x04, 0xbf,
    0x01, 0x0b, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x06, 0x01, 0x12, 0x04, 0xbf, 0x01,
    0x17, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x06, 0x03, 0x12, 0x04, 0xbf, 0x01, 0x21,
    0x22, 0x0a, 0xd5, 0x04, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x06, 0xcd, 0x01, 0x00, 0x82, 0x02, 0x01,
    0x1a, 0xc6, 0x04, 0x2a, 0x0a, 0x20, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x73, 0x20,
    0x61, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x2c, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75,
    0x74, 0x65, 0x64, 0x20, 0x76, 0x69, 0x61, 0x3a, 0x20, 0x27, 0x2f, 0x62, 0x69, 0x6e, 0x2f, 0x73,
    0x68, 0x20, 0x2d, 0x63, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x27, 0x2e, 0x20, 0x41, 0x6e, 0x79,
    0x20, 0x55, 0x52, 0x49, 0x73, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x0a,
    0x20, 0x61, 0x72, 0x65, 0x20, 0x66, 0x65, 0x74, 0x63, 0x68, 0x65, 0x64, 0x20, 0x62, 0x65, 0x66,
    0x6f, 0x72, 0x65, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x2e, 0x20, 0x20, 0x49, 0x66, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x66, 0x69,
    0x65, 0x6c, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x6e, 0x0a, 0x20, 0x75, 0x72, 0x69, 0x20,
    0x69, 0x73, 0x20, 0x73, 0x65, 0x74, 0x2c, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x61, 0x62,
    0x6c, 0x65, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x20, 0x70, 0x65, 0x72, 0x6d, 0x69, 0x73, 0x73, 0x69,
    0x6f, 0x6e, 0x20, 0x69, 0x73, 0x20, 0x73, 0x65, 0x74, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x64, 0x6f, 0x77, 0x6e, 0x6c, 0x6f, 0x61, 0x64, 0x65, 0x64, 0x20, 0x66, 0x69, 0x6c, 0x65,
    0x2e, 0x0a, 0x20, 0x4f, 0x74, 0x68, 0x65, 0x72, 0x77, 0x69, 0x73, 0x65, 0x2c, 0x20, 0x69, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x6f, 0x77, 0x6e, 0x6c, 0x6f, 0x61, 0x64, 0x65, 0x64, 0x20,
    0x66, 0x69, 0x6c, 0x65, 0x20, 0x68, 0x61, 0x73, 0x20, 0x61, 0x20, 0x72, 0x65, 0x63, 0x6f, 0x67,
    0x6e, 0x69, 0x7a, 0x65, 0x64, 0x20, 0x61, 0x72, 0x63, 0x68, 0x69, 0x76, 0x65, 0x20, 0x65, 0x78,
    0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x0a, 0x20, 0x28, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e,
    0x74, 0x6c, 0x79, 0x20, 0x5b, 0x63, 0x6f, 0x6d, 0x70, 0x72, 0x65, 0x73, 0x73, 0x65, 0x64, 0x5d,
    0x20, 0x74, 0x61, 0x72, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x7a, 0x69, 0x70, 0x29, 0x20, 0x69, 0x74,
    0x20, 0x69, 0x73, 0x20, 0x65, 0x78, 0x74, 0x72, 0x61, 0x63, 0x74, 0x65, 0x64, 0x20, 0x69, 0x6e,
    0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x27,
    0x73, 0x0a, 0x20, 0x77, 0x6f, 0x72, 0x6b, 0x69, 0x6e, 0x67, 0x20, 0x64, 0x69, 0x72, 0x65, 0x63,
    0x74, 0x6f, 0x72, 0x79, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x65, 0x78, 0x74, 0x72, 0x61,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x64, 0x69, 0x73,
    0x61, 0x62, 0x6c, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x73, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67,
    0x20, 0x60, 0x65, 0x78, 0x74, 0x72, 0x61, 0x63, 0x74, 0x60, 0x20, 0x74, 0x6f, 0x0a, 0x20, 0x66,
    0x61, 0x6c, 0x73, 0x65, 0x2e, 0x20, 0x49, 0x6e, 0x20, 0x61, 0x64, 0x64, 0x69, 0x74, 0x69, 0x6f,
    0x6e, 0x2c, 0x20, 0x61, 0x6e, 0x79, 0x20, 0x65, 0x6e, 0x76, 0x69, 0x72, 0x6f, 0x6e, 0x6d, 0x65,
    0x6e, 0x74, 0x20, 0x76, 0x61, 0x72, 0x69, 0x61, 0x62, 0x6c, 0x65, 0x73, 0x20, 0x61, 0x72, 0x65,
    0x20, 0x73, 0x65, 0x74, 0x20, 0x62, 0x65, 0x66, 0x6f, 0x72, 0x65, 0x20, 0x65, 0x78, 0x65, 0x63,
    0x75, 0x74, 0x69, 0x6e, 0x67, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61,
    0x6e, 0x64, 0x20, 0x28, 0x73, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x79, 0x20, 0x63, 0x61, 0x6e, 0x20,
    0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x22, 0x70, 0x61, 0x72, 0x61,
    0x6d, 0x65, 0x74, 0x65, 0x72, 0x69, 0x7a, 0x65, 0x22, 0x20, 0x79, 0x6f, 0x75, 0x72, 0x20, 0x63,
    0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x29, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x08, 0x01,
    0x12, 0x04, 0xcd, 0x01, 0x08, 0x13, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x08, 0x03, 0x00, 0x12, 0x06,
    0xce, 0x01, 0x02, 0xd2, 0x01, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x03, 0x00, 0x01, 0x12,
    0x04, 0xce, 0x01, 0x0a, 0x0d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x08, 0x03, 0x00, 0x02, 0x00, 0x12,
    0x04, 0xcf, 0x01, 0x04, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x03, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x04, 0xcf, 0x01, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x03, 0x00, 0x02, 0x00,
    0x05, 0x12, 0x04, 0xcf, 0x01, 0x0d, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x03, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xcf, 0x01, 0x14, 0x19, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x03, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xcf, 0x01, 0x1c, 0x1d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x08, 0x03,
    0x00, 0x02, 0x01, 0x12, 0x04, 0xd0, 0x01, 0x04, 0x21, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x03,
    0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd0, 0x01, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08,
    0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x04, 0xd0, 0x01, 0x0d, 0x11, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x08, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd0, 0x01, 0x12, 0x1c, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x08, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0xd0, 0x01, 0x1f, 0x20, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x08, 0x03, 0x00, 0x02, 0x02, 0x12, 0x04, 0xd1, 0x01, 0x04, 0x2f, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x08, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0xd1, 0x01, 0x04, 0x0c, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x08, 0x03, 0x00, 0x02, 0x02, 0x05, 0x12, 0x04, 0xd1, 0x01, 0x0d, 0x11, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x08, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xd1, 0x01, 0x12, 0x19,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x04, 0xd1, 0x01, 0x1c,
    0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x03, 0x00, 0x02, 0x02, 0x08, 0x12, 0x04, 0xd1, 0x01,
    0x1e, 0x2e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x03, 0x00, 0x02, 0x02, 0x07, 0x12, 0x04, 0xd1,
    0x01, 0x29, 0x2d, 0x0a, 0xb3, 0x03, 0x0a, 0x04, 0x04, 0x08, 0x03, 0x01, 0x12, 0x06, 0xdd, 0x01,
    0x02, 0xe3, 0x01, 0x03, 0x1a, 0xa2, 0x03, 0x20, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65,
    0x73, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x2e, 0x0a, 0x20,
    0x4e, 0x6f, 0x74, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65,
    0x72, 0x69, 0x7a, 0x65, 0x72, 0x73, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x6c, 0x79,
    0x20, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x43, 0x6f, 0x6e, 0x74, 0x61,
    0x69, 0x6e, 0x65, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x2c, 0x20, 0x73, 0x6f, 0x20, 0x69, 0x74, 0x0a,
    0x20, 0x69, 0x73, 0x20, 0x70, 0x6f, 0x73, 0x73, 0x69, 0x62, 0x6c, 0x65, 0x20, 0x74, 0x68, 0x61,
    0x74, 0x20, 0x61, 0x20, 0x6c, 0x61, 0x75, 0x6e, 0x63, 0x68, 0x65, 0x64, 0x20, 0x74, 0x61, 0x73,
    0x6b, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x20, 0x64, 0x75, 0x65, 0x20,
    0x74, 0x6f, 0x20, 0x73, 0x75, 0x70, 0x70, 0x6c, 0x79, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x69,
    0x73, 0x0a, 0x20, 0x61, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x2e, 0x0a, 0x20, 0x4e,
    0x4f, 0x54, 0x45, 0x3a, 0x20, 0x54, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e,
    0x65, 0x72, 0x69, 0x7a, 0x65, 0x72, 0x20, 0x41, 0x50, 0x49, 0x20, 0x69, 0x73, 0x20, 0x63, 0x75,
    0x72, 0x72, 0x65, 0x6e, 0x74, 0x6c, 0x79, 0x20, 0x69, 0x6e, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x61,
    0x72, 0x6c, 0x79, 0x20, 0x62, 0x65, 0x74, 0x61, 0x20, 0x6f, 0x72, 0x0a, 0x20, 0x65, 0x76, 0x65,
    0x6e, 0x20, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x2e, 0x20, 0x53,
    0x6f, 0x6d, 0x65, 0x20, 0x64, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x2c, 0x20, 0x6c, 0x69, 0x6b,
    0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x78, 0x61, 0x63, 0x74, 0x20, 0x73, 0x65, 0x6d, 0x61,
    0x6e, 0x74, 0x69, 0x63, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x6e, 0x0a, 0x20, 0x22, 0x69, 0x6d,
    0x61, 0x67, 0x65, 0x22, 0x20, 0x6f, 0x72, 0x20, 0x22, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x22, 0x20, 0x61, 0x72, 0x65, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x79, 0x65, 0x74, 0x20, 0x68, 0x61,
    0x72, 0x64, 0x65, 0x6e, 0x65, 0x64, 0x2e, 0x0a, 0x20, 0x54, 0x4f, 0x44, 0x4f, 0x28, 0x74, 0x69,
    0x6c, 0x6c, 0x74, 0x29, 0x3a, 0x20, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x65, 0x78, 0x61, 0x63, 0x74, 0x20, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x65, 0x20,
    0x61, 0x6e, 0x64, 0x20, 0x73, 0x65, 0x6d, 0x61, 0x6e, 0x74, 0x69, 0x63, 0x73, 0x20, 0x6f, 0x66,
    0x20, 0x22, 0x69, 0x6d, 0x61, 0x67, 0x65, 0x22, 0x0a, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x22, 0x6f,
    0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x22, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x03,
    0x01, 0x01, 0x12, 0x04, 0xdd, 0x01, 0x0a, 0x17, 0x0a, 0x3a, 0x0a, 0x06, 0x04, 0x08, 0x03, 0x01,
    0x02, 0x00, 0x12, 0x04, 0xdf, 0x01, 0x04, 0x1e, 0x1a, 0x2a, 0x20, 0x55, 0x52, 0x49, 0x20, 0x64,
    0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f,
    0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x20, 0x69, 0x6d, 0x61, 0x67, 0x65, 0x20, 0x6e, 0x61,
    0x6d, 0x65, 0x2e, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x03, 0x01, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xdf, 0x01, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x03, 0x01, 0x02, 0x00, 0x05,
    0x12, 0x04, 0xdf, 0x01, 0x0d, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x03, 0x01, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xdf, 0x01, 0x14, 0x19, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x03, 0x01, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xdf, 0x01, 0x1c, 0x1d, 0x0a, 0x4b, 0x0a, 0x06, 0x04, 0x08, 0x03, 0x01,
    0x02, 0x01, 0x12, 0x04, 0xe2, 0x01, 0x04, 0x20, 0x1a, 0x3b, 0x20, 0x44, 0x65, 0x73, 0x63, 0x72,
    0x69, 0x62, 0x65, 0x73, 0x20, 0x61, 0x64, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20,
    0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x70, 0x61, 0x73, 0x73, 0x65, 0x64, 0x20, 0x74,
    0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x69,
    0x7a, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x03, 0x01, 0x02, 0x01, 0x04,
    0x12, 0x04, 0xe2, 0x01, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x03, 0x01, 0x02, 0x01,
    0x05, 0x12, 0x04, 0xe2, 0x01, 0x0d, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x03, 0x01, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xe2, 0x01, 0x14, 0x1b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x08, 0x03, 0x01,
    0x02, 0x01, 0x03, 0x12, 0x04, 0xe2, 0x01, 0x1e, 0x1f, 0x0a, 0x80, 0x01, 0x0a, 0x04, 0x04, 0x08,
    0x02, 0x00, 0x12, 0x04, 0xe7, 0x01, 0x02, 0x27, 0x1a, 0x72, 0x20, 0x4e, 0x4f, 0x54, 0x45, 0x3a,
    0x20, 0x4d, 0x65, 0x73, 0x6f, 0x73, 0x43, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x69,
    0x7a, 0x65, 0x72, 0x20, 0x64, 0x6f, 0x65, 0x73, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74,
    0x6c, 0x79, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x74,
    0x68, 0x69, 0x73, 0x0a, 0x20, 0x61, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x20, 0x61,
    0x6e, 0x64, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x73, 0x20, 0x73, 0x75, 0x70, 0x70, 0x6c, 0x79, 0x69,
    0x6e, 0x67, 0x20, 0x61, 0x20, 0x27, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x27,
    0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe7, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x00, 0x06, 0x12, 0x04, 0xe7, 0x01, 0x0b, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xe7, 0x01, 0x19, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xe7, 0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01,
    0x12, 0x04, 0xe9, 0x01, 0x02, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12,
    0x04, 0xe9, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x06, 0x12, 0x04,
    0xe9, 0x01, 0x0b, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x04, 0xe9,
    0x01, 0x0f, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x04, 0xe9, 0x01,
    0x16, 0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x02, 0x12, 0x04, 0xeb, 0x01, 0x02, 0x27,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x04, 0x12, 0x04, 0xeb, 0x01, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x06, 0x12, 0x04, 0xeb, 0x01, 0x0b, 0x16, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x01, 0x12, 0x04, 0xeb, 0x01, 0x17, 0x22, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x02, 0x03, 0x12, 0x04, 0xeb, 0x01, 0x25, 0x26, 0x0a, 0xfb, 0x05, 0x0a,
    0x04, 0x04, 0x08, 0x02, 0x03, 0x12, 0x04, 0xfa, 0x01, 0x02, 0x2b, 0x1a, 0xec, 0x05, 0x20, 0x54,
    0x68, 0x65, 0x72, 0x65, 0x20, 0x61, 0x72, 0x65, 0x20, 0x74, 0x77, 0x6f, 0x20, 0x77, 0x61, 0x79,
    0x73, 0x20, 0x74, 0x6f, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x79, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x3a, 0x0a, 0x20, 0x31, 0x29, 0x20, 0x49, 0x66,
    0x20, 0x27, 0x73, 0x68, 0x65, 0x6c, 0x6c, 0x20, 0x3d, 0x3d, 0x20, 0x74, 0x72, 0x75, 0x65, 0x27,
    0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x20, 0x77, 0x69,
    0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x6c, 0x61, 0x75, 0x6e, 0x63, 0x68, 0x65, 0x64, 0x20, 0x76,
    0x69, 0x61, 0x20, 0x73, 0x68, 0x65, 0x6c, 0x6c, 0x0a, 0x09, 0x09, 0x28, 0x69, 0x2e, 0x65, 0x2e,
    0x2c, 0x20, 0x2f, 0x62, 0x69, 0x6e, 0x2f, 0x73, 0x68, 0x20, 0x2d, 0x63, 0x20, 0x27, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x27, 0x29, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x27, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x27, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x20, 0x77, 0x69, 0x6c,
    0x6c, 0x20, 0x62, 0x65, 0x0a, 0x09, 0x09, 0x74, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x20, 0x61,
    0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x68, 0x65, 0x6c, 0x6c, 0x20, 0x63, 0x6f, 0x6d, 0x6d,
    0x61, 0x6e, 0x64, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x27, 0x61, 0x72, 0x67, 0x75, 0x6d, 0x65,
    0x6e, 0x74, 0x73, 0x27, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x69, 0x67, 0x6e,
    0x6f, 0x72, 0x65, 0x64, 0x2e, 0x0a, 0x20, 0x32, 0x29, 0x20, 0x49, 0x66, 0x20, 0x27, 0x73, 0x68,
    0x65, 0x6c, 0x6c, 0x20, 0x3d, 0x3d, 0x20, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x27, 0x2c, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20,
    0x62, 0x65, 0x20, 0x6c, 0x61, 0x75, 0x6e, 0x63, 0x68, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x70,
    0x61, 0x73, 0x73, 0x69, 0x6e, 0x67, 0x0a, 0x09, 0x09, 0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e,
    0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x61,
    0x62, 0x6c, 0x65, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x27, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x27,
    0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20,
    0x62, 0x65, 0x0a, 0x09, 0x09, 0x74, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x20, 0x61, 0x73, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x2e, 0x20,
    0x54, 0x68, 0x65, 0x20, 0x27, 0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x27, 0x0a,
    0x09, 0x09, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x74, 0x72, 0x65, 0x61, 0x74, 0x65,
    0x64, 0x20, 0x61, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e,
    0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74,
    0x61, 0x62, 0x6c, 0x65, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x0a, 0x09, 0x09,
    0x73, 0x69, 0x6d, 0x69, 0x6c, 0x61, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x68, 0x6f, 0x77, 0x20, 0x50,
    0x4f, 0x53, 0x49, 0x58, 0x20, 0x65, 0x78, 0x65, 0x63, 0x20, 0x66, 0x61, 0x6d, 0x69, 0x6c, 0x69,
    0x65, 0x73, 0x20, 0x6c, 0x61, 0x75, 0x6e, 0x63, 0x68, 0x20, 0x70, 0x72, 0x6f, 0x63, 0x65, 0x73,
    0x73, 0x65, 0x73, 0x20, 0x28, 0x69, 0x2e, 0x65, 0x2e, 0x2c, 0x0a, 0x09, 0x09, 0x65, 0x78, 0x65,
    0x63, 0x6c, 0x70, 0x28, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x2c, 0x20, 0x61, 0x72, 0x67, 0x75, 0x6d,
    0x65, 0x6e, 0x74, 0x73, 0x28, 0x30, 0x29, 0x2c, 0x20, 0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e,
    0x74, 0x73, 0x28, 0x31, 0x29, 0x2c, 0x20, 0x2e, 0x2e, 0x2e, 0x29, 0x29, 0x2e, 0x0a, 0x20, 0x4e,
    0x4f, 0x54, 0x45, 0x3a, 0x20, 0x54, 0x68, 0x65, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x27,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x27, 0x20, 0x69, 0x73, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65,
    0x64, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x27, 0x72, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64,
    0x27, 0x20, 0x74, 0x6f, 0x20, 0x27, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x27, 0x0a,
    0x20, 0x69, 0x6e, 0x20, 0x30, 0x2e, 0x32, 0x30, 0x2e, 0x30, 0x2e, 0x20, 0x49, 0x74, 0x20, 0x77,
    0x69, 0x6c, 0x6c, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x63, 0x61, 0x75, 0x73, 0x65, 0x20, 0x69,
    0x73, 0x73, 0x75, 0x65, 0x73, 0x20, 0x69, 0x66, 0x20, 0x61, 0x20, 0x6e, 0x65, 0x77, 0x20, 0x66,
    0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x20, 0x69, 0x73, 0x0a, 0x20, 0x63, 0x6f, 0x6e,
    0x6e, 0x65, 0x63, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x6e, 0x20, 0x6f, 0x6c,
    0x64, 0x20, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x03, 0x04, 0x12, 0x04, 0xfa, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x03, 0x05, 0x12, 0x04, 0xfa, 0x01, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03,
    0x01, 0x12, 0x04, 0xfa, 0x01, 0x10, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x03,
    0x12, 0x04, 0xfa, 0x01, 0x18, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x08, 0x12,
    0x04, 0xfa, 0x01, 0x1a, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x07, 0x12, 0x04,
    0xfa, 0x01, 0x25, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x04, 0x12, 0x04, 0xfb, 0x01,
    0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x04, 0x12, 0x04, 0xfb, 0x01, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x05, 0x12, 0x04, 0xfb, 0x01, 0x0b, 0x11,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x01, 0x12, 0x04, 0xfb, 0x01, 0x12, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x03, 0x12, 0x04, 0xfb, 0x01, 0x1a, 0x1b, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x08, 0x02, 0x05, 0x12, 0x04, 0xfc, 0x01, 0x02, 0x20, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x05, 0x04, 0x12, 0x04, 0xfc, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x05, 0x05, 0x12, 0x04, 0xfc, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x05, 0x01, 0x12, 0x04, 0xfc, 0x01, 0x12, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x05, 0x03, 0x12, 0x04, 0xfc, 0x01, 0x1e, 0x1f, 0x0a, 0xb2, 0x01, 0x0a, 0x04, 0x04, 0x08, 0x02,
    0x06, 0x12, 0x04, 0x81, 0x02, 0x02, 0x1b, 0x1a, 0xa3, 0x01, 0x20, 0x45, 0x6e, 0x61, 0x62, 0x6c,
    0x65, 0x73, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x20, 0x61, 0x6e, 0x64, 0x20,
    0x74, 0x61, 0x73, 0x6b, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x75, 0x6e, 0x20, 0x61, 0x73, 0x20,
    0x61, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x63, 0x20, 0x75, 0x73, 0x65, 0x72, 0x2e,
    0x20, 0x49, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x75, 0x73, 0x65, 0x72, 0x0a, 0x20, 0x66, 0x69,
    0x65, 0x6c, 0x64, 0x20, 0x69, 0x73, 0x20, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x62,
    0x6f, 0x74, 0x68, 0x20, 0x69, 0x6e, 0x20, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b,
    0x49, 0x6e, 0x66, 0x6f, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x68, 0x65, 0x72, 0x65, 0x2c, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x49, 0x6e, 0x66, 0x6f, 0x0a, 0x20,
    0x75, 0x73, 0x65, 0x72, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x74, 0x61, 0x6b, 0x65, 0x73,
    0x20, 0x70, 0x72, 0x65, 0x63, 0x65, 0x64, 0x65, 0x6e, 0x63, 0x65, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x06, 0x04, 0x12, 0x04, 0x81, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x06, 0x05, 0x12, 0x04, 0x81, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x06, 0x01, 0x12, 0x04, 0x81, 0x02, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x06, 0x03, 0x12, 0x04, 0x81, 0x02, 0x19, 0x1a, 0x0a, 0x80, 0x01, 0x0a, 0x02, 0x04, 0x09,
    0x12, 0x06, 0x89, 0x02, 0x00, 0x9c, 0x02, 0x01, 0x1a, 0x72, 0x2a, 0x0a, 0x20, 0x44, 0x65, 0x73,
    0x63, 0x72, 0x69, 0x62, 0x65, 0x73, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x20, 0x61, 0x62, 0x6f, 0x75, 0x74, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x78, 0x65, 0x63,
    0x75, 0x74, 0x6f, 0x72, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x27, 0x64, 0x61, 0x74, 0x61, 0x27,
    0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x0a, 0x20, 0x75,
    0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x70, 0x61, 0x73, 0x73, 0x20, 0x61, 0x72, 0x62, 0x69,
    0x74, 0x72, 0x61, 0x72, 0x79, 0x20, 0x62, 0x79, 0x74, 0x65, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x61,
    0x6e, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x09, 0x01, 0x12, 0x04, 0x89, 0x02, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02,
    0x00, 0x12, 0x04, 0x8a, 0x02, 0x02, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x8a, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x06, 0x12,
    0x04, 0x8a, 0x02, 0x0b, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x04,
    0x8a, 0x02, 0x16, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8a,
    0x02, 0x24, 0x25, 0x0a, 0x2f, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x04, 0x8b, 0x02, 0x02,
    0x28, 0x22, 0x21, 0x20, 0x54, 0x4f, 0x44, 0x4f, 0x28, 0x62, 0x65, 0x6e, 0x68, 0x29, 0x3a, 0x20,
    0x4d, 0x61, 0x6b, 0x65, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x72, 0x65, 0x71, 0x75, 0x69, 0x72,
    0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x04, 0x12, 0x04, 0x8b,
    0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x06, 0x12, 0x04, 0x8b, 0x02,
    0x0b, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8b, 0x02, 0x17,
    0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8b, 0x02, 0x26, 0x27,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x02, 0x12, 0x04, 0x8c, 0x02, 0x02, 0x23, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x04, 0x12, 0x04, 0x8c, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x02, 0x06, 0x12, 0x04, 0x8c, 0x02, 0x0b, 0x16, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x02, 0x01, 0x12, 0x04, 0x8c, 0x02, 0x17, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x02, 0x03, 0x12, 0x04, 0x8c, 0x02, 0x21, 0x22, 0x0a, 0xa7, 0x01, 0x0a, 0x04, 0x04,
    0x09, 0x02, 0x03, 0x12, 0x04, 0x90, 0x02, 0x02, 0x28, 0x1a, 0x98, 0x01, 0x20, 0x45, 0x78, 0x65,
    0x63, 0x75, 0x74, 0x6f, 0x72, 0x20, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x64, 0x20, 0x77,
    0x69, 0x74, 0x68, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x20,
    0x77, 0x69, 0x6c, 0x6c, 0x20, 0x6c, 0x61, 0x75, 0x6e, 0x63, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x0a, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x27, 0x73, 0x20, 0x43,
    0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x49, 0x6e, 0x66, 0x6f, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x77,
    0x65, 0x20, 0x65, 0x78, 0x70, 0x65, 0x63, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e,
    0x74, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x0a, 0x20, 0x61, 0x63, 0x74, 0x20, 0x61,
    0x73, 0x20, 0x61, 0x20, 0x4d, 0x65, 0x73, 0x6f, 0x73, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74,
    0x6f, 0x72, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x04, 0x12, 0x04, 0x90,
    0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x06, 0x12, 0x04, 0x90, 0x02,
    0x0b, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x01, 0x12, 0x04, 0x90, 0x02, 0x19,
    0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x03, 0x12, 0x04, 0x90, 0x02, 0x25, 0x27,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x04, 0x12, 0x04, 0x91, 0x02, 0x02, 0x22, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x04, 0x04, 0x12, 0x04, 0x91, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x04, 0x06, 0x12, 0x04, 0x91, 0x02, 0x0b, 0x13, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x04, 0x01, 0x12, 0x04, 0x91, 0x02, 0x14, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x04, 0x03, 0x12, 0x04, 0x91, 0x02, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09,
    0x02, 0x05, 0x12, 0x04, 0x92, 0x02, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x05,
    0x04, 0x12, 0x04, 0x92, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x05, 0x05,
    0x12, 0x04, 0x92, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x05, 0x01, 0x12,
    0x04, 0x92, 0x02, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x05, 0x03, 0x12, 0x04,
    0x92, 0x02, 0x19, 0x1a, 0x0a, 0x80, 0x03, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x06, 0x12, 0x04, 0x9a,
    0x02, 0x02, 0x1e, 0x1a, 0xf1, 0x02, 0x20, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x20, 0x69, 0x73,
    0x20, 0x61, 0x6e, 0x20, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72, 0x20, 0x73,
    0x74, 0x79, 0x6c, 0x65, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x75, 0x73, 0x65, 0x64,
    0x20, 0x62, 0x79, 0x20, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x73, 0x20, 0x74,
    0x6f, 0x20, 0x74, 0x72, 0x61, 0x63, 0x6b, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x6f, 0x75,
    0x72, 0x63, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74,
    0x6f, 0x72, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x75, 0x73, 0x65, 0x66,
    0x75, 0x6c, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x69, 0x74, 0x27, 0x73, 0x20, 0x70, 0x6f, 0x73,
    0x73, 0x69, 0x62, 0x6c, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x0a, 0x20, 0x64, 0x69, 0x66, 0x66, 0x65,
    0x72, 0x65, 0x6e, 0x74, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x20, 0x69, 0x64,
    0x73, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x72, 0x65, 0x6c, 0x61, 0x74, 0x65, 0x64, 0x20,
    0x73, 0x65, 0x6d, 0x61, 0x6e, 0x74, 0x69, 0x63, 0x61, 0x6c, 0x6c, 0x79, 0x2e, 0x0a, 0x20, 0x4e,
    0x4f, 0x54, 0x45, 0x3a, 0x20, 0x53, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x20, 0x69, 0x73, 0x20, 0x65,
    0x78, 0x70, 0x6f, 0x73, 0x65, 0x64, 0x20, 0x61, 0x6c, 0x6f, 0x6e, 0x67, 0x73, 0x69, 0x64, 0x65,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x20, 0x75, 0x73,
    0x61, 0x67, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x65, 0x78, 0x65, 0x63,
    0x75, 0x74, 0x6f, 0x72, 0x20, 0x76, 0x69, 0x61, 0x20, 0x4a, 0x53, 0x4f, 0x4e, 0x20, 0x6f, 0x6e,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73,
    0x20, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x73, 0x20, 0x75, 0x73, 0x65, 0x72, 0x73, 0x20, 0x74, 0x6f,
    0x20, 0x69, 0x6d, 0x70, 0x6f, 0x72, 0x74, 0x0a, 0x20, 0x75, 0x73, 0x61, 0x67, 0x65, 0x20, 0x69,
    0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x6e, 0x74, 0x6f, 0x20,
    0x61, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x20, 0x73, 0x65, 0x72, 0x69, 0x65, 0x73, 0x20, 0x64, 0x61,
    0x74, 0x61, 0x62, 0x61, 0x73, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x6d, 0x6f, 0x6e, 0x69, 0x74,
    0x6f, 0x72, 0x69, 0x6e, 0x67, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x06, 0x04,
    0x12, 0x04, 0x9a, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x06, 0x05, 0x12,
    0x04, 0x9a, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x06, 0x01, 0x12, 0x04,
    0x9a, 0x02, 0x12, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x06, 0x03, 0x12, 0x04, 0x9a,
    0x02, 0x1b, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x07, 0x12, 0x04, 0x9b, 0x02, 0x02,
    0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x07, 0x04, 0x12, 0x04, 0x9b, 0x02, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x07, 0x05, 0x12, 0x04, 0x9b, 0x02, 0x0b, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x07, 0x01, 0x12, 0x04, 0x9b, 0x02, 0x11, 0x15, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x07, 0x03, 0x12, 0x04, 0x9b, 0x02, 0x18, 0x19, 0x0a, 0xa9, 0x01,
    0x0a, 0x02, 0x04, 0x0a, 0x12, 0x06, 0xa4, 0x02, 0x00, 0xaa, 0x02, 0x01, 0x1a, 0x9a, 0x01, 0x2a,
    0x0a, 0x20, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x73, 0x20, 0x61, 0x20, 0x6d, 0x61,
    0x73, 0x74, 0x65, 0x72, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20,
    0x70, 0x72, 0x6f, 0x62, 0x61, 0x62, 0x6c, 0x79, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20, 0x6d, 0x6f,
    0x72, 0x65, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65,
    0x0a, 0x20, 0x66, 0x75, 0x74, 0x75, 0x72, 0x65, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x6d,
    0x69, 0x67, 0x68, 0x74, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x2c, 0x20, 0x66, 0x6f,
    0x72, 0x20, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x2c, 0x20, 0x74, 0x6f, 0x20, 0x6c, 0x69,
    0x6e, 0x6b, 0x20, 0x61, 0x20, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x20, 0x77,
    0x65, 0x62, 0x75, 0x69, 0x0a, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x20, 0x6d, 0x61, 0x73, 0x74, 0x65,
    0x72, 0x20, 0x77, 0x65, 0x62, 0x75, 0x69, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0a, 0x01,
    0x12, 0x04, 0xa4, 0x02, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x04,
    0xa5, 0x02, 0x02, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa5,
    0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa5, 0x02,
    0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa5, 0x02, 0x12,
    0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa5, 0x02, 0x17, 0x18,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x01, 0x12, 0x04, 0xa6, 0x02, 0x02, 0x19, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa6, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0a, 0x02, 0x01, 0x05, 0x12, 0x04, 0xa6, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa6, 0x02, 0x12, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa6, 0x02, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0a,
    0x02, 0x02, 0x12, 0x04, 0xa7, 0x02, 0x02, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02,
    0x04, 0x12, 0x04, 0xa7, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x05,
    0x12, 0x04, 0xa7, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x01, 0x12,
    0x04, 0xa7, 0x02, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x03, 0x12, 0x04,
    0xa7, 0x02, 0x19, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x08, 0x12, 0x04, 0xa7,
    0x02, 0x1b, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x07, 0x12, 0x04, 0xa7, 0x02,
    0x26, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x03, 0x12, 0x04, 0xa8, 0x02, 0x02, 0x1a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x03, 0x04, 0x12, 0x04, 0xa8, 0x02, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x03, 0x05, 0x12, 0x04, 0xa8, 0x02, 0x0b, 0x11, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x04, 0xa8, 0x02, 0x12, 0x15, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0a, 0x02, 0x03, 0x03, 0x12, 0x04, 0xa8, 0x02, 0x18, 0x19, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x0a, 0x02, 0x04, 0x12, 0x04, 0xa9, 0x02, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a,
    0x02, 0x04, 0x04, 0x12, 0x04, 0xa9, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x04, 0x05, 0x12, 0x04, 0xa9, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x04,
    0x01, 0x12, 0x04, 0xa9, 0x02, 0x12, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x04, 0x03,
    0x12, 0x04, 0xa9, 0x02, 0x1d, 0x1e, 0x0a, 0xd5, 0x02, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x06, 0xb4,
    0x02, 0x00, 0xbb, 0x02, 0x01, 0x1a, 0xc6, 0x02, 0x2a, 0x0a, 0x20, 0x44, 0x65, 0x73, 0x63, 0x72,
    0x69, 0x62, 0x65, 0x73, 0x20, 0x61, 0x20, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x2e, 0x20, 0x4e, 0x6f,
    0x74, 0x65, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x27, 0x69, 0x64, 0x27,
    0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x69, 0x73, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x61,
    0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x61, 0x66, 0x74, 0x65, 0x72, 0x0a, 0x20,
    0x61, 0x20, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x20, 0x69, 0x73, 0x20, 0x72, 0x65, 0x67, 0x69, 0x73,
    0x74, 0x65, 0x72, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d,
    0x61, 0x73, 0x74, 0x65, 0x72, 0x2c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x69, 0x73, 0x20, 0x6d, 0x61,
    0x64, 0x65, 0x20, 0x61, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x68, 0x65, 0x72,
    0x65, 0x0a, 0x20, 0x74, 0x6f, 0x20, 0x66, 0x61, 0x63, 0x69, 0x6c, 0x69, 0x74, 0x61, 0x74, 0x65,
    0x20, 0x72, 0x65, 0x2d, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x2e, 0x20, 0x20, 0x49, 0x66, 0x20, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x70, 0x6f, 0x69, 0x6e, 0x74,
    0x20, 0x69, 0x73, 0x20, 0x73, 0x65, 0x74, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x6c, 0x61,
    0x76, 0x65, 0x20, 0x69, 0x73, 0x0a, 0x20, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x70, 0x6f, 0x69, 0x6e,
    0x74, 0x69, 0x6e, 0x67, 0x20, 0x69, 0x74, 0x73, 0x20, 0x6f, 0x77, 0x6e, 0x20, 0x69, 0x6e, 0x66,
    0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x70, 0x6f, 0x74,
    0x65, 0x6e, 0x74, 0x69, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f,
    0x72, 0x6b, 0x73, 0x27, 0x0a, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x20, 0x28, 0x69, 0x66, 0x20, 0x61, 0x20, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72,
    0x6b, 0x20, 0x68, 0x61, 0x73, 0x20, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x70, 0x6f, 0x69, 0x6e, 0x74,
    0x69, 0x6e, 0x67, 0x20, 0x65, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x29, 0x2e, 0x0a, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x04, 0xb4, 0x02, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0b, 0x02, 0x00, 0x12, 0x04, 0xb5, 0x02, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xb5, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00,
    0x05, 0x12, 0x04, 0xb5, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xb5, 0x02, 0x12, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12,
    0x04, 0xb5, 0x02, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x01, 0x12, 0x04, 0xb6,
    0x02, 0x02, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x04, 0x12, 0x04, 0xb6, 0x02,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x05, 0x12, 0x04, 0xb6, 0x02, 0x0b,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01, 0x12, 0x04, 0xb6, 0x02, 0x11, 0x15,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12, 0x04, 0xb6, 0x02, 0x18, 0x19, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x08, 0x12, 0x04, 0xb6, 0x02, 0x1a, 0x2a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x07, 0x12, 0x04, 0xb6, 0x02, 0x25, 0x29, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x0b, 0x02, 0x02, 0x12, 0x04, 0xb7, 0x02, 0x02, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x02, 0x04, 0x12, 0x04, 0xb7, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x02, 0x06, 0x12, 0x04, 0xb7, 0x02, 0x0b, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x02, 0x01, 0x12, 0x04, 0xb7, 0x02, 0x14, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02,
    0x03, 0x12, 0x04, 0xb7, 0x02, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x03, 0x12,
    0x04, 0xb8, 0x02, 0x02, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x04, 0x12, 0x04,
    0xb8, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x06, 0x12, 0x04, 0xb8,
    0x02, 0x0b, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x01, 0x12, 0x04, 0xb8, 0x02,
    0x15, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x03, 0x03, 0x12, 0x04, 0xb8, 0x02, 0x22,
    0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x04, 0x12, 0x04, 0xb9, 0x02, 0x02, 0x1a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x04, 0x12, 0x04, 0xb9, 0x02, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x04, 0x06, 0x12, 0x04, 0xb9, 0x02, 0x0b, 0x12, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x04, 0x01, 0x12, 0x04, 0xb9, 0x02, 0x13, 0x15, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x04, 0x03, 0x12, 0x04, 0xb9, 0x02, 0x18, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0b, 0x02, 0x05, 0x12, 0x04, 0xba, 0x02, 0x02, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x05, 0x04, 0x12, 0x04, 0xba, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05,
    0x05, 0x12, 0x04, 0xba, 0x02, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05, 0x01,
    0x12, 0x04, 0xba, 0x02, 0x10, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05, 0x03, 0x12,
    0x04, 0xba, 0x02, 0x1d, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05, 0x08, 0x12, 0x04,
    0xba, 0x02, 0x1f, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x05, 0x07, 0x12, 0x04, 0xba,
    0x02, 0x2a, 0x2f, 0x0a, 0x85, 0x01, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x06, 0xc2, 0x02, 0x00, 0xe4,
    0x02, 0x01, 0x1a, 0x77, 0x2a, 0x0a, 0x20, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x73,
    0x20, 0x61, 0x6e, 0x20, 0x41, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x20, 0x6f, 0x72,
    0x20, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x20, 0x22, 0x76, 0x61, 0x6c, 0x75, 0x65,
    0x22, 0x2e, 0x20, 0x41, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x69, 0x73, 0x20, 0x64, 0x65,
    0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x64, 0x0a, 0x20, 0x75, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x73, 0x74, 0x61, 0x6e, 0x64, 0x61, 0x72, 0x64, 0x20, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x63, 0x6f, 0x6c, 0x20, 0x62, 0x75, 0x66, 0x66, 0x65, 0x72, 0x20, 0x22, 0x75, 0x6e, 0x69,
    0x6f, 0x6e, 0x22, 0x20, 0x74, 0x72, 0x69, 0x63, 0x6b, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x0c, 0x01, 0x12, 0x04, 0xc2, 0x02, 0x08, 0x0d, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x0c, 0x04, 0x00,
    0x12, 0x06, 0xc3, 0x02, 0x02, 0xc8, 0x02, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x04, 0x00,
    0x01, 0x12, 0x04, 0xc3, 0x02, 0x07, 0x0b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0c, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x04, 0xc4, 0x02, 0x04, 0x0f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xc4, 0x02, 0x04, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x04, 0x00,
    0x02, 0x00, 0x02, 0x12, 0x04, 0xc4, 0x02, 0x0d, 0x0e, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0c, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x04, 0xc5, 0x02, 0x04, 0x0f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc5, 0x02, 0x04, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c,
    0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xc5, 0x02, 0x0d, 0x0e, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x0c, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0xc6, 0x02, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x0c, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xc6, 0x02, 0x04, 0x07, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x0c, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0xc6, 0x02, 0x0a, 0x0b, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x0c, 0x04, 0x00, 0x02, 0x03, 0x12, 0x04, 0xc7, 0x02, 0x04, 0x0d, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x0c, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0xc7, 0x02, 0x04, 0x08, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x0c, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0xc7, 0x02, 0x0b, 0x0c, 0x0a,
    0x0e, 0x0a, 0x04, 0x04, 0x0c, 0x03, 0x00, 0x12, 0x06, 0xca, 0x02, 0x02, 0xcc, 0x02, 0x03, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x03, 0x00, 0x01, 0x12, 0x04, 0xca, 0x02, 0x0a, 0x10, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x00, 0x12, 0x04, 0xcb, 0x02, 0x04, 0x1e, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0xcb, 0x02, 0x04, 0x0c, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x04, 0xcb, 0x02, 0x0d, 0x13,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xcb, 0x02, 0x14,
    0x19, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0xcb, 0x02,
    0x1c, 0x1d, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x0c, 0x03, 0x01, 0x12, 0x06, 0xce, 0x02, 0x02, 0xd1,
    0x02, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x03, 0x01, 0x01, 0x12, 0x04, 0xce, 0x02, 0x0a,
    0x0f, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0c, 0x03, 0x01, 0x02, 0x00, 0x12, 0x04, 0xcf, 0x02, 0x04,
    0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0xcf, 0x02,
    0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x01, 0x02, 0x00, 0x05, 0x12, 0x04, 0xcf,
    0x02, 0x0d, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x01, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xcf, 0x02, 0x14, 0x19, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x01, 0x02, 0x00, 0x03, 0x12,
    0x04, 0xcf, 0x02, 0x1c, 0x1d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0c, 0x03, 0x01, 0x02, 0x01, 0x12,
    0x04, 0xd0, 0x02, 0x04, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x01, 0x02, 0x01, 0x04,
    0x12, 0x04, 0xd0, 0x02, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x01, 0x02, 0x01,
    0x05, 0x12, 0x04, 0xd0, 0x02, 0x0d, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x01, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xd0, 0x02, 0x14, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x01,
    0x02, 0x01, 0x03, 0x12, 0x04, 0xd0, 0x02, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x0c, 0x03,
    0x02, 0x12, 0x06, 0xd3, 0x02, 0x02, 0xd5, 0x02, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x03,
    0x02, 0x01, 0x12, 0x04, 0xd3, 0x02, 0x0a, 0x10, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x0c, 0x03, 0x02,
    0x02, 0x00, 0x12, 0x04, 0xd4, 0x02, 0x04, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x02,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xd4, 0x02, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x03,
    0x02, 0x02, 0x00, 0x06, 0x12, 0x04, 0xd4, 0x02, 0x0d, 0x12, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c,
    0x03, 0x02, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd4, 0x02, 0x13, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x0c, 0x03, 0x02, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd4, 0x02, 0x1b, 0x1c, 0x0a, 0x0e, 0x0a, 0x04,
    0x04, 0x0c, 0x03, 0x03, 0x12, 0x06, 0xd7, 0x02, 0x02, 0xd9, 0x02, 0x03, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0c, 0x03, 0x03, 0x01, 0x12, 0x04, 0xd7, 0x02, 0x0a, 0x0d, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x0c, 0x03, 0x03, 0x02, 0x00, 0x12, 0x04, 0xd8, 0x02, 0x04, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x0c, 0x03, 0x03, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd8, 0x02, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x0c, 0x03, 0x03, 0x02, 0x00, 0x05, 0x12, 0x04, 0xd8, 0x02, 0x0d, 0x13, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x0c, 0x03, 0x03, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd8, 0x02, 0x14, 0x18, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x0c, 0x03, 0x03, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd8, 0x02, 0x1b, 0x1c, 0x0a,
    0x0e, 0x0a, 0x04, 0x04, 0x0c, 0x03, 0x04, 0x12, 0x06, 0xdb, 0x02, 0x02, 0xdd, 0x02, 0x03, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x03, 0x04, 0x01, 0x12, 0x04, 0xdb, 0x02, 0x0a, 0x0e, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x0c, 0x03, 0x04, 0x02, 0x00, 0x12, 0x04, 0xdc, 0x02, 0x04, 0x1e, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x0c, 0x03, 0x04, 0x02, 0x00, 0x04, 0x12, 0x04, 0xdc, 0x02, 0x04, 0x0c, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x04, 0x02, 0x00, 0x05, 0x12, 0x04, 0xdc, 0x02, 0x0d, 0x13,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x04, 0x02, 0x00, 0x01, 0x12, 0x04, 0xdc, 0x02, 0x14,
    0x19, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x0c, 0x03, 0x04, 0x02, 0x00, 0x03, 0x12, 0x04, 0xdc, 0x02,
    0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x04, 0xdf, 0x02, 0x02, 0x19,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12, 0x04, 0xdf, 0x02, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x06, 0x12, 0x04, 0xdf, 0x02, 0x0b, 0x0f, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x04, 0xdf, 0x02, 0x10, 0x14, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x04, 0xdf, 0x02, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x0c, 0x02, 0x01, 0x12, 0x04, 0xe0, 0x02, 0x02, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x01, 0x04, 0x12, 0x04, 0xe0, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x01, 0x06, 0x12, 0x04, 0xe0, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01,
    0x01, 0x12, 0x04, 0xe0, 0x02, 0x12, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x03,
    0x12, 0x04, 0xe0, 0x02, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x02, 0x12, 0x04,
    0xe1, 0x02, 0x02, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x04, 0x12, 0x04, 0xe1,
    0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x06, 0x12, 0x04, 0xe1, 0x02,
    0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x01, 0x12, 0x04, 0xe1, 0x02, 0x12,
    0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x03, 0x12, 0x04, 0xe1, 0x02, 0x1b, 0x1c,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x03, 0x12, 0x04, 0xe2, 0x02, 0x02, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x04, 0x12, 0x04, 0xe2, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x03, 0x06, 0x12, 0x04, 0xe2, 0x02, 0x0b, 0x0e, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x03, 0x01, 0x12, 0x04, 0xe2, 0x02, 0x0f, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x03, 0x03, 0x12, 0x04, 0xe2, 0x02, 0x15, 0x16, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c,
    0x02, 0x04, 0x12, 0x04, 0xe3, 0x02, 0x02, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x04,
    0x04, 0x12, 0x04, 0xe3, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x04, 0x06,
    0x12, 0x04, 0xe3, 0x02, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x04, 0x01, 0x12,
    0x04, 0xe3, 0x02, 0x10, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x04, 0x03, 0x12, 0x04,
    0xe3, 0x02, 0x17, 0x18, 0x0a, 0xd3, 0x01, 0x0a, 0x02, 0x04, 0x0d, 0x12, 0x06, 0xec, 0x02, 0x00,
    0xf3, 0x02, 0x01, 0x1a, 0xc4, 0x01, 0x2a, 0x0a, 0x20, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62,
    0x65, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x61, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x20,
    0x74, 0x68, 0x61, 0x74, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x73, 0x65, 0x74, 0x20,
    0x6f, 0x6e, 0x20, 0x61, 0x20, 0x6d, 0x61, 0x63, 0x68, 0x69, 0x6e, 0x65, 0x2e, 0x20, 0x46, 0x6f,
    0x72, 0x20, 0x6e, 0x6f, 0x77, 0x2c, 0x0a, 0x20, 0x61, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74,
    0x65, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x73,
    0x20, 0x73, 0x68, 0x61, 0x72, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x61, 0x6d, 0x65, 0x20,
    0x22, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x20, 0x74, 0x79, 0x70, 0x65, 0x2c, 0x20, 0x62, 0x75,
    0x74, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x6d, 0x61, 0x79, 0x0a, 0x20, 0x63, 0x68, 0x61, 0x6e,
    0x67, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x75, 0x74, 0x75, 0x72, 0x65,
    0x20, 0x61, 0x6e, 0x64, 0x20, 0x61, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x20,
    0x6d, 0x61, 0x79, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x62, 0x65, 0x20, 0x73, 0x74, 0x72, 0x69,
    0x6e, 0x67, 0x20, 0x62, 0x61, 0x73, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0d,
    0x01, 0x12, 0x04, 0xec, 0x02, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12,
    0x04, 0xed, 0x02, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xed, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x05, 0x12, 0x04, 0xed,
    0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x04, 0xed, 0x02,
    0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x04, 0xed, 0x02, 0x19,
    0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x01, 0x12, 0x04, 0xee, 0x02, 0x02, 0x1f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x04, 0x12, 0x04, 0xee, 0x02, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x06, 0x12, 0x04, 0xee, 0x02, 0x0b, 0x15, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x01, 0x01, 0x12, 0x04, 0xee, 0x02, 0x16, 0x1a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x01, 0x03, 0x12, 0x04, 0xee, 0x02, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0d, 0x02, 0x02, 0x12, 0x04, 0xef, 0x02, 0x02, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x02, 0x04, 0x12, 0x04, 0xef, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02,
    0x06, 0x12, 0x04, 0xef, 0x02, 0x0b, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x01,
    0x12, 0x04, 0xef, 0x02, 0x18, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x03, 0x12,
    0x04, 0xef, 0x02, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x03, 0x12, 0x04, 0xf0,
    0x02, 0x02, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x04, 0x12, 0x04, 0xf0, 0x02,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x06, 0x12, 0x04, 0xf0, 0x02, 0x0b,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x01, 0x12, 0x04, 0xf0, 0x02, 0x18, 0x1e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x03, 0x12, 0x04, 0xf0, 0x02, 0x21, 0x22, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x04, 0x12, 0x04, 0xf1, 0x02, 0x02, 0x1d, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x04, 0x04, 0x12, 0x04, 0xf1, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x04, 0x06, 0x12, 0x04, 0xf1, 0x02, 0x0b, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x04, 0x01, 0x12, 0x04, 0xf1, 0x02, 0x15, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x04, 0x03, 0x12, 0x04, 0xf1, 0x02, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02,
    0x05, 0x12, 0x04, 0xf2, 0x02, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x04,
    0x12, 0x04, 0xf2, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x06, 0x12,
    0x04, 0xf2, 0x02, 0x0b, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x01, 0x12, 0x04,
    0xf2, 0x02, 0x16, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x03, 0x12, 0x04, 0xf2,
    0x02, 0x1d, 0x1e, 0x0a, 0xed, 0x02, 0x0a, 0x02, 0x04, 0x0e, 0x12, 0x06, 0xff, 0x02, 0x00, 0x86,
    0x03, 0x01, 0x1a, 0xde, 0x02, 0x2a, 0x0a, 0x20, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65,
    0x73, 0x20, 0x61, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x20, 0x6f, 0x6e, 0x20,
    0x61, 0x20, 0x6d, 0x61, 0x63, 0x68, 0x69, 0x6e, 0x65, 0x2e, 0x20, 0x41, 0x20, 0x72, 0x65, 0x73,
    0x6f, 0x75, 0x72, 0x63, 0x65, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x74, 0x61, 0x6b, 0x65, 0x20, 0x6f,
    0x6e, 0x20, 0x6f, 0x6e, 0x65, 0x20, 0x6f, 0x66, 0x0a, 0x20, 0x74, 0x68, 0x72, 0x65, 0x65, 0x20,
    0x74, 0x79, 0x70, 0x65, 0x73, 0x3a, 0x20, 0x73, 0x63, 0x61, 0x6c, 0x61, 0x72, 0x20, 0x28, 0x64,
    0x6f, 0x75, 0x62, 0x6c, 0x65, 0x29, 0x2c, 0x20, 0x61, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x6f,
    0x66, 0x20, 0x66, 0x69, 0x6e, 0x69, 0x74, 0x65, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x64, 0x69, 0x73,
    0x63, 0x72, 0x65, 0x74, 0x65, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x0a, 0x20, 0x28, 0x65,
    0x2e, 0x67, 0x2e, 0x2c, 0x20, 0x5b, 0x31, 0x2d, 0x31, 0x30, 0x2c, 0x20, 0x32, 0x30, 0x2d, 0x33,
    0x30, 0x5d, 0x29, 0x2c, 0x20, 0x6f, 0x72, 0x20, 0x61, 0x20, 0x73, 0x65, 0x74, 0x20, 0x6f, 0x66,
    0x20, 0x69, 0x74, 0x65, 0x6d, 0x73, 0x2e, 0x20, 0x41, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72,
    0x63, 0x65, 0x20, 0x69, 0x73, 0x20, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x64, 0x0a,
    0x20, 0x75, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x74, 0x61, 0x6e, 0x64,
    0x61, 0x72, 0x64, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x20, 0x62, 0x75, 0x66,
    0x66, 0x65, 0x72, 0x20, 0x22, 0x75, 0x6e, 0x69, 0x6f, 0x6e, 0x22, 0x20, 0x74, 0x72, 0x69, 0x63,
    0x6b, 0x2e, 0x0a, 0x0a, 0x20, 0x54, 0x4f, 0x44, 0x4f, 0x28, 0x62, 0x65, 0x6e, 0x68, 0x29, 0x3a,
    0x20, 0x41, 0x64, 0x64, 0x20, 0x62, 0x65, 0x74, 0x74, 0x65, 0x72, 0x20, 0x73, 0x75, 0x70, 0x70,
    0x6f, 0x72, 0x74, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x22, 0x65, 0x78, 0x70, 0x65, 0x63, 0x74, 0x65,
    0x64, 0x22, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x73, 0x20, 0x28, 0x65, 0x2e,
    0x67, 0x2e, 0x2c, 0x0a, 0x20, 0x63, 0x70, 0x75, 0x73, 0x2c, 0x20, 0x6d, 0x65, 0x6d, 0x6f, 0x72,
    0x79, 0x2c, 0x20, 0x64, 0x69, 0x73, 0x6b, 0x2c, 0x20, 0x6e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b,
    0x29, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x04, 0xff, 0x02, 0x08, 0x10,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x04, 0x80, 0x03, 0x02, 0x1b, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x04, 0x80, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x00, 0x05, 0x12, 0x04, 0x80, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x04, 0x80, 0x03, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x00, 0x03, 0x12, 0x04, 0x80, 0x03, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e,
    0x02, 0x01, 0x12, 0x04, 0x81, 0x03, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01,
    0x04, 0x12, 0x04, 0x81, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x06,
    0x12, 0x04, 0x81, 0x03, 0x0b, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x01, 0x12,
    0x04, 0x81, 0x03, 0x16, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x03, 0x12, 0x04,
    0x81, 0x03, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x02, 0x12, 0x04, 0x82, 0x03,
    0x02, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x04, 0x12, 0x04, 0x82, 0x03, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x06, 0x12, 0x04, 0x82, 0x03, 0x0b, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x01, 0x12, 0x04, 0x82, 0x03, 0x18, 0x1e, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x03, 0x12, 0x04, 0x82, 0x03, 0x21, 0x22, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x0e, 0x02, 0x03, 0x12, 0x04, 0x83, 0x03, 0x02, 0x23, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x03, 0x04, 0x12, 0x04, 0x83, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x03, 0x06, 0x12, 0x04, 0x83, 0x03, 0x0b, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x03, 0x01, 0x12, 0x04, 0x83, 0x03, 0x18, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x03, 0x03, 0x12, 0x04, 0x83, 0x03, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x04,
    0x12, 0x04, 0x84, 0x03, 0x02, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x04, 0x04, 0x12,
    0x04, 0x84, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x04, 0x06, 0x12, 0x04,
    0x84, 0x03, 0x0b, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x04, 0x01, 0x12, 0x04, 0x84,
    0x03, 0x15, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x04, 0x03, 0x12, 0x04, 0x84, 0x03,
    0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x05, 0x12, 0x04, 0x85, 0x03, 0x02, 0x2b,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x05, 0x04, 0x12, 0x04, 0x85, 0x03, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x05, 0x05, 0x12, 0x04, 0x85, 0x03, 0x0b, 0x11, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x05, 0x01, 0x12, 0x04, 0x85, 0x03, 0x12, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x05, 0x03, 0x12, 0x04, 0x85, 0x03, 0x19, 0x1a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x05, 0x08, 0x12, 0x04, 0x85, 0x03, 0x1b, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x05, 0x07, 0x12, 0x04, 0x85, 0x03, 0x26, 0x29, 0x0a, 0x39, 0x0a, 0x02, 0x04, 0x0f,
    0x12, 0x06, 0x8c, 0x03, 0x00, 0xbc, 0x03, 0x01, 0x1a, 0x2b, 0x0a, 0x20, 0x41, 0x20, 0x73, 0x6e,
    0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72,
    0x63, 0x65, 0x20, 0x75, 0x73, 0x61, 0x67, 0x65, 0x20, 0x73, 0x74, 0x61, 0x74, 0x69, 0x73, 0x74,
    0x69, 0x63, 0x73, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x04, 0x8c, 0x03,
    0x08, 0x1a, 0x0a, 0x3a, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x00, 0x12, 0x04, 0x8d, 0x03, 0x02, 0x20,
    0x22, 0x2c, 0x20, 0x53, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x20, 0x74, 0x69, 0x6d, 0x65,
    0x2c, 0x20, 0x69, 0x6e, 0x20, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x20, 0x73, 0x69, 0x6e,
    0x63, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x45, 0x70, 0x6f, 0x63, 0x68, 0x2e, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x04, 0x12, 0x04, 0x8d, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x00, 0x05, 0x12, 0x04, 0x8d, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8d, 0x03, 0x12, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8d, 0x03, 0x1e, 0x1f, 0x0a, 0x5b, 0x0a, 0x04, 0x04, 0x0f,
    0x02, 0x01, 0x12, 0x04, 0x91, 0x03, 0x02, 0x2a, 0x1a, 0x4d, 0x20, 0x43, 0x50, 0x55, 0x20, 0x55,
    0x73, 0x61, 0x67, 0x65, 0x20, 0x49, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x3a, 0x0a, 0x20, 0x54, 0x6f, 0x74, 0x61, 0x6c, 0x20, 0x43, 0x50, 0x55, 0x20, 0x74, 0x69, 0x6d,
    0x65, 0x20, 0x73, 0x70, 0x65, 0x6e, 0x74, 0x20, 0x69, 0x6e, 0x20, 0x75, 0x73, 0x65, 0x72, 0x20,
    0x6d, 0x6f, 0x64, 0x65, 0x2c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x6b, 0x65, 0x72, 0x6e, 0x65, 0x6c,
    0x20, 0x6d, 0x6f, 0x64, 0x65, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x91, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x05, 0x12,
    0x04, 0x91, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x01, 0x12, 0x04,
    0x91, 0x03, 0x12, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x03, 0x12, 0x04, 0x91,
    0x03, 0x28, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x02, 0x12, 0x04, 0x92, 0x03, 0x02,
    0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x04, 0x12, 0x04, 0x92, 0x03, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x05, 0x12, 0x04, 0x92, 0x03, 0x0b, 0x11, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x01, 0x12, 0x04, 0x92, 0x03, 0x12, 0x27, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x03, 0x12, 0x04, 0x92, 0x03, 0x2a, 0x2b, 0x0a, 0x29, 0x0a,
    0x04, 0x04, 0x0f, 0x02, 0x03, 0x12, 0x04, 0x95, 0x03, 0x02, 0x21, 0x1a, 0x1b, 0x20, 0x4e, 0x75,
    0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x43, 0x50, 0x55, 0x73, 0x20, 0x61, 0x6c, 0x6c,
    0x6f, 0x63, 0x61, 0x74, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03,
    0x04, 0x12, 0x04, 0x95, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x05,
    0x12, 0x04, 0x95, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x01, 0x12,
    0x04, 0x95, 0x03, 0x12, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x03, 0x12, 0x04,
    0x95, 0x03, 0x1f, 0x20, 0x0a, 0x47, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x04, 0x12, 0x04, 0x98, 0x03,
    0x02, 0x26, 0x1a, 0x39, 0x20, 0x63, 0x70, 0x75, 0x2e, 0x73, 0x74, 0x61, 0x74, 0x20, 0x6f, 0x6e,
    0x20, 0x70, 0x72, 0x6f, 0x63, 0x65, 0x73, 0x73, 0x20, 0x74, 0x68, 0x72, 0x6f, 0x74, 0x74, 0x6c,
    0x69, 0x6e, 0x67, 0x20, 0x28, 0x66, 0x6f, 0x72, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74,
    0x69, 0x6f, 0x6e, 0x20, 0x69, 0x73, 0x73, 0x75, 0x65, 0x73, 0x29, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x04, 0x04, 0x12, 0x04, 0x98, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x04, 0x05, 0x12, 0x04, 0x98, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x04, 0x01, 0x12, 0x04, 0x98, 0x03, 0x12, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x04, 0x03, 0x12, 0x04, 0x98, 0x03, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02,
    0x05, 0x12, 0x04, 0x99, 0x03, 0x02, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x05, 0x04,
    0x12, 0x04, 0x99, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x05, 0x05, 0x12,
    0x04, 0x99, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x05, 0x01, 0x12, 0x04,
    0x99, 0x03, 0x12, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x05, 0x03, 0x12, 0x04, 0x99,
    0x03, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x06, 0x12, 0x04, 0x9a, 0x03, 0x02,
    0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x06, 0x04, 0x12, 0x04, 0x9a, 0x03, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x06, 0x05, 0x12, 0x04, 0x9a, 0x03, 0x0b, 0x11, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x06, 0x01, 0x12, 0x04, 0x9a, 0x03, 0x12, 0x2a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x06, 0x03, 0x12, 0x04, 0x9a, 0x03, 0x2d, 0x2e, 0x0a, 0x3f, 0x0a,
    0x04, 0x04, 0x0f, 0x02, 0x07, 0x12, 0x04, 0x9d, 0x03, 0x02, 0x24, 0x1a, 0x1b, 0x20, 0x4d, 0x65,
    0x6d, 0x6f, 0x72, 0x79, 0x20, 0x55, 0x73, 0x61, 0x67, 0x65, 0x20, 0x49, 0x6e, 0x66, 0x6f, 0x72,
    0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x3a, 0x0a, 0x22, 0x14, 0x20, 0x52, 0x65, 0x73, 0x69, 0x64,
    0x65, 0x6e, 0x74, 0x20, 0x53, 0x65, 0x74, 0x20, 0x53, 0x69, 0x7a, 0x65, 0x2e, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x07, 0x04, 0x12, 0x04, 0x9d, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x07, 0x05, 0x12, 0x04, 0x9d, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x07, 0x01, 0x12, 0x04, 0x9d, 0x03, 0x12, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x07, 0x03, 0x12, 0x04, 0x9d, 0x03, 0x22, 0x23, 0x0a, 0x35, 0x0a, 0x04, 0x04, 0x0f,
    0x02, 0x08, 0x12, 0x04, 0xa0, 0x03, 0x02, 0x26, 0x1a, 0x27, 0x20, 0x41, 0x6d, 0x6f, 0x75, 0x6e,
    0x74, 0x20, 0x6f, 0x66, 0x20, 0x6d, 0x65, 0x6d, 0x6f, 0x72, 0x79, 0x20, 0x72, 0x65, 0x73, 0x6f,
    0x75, 0x72, 0x63, 0x65, 0x73, 0x20, 0x61, 0x6c, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x65, 0x64, 0x2e,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x08, 0x04, 0x12, 0x04, 0xa0, 0x03, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x08, 0x05, 0x12, 0x04, 0xa0, 0x03, 0x0b, 0x11, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x08, 0x01, 0x12, 0x04, 0xa0, 0x03, 0x12, 0x21, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x08, 0x03, 0x12, 0x04, 0xa0, 0x03, 0x24, 0x25, 0x0a, 0x58, 0x0a,
    0x04, 0x04, 0x0f, 0x02, 0x09, 0x12, 0x04, 0xa3, 0x03, 0x02, 0x26, 0x1a, 0x4a, 0x20, 0x42, 0x72,
    0x6f, 0x6b, 0x65, 0x6e, 0x20, 0x6f, 0x75, 0x74, 0x20, 0x6d, 0x65, 0x6d, 0x6f, 0x72, 0x79, 0x20,
    0x75, 0x73, 0x61, 0x67, 0x65, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x20, 0x28, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x2c, 0x20, 0x61, 0x6e, 0x6f, 0x6e, 0x79, 0x6d,
    0x6f, 0x75, 0x73, 0x2c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x6d, 0x6d, 0x61, 0x70, 0x65, 0x64, 0x20,
    0x66, 0x69, 0x6c, 0x65, 0x73, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x09, 0x04,
    0x12, 0x04, 0xa3, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x09, 0x05, 0x12,
    0x04, 0xa3, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x09, 0x01, 0x12, 0x04,
    0xa3, 0x03, 0x12, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x09, 0x03, 0x12, 0x04, 0xa3,
    0x03, 0x23, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x0a, 0x12, 0x04, 0xa4, 0x03, 0x02,
    0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0a, 0x04, 0x12, 0x04, 0xa4, 0x03, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0a, 0x05, 0x12, 0x04, 0xa4, 0x03, 0x0b, 0x11, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0a, 0x01, 0x12, 0x04, 0xa4, 0x03, 0x12, 0x20, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0a, 0x03, 0x12, 0x04, 0xa4, 0x03, 0x23, 0x25, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x0f, 0x02, 0x0b, 0x12, 0x04, 0xa5, 0x03, 0x02, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x0b, 0x04, 0x12, 0x04, 0xa5, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x0b, 0x05, 0x12, 0x04, 0xa5, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x0b, 0x01, 0x12, 0x04, 0xa5, 0x03, 0x12, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0b,
    0x03, 0x12, 0x04, 0xa5, 0x03, 0x2a, 0x2c, 0x0a, 0x20, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x0c, 0x12,
    0x04, 0xaa, 0x03, 0x02, 0x24, 0x1a, 0x12, 0x20, 0x50, 0x65, 0x72, 0x66, 0x20, 0x73, 0x74, 0x61,
    0x74, 0x69, 0x73, 0x74, 0x69, 0x63, 0x73, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x0c, 0x04, 0x12, 0x04, 0xaa, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0c,
    0x06, 0x12, 0x04, 0xaa, 0x03, 0x0b, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0c, 0x01,
    0x12, 0x04, 0xaa, 0x03, 0x1a, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0c, 0x03, 0x12,
    0x04, 0xaa, 0x03, 0x21, 0x23, 0x0a, 0x2a, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x0d, 0x12, 0x04, 0xad,
    0x03, 0x02, 0x26, 0x1a, 0x1c, 0x20, 0x4e, 0x65, 0x74, 0x77, 0x6f, 0x72, 0x6b, 0x20, 0x55, 0x73,
    0x61, 0x67, 0x65, 0x20, 0x49, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x3a,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0d, 0x04, 0x12, 0x04, 0xad, 0x03, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0d, 0x05, 0x12, 0x04, 0xad, 0x03, 0x0b, 0x11, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0d, 0x01, 0x12, 0x04, 0xad, 0x03, 0x12, 0x20, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0d, 0x03, 0x12, 0x04, 0xad, 0x03, 0x23, 0x25, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x0f, 0x02, 0x0e, 0x12, 0x04, 0xae, 0x03, 0x02, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x0e, 0x04, 0x12, 0x04, 0xae, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x0e, 0x05, 0x12, 0x04, 0xae, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x0e, 0x01, 0x12, 0x04, 0xae, 0x03, 0x12, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0e,
    0x03, 0x12, 0x04, 0xae, 0x03, 0x21, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x0f, 0x12,
    0x04, 0xaf, 0x03, 0x02, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0f, 0x04, 0x12, 0x04,
    0xaf, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0f, 0x05, 0x12, 0x04, 0xaf,
    0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0f, 0x01, 0x12, 0x04, 0xaf, 0x03,
    0x12, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x0f, 0x03, 0x12, 0x04, 0xaf, 0x03, 0x22,
    0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x10, 0x12, 0x04, 0xb0, 0x03, 0x02, 0x26, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x10, 0x04, 0x12, 0x04, 0xb0, 0x03, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x10, 0x05, 0x12, 0x04, 0xb0, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x10, 0x01, 0x12, 0x04, 0xb0, 0x03, 0x12, 0x20, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x10, 0x03, 0x12, 0x04, 0xb0, 0x03, 0x23, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0f, 0x02, 0x11, 0x12, 0x04, 0xb1, 0x03, 0x02, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x11, 0x04, 0x12, 0x04, 0xb1, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x11,
    0x05, 0x12, 0x04, 0xb1, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x11, 0x01,
    0x12, 0x04, 0xb1, 0x03, 0x12, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x11, 0x03, 0x12,
    0x04, 0xb1, 0x03, 0x23, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x12, 0x12, 0x04, 0xb2,
    0x03, 0x02, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x12, 0x04, 0x12, 0x04, 0xb2, 0x03,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x12, 0x05, 0x12, 0x04, 0xb2, 0x03, 0x0b,
    0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x12, 0x01, 0x12, 0x04, 0xb2, 0x03, 0x12, 0x1e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x12, 0x03, 0x12, 0x04, 0xb2, 0x03, 0x21, 0x23, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x13, 0x12, 0x04, 0xb3, 0x03, 0x02, 0x25, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x13, 0x04, 0x12, 0x04, 0xb3, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x13, 0x05, 0x12, 0x04, 0xb3, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x13, 0x01, 0x12, 0x04, 0xb3, 0x03, 0x12, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x13, 0x03, 0x12, 0x04, 0xb3, 0x03, 0x22, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02,
    0x14, 0x12, 0x04, 0xb4, 0x03, 0x02, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x14, 0x04,
    0x12, 0x04, 0xb4, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x14, 0x05, 0x12,
    0x04, 0xb4, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x14, 0x01, 0x12, 0x04,
    0xb4, 0x03, 0x12, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x14, 0x03, 0x12, 0x04, 0xb4,
    0x03, 0x23, 0x25, 0x0a, 0x86, 0x01, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x15, 0x12, 0x04, 0xb8, 0x03,
    0x02, 0x31, 0x1a, 0x78, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6b, 0x65, 0x72, 0x6e, 0x65, 0x6c, 0x20,
    0x6b, 0x65, 0x65, 0x70, 0x73, 0x20, 0x74, 0x72, 0x61, 0x63, 0x6b, 0x20, 0x6f, 0x66, 0x20, 0x52,
    0x54, 0x54, 0x20, 0x28, 0x72, 0x6f, 0x75, 0x6e, 0x64, 0x2d, 0x74, 0x72, 0x69, 0x70, 0x20, 0x74,
    0x69, 0x6d, 0x65, 0x29, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x69, 0x74, 0x73, 0x20, 0x54, 0x43, 0x50,
    0x0a, 0x20, 0x73, 0x6f, 0x63, 0x6b, 0x65, 0x74, 0x73, 0x2e, 0x20, 0x52, 0x54, 0x54, 0x20, 0x69,
    0x73, 0x20, 0x61, 0x20, 0x77, 0x61, 0x79, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x65, 0x6c, 0x6c, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x6c, 0x61, 0x74, 0x65, 0x6e, 0x63, 0x79, 0x20, 0x6f, 0x66, 0x20, 0x61,
    0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x15, 0x04, 0x12, 0x04, 0xb8, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x15, 0x05, 0x12, 0x04, 0xb8, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x15, 0x01, 0x12, 0x04, 0xb8, 0x03, 0x12, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x15, 0x03, 0x12, 0x04, 0xb8, 0x03, 0x2e, 0x30, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x16,
    0x12, 0x04, 0xb9, 0x03, 0x02, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x16, 0x04, 0x12,
    0x04, 0xb9, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x16, 0x05, 0x12, 0x04,
    0xb9, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x16, 0x01, 0x12, 0x04, 0xb9,
    0x03, 0x12, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x16, 0x03, 0x12, 0x04, 0xb9, 0x03,
    0x2e, 0x30, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x17, 0x12, 0x04, 0xba, 0x03, 0x02, 0x31,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x17, 0x04, 0x12, 0x04, 0xba, 0x03, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x17, 0x05, 0x12, 0x04, 0xba, 0x03, 0x0b, 0x11, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x17, 0x01, 0x12, 0x04, 0xba, 0x03, 0x12, 0x2b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x17, 0x03, 0x12, 0x04, 0xba, 0x03, 0x2e, 0x30, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x0f, 0x02, 0x18, 0x12, 0x04, 0xbb, 0x03, 0x02, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x18, 0x04, 0x12, 0x04, 0xbb, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x18, 0x05, 0x12, 0x04, 0xbb, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x18,
    0x01, 0x12, 0x04, 0xbb, 0x03, 0x12, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x18, 0x03,
    0x12, 0x04, 0xbb, 0x03, 0x2e, 0x30, 0x0a, 0xcd, 0x02, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x06, 0xc7,
    0x03, 0x00, 0xd8, 0x03, 0x01, 0x1a, 0xbe, 0x02, 0x2a, 0x0a, 0x20, 0x44, 0x65, 0x73, 0x63, 0x72,
    0x69, 0x62, 0x65, 0x73, 0x20, 0x61, 0x20, 0x73, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x20,
    0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x20,
    0x75, 0x73, 0x61, 0x67, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x78, 0x65,
    0x63, 0x75, 0x74, 0x6f, 0x72, 0x2e, 0x0a, 0x0a, 0x20, 0x54, 0x4f, 0x44, 0x4f, 0x28, 0x62, 0x6d,
    0x61, 0x68, 0x6c, 0x65, 0x72, 0x29, 0x3a, 0x20, 0x4e, 0x6f, 0x74, 0x65, 0x20, 0x74, 0x68, 0x61,
    0x74, 0x20, 0x77, 0x65, 0x20, 0x77, 0x61, 0x6e, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20,
    0x73, 0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x69, 0x6e, 0x66,
    0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x73, 0x75, 0x62,
    0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x74, 0x6c, 0x79, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x72, 0x65, 0x6c, 0x65, 0x76, 0x61, 0x6e, 0x74, 0x20, 0x73, 0x63, 0x68, 0x65, 0x64, 0x75,
    0x6c, 0x65, 0x72, 0x2e, 0x20, 0x53, 0x6f, 0x0a, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x20, 0x69, 0x73, 0x20, 0x64, 0x65, 0x73, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x20,
    0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x65, 0x61, 0x73, 0x79, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x73, 0x63, 0x68, 0x65, 0x64, 0x75, 0x6c, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20,
    0x75, 0x73, 0x65, 0x2c, 0x20, 0x74, 0x68, 0x69, 0x73, 0x0a, 0x20, 0x69, 0x73, 0x20, 0x77, 0x68,
    0x79, 0x20, 0x77, 0x65, 0x20, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x20, 0x69, 0x64, 0x2c, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75,
    0x74, 0x6f, 0x72, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x20, 0x2f, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x20,
    0x69, 0x6e, 0x66, 0x6f, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x04, 0xc7,
    0x03, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x00, 0x12, 0x04, 0xc8, 0x03, 0x02,
    0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc8, 0x03, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x06, 0x12, 0x04, 0xc8, 0x03, 0x0b, 0x12, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc8, 0x03, 0x13, 0x1b, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc8, 0x03, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x10, 0x02, 0x01, 0x12, 0x04, 0xc9, 0x03, 0x02, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x01, 0x04, 0x12, 0x04, 0xc9, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x01, 0x06, 0x12, 0x04, 0xc9, 0x03, 0x0b, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xc9, 0x03, 0x17, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01,
    0x03, 0x12, 0x04, 0xc9, 0x03, 0x26, 0x27, 0x0a, 0x2d, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x02, 0x12,
    0x04, 0xd1, 0x03, 0x02, 0x26, 0x22, 0x1f, 0x20, 0x49, 0x66, 0x20, 0x70, 0x72, 0x65, 0x73, 0x65,
    0x6e, 0x74, 0x2c, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f,
    0x72, 0x20, 0x77, 0x61, 0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x04, 0x12,
    0x04, 0xd1, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x06, 0x12, 0x04,
    0xd1, 0x03, 0x0b, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x01, 0x12, 0x04, 0xd1,
    0x03, 0x16, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x03, 0x12, 0x04, 0xd1, 0x03,
    0x24, 0x25, 0x0a, 0x25, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x03, 0x12, 0x04, 0xd2, 0x03, 0x02, 0x24,
    0x22, 0x17, 0x20, 0x65, 0x78, 0x70, 0x6c, 0x69, 0x63, 0x69, 0x74, 0x6c, 0x79, 0x20, 0x73, 0x70,
    0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x03, 0x04, 0x12, 0x04, 0xd2, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03,
    0x05, 0x12, 0x04, 0xd2, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x01,
    0x12, 0x04, 0xd2, 0x03, 0x12, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x03, 0x12,
    0x04, 0xd2, 0x03, 0x22, 0x23, 0x0a, 0x3e, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x04, 0x12, 0x04, 0xd4,
    0x03, 0x02, 0x1e, 0x22, 0x30, 0x20, 0x49, 0x66, 0x20, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74,
    0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x20, 0x64, 0x69, 0x64, 0x20, 0x6e,
    0x6f, 0x74, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75,
    0x74, 0x6f, 0x72, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x04, 0x04, 0x12, 0x04,
    0xd4, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x04, 0x06, 0x12, 0x04, 0xd4,
    0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x04, 0x01, 0x12, 0x04, 0xd4, 0x03,
    0x12, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x04, 0x03, 0x12, 0x04, 0xd4, 0x03, 0x1c,
    0x1d, 0x0a, 0x4f, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x05, 0x12, 0x04, 0xd7, 0x03, 0x02, 0x2d, 0x1a,
    0x41, 0x20, 0x49, 0x66, 0x20, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6e, 0x67, 0x2c, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x69, 0x73, 0x6f, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6d, 0x6f, 0x64, 0x75,
    0x6c, 0x65, 0x20, 0x63, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x20, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64,
    0x65, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x20, 0x75, 0x73, 0x61, 0x67, 0x65,
    0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x05, 0x04, 0x12, 0x04, 0xd7, 0x03, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x05, 0x06, 0x12, 0x04, 0xd7, 0x03, 0x0b, 0x1d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x05, 0x01, 0x12, 0x04, 0xd7, 0x03, 0x1e, 0x28, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x05, 0x03, 0x12, 0x04, 0xd7, 0x03, 0x2b, 0x2c, 0x0a, 0xa8,
    0x03, 0x0a, 0x02, 0x04, 0x11, 0x12, 0x06, 0xe7, 0x03, 0x00, 0xa3, 0x04, 0x01, 0x1a, 0x99, 0x03,
    0x2a, 0x0a, 0x20, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x73, 0x20, 0x61, 0x20, 0x73,
    0x61, 0x6d, 0x70, 0x6c, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x20,
    0x66, 0x72, 0x6f, 0x6d, 0x20, 0x22, 0x70, 0x65, 0x72, 0x66, 0x20, 0x73, 0x74, 0x61, 0x74, 0x22,
    0x2e, 0x20, 0x4f, 0x6e, 0x6c, 0x79, 0x20, 0x61, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65,
    0x20, 0x6f, 0x6e, 0x0a, 0x20, 0x4c, 0x69, 0x6e, 0x75, 0x78, 0x2e, 0x0a, 0x0a, 0x20, 0x4e, 0x4f,
    0x54, 0x45, 0x3a, 0x20, 0x45, 0x61, 0x63, 0x68, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61,
    0x6c, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x73, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x20, 0x70, 0x65,
    0x72, 0x66, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x20, 0x28, 0x73, 0x65, 0x65, 0x0a, 0x20, 0x22,
    0x70, 0x65, 0x72, 0x66, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x22, 0x29, 0x20, 0x77, 0x69, 0x74, 0x68,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x6f, 0x6c, 0x6c, 0x6f, 0x77, 0x69, 0x6e, 0x67, 0x20, 0x63,
    0x68, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x3a, 0x0a, 0x20, 0x31, 0x2e, 0x20, 0x4e, 0x61, 0x6d, 0x65,
    0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x64, 0x6f, 0x77, 0x6e, 0x63, 0x61, 0x73, 0x65, 0x64, 0x2e,
    0x0a, 0x20, 0x32, 0x2e, 0x20, 0x48, 0x79, 0x70, 0x68, 0x65, 0x6e, 0x73, 0x20, 0x28, 0x27, 0x2d,
    0x27, 0x29, 0x20, 0x61, 0x72, 0x65, 0x20, 0x72, 0x65, 0x70, 0x6c, 0x61, 0x63, 0x65, 0x64, 0x20,
    0x77, 0x69, 0x74, 0x68, 0x20, 0x75, 0x6e, 0x64, 0x65, 0x72, 0x73, 0x63, 0x6f, 0x72, 0x65, 0x73,
    0x20, 0x28, 0x27, 0x5f, 0x27, 0x29, 0x2e, 0x0a, 0x20, 0x33, 0x2e, 0x20, 0x45, 0x76, 0x65, 0x6e,
    0x74, 0x73, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x61, 0x6c, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x74,
    0x65, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x20, 0x75, 0x73, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x6e, 0x61, 0x6d, 0x65, 0x20, 0x22, 0x70, 0x65, 0x72, 0x66, 0x20, 0x73, 0x74, 0x61, 0x74, 0x22,
    0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x73, 0x2c, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x65, 0x2e,
    0x67, 0x2e, 0x2c, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x76, 0x65, 0x6e,
    0x74, 0x20, 0x22, 0x63, 0x79, 0x63, 0x6c, 0x65, 0x73, 0x20, 0x4f, 0x52, 0x20, 0x63, 0x70, 0x75,
    0x2d, 0x63, 0x79, 0x63, 0x6c, 0x65, 0x73, 0x22, 0x20, 0x70, 0x65, 0x72, 0x66, 0x20, 0x61, 0x6c,
    0x77, 0x61, 0x79, 0x73, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x73, 0x0a, 0x20, 0x20, 0x20,
    0x20, 0x63, 0x79, 0x63, 0x6c, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x11, 0x01,
    0x12, 0x04, 0xe7, 0x03, 0x08, 0x16, 0x0a, 0x45, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00, 0x12, 0x04,
    0xe8, 0x03, 0x02, 0x20, 0x22, 0x37, 0x20, 0x53, 0x74, 0x61, 0x72, 0x74, 0x20, 0x6f, 0x66, 0x20,
    0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6c, 0x2c,
    0x20, 0x69, 0x6e, 0x20, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x20, 0x73, 0x69, 0x6e, 0x63,
    0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x45, 0x70, 0x6f, 0x63, 0x68, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe8, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x00, 0x05, 0x12, 0x04, 0xe8, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe8, 0x03, 0x12, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xe8, 0x03, 0x1e, 0x1f, 0x0a, 0x38, 0x0a, 0x04, 0x04, 0x11, 0x02,
    0x01, 0x12, 0x04, 0xe9, 0x03, 0x02, 0x1f, 0x22, 0x2a, 0x20, 0x44, 0x75, 0x72, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x20, 0x69, 0x6e, 0x74,
    0x65, 0x72, 0x76, 0x61, 0x6c, 0x2c, 0x20, 0x69, 0x6e, 0x20, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64,
    0x73, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x04, 0x12, 0x04, 0xe9, 0x03,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x05, 0x12, 0x04, 0xe9, 0x03, 0x0b,
    0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x01, 0x12, 0x04, 0xe9, 0x03, 0x12, 0x1a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x03, 0x12, 0x04, 0xe9, 0x03, 0x1d, 0x1e, 0x0a,
    0x1f, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x02, 0x12, 0x04, 0xec, 0x03, 0x02, 0x1d, 0x1a, 0x11, 0x20,
    0x48, 0x61, 0x72, 0x64, 0x77, 0x61, 0x72, 0x65, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x2e, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x04, 0x12, 0x04, 0xec, 0x03, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x05, 0x12, 0x04, 0xec, 0x03, 0x0b, 0x11, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x01, 0x12, 0x04, 0xec, 0x03, 0x12, 0x18, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x02, 0x03, 0x12, 0x04, 0xec, 0x03, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x11, 0x02, 0x03, 0x12, 0x04, 0xed, 0x03, 0x02, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x03, 0x04, 0x12, 0x04, 0xed, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x03, 0x05, 0x12, 0x04, 0xed, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x03,
    0x01, 0x12, 0x04, 0xed, 0x03, 0x12, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x03, 0x03,
    0x12, 0x04, 0xed, 0x03, 0x2c, 0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x04, 0x12, 0x04,
    0xee, 0x03, 0x02, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x04, 0x04, 0x12, 0x04, 0xee,
    0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x04, 0x05, 0x12, 0x04, 0xee, 0x03,
    0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x04, 0x01, 0x12, 0x04, 0xee, 0x03, 0x12,
    0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x04, 0x03, 0x12, 0x04, 0xee, 0x03, 0x2b, 0x2c,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x05, 0x12, 0x04, 0xef, 0x03, 0x02, 0x23, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x05, 0x04, 0x12, 0x04, 0xef, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x05, 0x05, 0x12, 0x04, 0xef, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x05, 0x01, 0x12, 0x04, 0xef, 0x03, 0x12, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x05, 0x03, 0x12, 0x04, 0xef, 0x03, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11,
    0x02, 0x06, 0x12, 0x04, 0xf0, 0x03, 0x02, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x06,
    0x04, 0x12, 0x04, 0xf0, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x06, 0x05,
    0x12, 0x04, 0xf0, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x06, 0x01, 0x12,
    0x04, 0xf0, 0x03, 0x12, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x06, 0x03, 0x12, 0x04,
    0xf0, 0x03, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x07, 0x12, 0x04, 0xf1, 0x03,
    0x02, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x07, 0x04, 0x12, 0x04, 0xf1, 0x03, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x07, 0x05, 0x12, 0x04, 0xf1, 0x03, 0x0b, 0x11,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x07, 0x01, 0x12, 0x04, 0xf1, 0x03, 0x12, 0x1e, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x07, 0x03, 0x12, 0x04, 0xf1, 0x03, 0x21, 0x22, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x11, 0x02, 0x08, 0x12, 0x04, 0xf2, 0x03, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x08, 0x04, 0x12, 0x04, 0xf2, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x08, 0x05, 0x12, 0x04, 0xf2, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x08, 0x01, 0x12, 0x04, 0xf2, 0x03, 0x12, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x08, 0x03, 0x12, 0x04, 0xf2, 0x03, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x09,
    0x12, 0x04, 0xf3, 0x03, 0x02, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x09, 0x04, 0x12,
    0x04, 0xf3, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x09, 0x05, 0x12, 0x04,
    0xf3, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x09, 0x01, 0x12, 0x04, 0xf3,
    0x03, 0x12, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x09, 0x03, 0x12, 0x04, 0xf3, 0x03,
    0x22, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x0a, 0x12, 0x04, 0xf4, 0x03, 0x02, 0x22,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x0a, 0x04, 0x12, 0x04, 0xf4, 0x03, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x0a, 0x05, 0x12, 0x04, 0xf4, 0x03, 0x0b, 0x11, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x0a, 0x01, 0x12, 0x04, 0xf4, 0x03, 0x12, 0x1c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x0a, 0x03, 0x12, 0x04, 0xf4, 0x03, 0x1f, 0x21, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x11, 0x02, 0x0b, 0x12, 0x04, 0xf5, 0x03, 0x02, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x0b, 0x04, 0x12, 0x04, 0xf5, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x0b, 0x05, 0x12, 0x04, 0xf5, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x0b,
    0x01, 0x12, 0x04, 0xf5, 0x03, 0x12, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x0b, 0x03,
    0x12, 0x04, 0xf5, 0x03, 0x1f, 0x21, 0x0a, 0x1f, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x0c, 0x12, 0x04,
    0xf8, 0x03, 0x02, 0x21, 0x1a, 0x11, 0x20, 0x53, 0x6f, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65, 0x20,
    0x65, 0x76, 0x65, 0x6e, 0x74, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x0c, 0x04,
    0x12, 0x04, 0xf8, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x0c, 0x05, 0x12,
    0x04, 0xf8, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x0c, 0x01, 0x12, 0x04,
    0xf8, 0x03, 0x12, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x0c, 0x03, 0x12, 0x04, 0xf8,
    0x03, 0x1e, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x0d, 0x12, 0x04, 0xf9, 0x03, 0x02,
    0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x0d, 0x04, 0x12, 0x04, 0xf9, 0x03, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x0d, 0x05, 0x12, 0x04, 0xf9, 0x03, 0x0b, 0x11, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x0d, 0x01, 0x12, 0x04, 0xf9, 0x03, 0x12, 0x1c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x0d, 0x03, 0x12, 0x04, 0xf9, 0x03, 0x1f, 0x21, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x11, 0x02, 0x0e, 0x12, 0x04, 0xfa, 0x03, 0x02, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x0e, 0x04, 0x12, 0x04, 0xfa, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x0e, 0x05, 0x12, 0x04, 0xfa, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x0e, 0x01, 0x12, 0x04, 0xfa, 0x03, 0x12, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x0e,
    0x03, 0x12, 0x04, 0xfa, 0x03, 0x20, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x0f, 0x12,
    0x04, 0xfb, 0x03, 0x02, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x0f, 0x04, 0x12, 0x04,
    0xfb, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x0f, 0x05, 0x12, 0x04, 0xfb,
    0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x0f, 0x01, 0x12, 0x04, 0xfb, 0x03,
    0x12, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x0f, 0x03, 0x12, 0x04, 0xfb, 0x03, 0x21,
    0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x10, 0x12, 0x04, 0xfc, 0x03, 0x02, 0x24, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x10, 0x04, 0x12, 0x04, 0xfc, 0x03, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x10, 0x05, 0x12, 0x04, 0xfc, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x10, 0x01, 0x12, 0x04, 0xfc, 0x03, 0x12, 0x1e, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x10, 0x03, 0x12, 0x04, 0xfc, 0x03, 0x21, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x11, 0x02, 0x11, 0x12, 0x04, 0xfd, 0x03, 0x02, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x11, 0x04, 0x12, 0x04, 0xfd, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x11,
    0x05, 0x12, 0x04, 0xfd, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x11, 0x01,
    0x12, 0x04, 0xfd, 0x03, 0x12, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x11, 0x03, 0x12,
    0x04, 0xfd, 0x03, 0x25, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x12, 0x12, 0x04, 0xfe,
    0x03, 0x02, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x12, 0x04, 0x12, 0x04, 0xfe, 0x03,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x12, 0x05, 0x12, 0x04, 0xfe, 0x03, 0x0b,
    0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x12, 0x01, 0x12, 0x04, 0xfe, 0x03, 0x12, 0x20,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x12, 0x03, 0x12, 0x04, 0xfe, 0x03, 0x23, 0x25, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x13, 0x12, 0x04, 0xff, 0x03, 0x02, 0x28, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x13, 0x04, 0x12, 0x04, 0xff, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x13, 0x05, 0x12, 0x04, 0xff, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x13, 0x01, 0x12, 0x04, 0xff, 0x03, 0x12, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x13, 0x03, 0x12, 0x04, 0xff, 0x03, 0x25, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02,
    0x14, 0x12, 0x04, 0x80, 0x04, 0x02, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x14, 0x04,
    0x12, 0x04, 0x80, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x14, 0x05, 0x12,
    0x04, 0x80, 0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x14, 0x01, 0x12, 0x04,
    0x80, 0x04, 0x12, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x14, 0x03, 0x12, 0x04, 0x80,
    0x04, 0x25, 0x27, 0x0a, 0x25, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x15, 0x12, 0x04, 0x83, 0x04, 0x02,
    0x27, 0x1a, 0x17, 0x20, 0x48, 0x61, 0x72, 0x64, 0x77, 0x61, 0x72, 0x65, 0x20, 0x63, 0x61, 0x63,
    0x68, 0x65, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x15, 0x04, 0x12, 0x04, 0x83, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x15, 0x05, 0x12, 0x04, 0x83, 0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x15,
    0x01, 0x12, 0x04, 0x83, 0x04, 0x12, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x15, 0x03,
    0x12, 0x04, 0x83, 0x04, 0x24, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x16, 0x12, 0x04,
    0x84, 0x04, 0x02, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x16, 0x04, 0x12, 0x04, 0x84,
    0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x16, 0x05, 0x12, 0x04, 0x84, 0x04,
    0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x16, 0x01, 0x12, 0x04, 0x84, 0x04, 0x12,
    0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x16, 0x03, 0x12, 0x04, 0x84, 0x04, 0x2a, 0x2c,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x17, 0x12, 0x04, 0x85, 0x04, 0x02, 0x28, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x17, 0x04, 0x12, 0x04, 0x85, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x17, 0x05, 0x12, 0x04, 0x85, 0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x17, 0x01, 0x12, 0x04, 0x85, 0x04, 0x12, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x17, 0x03, 0x12, 0x04, 0x85, 0x04, 0x25, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11,
    0x02, 0x18, 0x12, 0x04, 0x86, 0x04, 0x02, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x18,
    0x04, 0x12, 0x04, 0x86, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x18, 0x05,
    0x12, 0x04, 0x86, 0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x18, 0x01, 0x12,
    0x04, 0x86, 0x04, 0x12, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x18, 0x03, 0x12, 0x04,
    0x86, 0x04, 0x2b, 0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x19, 0x12, 0x04, 0x87, 0x04,
    0x02, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x19, 0x04, 0x12, 0x04, 0x87, 0x04, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x19, 0x05, 0x12, 0x04, 0x87, 0x04, 0x0b, 0x11,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x19, 0x01, 0x12, 0x04, 0x87, 0x04, 0x12, 0x26, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x19, 0x03, 0x12, 0x04, 0x87, 0x04, 0x29, 0x2b, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x11, 0x02, 0x1a, 0x12, 0x04, 0x88, 0x04, 0x02, 0x31, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x1a, 0x04, 0x12, 0x04, 0x88, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x1a, 0x05, 0x12, 0x04, 0x88, 0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x1a, 0x01, 0x12, 0x04, 0x88, 0x04, 0x12, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x1a, 0x03, 0x12, 0x04, 0x88, 0x04, 0x2e, 0x30, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x1b,
    0x12, 0x04, 0x89, 0x04, 0x02, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x1b, 0x04, 0x12,
    0x04, 0x89, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x1b, 0x05, 0x12, 0x04,
    0x89, 0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x1b, 0x01, 0x12, 0x04, 0x89,
    0x04, 0x12, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x1b, 0x03, 0x12, 0x04, 0x89, 0x04,
    0x24, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x1c, 0x12, 0x04, 0x8a, 0x04, 0x02, 0x2d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x1c, 0x04, 0x12, 0x04, 0x8a, 0x04, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x1c, 0x05, 0x12, 0x04, 0x8a, 0x04, 0x0b, 0x11, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x1c, 0x01, 0x12, 0x04, 0x8a, 0x04, 0x12, 0x27, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x1c, 0x03, 0x12, 0x04, 0x8a, 0x04, 0x2a, 0x2c, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x11, 0x02, 0x1d, 0x12, 0x04, 0x8b, 0x04, 0x02, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x1d, 0x04, 0x12, 0x04, 0x8b, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x1d, 0x05, 0x12, 0x04, 0x8b, 0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x1d,
    0x01, 0x12, 0x04, 0x8b, 0x04, 0x12, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x1d, 0x03,
    0x12, 0x04, 0x8b, 0x04, 0x29, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x1e, 0x12, 0x04,
    0x8c, 0x04, 0x02, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x1e, 0x04, 0x12, 0x04, 0x8c,
    0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x1e, 0x05, 0x12, 0x04, 0x8c, 0x04,
    0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x1e, 0x01, 0x12, 0x04, 0x8c, 0x04, 0x12,
    0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x1e, 0x03, 0x12, 0x04, 0x8c, 0x04, 0x2e, 0x30,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x1f, 0x12, 0x04, 0x8d, 0x04, 0x02, 0x21, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x1f, 0x04, 0x12, 0x04, 0x8d, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x1f, 0x05, 0x12, 0x04, 0x8d, 0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x1f, 0x01, 0x12, 0x04, 0x8d, 0x04, 0x12, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x1f, 0x03, 0x12, 0x04, 0x8d, 0x04, 0x1e, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11,
    0x02, 0x20, 0x12, 0x04, 0x8e, 0x04, 0x02, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x20,
    0x04, 0x12, 0x04, 0x8e, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x20, 0x05,
    0x12, 0x04, 0x8e, 0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x20, 0x01, 0x12,
    0x04, 0x8e, 0x04, 0x12, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x20, 0x03, 0x12, 0x04,
    0x8e, 0x04, 0x24, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x21, 0x12, 0x04, 0x8f, 0x04,
    0x02, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x21, 0x04, 0x12, 0x04, 0x8f, 0x04, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x21, 0x05, 0x12, 0x04, 0x8f, 0x04, 0x0b, 0x11,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x21, 0x01, 0x12, 0x04, 0x8f, 0x04, 0x12, 0x1c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x21, 0x03, 0x12, 0x04, 0x8f, 0x04, 0x1f, 0x21, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x11, 0x02, 0x22, 0x12, 0x04, 0x90, 0x04, 0x02, 0x28, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x22, 0x04, 0x12, 0x04, 0x90, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x22, 0x05, 0x12, 0x04, 0x90, 0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x22, 0x01, 0x12, 0x04, 0x90, 0x04, 0x12, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x22, 0x03, 0x12, 0x04, 0x90, 0x04, 0x25, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x23,
    0x12, 0x04, 0x91, 0x04, 0x02, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x23, 0x04, 0x12,
    0x04, 0x91, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x23, 0x05, 0x12, 0x04,
    0x91, 0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x23, 0x01, 0x12, 0x04, 0x91,
    0x04, 0x12, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x23, 0x03, 0x12, 0x04, 0x91, 0x04,
    0x23, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x24, 0x12, 0x04, 0x92, 0x04, 0x02, 0x2b,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x24, 0x04, 0x12, 0x04, 0x92, 0x04, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x24, 0x05, 0x12, 0x04, 0x92, 0x04, 0x0b, 0x11, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x24, 0x01, 0x12, 0x04, 0x92, 0x04, 0x12, 0x25, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x24, 0x03, 0x12, 0x04, 0x92, 0x04, 0x28, 0x2a, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x11, 0x02, 0x25, 0x12, 0x04, 0x93, 0x04, 0x02, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x25, 0x04, 0x12, 0x04, 0x93, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x25, 0x05, 0x12, 0x04, 0x93, 0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x25,
    0x01, 0x12, 0x04, 0x93, 0x04, 0x12, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x25, 0x03,
    0x12, 0x04, 0x93, 0x04, 0x1f, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x26, 0x12, 0x04,
    0x94, 0x04, 0x02, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x26, 0x04, 0x12, 0x04, 0x94,
    0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x26, 0x05, 0x12, 0x04, 0x94, 0x04,
    0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x26, 0x01, 0x12, 0x04, 0x94, 0x04, 0x12,
    0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x26, 0x03, 0x12, 0x04, 0x94, 0x04, 0x25, 0x27,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x27, 0x12, 0x04, 0x95, 0x04, 0x02, 0x23, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x27, 0x04, 0x12, 0x04, 0x95, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x27, 0x05, 0x12, 0x04, 0x95, 0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x27, 0x01, 0x12, 0x04, 0x95, 0x04, 0x12, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x27, 0x03, 0x12, 0x04, 0x95, 0x04, 0x20, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11,
    0x02, 0x28, 0x12, 0x04, 0x96, 0x04, 0x02, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x28,
    0x04, 0x12, 0x04, 0x96, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x28, 0x05,
    0x12, 0x04, 0x96, 0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x28, 0x01, 0x12,
    0x04, 0x96, 0x04, 0x12, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x28, 0x03, 0x12, 0x04,
    0x96, 0x04, 0x26, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x29, 0x12, 0x04, 0x97, 0x04,
    0x02, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x29, 0x04, 0x12, 0x04, 0x97, 0x04, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x29, 0x05, 0x12, 0x04, 0x97, 0x04, 0x0b, 0x11,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x29, 0x01, 0x12, 0x04, 0x97, 0x04, 0x12, 0x21, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x29, 0x03, 0x12, 0x04, 0x97, 0x04, 0x24, 0x26, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x11, 0x02, 0x2a, 0x12, 0x04, 0x98, 0x04, 0x02, 0x2c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x2a, 0x04, 0x12, 0x04, 0x98, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x2a, 0x05, 0x12, 0x04, 0x98, 0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x2a, 0x01, 0x12, 0x04, 0x98, 0x04, 0x12, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x2a, 0x03, 0x12, 0x04, 0x98, 0x04, 0x29, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x2b,
    0x12, 0x04, 0x99, 0x04, 0x02, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x2b, 0x04, 0x12,
    0x04, 0x99, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x2b, 0x05, 0x12, 0x04,
    0x99, 0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x2b, 0x01, 0x12, 0x04, 0x99,
    0x04, 0x12, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x2b, 0x03, 0x12, 0x04, 0x99, 0x04,
    0x1f, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x2c, 0x12, 0x04, 0x9a, 0x04, 0x02, 0x28,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x2c, 0x04, 0x12, 0x04, 0x9a, 0x04, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x2c, 0x05, 0x12, 0x04, 0x9a, 0x04, 0x0b, 0x11, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x2c, 0x01, 0x12, 0x04, 0x9a, 0x04, 0x12, 0x22, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x2c, 0x03, 0x12, 0x04, 0x9a, 0x04, 0x25, 0x27, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x11, 0x02, 0x2d, 0x12, 0x04, 0x9b, 0x04, 0x02, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x2d, 0x04, 0x12, 0x04, 0x9b, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x2d, 0x05, 0x12, 0x04, 0x9b, 0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x2d,
    0x01, 0x12, 0x04, 0x9b, 0x04, 0x12, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x2d, 0x03,
    0x12, 0x04, 0x9b, 0x04, 0x21, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x2e, 0x12, 0x04,
    0x9c, 0x04, 0x02, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x2e, 0x04, 0x12, 0x04, 0x9c,
    0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x2e, 0x05, 0x12, 0x04, 0x9c, 0x04,
    0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x2e, 0x01, 0x12, 0x04, 0x9c, 0x04, 0x12,
    0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x2e, 0x03, 0x12, 0x04, 0x9c, 0x04, 0x27, 0x29,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x2f, 0x12, 0x04, 0x9d, 0x04, 0x02, 0x22, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x2f, 0x04, 0x12, 0x04, 0x9d, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x2f, 0x05, 0x12, 0x04, 0x9d, 0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x2f, 0x01, 0x12, 0x04, 0x9d, 0x04, 0x12, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x2f, 0x03, 0x12, 0x04, 0x9d, 0x04, 0x1f, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11,
    0x02, 0x30, 0x12, 0x04, 0x9e, 0x04, 0x02, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x30,
    0x04, 0x12, 0x04, 0x9e, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x30, 0x05,
    0x12, 0x04, 0x9e, 0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x30, 0x01, 0x12,
    0x04, 0x9e, 0x04, 0x12, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x30, 0x03, 0x12, 0x04,
    0x9e, 0x04, 0x25, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x31, 0x12, 0x04, 0x9f, 0x04,
    0x02, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x31, 0x04, 0x12, 0x04, 0x9f, 0x04, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x31, 0x05, 0x12, 0x04, 0x9f, 0x04, 0x0b, 0x11,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x31, 0x01, 0x12, 0x04, 0x9f, 0x04, 0x12, 0x1d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x31, 0x03, 0x12, 0x04, 0x9f, 0x04, 0x20, 0x22, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x11, 0x02, 0x32, 0x12, 0x04, 0xa0, 0x04, 0x02, 0x29, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x32, 0x04, 0x12, 0x04, 0xa0, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x32, 0x05, 0x12, 0x04, 0xa0, 0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x32, 0x01, 0x12, 0x04, 0xa0, 0x04, 0x12, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x32, 0x03, 0x12, 0x04, 0xa0, 0x04, 0x26, 0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x33,
    0x12, 0x04, 0xa1, 0x04, 0x02, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x33, 0x04, 0x12,
    0x04, 0xa1, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x33, 0x05, 0x12, 0x04,
    0xa1, 0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x33, 0x01, 0x12, 0x04, 0xa1,
    0x04, 0x12, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x33, 0x03, 0x12, 0x04, 0xa1, 0x04,
    0x24, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x34, 0x12, 0x04, 0xa2, 0x04, 0x02, 0x2c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x34, 0x04, 0x12, 0x04, 0xa2, 0x04, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x34, 0x05, 0x12, 0x04, 0xa2, 0x04, 0x0b, 0x11, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x34, 0x01, 0x12, 0x04, 0xa2, 0x04, 0x12, 0x26, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x34, 0x03, 0x12, 0x04, 0xa2, 0x04, 0x29, 0x2b, 0x0a, 0xe1, 0x01, 0x0a,
    0x02, 0x04, 0x12, 0x12, 0x06, 0xac, 0x04, 0x00, 0xaf, 0x04, 0x01, 0x1a, 0xd2, 0x01, 0x2a, 0x0a,
    0x20, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x73, 0x20, 0x61, 0x20, 0x72, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63,
    0x65, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x75,
    0x73, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x61, 0x20, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f,
    0x72, 0x6b, 0x0a, 0x20, 0x74, 0x6f, 0x20, 0x70, 0x72, 0x6f, 0x61, 0x63, 0x74, 0x69, 0x76, 0x65,
    0x6c, 0x79, 0x20, 0x69, 0x6e, 0x66, 0x6c, 0x75, 0x65, 0x6e, 0x63, 0x65, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x61, 0x6c, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x6f, 0x72, 0x2e, 0x20, 0x20, 0x49, 0x66, 0x20,
    0x27, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x5f, 0x69, 0x64, 0x27, 0x20, 0x69, 0x73, 0x20, 0x70, 0x72,
    0x6f, 0x76, 0x69, 0x64, 0x65, 0x64, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x6e, 0x20, 0x74, 0x68, 0x69,
    0x73, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x69, 0x73, 0x20, 0x61, 0x73, 0x73,
    0x75, 0x6d, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x61, 0x70, 0x70,
    0x6c, 0x79, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x73, 0x20,
    0x6f, 0x6e, 0x20, 0x74, 0x68, 0x61, 0x74, 0x0a, 0x20, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x2e, 0x0a,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x12, 0x01, 0x12, 0x04, 0xac, 0x04, 0x08, 0x0f, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x12, 0x02, 0x00, 0x12, 0x04, 0xad, 0x04, 0x02, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x12, 0x02, 0x00, 0x04, 0x12, 0x04, 0xad, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12,
    0x02, 0x00, 0x06, 0x12, 0x04, 0xad, 0x04, 0x0b, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xad, 0x04, 0x13, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xad, 0x04, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x01, 0x12,
    0x04, 0xae, 0x04, 0x02, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x04, 0x12, 0x04,
    0xae, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x06, 0x12, 0x04, 0xae,
    0x04, 0x0b, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x01, 0x12, 0x04, 0xae, 0x04,
    0x14, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x03, 0x12, 0x04, 0xae, 0x04, 0x20,
    0x21, 0x0a, 0x77, 0x0a, 0x02, 0x04, 0x13, 0x12, 0x06, 0xb6, 0x04, 0x00, 0xbe, 0x04, 0x01, 0x1a,
    0x69, 0x2a, 0x0a, 0x20, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x73, 0x20, 0x73, 0x6f,
    0x6d, 0x65, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x73, 0x20, 0x61, 0x76, 0x61,
    0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x6f, 0x6e, 0x20, 0x61, 0x20, 0x73, 0x6c, 0x61, 0x76,
    0x65, 0x2e, 0x20, 0x41, 0x6e, 0x20, 0x6f, 0x66, 0x66, 0x65, 0x72, 0x20, 0x6f, 0x6e, 0x6c, 0x79,
    0x0a, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75,
    0x72, 0x63, 0x65, 0x73, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x61, 0x20, 0x73, 0x69, 0x6e, 0x67,
    0x6c, 0x65, 0x20, 0x73, 0x6c, 0x61, 0x76, 0x65, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x13,
    0x01, 0x12, 0x04, 0xb6, 0x04, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x00, 0x12,
    0x04, 0xb7, 0x04, 0x02, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xb7, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x06, 0x12, 0x04, 0xb7,
    0x04, 0x0b, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb7, 0x04,
    0x13, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb7, 0x04, 0x18,
    0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x01, 0x12, 0x04, 0xb8, 0x04, 0x02, 0x28, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x04, 0x12, 0x04, 0xb8, 0x04, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x06, 0x12, 0x04, 0xb8, 0x04, 0x0b, 0x16, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x01, 0x01, 0x12, 0x04, 0xb8, 0x04, 0x17, 0x23, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x01, 0x03, 0x12, 0x04, 0xb8, 0x04, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x13, 0x02, 0x02, 0x12, 0x04, 0xb9, 0x04, 0x02, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x02, 0x04, 0x12, 0x04, 0xb9, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02,
    0x06, 0x12, 0x04, 0xb9, 0x04, 0x0b, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x01,
    0x12, 0x04, 0xb9, 0x04, 0x13, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x03, 0x12,
    0x04, 0xb9, 0x04, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x03, 0x12, 0x04, 0xba,
    0x04, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x03, 0x04, 0x12, 0x04, 0xba, 0x04,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x03, 0x05, 0x12, 0x04, 0xba, 0x04, 0x0b,
    0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x03, 0x01, 0x12, 0x04, 0xba, 0x04, 0x12, 0x1a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x03, 0x03, 0x12, 0x04, 0xba, 0x04, 0x1d, 0x1e, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x04, 0x12, 0x04, 0xbb, 0x04, 0x02, 0x22, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x04, 0x04, 0x12, 0x04, 0xbb, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x04, 0x06, 0x12, 0x04, 0xbb, 0x04, 0x0b, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x04, 0x01, 0x12, 0x04, 0xbb, 0x04, 0x14, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x04, 0x03, 0x12, 0x04, 0xbb, 0x04, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02,
    0x05, 0x12, 0x04, 0xbc, 0x04, 0x02, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x05, 0x04,
    0x12, 0x04, 0xbc, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x05, 0x06, 0x12,
    0x04, 0xbc, 0x04, 0x0b, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x05, 0x01, 0x12, 0x04,
    0xbc, 0x04, 0x15, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x05, 0x03, 0x12, 0x04, 0xbc,
    0x04, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x06, 0x12, 0x04, 0xbd, 0x04, 0x02,
    0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x06, 0x04, 0x12, 0x04, 0xbd, 0x04, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x06, 0x06, 0x12, 0x04, 0xbd, 0x04, 0x0b, 0x15, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x06, 0x01, 0x12, 0x04, 0xbd, 0x04, 0x16, 0x22, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x06, 0x03, 0x12, 0x04, 0xbd, 0x04, 0x25, 0x26, 0x0a, 0xdb, 0x02,
    0x0a, 0x02, 0x04, 0x14, 0x12, 0x06, 0xc8, 0x04, 0x00, 0xd6, 0x04, 0x01, 0x1a, 0xcc, 0x02, 0x2a,
    0x0a, 0x20, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x73, 0x20, 0x61, 0x20, 0x74, 0x61,
    0x73, 0x6b, 0x2e, 0x20, 0x50, 0x61, 0x73, 0x73, 0x65, 0x64, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x73, 0x63, 0x68, 0x65, 0x64, 0x75, 0x6c, 0x65, 0x72, 0x20, 0x61, 0x6c,
    0x6c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x77, 0x61, 0x79, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x6e, 0x0a,
    0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x20, 0x28, 0x73, 0x65, 0x65, 0x20, 0x53,
    0x63, 0x68, 0x65, 0x64, 0x75, 0x6c, 0x65, 0x72, 0x44, 0x72, 0x69, 0x76, 0x65, 0x72, 0x3a, 0x3a,
    0x6c, 0x61, 0x75, 0x6e, 0x63, 0x68, 0x54, 0x61, 0x73, 0x6b, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x0a,
    0x20, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x3a, 0x3a, 0x6c, 0x61, 0x75, 0x6e, 0x63,
    0x68, 0x54, 0x61, 0x73, 0x6b, 0x29, 0x2e, 0x20, 0x45, 0x69, 0x74, 0x68, 0x65, 0x72, 0x20, 0x45,
    0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x49, 0x6e, 0x66, 0x6f, 0x20, 0x6f, 0x72, 0x20, 0x43,
    0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x49, 0x6e, 0x66, 0x6f, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c,
    0x64, 0x20, 0x62, 0x65, 0x20, 0x73, 0x65, 0x74, 0x2e, 0x0a, 0x20, 0x41, 0x20, 0x64, 0x69, 0x66,
    0x66, 0x65, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x20,
    0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x6c,
    0x61, 0x75, 0x6e, 0x63, 0x68, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x2c,
    0x20, 0x61, 0x6e, 0x64, 0x20, 0x73, 0x75, 0x62, 0x73, 0x65, 0x71, 0x75, 0x65, 0x6e, 0x74, 0x20,
    0x74, 0x61, 0x73, 0x6b, 0x73, 0x0a, 0x20, 0x6d, 0x65, 0x61, 0x6e, 0x74, 0x20, 0x66, 0x6f, 0x72,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x61, 0x6d, 0x65, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74,
    0x6f, 0x72, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x72, 0x65, 0x75, 0x73, 0x65, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x73, 0x61, 0x6d, 0x65, 0x20, 0x45, 0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x49, 0x6e,
    0x66, 0x6f, 0x20, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x14, 0x01, 0x12, 0x04, 0xc8, 0x04, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x00,
    0x12, 0x04, 0xc9, 0x04, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xc9, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x05, 0x12, 0x04,
    0xc9, 0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc9,
    0x04, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc9, 0x04,
    0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x01, 0x12, 0x04, 0xca, 0x04, 0x02, 0x1e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x04, 0x12, 0x04, 0xca, 0x04, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x06, 0x12, 0x04, 0xca, 0x04, 0x0b, 0x11, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x01, 0x12, 0x04, 0xca, 0x04, 0x12, 0x19, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x01, 0x03, 0x12, 0x04, 0xca, 0x04, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x14, 0x02, 0x02, 0x12, 0x04, 0xcb, 0x04, 0x02, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14,
    0x02, 0x02, 0x04, 0x12, 0x04, 0xcb, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x02, 0x06, 0x12, 0x04, 0xcb, 0x04, 0x0b, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02,
    0x01, 0x12, 0x04, 0xcb, 0x04, 0x13, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x03,
    0x12, 0x04, 0xcb, 0x04, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x03, 0x12, 0x04,
    0xcc, 0x04, 0x02, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x04, 0x12, 0x04, 0xcc,
    0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x06, 0x12, 0x04, 0xcc, 0x04,
    0x0b, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x01, 0x12, 0x04, 0xcc, 0x04, 0x14,
    0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x03, 0x12, 0x04, 0xcc, 0x04, 0x20, 0x21,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x04, 0x12, 0x04, 0xcd, 0x04, 0x02, 0x25, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x14, 0x02, 0x04, 0x04, 0x12, 0x04, 0xcd, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x04, 0x06, 0x12, 0x04, 0xcd, 0x04, 0x0b, 0x17, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x14, 0x02, 0x04, 0x01, 0x12, 0x04, 0xcd, 0x04, 0x18, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x04, 0x03, 0x12, 0x04, 0xcd, 0x04, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14,
    0x02, 0x05, 0x12, 0x04, 0xce, 0x04, 0x02, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x05,
    0x04, 0x12, 0x04, 0xce, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x05, 0x06,
    0x12, 0x04, 0xce, 0x04, 0x0b, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x05, 0x01, 0x12,
    0x04, 0xce, 0x04, 0x17, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x05, 0x03, 0x12, 0x04,
    0xce, 0x04, 0x21, 0x22, 0x0a, 0x82, 0x01, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x06, 0x12, 0x04, 0xd1,
    0x04, 0x02, 0x27, 0x1a, 0x74, 0x20, 0x54, 0x61, 0x73, 0x6b, 0x20, 0x70, 0x72, 0x6f, 0x76, 0x69,
    0x64, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61,
    0x69, 0x6e, 0x65, 0x72, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x6c, 0x61, 0x75, 0x6e, 0x63, 0x68,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x20, 0x61,
    0x73, 0x20, 0x70, 0x61, 0x72, 0x74, 0x0a, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20,
    0x74, 0x61, 0x73, 0x6b, 0x20, 0x70, 0x61, 0x69, 0x72, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x27, 0x73, 0x20, 0x43, 0x6f, 0x6d, 0x6d,
    0x61, 0x6e, 0x64, 0x49, 0x6e, 0x66, 0x6f, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x06, 0x04, 0x12, 0x04, 0xd1, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x06,
    0x06, 0x12, 0x04, 0xd1, 0x04, 0x0b, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x06, 0x01,
    0x12, 0x04, 0xd1, 0x04, 0x19, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x06, 0x03, 0x12,
    0x04, 0xd1, 0x04, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x07, 0x12, 0x04, 0xd2,
    0x04, 0x02, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x07, 0x04, 0x12, 0x04, 0xd2, 0x04,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x07, 0x05, 0x12, 0x04, 0xd2, 0x04, 0x0b,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x07, 0x01, 0x12, 0x04, 0xd2, 0x04, 0x11, 0x15,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x07, 0x03, 0x12, 0x04, 0xd2, 0x04, 0x18, 0x19, 0x0a,
    0x8c, 0x01, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x08, 0x12, 0x04, 0xd5, 0x04, 0x02, 0x28, 0x1a, 0x7e,
    0x20, 0x41, 0x20, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x20, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x20, 0x28, 0x63, 0x75,
    0x72, 0x72, 0x65, 0x6e, 0x74, 0x6c, 0x79, 0x20, 0x69, 0x6e, 0x20, 0x2a, 0x61, 0x6c, 0x70, 0x68,
    0x61, 0x2a, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x69, 0x6e, 0x69, 0x74, 0x69, 0x61, 0x6c, 0x0a, 0x20,
    0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x6f, 0x6e, 0x6c,
    0x79, 0x20, 0x62, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x54, 0x61, 0x73, 0x6b, 0x49, 0x6e, 0x66,
    0x6f, 0x27, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20, 0x61, 0x20,
    0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x49, 0x6e, 0x66, 0x6f, 0x29, 0x2e, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x14, 0x02, 0x08, 0x04, 0x12, 0x04, 0xd5, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x08, 0x06, 0x12, 0x04, 0xd5, 0x04, 0x0b, 0x16, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x14, 0x02, 0x08, 0x01, 0x12, 0x04, 0xd5, 0x04, 0x17, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x08, 0x03, 0x12, 0x04, 0xd5, 0x04, 0x26, 0x27, 0x0a, 0xa6, 0x02, 0x0a, 0x02, 0x05,
    0x01, 0x12, 0x06, 0xe0, 0x04, 0x00, 0xeb, 0x04, 0x01, 0x1a, 0x97, 0x02, 0x2a, 0x0a, 0x20, 0x44,
    0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x73, 0x20, 0x70, 0x6f, 0x73, 0x73, 0x69, 0x62, 0x6c,
    0x65, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x73, 0x2e, 0x20, 0x49,
    0x4d, 0x50, 0x4f, 0x52, 0x54, 0x41, 0x4e, 0x54, 0x3a, 0x20, 0x4d, 0x65, 0x73, 0x6f, 0x73, 0x20,
    0x61, 0x73, 0x73, 0x75, 0x6d, 0x65, 0x73, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x73, 0x20, 0x74, 0x68,
    0x61, 0x74, 0x0a, 0x20, 0x65, 0x6e, 0x74, 0x65, 0x72, 0x20, 0x74, 0x65, 0x72, 0x6d, 0x69, 0x6e,
    0x61, 0x6c, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x73, 0x20, 0x28, 0x73, 0x65, 0x65, 0x20, 0x62,
    0x65, 0x6c, 0x6f, 0x77, 0x29, 0x20, 0x69, 0x6d, 0x70, 0x6c, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x74, 0x61, 0x73, 0x6b, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x20, 0x6c, 0x6f, 0x6e, 0x67, 0x65,
    0x72, 0x0a, 0x20, 0x72, 0x75, 0x6e, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x74,
    0x68, 0x75, 0x73, 0x20, 0x63, 0x6c, 0x65, 0x61, 0x6e, 0x20, 0x75, 0x70, 0x20, 0x61, 0x6e, 0x79,
    0x20, 0x74, 0x68, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x73, 0x73, 0x6f, 0x63, 0x69, 0x61, 0x74, 0x65,
    0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x0a,
    0x20, 0x28, 0x75, 0x6c, 0x74, 0x69, 0x6d, 0x61, 0x74, 0x65, 0x6c, 0x79, 0x20, 0x6f, 0x66, 0x66,
    0x65, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x6e, 0x79, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72,
    0x63, 0x65, 0x73, 0x20, 0x62, 0x65, 0x69, 0x6e, 0x67, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x75, 0x6d,
    0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x20,
    0x74, 0x6f, 0x0a, 0x20, 0x61, 0x6e, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x20, 0x74, 0x61, 0x73, 0x6b,
    0x29, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x05, 0x01, 0x01, 0x12, 0x04, 0xe0, 0x04, 0x05, 0x0e,
    0x0a, 0x47, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x00, 0x12, 0x04, 0xe1, 0x04, 0x02, 0x13, 0x22, 0x39,
    0x20, 0x49, 0x6e, 0x69, 0x74, 0x69, 0x61, 0x6c, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x2e, 0x20,
    0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x20, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20,
    0x6e, 0x6f, 0x74, 0x20, 0x75, 0x73, 0x65, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xe1, 0x04, 0x02, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00,
    0x02, 0x12, 0x04, 0xe1, 0x04, 0x11, 0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x01, 0x12,
    0x04, 0xe2, 0x04, 0x02, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xe2, 0x04, 0x02, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x02, 0x12, 0x04, 0xe2,
    0x04, 0x12, 0x13, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x02, 0x12, 0x04, 0xe3, 0x04, 0x02,
    0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x01, 0x12, 0x04, 0xe3, 0x04, 0x02, 0x0e,
    0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x02, 0x12, 0x04, 0xe3, 0x04, 0x11, 0x12, 0x0a,
    0x39, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x03, 0x12, 0x04, 0xe4, 0x04, 0x02, 0x14, 0x22, 0x2b, 0x20,
    0x54, 0x45, 0x52, 0x4d, 0x49, 0x4e, 0x41, 0x4c, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x74, 0x61,
    0x73, 0x6b, 0x20, 0x66, 0x69, 0x6e, 0x69, 0x73, 0x68, 0x65, 0x64, 0x20, 0x73, 0x75, 0x63, 0x63,
    0x65, 0x73, 0x73, 0x66, 0x75, 0x6c, 0x6c, 0x79, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01,
    0x02, 0x03, 0x01, 0x12, 0x04, 0xe4, 0x04, 0x02, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02,
    0x03, 0x02, 0x12, 0x04, 0xe4, 0x04, 0x12, 0x13, 0x0a, 0x41, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x04,
    0x12, 0x04, 0xe5, 0x04, 0x02, 0x12, 0x22, 0x33, 0x20, 0x54, 0x45, 0x52, 0x4d, 0x49, 0x4e, 0x41,
    0x4c, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x20, 0x66, 0x61, 0x69, 0x6c,
    0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x66, 0x69, 0x6e, 0x69, 0x73, 0x68, 0x20, 0x73, 0x75, 0x63,
    0x63, 0x65, 0x73, 0x73, 0x66, 0x75, 0x6c, 0x6c, 0x79, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x05,
    0x01, 0x02, 0x04, 0x01, 0x12, 0x04, 0xe5, 0x04, 0x02, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01,
    0x02, 0x04, 0x02, 0x12, 0x04, 0xe5, 0x04, 0x10, 0x11, 0x0a, 0x3e, 0x0a, 0x04, 0x05, 0x01, 0x02,
    0x05, 0x12, 0x04, 0xe6, 0x04, 0x02, 0x12, 0x22, 0x30, 0x20, 0x54, 0x45, 0x52, 0x4d, 0x49, 0x4e,
    0x41, 0x4c, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x20, 0x77, 0x61, 0x73,
    0x20, 0x6b, 0x69, 0x6c, 0x6c, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65,
    0x78, 0x65, 0x63, 0x75, 0x74, 0x6f, 0x72, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02,
    0x05, 0x01, 0x12, 0x04, 0xe6, 0x04, 0x02, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x05,
    0x02, 0x12, 0x04, 0xe6, 0x04, 0x10, 0x11, 0x0a, 0x41, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x06, 0x12,
    0x04, 0xe7, 0x04, 0x02, 0x10, 0x22, 0x33, 0x20, 0x54, 0x45, 0x52, 0x4d, 0x49, 0x4e, 0x41, 0x4c,
    0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x65,
    0x64, 0x20, 0x62, 0x75, 0x74, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x72, 0x65, 0x73,
    0x63, 0x68, 0x65, 0x64, 0x75, 0x6c, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01,
    0x02, 0x06, 0x01, 0x12, 0x04, 0xe7, 0x04, 0x02, 0x0b, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02,
    0x06, 0x02, 0x12, 0x04, 0xe7, 0x04, 0x0e, 0x0f, 0x0a, 0xac, 0x01, 0x0a, 0x04, 0x05, 0x01, 0x02,
    0x07, 0x12, 0x04, 0xea, 0x04, 0x02, 0x11, 0x1a, 0x69, 0x20, 0x54, 0x41, 0x53, 0x4b, 0x5f, 0x45,
    0x52, 0x52, 0x4f, 0x52, 0x20, 0x69, 0x73, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x6c,
    0x79, 0x20, 0x75, 0x6e, 0x75, 0x73, 0x65, 0x64, 0x20, 0x62, 0x75, 0x74, 0x20, 0x77, 0x69, 0x6c,
    0x6c, 0x20, 0x62, 0x65, 0x20, 0x69, 0x6e, 0x74, 0x72, 0x6f, 0x64, 0x75, 0x63, 0x65, 0x64, 0x20,
    0x69, 0x6e, 0x20, 0x30, 0x2e, 0x32, 0x32, 0x2e, 0x30, 0x2e, 0x0a, 0x20, 0x54, 0x4f, 0x44, 0x4f,
    0x28, 0x64, 0x68, 0x61, 0x6d, 0x6f, 0x6e, 0x29, 0x3a, 0x20, 0x53, 0x74, 0x61, 0x72, 0x74, 0x20,
    0x75, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x54, 0x41, 0x53, 0x4b, 0x5f, 0x45, 0x52, 0x52, 0x4f, 0x52,
    0x2e, 0x0a, 0x22, 0x33, 0x20, 0x54, 0x45, 0x52, 0x4d, 0x49, 0x4e, 0x41, 0x4c, 0x2e, 0x20, 0x54,
    0x68, 0x65, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x20, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74,
    0x69, 0x6f, 0x6e, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x20, 0x61, 0x6e, 0x20,
    0x65, 0x72, 0x72, 0x6f, 0x72, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x07, 0x01,
    0x12, 0x04, 0xea, 0x04, 0x02, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x07, 0x02, 0x12,
    0x04, 0xea, 0x04, 0x0f, 0x10, 0x0a, 0x39, 0x0a, 0x02, 0x04, 0x15, 0x12, 0x06, 0xf1, 0x04, 0x00,
    0x9c, 0x05, 0x01, 0x1a, 0x2b, 0x2a, 0x0a, 0x20, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65,
    0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x73, 0x74,
    0x61, 0x74, 0x75, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x2e, 0x0a,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x15, 0x01, 0x12, 0x04, 0xf1, 0x04, 0x08, 0x12, 0x0a, 0x42, 0x0a,
    0x04, 0x04, 0x15, 0x04, 0x00, 0x12, 0x06, 0xf3, 0x04, 0x02, 0xf7, 0x04, 0x03, 0x1a, 0x32, 0x2a,
    0x20, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73,
    0x6f, 0x75, 0x72, 0x63, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x61, 0x73,
    0x6b, 0x20, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x2e,
    0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x04, 0x00, 0x01, 0x12, 0x04, 0xf3, 0x04, 0x07, 0x0d,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0xf4, 0x04, 0x04, 0x16,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xf4, 0x04, 0x04,
    0x11, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0xf4, 0x04,
    0x14, 0x15, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0xf5, 0x04,
    0x04, 0x15, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xf5,
    0x04, 0x04, 0x10, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04,
    0xf5, 0x04, 0x13, 0x14, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04,
    0xf6, 0x04, 0x04, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12,
    0x04, 0xf6, 0x04, 0x04, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x00, 0x02, 0x02, 0x02,
    0x12, 0x04, 0xf6, 0x04, 0x16, 0x17, 0x0a, 0x3e, 0x0a, 0x04, 0x04, 0x15, 0x04, 0x01, 0x12, 0x06,
    0xfa, 0x04, 0x02, 0x8c, 0x05, 0x03, 0x1a, 0x2e, 0x2a, 0x20, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c,
    0x65, 0x64, 0x20, 0x72, 0x65, 0x61, 0x73, 0x6f, 0x6e, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x20, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x20, 0x75, 0x70,
    0x64, 0x61, 0x74, 0x65, 0x2e, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x04, 0x01, 0x01, 0x12,
    0x04, 0xfa, 0x04, 0x07, 0x0d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x04, 0x01, 0x02, 0x00, 0x12,
    0x04, 0xfb, 0x04, 0x04, 0x27, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x01, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xfb, 0x04, 0x04, 0x22, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x01, 0x02, 0x00,
    0x02, 0x12, 0x04, 0xfb, 0x04, 0x25, 0x26, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x04, 0x01, 0x02,
    0x01, 0x12, 0x04, 0xfc, 0x04, 0x04, 0x23, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x01, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xfc, 0x04, 0x04, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x01,
    0x02, 0x01, 0x02, 0x12, 0x04, 0xfc, 0x04, 0x21, 0x22, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x04,
    0x01, 0x02, 0x02, 0x12, 0x04, 0xfd, 0x04, 0x04, 0x25, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04,
    0x01, 0x02, 0x02, 0x01, 0x12, 0x04, 0xfd, 0x04, 0x04, 0x20, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15,
    0x04, 0x01, 0x02, 0x02, 0x02, 0x12, 0x04, 0xfd, 0x04, 0x23, 0x24, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x15, 0x04, 0x01, 0x02, 0x03, 0x12, 0x04, 0xfe, 0x04, 0x04, 0x21, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x15, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x04, 0xfe, 0x04, 0x04, 0x1c, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x15, 0x04, 0x01, 0x02, 0x03, 0x02, 0x12, 0x04, 0xfe, 0x04, 0x1f, 0x20, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x15, 0x04, 0x01, 0x02, 0x04, 0x12, 0x04, 0xff, 0x04, 0x04, 0x18, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x15, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x04, 0xff, 0x04, 0x04, 0x13, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x15, 0x04, 0x01, 0x02, 0x04, 0x02, 0x12, 0x04, 0xff, 0x04, 0x16, 0x17, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x15, 0x04, 0x01, 0x02, 0x05, 0x12, 0x04, 0x80, 0x05, 0x04, 0x23, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x01, 0x02, 0x05, 0x01, 0x12, 0x04, 0x80, 0x05, 0x04, 0x1e,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x01, 0x02, 0x05, 0x02, 0x12, 0x04, 0x80, 0x05, 0x21,
    0x22, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x04, 0x01, 0x02, 0x06, 0x12, 0x04, 0x81, 0x05, 0x04,
    0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x01, 0x02, 0x06, 0x01, 0x12, 0x04, 0x81, 0x05,
    0x04, 0x19, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x01, 0x02, 0x06, 0x02, 0x12, 0x04, 0x81,
    0x05, 0x1c, 0x1d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x04, 0x01, 0x02, 0x07, 0x12, 0x04, 0x82,
    0x05, 0x04, 0x23, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x01, 0x02, 0x07, 0x01, 0x12, 0x04,
    0x82, 0x05, 0x04, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x01, 0x02, 0x07, 0x02, 0x12,
    0x04, 0x82, 0x05, 0x21, 0x22, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x04, 0x01, 0x02, 0x08, 0x12,
    0x04, 0x83, 0x05, 0x04, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x01, 0x02, 0x08, 0x01,
    0x12, 0x04, 0x83, 0x05, 0x04, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x01, 0x02, 0x08,
    0x02, 0x12, 0x04, 0x83, 0x05, 0x1a, 0x1b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x04, 0x01, 0x02,
    0x09, 0x12, 0x04, 0x84, 0x05, 0x04, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x01, 0x02,
    0x09, 0x01, 0x12, 0x04, 0x84, 0x05, 0x04, 0x19, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x01,
    0x02, 0x09, 0x02, 0x12, 0x04, 0x84, 0x05, 0x1c, 0x1d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x04,
    0x01, 0x02, 0x0a, 0x12, 0x04, 0x85, 0x05, 0x04, 0x23, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04,
    0x01, 0x02, 0x0a, 0x01, 0x12, 0x04, 0x85, 0x05, 0x04, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15,
    0x04, 0x01, 0x02, 0x0a, 0x02, 0x12, 0x04, 0x85, 0x05, 0x20, 0x22, 0x0a, 0x0e, 0x0a, 0x06, 0x04,
    0x15, 0x04, 0x01, 0x02, 0x0b, 0x12, 0x04, 0x86, 0x05, 0x04, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x15, 0x04, 0x01, 0x02, 0x0b, 0x01, 0x12, 0x04, 0x86, 0x05, 0x04, 0x18, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x15, 0x04, 0x01, 0x02, 0x0b, 0x02, 0x12, 0x04, 0x86, 0x05, 0x1b, 0x1d, 0x0a, 0x0e, 0x0a,
    0x06, 0x04, 0x15, 0x04, 0x01, 0x02, 0x0c, 0x12, 0x04, 0x87, 0x05, 0x04, 0x20, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x15, 0x04, 0x01, 0x02, 0x0c, 0x01, 0x12, 0x04, 0x87, 0x05, 0x04, 0x1a, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x15, 0x04, 0x01, 0x02, 0x0c, 0x02, 0x12, 0x04, 0x87, 0x05, 0x1d, 0x1f, 0x0a,
    0x0e, 0x0a, 0x06, 0x04, 0x15, 0x04, 0x01, 0x02, 0x0d, 0x12, 0x04, 0x88, 0x05, 0x04, 0x1e, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x01, 0x02, 0x0d, 0x01, 0x12, 0x04, 0x88, 0x05, 0x04, 0x18,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x01, 0x02, 0x0d, 0x02, 0x12, 0x04, 0x88, 0x05, 0x1b,
    0x1d, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x04, 0x01, 0x02, 0x0e, 0x12, 0x04, 0x89, 0x05, 0x04,
    0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x01, 0x02, 0x0e, 0x01, 0x12, 0x04, 0x89, 0x05,
    0x04, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x01, 0x02, 0x0e, 0x02, 0x12, 0x04, 0x89,
    0x05, 0x1a, 0x1c, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x04, 0x01, 0x02, 0x0f, 0x12, 0x04, 0x8a,
    0x05, 0x04, 0x22, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x01, 0x02, 0x0f, 0x01, 0x12, 0x04,
    0x8a, 0x05, 0x04, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x01, 0x02, 0x0f, 0x02, 0x12,
    0x04, 0x8a, 0x05, 0x1f, 0x21, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x15, 0x04, 0x01, 0x02, 0x10, 0x12,
    0x04, 0x8b, 0x05, 0x04, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x01, 0x02, 0x10, 0x01,
    0x12, 0x04, 0x8b, 0x05, 0x04, 0x17, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x15, 0x04, 0x01, 0x02, 0x10,
    0x02, 0x12, 0x04, 0x8b, 0x05, 0x1a, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x00, 0x12,
    0x04, 0x8e, 0x05, 0x02, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x8e, 0x05, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x06, 0x12, 0x04, 0x8e,
    0x05, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8e, 0x05,
    0x12, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8e, 0x05, 0x1c,
    0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x01, 0x12, 0x04, 0x8f, 0x05, 0x02, 0x1f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x04, 0x12, 0x04, 0x8f, 0x05, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x06, 0x12, 0x04, 0x8f, 0x05, 0x0b, 0x14, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8f, 0x05, 0x15, 0x1a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8f, 0x05, 0x1d, 0x1e, 0x0a, 0x32, 0x0a, 0x04, 0x04,
    0x15, 0x02, 0x02, 0x12, 0x04, 0x90, 0x05, 0x02, 0x1e, 0x22, 0x24, 0x20, 0x50, 0x6f, 0x73, 0x73,
    0x69, 0x62, 0x6c, 0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x65, 0x78, 0x70,
    0x6c, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x2e, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x04, 0x12, 0x04, 0x90, 0x05, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x05, 0x12, 0x04, 0x90, 0x05, 0x0b, 0x11, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x02, 0x01, 0x12, 0x04, 0x90, 0x05, 0x12, 0x19, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x02, 0x03, 0x12, 0x04, 0x90, 0x05, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x15, 0x02, 0x03, 0x12, 0x04, 0x91, 0x05, 0x02, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x03, 0x04, 0x12, 0x04, 0x91, 0x05, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03,
    0x06, 0x12, 0x04, 0x91, 0x05, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x01,
    0x12, 0x04, 0x91, 0x05, 0x12, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x03, 0x12,
    0x04, 0x91, 0x05, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x04, 0x12, 0x04, 0x92,
    0x05, 0x02, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x04, 0x04, 0x12, 0x04, 0x92, 0x05,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x04, 0x06, 0x12, 0x04, 0x92, 0x05, 0x0b,
    0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x04, 0x01, 0x12, 0x04, 0x92, 0x05, 0x12, 0x18,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x04, 0x03, 0x12, 0x04, 0x92, 0x05, 0x1b, 0x1d, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x05, 0x12, 0x04, 0x93, 0x05, 0x02, 0x1a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x05, 0x04, 0x12, 0x04, 0x93, 0x05, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x05, 0x05, 0x12, 0x04, 0x93, 0x05, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x05, 0x01, 0x12, 0x04, 0x93, 0x05, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x05, 0x03, 0x12, 0x04, 0x93, 0x05, 0x18, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02,
    0x06, 0x12, 0x04, 0x94, 0x05, 0x02, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x06, 0x04,
    0x12, 0x04, 0x94, 0x05, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x06, 0x06, 0x12,
    0x04, 0x94, 0x05, 0x0b, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x06, 0x01, 0x12, 0x04,
    0x94, 0x05, 0x13, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x06, 0x03, 0x12, 0x04, 0x94,
    0x05, 0x1e, 0x1f, 0x0a, 0x30, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x07, 0x12, 0x04, 0x95, 0x05, 0x02,
    0x26, 0x22, 0x22, 0x20, 0x54, 0x4f, 0x44, 0x4f, 0x28, 0x62, 0x65, 0x6e, 0x68, 0x29, 0x3a, 0x20,
    0x55, 0x73, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2f, 0x73, 0x6c,
    0x61, 0x76, 0x65, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x07, 0x04, 0x12, 0x04,
    0x95, 0x05, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x07, 0x06, 0x12, 0x04, 0x95,
    0x05, 0x0b, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x07, 0x01, 0x12, 0x04, 0x95, 0x05,
    0x16, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x07, 0x03, 0x12, 0x04, 0x95, 0x05, 0x24,
    0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x08, 0x12, 0x04, 0x96, 0x05, 0x02, 0x20, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x08, 0x04, 0x12, 0x04, 0x96, 0x05, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x08, 0x05, 0x12, 0x04, 0x96, 0x05, 0x0b, 0x11, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x08, 0x01, 0x12, 0x04, 0x96, 0x05, 0x12, 0x1b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x08, 0x03, 0x12, 0x04, 0x96, 0x05, 0x1e, 0x1f, 0x0a, 0xa3, 0x01, 0x0a, 0x04,
    0x04, 0x15, 0x02, 0x09, 0x12, 0x04, 0x9b, 0x05, 0x02, 0x1c, 0x1a, 0x94, 0x01, 0x20, 0x44, 0x65,
    0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x73, 0x20, 0x77, 0x68, 0x65, 0x74, 0x68, 0x65, 0x72, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x20, 0x68, 0x61, 0x73, 0x20, 0x62, 0x65, 0x65,
    0x6e, 0x20, 0x64, 0x65, 0x74, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20,
    0x62, 0x65, 0x20, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x79, 0x0a, 0x20, 0x28, 0x74, 0x72, 0x75,
    0x65, 0x29, 0x20, 0x6f, 0x72, 0x20, 0x75, 0x6e, 0x68, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x79, 0x20,
    0x28, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x29, 0x20, 0x61, 0x63, 0x63, 0x6f, 0x72, 0x64, 0x69, 0x6e,
    0x67, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x48, 0x65, 0x61, 0x6c, 0x74, 0x68, 0x43,
    0x68, 0x65, 0x63, 0x6b, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x69, 0x6e, 0x0a, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x2e,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x09, 0x04, 0x12, 0x04, 0x9b, 0x05, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x09, 0x05, 0x12, 0x04, 0x9b, 0x05, 0x0b, 0x0f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x09, 0x01, 0x12, 0x04, 0x9b, 0x05, 0x10, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x09, 0x03, 0x12, 0x04, 0x9b, 0x05, 0x1a, 0x1b, 0x0a, 0x95, 0x01,
    0x0a, 0x02, 0x04, 0x16, 0x12, 0x06, 0xa3, 0x05, 0x00, 0xab, 0x05, 0x01, 0x1a, 0x86, 0x01, 0x2a,
    0x0a, 0x20, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x73, 0x20, 0x70, 0x6f, 0x73, 0x73,
    0x69, 0x62, 0x6c, 0x65, 0x20, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x20, 0x74, 0x68, 0x61,
    0x74, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x65, 0x64,
    0x20, 0x74, 0x6f, 0x20, 0x75, 0x6e, 0x75, 0x73, 0x65, 0x64, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75,
    0x72, 0x63, 0x65, 0x73, 0x0a, 0x20, 0x28, 0x73, 0x65, 0x65, 0x20, 0x53, 0x63, 0x68, 0x65, 0x64,
    0x75, 0x6c, 0x65, 0x72, 0x44, 0x72, 0x69, 0x76, 0x65, 0x72, 0x3a, 0x3a, 0x6c, 0x61, 0x75, 0x6e,
    0x63, 0x68, 0x54, 0x61, 0x73, 0x6b, 0x73, 0x29, 0x20, 0x74, 0x6f, 0x20, 0x69, 0x6e, 0x66, 0x6c,
    0x75, 0x65, 0x6e, 0x63, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x6c, 0x6c, 0x6f, 0x63, 0x61,
    0x74, 0x6f, 0x72, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x16, 0x01, 0x12, 0x04, 0xa3, 0x05,
    0x08, 0x0f, 0x0a, 0x84, 0x03, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x00, 0x12, 0x04, 0xaa, 0x05, 0x02,
    0x35, 0x1a, 0xf5, 0x02, 0x20, 0x54, 0x69, 0x6d, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x63, 0x6f, 0x6e,
    0x73, 0x69, 0x64, 0x65, 0x72, 0x20, 0x75, 0x6e, 0x75, 0x73, 0x65, 0x64, 0x20, 0x72, 0x65, 0x73,
    0x6f, 0x75, 0x72, 0x63, 0x65, 0x73, 0x20, 0x72, 0x65, 0x66, 0x75, 0x73, 0x65, 0x64, 0x2e, 0x20,
    0x4e, 0x6f, 0x74, 0x65, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x75, 0x6e,
    0x75, 0x73, 0x65, 0x64, 0x0a, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x73, 0x20,
    0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x73, 0x69, 0x64, 0x65, 0x72,
    0x65, 0x64, 0x20, 0x72, 0x65, 0x66, 0x75, 0x73, 0x65, 0x64, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x75,
    0x73, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x0a, 0x20, 0x28, 0x62, 0x65, 0x6c, 0x6f, 0x77, 0x29, 0x20, 0x72, 0x65,
    0x67, 0x61, 0x72, 0x64, 0x6c, 0x65, 0x73, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x77, 0x68, 0x65, 0x74,
    0x68, 0x65, 0x72, 0x20, 0x46, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x20, 0x77, 0x61, 0x73, 0x20,
    0x70, 0x61, 0x73, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x0a, 0x20, 0x53, 0x63, 0x68, 0x65, 0x64,
    0x75, 0x6c, 0x65, 0x72, 0x44, 0x72, 0x69, 0x76, 0x65, 0x72, 0x3a, 0x3a, 0x6c, 0x61, 0x75, 0x6e,
    0x63, 0x68, 0x54, 0x61, 0x73, 0x6b, 0x73, 0x2e, 0x20, 0x59, 0x6f, 0x75, 0x20, 0x4d, 0x55, 0x53,
    0x54, 0x20, 0x70, 0x61, 0x73, 0x73, 0x20, 0x46, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x73, 0x20, 0x77,
    0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x69, 0x73, 0x0a, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20,
    0x73, 0x65, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x74, 0x68,
    0x69, 0x73, 0x20, 0x62, 0x65, 0x68, 0x61, 0x76, 0x69, 0x6f, 0x72, 0x20, 0x28, 0x69, 0x2e, 0x65,
    0x2e, 0x2c, 0x20, 0x67, 0x65, 0x74, 0x20, 0x61, 0x6e, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x20, 0x6f,
    0x66, 0x66, 0x65, 0x72, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x0a, 0x20, 0x69, 0x6e, 0x63, 0x6c,
    0x75, 0x64, 0x65, 0x73, 0x20, 0x75, 0x6e, 0x75, 0x73, 0x65, 0x64, 0x20, 0x72, 0x65, 0x73, 0x6f,
    0x75, 0x72, 0x63, 0x65, 0x73, 0x20, 0x73, 0x6f, 0x6f, 0x6e, 0x65, 0x72, 0x20, 0x6f, 0x72, 0x20,
    0x6c, 0x61, 0x74, 0x65, 0x72, 0x20, 0x74, 0x68, 0x61, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64,
    0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x29, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xaa, 0x05, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00,
    0x05, 0x12, 0x04, 0xaa, 0x05, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xaa, 0x05, 0x12, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x03, 0x12,
    0x04, 0xaa, 0x05, 0x23, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x08, 0x12, 0x04,
    0xaa, 0x05, 0x25, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x07, 0x12, 0x04, 0xaa,
    0x05, 0x30, 0x33, 0x0a, 0xa3, 0x01, 0x0a, 0x02, 0x04, 0x17, 0x12, 0x06, 0xb3, 0x05, 0x00, 0xba,
    0x05, 0x01, 0x1a, 0x94, 0x01, 0x2a, 0x0a, 0x20, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65,
    0x73, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f,
    0x66, 0x20, 0x65, 0x6e, 0x76, 0x69, 0x72, 0x6f, 0x6e, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x76, 0x61,
    0x72, 0x69, 0x61, 0x62, 0x6c, 0x65, 0x73, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73,
    0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x0a, 0x20, 0x43, 0x6f, 0x6d, 0x6d,
    0x61, 0x6e, 0x64, 0x49, 0x6e, 0x66, 0x6f, 0x20, 0x69, 0x6e, 0x20, 0x6f, 0x72, 0x64, 0x65, 0x72,
    0x20, 0x74, 0x6f, 0x20, 0x73, 0x65, 0x74, 0x20, 0x65, 0x6e, 0x76, 0x69, 0x72, 0x6f, 0x6e, 0x6d,
    0x65, 0x6e, 0x74, 0x20, 0x76, 0x61, 0x72, 0x69, 0x61, 0x62, 0x6c, 0x65, 0x73, 0x20, 0x62, 0x65,
    0x66, 0x6f, 0x72, 0x65, 0x20, 0x72, 0x75, 0x6e, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x0a, 0x20,
    0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x17, 0x01,
    0x12, 0x04, 0xb3, 0x05, 0x08, 0x13, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x17, 0x03, 0x00, 0x12, 0x06,
    0xb4, 0x05, 0x02, 0xb7, 0x05, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x03, 0x00, 0x01, 0x12,
    0x04, 0xb4, 0x05, 0x0a, 0x12, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x17, 0x03, 0x00, 0x02, 0x00, 0x12,
    0x04, 0xb5, 0x05, 0x04, 0x1d, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x17, 0x03, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x04, 0xb5, 0x05, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x17, 0x03, 0x00, 0x02, 0x00,
    0x05, 0x12, 0x04, 0xb5, 0x05, 0x0d, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x17, 0x03, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xb5, 0x05, 0x14, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x17, 0x03, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xb5, 0x05, 0x1b, 0x1c, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x17, 0x03,
    0x00, 0x02, 0x01, 0x12, 0x04, 0xb6, 0x05, 0x04, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x17, 0x03,
    0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0xb6, 0x05, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x17,
    0x03, 0x00, 0x02, 0x01, 0x05, 0x12, 0x04, 0xb6, 0x05, 0x0d, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x17, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xb6, 0x05, 0x14, 0x19, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x17, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0xb6, 0x05, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x17, 0x02, 0x00, 0x12, 0x04, 0xb9, 0x05, 0x02, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x17, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb9, 0x05, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17,
    0x02, 0x00, 0x06, 0x12, 0x04, 0xb9, 0x05, 0x0b, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xb9, 0x05, 0x14, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xb9, 0x05, 0x20, 0x21, 0x0a, 0x54, 0x0a, 0x02, 0x04, 0x18, 0x12, 0x06, 0xc0,
    0x05, 0x00, 0xc3, 0x05, 0x01, 0x1a, 0x46, 0x2a, 0x0a, 0x20, 0x41, 0x20, 0x67, 0x65, 0x6e, 0x65,
    0x72, 0x69, 0x63, 0x20, 0x28, 0x6b, 0x65, 0x79, 0x2c, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x29,
    0x20, 0x70, 0x61, 0x69, 0x72, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x76, 0x61,
    0x72, 0x69, 0x6f, 0x75, 0x73, 0x20, 0x70, 0x6c, 0x61, 0x63, 0x65, 0x73, 0x20, 0x66, 0x6f, 0x72,
    0x20, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x18, 0x01, 0x12, 0x04, 0xc0, 0x05, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18,
    0x02, 0x00, 0x12, 0x04, 0xc1, 0x05, 0x02, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xc1, 0x05, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x05,
    0x12, 0x04, 0xc1, 0x05, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xc1, 0x05, 0x12, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xc1, 0x05, 0x18, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x01, 0x12, 0x04, 0xc2, 0x05,
    0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x04, 0x12, 0x04, 0xc2, 0x05, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x05, 0x12, 0x04, 0xc2, 0x05, 0x0b, 0x11,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc2, 0x05, 0x12, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x03, 0x12, 0x04, 0xc2, 0x05, 0x1a, 0x1b, 0x0a, 0x2a,
    0x0a, 0x02, 0x04, 0x19, 0x12, 0x06, 0xc9, 0x05, 0x00, 0xcb, 0x05, 0x01, 0x1a, 0x1c, 0x2a, 0x0a,
    0x20, 0x43, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x50,
    0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x19,
    0x01, 0x12, 0x04, 0xc9, 0x05, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x00, 0x12,
    0x04, 0xca, 0x05, 0x02, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xca, 0x05, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x06, 0x12, 0x04, 0xca,
    0x05, 0x0b, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x01, 0x12, 0x04, 0xca, 0x05,
    0x15, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x03, 0x12, 0x04, 0xca, 0x05, 0x21,
    0x22, 0x0a, 0xc0, 0x02, 0x0a, 0x02, 0x04, 0x1a, 0x12, 0x06, 0xd7, 0x05, 0x00, 0xda, 0x05, 0x01,
    0x1a, 0xb1, 0x02, 0x2a, 0x0a, 0x20, 0x43, 0x72, 0x65, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x61, 0x6c,
    0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x76, 0x61, 0x72, 0x69, 0x6f, 0x75, 0x73,
    0x20, 0x70, 0x6c, 0x61, 0x63, 0x65, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x75, 0x74, 0x68,
    0x65, 0x6e, 0x74, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x61, 0x6e, 0x64, 0x0a, 0x20,
    0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a,
    0x20, 0x4e, 0x4f, 0x54, 0x45, 0x3a, 0x20, 0x41, 0x20, 0x27, 0x70, 0x72, 0x69, 0x6e, 0x63, 0x69,
    0x70, 0x61, 0x6c, 0x27, 0x20, 0x69, 0x73, 0x20, 0x64, 0x69, 0x66, 0x66, 0x65, 0x72, 0x65, 0x6e,
    0x74, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x27, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72,
    0x6b, 0x49, 0x6e, 0x66, 0x6f, 0x2e, 0x75, 0x73, 0x65, 0x72, 0x27, 0x2e, 0x20, 0x54, 0x68, 0x65,
    0x0a, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x65, 0x72, 0x20, 0x69, 0x73, 0x20, 0x75, 0x73, 0x65, 0x64,
    0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x75, 0x74, 0x68, 0x65, 0x6e, 0x74, 0x69, 0x63, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x77, 0x68, 0x69, 0x6c, 0x65, 0x20, 0x74, 0x68, 0x65, 0x0a,
    0x20, 0x6c, 0x61, 0x74, 0x74, 0x65, 0x72, 0x20, 0x69, 0x73, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20,
    0x74, 0x6f, 0x20, 0x64, 0x65, 0x74, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x65, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20, 0x75, 0x73, 0x65, 0x72, 0x20, 0x75, 0x6e,
    0x64, 0x65, 0x72, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x66,
    0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x27, 0x73, 0x20, 0x65, 0x78, 0x65, 0x63, 0x75,
    0x74, 0x6f, 0x72, 0x73, 0x2f, 0x74, 0x61, 0x73, 0x6b, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x72,
    0x75, 0x6e, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1a, 0x01, 0x12, 0x04, 0xd7, 0x05, 0x08,
    0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x00, 0x12, 0x04, 0xd8, 0x05, 0x02, 0x20, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd8, 0x05, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x05, 0x12, 0x04, 0xd8, 0x05, 0x0b, 0x11, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1a, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd8, 0x05, 0x12, 0x1b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1a, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd8, 0x05, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x1a, 0x02, 0x01, 0x12, 0x04, 0xd9, 0x05, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xd9, 0x05, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01,
    0x05, 0x12, 0x04, 0xd9, 0x05, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xd9, 0x05, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xd9, 0x05, 0x1a, 0x1b, 0x0a, 0xc0, 0x01, 0x0a, 0x02, 0x04, 0x1b, 0x12, 0x06, 0xe2, 0x05,
    0x00, 0xe4, 0x05, 0x01, 0x1a, 0xb1, 0x01, 0x2a, 0x0a, 0x20, 0x43, 0x72, 0x65, 0x64, 0x65, 0x6e,
    0x74, 0x69, 0x61, 0x6c, 0x73, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x66,
    0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x20, 0x61, 0x75, 0x74, 0x68, 0x65, 0x6e, 0x74,
    0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2c, 0x20, 0x48, 0x54, 0x54, 0x50, 0x20, 0x61, 0x75,
    0x74, 0x68, 0x65, 0x6e, 0x74, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x20, 0x28, 0x77,
    0x68, 0x65, 0x72, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x20,
    0x27, 0x75, 0x73, 0x65, 0x72, 0x6e, 0x61, 0x6d, 0x65, 0x27, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x27,
    0x70, 0x61, 0x73, 0x73, 0x77, 0x6f, 0x72, 0x64, 0x27, 0x20, 0x61, 0x72, 0x65, 0x20, 0x63, 0x61,
    0x70, 0x74, 0x75, 0x72, 0x65, 0x64, 0x20, 0x61, 0x73, 0x0a, 0x20, 0x27, 0x70, 0x72, 0x69, 0x6e,
    0x63, 0x69, 0x70, 0x61, 0x6c, 0x27, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x27, 0x73, 0x65, 0x63, 0x72,
    0x65, 0x74, 0x27, 0x20, 0x72, 0x65, 0x73, 0x70, 0x65, 0x63, 0x74, 0x69, 0x76, 0x65, 0x6c, 0x79,
    0x29, 0x2c, 0x20, 0x65, 0x74, 0x63, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1b, 0x01, 0x12,
    0x04, 0xe2, 0x05, 0x08, 0x13, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x00, 0x12, 0x04, 0xe3,
    0x05, 0x02, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe3, 0x05,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x06, 0x12, 0x04, 0xe3, 0x05, 0x0b,
    0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe3, 0x05, 0x16, 0x21,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe3, 0x05, 0x24, 0x25, 0x0a,
    0x2e, 0x0a, 0x02, 0x04, 0x1c, 0x12, 0x06, 0xea, 0x05, 0x00, 0x94, 0x06, 0x01, 0x1a, 0x20, 0x2a,
    0x0a, 0x20, 0x41, 0x43, 0x4c, 0x73, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20,
    0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x1c, 0x01, 0x12, 0x04, 0xea, 0x05, 0x08, 0x0b, 0x0a, 0xcf, 0x01, 0x0a,
    0x04, 0x04, 0x1c, 0x03, 0x00, 0x12, 0x06, 0xf0, 0x05, 0x02, 0xf8, 0x05, 0x03, 0x1a, 0xbe, 0x01,
    0x20, 0x45, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x20, 0x69, 0x73, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20,
    0x74, 0x6f, 0x20, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x20, 0x61, 0x20, 0x73, 0x75,
    0x62, 0x6a, 0x65, 0x63, 0x74, 0x28, 0x73, 0x29, 0x20, 0x6f, 0x72, 0x20, 0x61, 0x6e, 0x20, 0x6f,
    0x62, 0x6a, 0x65, 0x63, 0x74, 0x28, 0x73, 0x29, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x6e, 0x20, 0x41,
    0x43, 0x4c, 0x2e, 0x0a, 0x20, 0x4e, 0x4f, 0x54, 0x45, 0x3a, 0x0a, 0x20, 0x54, 0x6f, 0x20, 0x61,
    0x6c, 0x6c, 0x6f, 0x77, 0x20, 0x65, 0x76, 0x65, 0x72, 0x79, 0x6f, 0x6e, 0x65, 0x20, 0x61, 0x63,
    0x63, 0x65, 0x73, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x6e, 0x20, 0x45, 0x6e, 0x74, 0x69, 0x74,
    0x79, 0x20, 0x73, 0x65, 0x74, 0x20, 0x69, 0x74, 0x73, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x74,
    0x6f, 0x20, 0x27, 0x41, 0x4e, 0x59, 0x27, 0x2e, 0x0a, 0x20, 0x54, 0x6f, 0x20, 0x64, 0x65, 0x6e,
    0x79, 0x20, 0x61, 0x63, 0x63, 0x65, 0x73, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x6e, 0x20, 0x45,
    0x6e, 0x74, 0x69, 0x74, 0x79, 0x20, 0x73, 0x65, 0x74, 0x20, 0x69, 0x74, 0x73, 0x20, 0x74, 0x79,
    0x70, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x27, 0x4e, 0x4f, 0x4e, 0x45, 0x27, 0x2e, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1c, 0x03, 0x00, 0x01, 0x12, 0x04, 0xf0, 0x05, 0x0a, 0x10, 0x0a, 0x10, 0x0a,
    0x06, 0x04, 0x1c, 0x03, 0x00, 0x04, 0x00, 0x12, 0x06, 0xf1, 0x05, 0x04, 0xf5, 0x05, 0x05, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03, 0x00, 0x04, 0x00, 0x01, 0x12, 0x04, 0xf1, 0x05, 0x09, 0x0d,
    0x0a, 0x10, 0x0a, 0x08, 0x04, 0x1c, 0x03, 0x00, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0xf2, 0x05,
    0x06, 0x0f, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x1c, 0x03, 0x00, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xf2, 0x05, 0x06, 0x0a, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x1c, 0x03, 0x00, 0x04, 0x00, 0x02,
    0x00, 0x02, 0x12, 0x04, 0xf2, 0x05, 0x0d, 0x0e, 0x0a, 0x10, 0x0a, 0x08, 0x04, 0x1c, 0x03, 0x00,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0xf3, 0x05, 0x06, 0x0e, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x1c,
    0x03, 0x00, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xf3, 0x05, 0x06, 0x09, 0x0a, 0x11, 0x0a,
    0x09, 0x04, 0x1c, 0x03, 0x00, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xf3, 0x05, 0x0c, 0x0d,
    0x0a, 0x10, 0x0a, 0x08, 0x04, 0x1c, 0x03, 0x00, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0xf4, 0x05,
    0x06, 0x0f, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x1c, 0x03, 0x00, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12,
    0x04, 0xf4, 0x05, 0x06, 0x0a, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x1c, 0x03, 0x00, 0x04, 0x00, 0x02,
    0x02, 0x02, 0x12, 0x04, 0xf4, 0x05, 0x0d, 0x0e, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1c, 0x03, 0x00,
    0x02, 0x00, 0x12, 0x04, 0xf6, 0x05, 0x04, 0x2c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03, 0x00,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xf6, 0x05, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03,
    0x00, 0x02, 0x00, 0x06, 0x12, 0x04, 0xf6, 0x05, 0x0d, 0x11, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c,
    0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xf6, 0x05, 0x12, 0x16, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x1c, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0xf6, 0x05, 0x19, 0x1a, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x1c, 0x03, 0x00, 0x02, 0x00, 0x08, 0x12, 0x04, 0xf6, 0x05, 0x1b, 0x2b, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x1c, 0x03, 0x00, 0x02, 0x00, 0x07, 0x12, 0x04, 0xf6, 0x05, 0x26, 0x2a, 0x0a, 0x27,
    0x0a, 0x06, 0x04, 0x1c, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0xf7, 0x05, 0x04, 0x1f, 0x22, 0x17,
    0x20, 0x49, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x41, 0x4e, 0x59,
    0x2f, 0x4e, 0x4f, 0x4e, 0x45, 0x2e, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xf7, 0x05, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03, 0x00,
    0x02, 0x01, 0x05, 0x12, 0x04, 0xf7, 0x05, 0x0d, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xf7, 0x05, 0x14, 0x1a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c,
    0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0xf7, 0x05, 0x1d, 0x1e, 0x0a, 0x17, 0x0a, 0x04, 0x04,
    0x1c, 0x03, 0x01, 0x12, 0x06, 0xfb, 0x05, 0x02, 0x81, 0x06, 0x03, 0x1a, 0x07, 0x20, 0x41, 0x43,
    0x4c, 0x73, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x03, 0x01, 0x01, 0x12, 0x04, 0xfb,
    0x05, 0x0a, 0x1b, 0x0a, 0x34, 0x0a, 0x06, 0x04, 0x1c, 0x03, 0x01, 0x02, 0x00, 0x12, 0x04, 0xfd,
    0x05, 0x04, 0x23, 0x1a, 0x0b, 0x20, 0x53, 0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x73, 0x2e, 0x0a,
    0x22, 0x17, 0x20, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x20, 0x70, 0x72, 0x69,
    0x6e, 0x63, 0x69, 0x70, 0x61, 0x6c, 0x73, 0x2e, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03,
    0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0xfd, 0x05, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c,
    0x03, 0x01, 0x02, 0x00, 0x06, 0x12, 0x04, 0xfd, 0x05, 0x0d, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x1c, 0x03, 0x01, 0x02, 0x00, 0x01, 0x12, 0x04, 0xfd, 0x05, 0x14, 0x1e, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x1c, 0x03, 0x01, 0x02, 0x00, 0x03, 0x12, 0x04, 0xfd, 0x05, 0x21, 0x22, 0x0a, 0x38, 0x0a,
    0x06, 0x04, 0x1c, 0x03, 0x01, 0x02, 0x01, 0x12, 0x04, 0x80, 0x06, 0x04, 0x1e, 0x1a, 0x0a, 0x20,
    0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x73, 0x2e, 0x0a, 0x22, 0x1c, 0x20, 0x52, 0x6f, 0x6c, 0x65,
    0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x20, 0x6f,
    0x66, 0x66, 0x65, 0x72, 0x73, 0x2e, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03, 0x01, 0x02,
    0x01, 0x04, 0x12, 0x04, 0x80, 0x06, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03, 0x01,
    0x02, 0x01, 0x06, 0x12, 0x04, 0x80, 0x06, 0x0d, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03,
    0x01, 0x02, 0x01, 0x01, 0x12, 0x04, 0x80, 0x06, 0x14, 0x19, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c,
    0x03, 0x01, 0x02, 0x01, 0x03, 0x12, 0x04, 0x80, 0x06, 0x1c, 0x1d, 0x0a, 0x0e, 0x0a, 0x04, 0x04,
    0x1c, 0x03, 0x02, 0x12, 0x06, 0x83, 0x06, 0x02, 0x89, 0x06, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1c, 0x03, 0x02, 0x01, 0x12, 0x04, 0x83, 0x06, 0x0a, 0x11, 0x0a, 0x34, 0x0a, 0x06, 0x04, 0x1c,
    0x03, 0x02, 0x02, 0x00, 0x12, 0x04, 0x85, 0x06, 0x04, 0x23, 0x1a, 0x0b, 0x20, 0x53, 0x75, 0x62,
    0x6a, 0x65, 0x63, 0x74, 0x73, 0x2e, 0x0a, 0x22, 0x17, 0x20, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77,
    0x6f, 0x72, 0x6b, 0x20, 0x70, 0x72, 0x69, 0x6e, 0x63, 0x69, 0x70, 0x61, 0x6c, 0x73, 0x2e, 0x0a,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03, 0x02, 0x02, 0x00, 0x04, 0x12, 0x04, 0x85, 0x06, 0x04,
    0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03, 0x02, 0x02, 0x00, 0x06, 0x12, 0x04, 0x85, 0x06,
    0x0d, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03, 0x02, 0x02, 0x00, 0x01, 0x12, 0x04, 0x85,
    0x06, 0x14, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03, 0x02, 0x02, 0x00, 0x03, 0x12, 0x04,
    0x85, 0x06, 0x21, 0x22, 0x0a, 0x42, 0x0a, 0x06, 0x04, 0x1c, 0x03, 0x02, 0x02, 0x01, 0x12, 0x04,
    0x88, 0x06, 0x04, 0x1e, 0x1a, 0x0a, 0x20, 0x4f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x73, 0x2e, 0x0a,
    0x22, 0x26, 0x20, 0x55, 0x73, 0x65, 0x72, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x75, 0x6e, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x74, 0x61, 0x73, 0x6b, 0x73, 0x2f, 0x65, 0x78, 0x65, 0x63, 0x75, 0x74,
    0x6f, 0x72, 0x73, 0x20, 0x61, 0x73, 0x2e, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03, 0x02,
    0x02, 0x01, 0x04, 0x12, 0x04, 0x88, 0x06, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03,
    0x02, 0x02, 0x01, 0x06, 0x12, 0x04, 0x88, 0x06, 0x0d, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c,
    0x03, 0x02, 0x02, 0x01, 0x01, 0x12, 0x04, 0x88, 0x06, 0x14, 0x19, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x1c, 0x03, 0x02, 0x02, 0x01, 0x03, 0x12, 0x04, 0x88, 0x06, 0x1c, 0x1d, 0x0a, 0x5e, 0x0a, 0x04,
    0x04, 0x1c, 0x03, 0x03, 0x12, 0x06, 0x8d, 0x06, 0x02, 0x93, 0x06, 0x03, 0x1a, 0x4e, 0x20, 0x57,
    0x68, 0x69, 0x63, 0x68, 0x20, 0x70, 0x72, 0x69, 0x6e, 0x63, 0x69, 0x70, 0x61, 0x6c, 0x73, 0x20,
    0x61, 0x72, 0x65, 0x20, 0x61, 0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x7a, 0x65, 0x64, 0x20, 0x74,
    0x6f, 0x20, 0x73, 0x68, 0x75, 0x74, 0x64, 0x6f, 0x77, 0x6e, 0x20, 0x66, 0x72, 0x61, 0x6d, 0x65,
    0x77, 0x6f, 0x72, 0x6b, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x0a, 0x20,
    0x70, 0x72, 0x69, 0x6e, 0x63, 0x69, 0x70, 0x61, 0x6c, 0x73, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1c, 0x03, 0x03, 0x01, 0x12, 0x04, 0x8d, 0x06, 0x0a, 0x1b, 0x0a, 0x1b, 0x0a, 0x06, 0x04,
    0x1c, 0x03, 0x03, 0x02, 0x00, 0x12, 0x04, 0x8f, 0x06, 0x04, 0x23, 0x1a, 0x0b, 0x20, 0x53, 0x75,
    0x62, 0x6a, 0x65, 0x63, 0x74, 0x73, 0x2e, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03, 0x03,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x8f, 0x06, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03,
    0x03, 0x02, 0x00, 0x06, 0x12, 0x04, 0x8f, 0x06, 0x0d, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c,
    0x03, 0x03, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8f, 0x06, 0x14, 0x1e, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x1c, 0x03, 0x03, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8f, 0x06, 0x21, 0x22, 0x0a, 0x1a, 0x0a, 0x06,
    0x04, 0x1c, 0x03, 0x03, 0x02, 0x01, 0x12, 0x04, 0x92, 0x06, 0x04, 0x2d, 0x1a, 0x0a, 0x20, 0x4f,
    0x62, 0x6a, 0x65, 0x63, 0x74, 0x73, 0x2e, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03, 0x03,
    0x02, 0x01, 0x04, 0x12, 0x04, 0x92, 0x06, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c, 0x03,
    0x03, 0x02, 0x01, 0x06, 0x12, 0x04, 0x92, 0x06, 0x0d, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1c,
    0x03, 0x03, 0x02, 0x01, 0x01, 0x12, 0x04, 0x92, 0x06, 0x14, 0x28, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x1c, 0x03, 0x03, 0x02, 0x01, 0x03, 0x12, 0x04, 0x92, 0x06, 0x2b, 0x2c, 0x0a, 0x97, 0x05, 0x0a,
    0x02, 0x04, 0x1d, 0x12, 0x06, 0xaa, 0x06, 0x00, 0xaf, 0x06, 0x01, 0x1a, 0x88, 0x05, 0x2a, 0x0a,
    0x20, 0x43, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x41,
    0x43, 0x4c, 0x2e, 0x0a, 0x0a, 0x20, 0x45, 0x61, 0x63, 0x68, 0x20, 0x61, 0x75, 0x74, 0x68, 0x6f,
    0x72, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x20, 0x69, 0x73, 0x20, 0x65, 0x76, 0x61, 0x6c, 0x75, 0x61, 0x74, 0x65, 0x64, 0x20, 0x61, 0x67,
    0x61, 0x69, 0x6e, 0x73, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x41, 0x43, 0x4c, 0x73, 0x20, 0x69,
    0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x0a, 0x20, 0x74, 0x68, 0x65,
    0x79, 0x20, 0x61, 0x72, 0x65, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x64, 0x2e, 0x0a, 0x0a,
    0x20, 0x46, 0x6f, 0x72, 0x20, 0x73, 0x69, 0x6d, 0x70, 0x6c, 0x69, 0x63, 0x69, 0x74, 0x79, 0x2c,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x41, 0x43, 0x4c, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x20,
    0x67, 0x69, 0x76, 0x65, 0x6e, 0x20, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x61, 0x72, 0x65,
    0x20, 0x6e, 0x6f, 0x74, 0x20, 0x61, 0x67, 0x67, 0x72, 0x65, 0x67, 0x61, 0x74, 0x65, 0x64, 0x20,
    0x65, 0x76, 0x65, 0x6e, 0x0a, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x79, 0x20,
    0x68, 0x61, 0x76, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x61, 0x6d, 0x65, 0x20, 0x73, 0x75,
    0x62, 0x6a, 0x65, 0x63, 0x74, 0x73, 0x20, 0x6f, 0x72, 0x20, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74,
    0x73, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x20, 0x41, 0x43, 0x4c,
    0x20, 0x74, 0x68, 0x61, 0x74, 0x0a, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x65, 0x73, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x64, 0x65, 0x74, 0x65, 0x72,
    0x6d, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x77, 0x68, 0x65, 0x74, 0x68, 0x65, 0x72, 0x20, 0x74, 0x68,
    0x61, 0x74, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c,
    0x64, 0x20, 0x62, 0x65, 0x0a, 0x20, 0x70, 0x65, 0x72, 0x6d, 0x69, 0x74, 0x74, 0x65, 0x64, 0x20,
    0x6f, 0x72, 0x20, 0x6e, 0x6f, 0x74, 0x2e, 0x20, 0x41, 0x6e, 0x20, 0x41, 0x43, 0x4c, 0x20, 0x6d,
    0x61, 0x74, 0x63, 0x68, 0x65, 0x73, 0x20, 0x69, 0x66, 0x66, 0x20, 0x62, 0x6f, 0x74, 0x68, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x73, 0x75, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x73, 0x0a, 0x20, 0x28, 0x65,
    0x2e, 0x67, 0x2e, 0x2c, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x73, 0x2c, 0x20, 0x70, 0x72,
    0x69, 0x6e, 0x63, 0x69, 0x70, 0x61, 0x6c, 0x73, 0x29, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x73, 0x20, 0x28, 0x65, 0x2e, 0x67, 0x2e, 0x2c,
    0x20, 0x75, 0x72, 0x6c, 0x73, 0x2c, 0x20, 0x75, 0x73, 0x65, 0x72, 0x73, 0x2c, 0x0a, 0x20, 0x72,
    0x6f, 0x6c, 0x65, 0x73, 0x29, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x41, 0x43, 0x4c,
    0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x2e, 0x0a, 0x0a, 0x20, 0x49, 0x66, 0x20, 0x6e, 0x6f, 0x6e, 0x65, 0x20, 0x6f, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x41, 0x43, 0x4c, 0x73, 0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2c, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x27, 0x70, 0x65, 0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x76, 0x65, 0x27, 0x20, 0x66, 0x69,
    0x65, 0x6c, 0x64, 0x0a, 0x20, 0x64, 0x65, 0x74, 0x65, 0x72, 0x6d, 0x69, 0x6e, 0x65, 0x73, 0x20,
    0x77, 0x68, 0x65, 0x74, 0x68, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x70, 0x65,
    0x72, 0x6d, 0x69, 0x74, 0x74, 0x65, 0x64, 0x20, 0x6f, 0x72, 0x20, 0x6e, 0x6f, 0x74, 0x2e, 0x0a,
    0x0a, 0x20, 0x54, 0x4f, 0x44, 0x4f, 0x28, 0x76, 0x69, 0x6e, 0x6f, 0x64, 0x29, 0x3a, 0x20, 0x44,
    0x6f, 0x20, 0x61, 0x67, 0x67, 0x72, 0x65, 0x67, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66,
    0x20, 0x41, 0x43, 0x4c, 0x73, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x70, 0x6f, 0x73, 0x73, 0x69,
    0x62, 0x6c, 0x65, 0x2e, 0x0a, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1d, 0x01, 0x12, 0x04, 0xaa,
    0x06, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x00, 0x12, 0x04, 0xab, 0x06, 0x02,
    0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x04, 0x12, 0x04, 0xab, 0x06, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x05, 0x12, 0x04, 0xab, 0x06, 0x0b, 0x0f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x01, 0x12, 0x04, 0xab, 0x06, 0x10, 0x1a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x03, 0x12, 0x04, 0xab, 0x06, 0x1d, 0x1e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1d, 0x02, 0x00, 0x08, 0x12, 0x04, 0xab, 0x06, 0x1f, 0x2f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1d, 0x02, 0x00, 0x07, 0x12, 0x04, 0xab, 0x06, 0x2a, 0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x1d, 0x02, 0x01, 0x12, 0x04, 0xac, 0x06, 0x02, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xac, 0x06, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x01,
    0x06, 0x12, 0x04, 0xac, 0x06, 0x0b, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xac, 0x06, 0x21, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xac, 0x06, 0x37, 0x38, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x02, 0x12, 0x04, 0xad,
    0x06, 0x02, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x02, 0x04, 0x12, 0x04, 0xad, 0x06,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x02, 0x06, 0x12, 0x04, 0xad, 0x06, 0x0b,
    0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x02, 0x01, 0x12, 0x04, 0xad, 0x06, 0x17, 0x20,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x02, 0x03, 0x12, 0x04, 0xad, 0x06, 0x23, 0x24, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x03, 0x12, 0x04, 0xae, 0x06, 0x02, 0x39, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1d, 0x02, 0x03, 0x04, 0x12, 0x04, 0xae, 0x06, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1d, 0x02, 0x03, 0x06, 0x12, 0x04, 0xae, 0x06, 0x0b, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1d, 0x02, 0x03, 0x01, 0x12, 0x04, 0xae, 0x06, 0x21, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d,
    0x02, 0x03, 0x03, 0x12, 0x04, 0xae, 0x06, 0x37, 0x38, 0x0a, 0xba, 0x01, 0x0a, 0x02, 0x04, 0x1e,
    0x12, 0x06, 0xb7, 0x06, 0x00, 0xc7, 0x06, 0x01, 0x1a, 0xab, 0x01, 0x2a, 0x0a, 0x20, 0x52, 0x61,
    0x74, 0x65, 0x20, 0x28, 0x71, 0x75, 0x65, 0x72, 0x69, 0x65, 0x73, 0x20, 0x70, 0x65, 0x72, 0x20,
    0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x2c, 0x20, 0x51, 0x50, 0x53, 0x29, 0x20, 0x6c, 0x69, 0x6d,
    0x69, 0x74, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x20,
    0x66, 0x72, 0x6f, 0x6d, 0x20, 0x61, 0x20, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b,
    0x20, 0x74, 0x6f, 0x20, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x2e, 0x0a, 0x20, 0x53, 0x74, 0x72,
    0x69, 0x63, 0x74, 0x6c, 0x79, 0x20, 0x73, 0x70, 0x65, 0x61, 0x6b, 0x69, 0x6e, 0x67, 0x20, 0x74,
    0x68, 0x65, 0x79, 0x20, 0x61, 0x72, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6d, 0x62,
    0x69, 0x6e, 0x65, 0x64, 0x20, 0x72, 0x61, 0x74, 0x65, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x61,
    0x6c, 0x6c, 0x20, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x73, 0x20, 0x6f, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x61, 0x6d, 0x65, 0x0a, 0x20, 0x70, 0x72, 0x69, 0x6e, 0x63,
    0x69, 0x70, 0x61, 0x6c, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1e, 0x01, 0x12, 0x04, 0xb7,
    0x06, 0x08, 0x11, 0x0a, 0x78, 0x0a, 0x04, 0x04, 0x1e, 0x02, 0x00, 0x12, 0x04, 0xba, 0x06, 0x02,
    0x1a, 0x1a, 0x6a, 0x20, 0x4c, 0x65, 0x61, 0x76, 0x69, 0x6e, 0x67, 0x20, 0x51, 0x50, 0x53, 0x20,
    0x75, 0x6e, 0x73, 0x65, 0x74, 0x20, 0x67, 0x69, 0x76, 0x65, 0x73, 0x20, 0x69, 0x74, 0x20, 0x75,
    0x6e, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x65, 0x64, 0x20, 0x72, 0x61, 0x74, 0x65, 0x20, 0x28, 0x69,
    0x2e, 0x65, 0x2e, 0x2c, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x74, 0x68, 0x72, 0x6f, 0x74, 0x74, 0x6c,
    0x65, 0x64, 0x29, 0x2c, 0x0a, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x61, 0x6c, 0x73, 0x6f,
    0x20, 0x69, 0x6d, 0x70, 0x6c, 0x69, 0x65, 0x73, 0x20, 0x75, 0x6e, 0x6c, 0x69, 0x6d, 0x69, 0x74,
    0x65, 0x64, 0x20, 0x63, 0x61, 0x70, 0x61, 0x63, 0x69, 0x74, 0x79, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1e, 0x02, 0x00, 0x04, 0x12, 0x04, 0xba, 0x06, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1e, 0x02, 0x00, 0x05, 0x12, 0x04, 0xba, 0x06, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1e, 0x02, 0x00, 0x01, 0x12, 0x04, 0xba, 0x06, 0x12, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xba, 0x06, 0x18, 0x19, 0x0a, 0x94, 0x01, 0x0a, 0x04, 0x04, 0x1e,
    0x02, 0x01, 0x12, 0x04, 0xbe, 0x06, 0x02, 0x20, 0x1a, 0x85, 0x01, 0x20, 0x50, 0x72, 0x69, 0x6e,
    0x63, 0x69, 0x70, 0x61, 0x6c, 0x20, 0x6f, 0x66, 0x20, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f,
    0x72, 0x6b, 0x28, 0x73, 0x29, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x74, 0x68, 0x72, 0x6f,
    0x74, 0x74, 0x6c, 0x65, 0x64, 0x2e, 0x20, 0x53, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x6d, 0x61,
    0x74, 0x63, 0x68, 0x0a, 0x20, 0x46, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x49, 0x6e,
    0x66, 0x6f, 0x2e, 0x70, 0x72, 0x69, 0x6e, 0x63, 0x70, 0x61, 0x6c, 0x20, 0x61, 0x6e, 0x64, 0x20,
    0x43, 0x72, 0x65, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x61, 0x6c, 0x2e, 0x70, 0x72, 0x69, 0x6e, 0x63,
    0x69, 0x70, 0x61, 0x6c, 0x20, 0x28, 0x69, 0x66, 0x20, 0x75, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x61,
    0x75, 0x74, 0x68, 0x65, 0x6e, 0x74, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x29, 0x2e, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x04, 0x12, 0x04, 0xbe, 0x06, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x05, 0x12, 0x04, 0xbe, 0x06, 0x0b, 0x11, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x01, 0x12, 0x04, 0xbe, 0x06, 0x12, 0x1b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1e, 0x02, 0x01, 0x03, 0x12, 0x04, 0xbe, 0x06, 0x1e, 0x1f, 0x0a, 0x92, 0x03, 0x0a,
    0x04, 0x04, 0x1e, 0x02, 0x02, 0x12, 0x04, 0xc6, 0x06, 0x02, 0x1f, 0x1a, 0x83, 0x03, 0x20, 0x4d,
    0x61, 0x78, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x6f, 0x75, 0x74,
    0x73, 0x74, 0x61, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x73, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b,
    0x73, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x70, 0x72, 0x69, 0x6e, 0x63, 0x69,
    0x70, 0x61, 0x6c, 0x0a, 0x20, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20,
    0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x20, 0x62, 0x65, 0x66, 0x6f, 0x72, 0x65, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x6e, 0x65, 0x78, 0x74, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x69,
    0x73, 0x20, 0x64, 0x72, 0x6f, 0x70, 0x70, 0x65, 0x64, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x61, 0x6e,
    0x20, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x69, 0x73, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x0a, 0x20,
    0x62, 0x61, 0x63, 0x6b, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x6e, 0x64,
    0x65, 0x72, 0x2e, 0x20, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x20, 0x72, 0x65, 0x63,
    0x65, 0x69, 0x76, 0x65, 0x64, 0x20, 0x62, 0x65, 0x66, 0x6f, 0x72, 0x65, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x63, 0x61, 0x70, 0x61, 0x63, 0x69, 0x74, 0x79, 0x20, 0x69, 0x73, 0x20, 0x72, 0x65, 0x61,
    0x63, 0x68, 0x65, 0x64, 0x20, 0x61, 0x72, 0x65, 0x0a, 0x20, 0x73, 0x74, 0x69, 0x6c, 0x6c, 0x20,
    0x67, 0x6f, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x70, 0x72, 0x6f, 0x63,
    0x65, 0x73, 0x73, 0x65, 0x64, 0x20, 0x61, 0x66, 0x74, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x65, 0x72, 0x72, 0x6f, 0x72, 0x20, 0x69, 0x73, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x2e, 0x0a, 0x20,
    0x49, 0x66, 0x20, 0x75, 0x6e, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x2c, 0x20,
    0x74, 0x68, 0x69, 0x73, 0x20, 0x70, 0x72, 0x69, 0x6e, 0x63, 0x69, 0x70, 0x61, 0x6c, 0x20, 0x69,
    0x73, 0x20, 0x61, 0x73, 0x73, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x20, 0x75, 0x6e, 0x6c, 0x69, 0x6d,
    0x69, 0x74, 0x65, 0x64, 0x20, 0x63, 0x61, 0x70, 0x61, 0x63, 0x69, 0x74, 0x79, 0x2e, 0x0a, 0x20,
    0x4e, 0x4f, 0x54, 0x45, 0x3a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65,
    0x20, 0x69, 0x73, 0x20, 0x69, 0x67, 0x6e, 0x6f, 0x72, 0x65, 0x64, 0x20, 0x69, 0x66, 0x20, 0x27,
    0x71, 0x70, 0x73, 0x27, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x65, 0x74, 0x2e,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x02, 0x04, 0x12, 0x04, 0xc6, 0x06, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x02, 0x05, 0x12, 0x04, 0xc6, 0x06, 0x0b, 0x11, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x02, 0x01, 0x12, 0x04, 0xc6, 0x06, 0x12, 0x1a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1e, 0x02, 0x02, 0x03, 0x12, 0x04, 0xc6, 0x06, 0x1d, 0x1e, 0x0a, 0x98, 0x01,
    0x0a, 0x02, 0x04, 0x1f, 0x12, 0x06, 0xcf, 0x06, 0x00, 0xdb, 0x06, 0x01, 0x1a, 0x89, 0x01, 0x2a,
    0x0a, 0x20, 0x43, 0x6f, 0x6c, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20,
    0x52, 0x61, 0x74, 0x65, 0x4c, 0x69, 0x6d, 0x69, 0x74, 0x2e, 0x0a, 0x20, 0x46, 0x72, 0x61, 0x6d,
    0x65, 0x77, 0x6f, 0x72, 0x6b, 0x73, 0x20, 0x77, 0x69, 0x74, 0x68, 0x6f, 0x75, 0x74, 0x20, 0x72,
    0x61, 0x74, 0x65, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x73, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e,
    0x65, 0x64, 0x20, 0x68, 0x65, 0x72, 0x65, 0x20, 0x61, 0x72, 0x65, 0x20, 0x6e, 0x6f, 0x74, 0x20,
    0x74, 0x68, 0x72, 0x6f, 0x74, 0x74, 0x6c, 0x65, 0x64, 0x20, 0x75, 0x6e, 0x6c, 0x65, 0x73, 0x73,
    0x0a, 0x20, 0x27, 0x61, 0x67, 0x67, 0x72, 0x65, 0x67, 0x61, 0x74, 0x65, 0x5f, 0x64, 0x65, 0x66,
    0x61, 0x75, 0x6c, 0x74, 0x5f, 0x71, 0x70, 0x73, 0x27, 0x20, 0x69, 0x73, 0x20, 0x73, 0x70, 0x65,
    0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1f, 0x01, 0x12,
    0x04, 0xcf, 0x06, 0x08, 0x12, 0x0a, 0x34, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x00, 0x12, 0x04, 0xd1,
    0x06, 0x02, 0x20, 0x1a, 0x26, 0x20, 0x49, 0x74, 0x65, 0x6d, 0x73, 0x20, 0x73, 0x68, 0x6f, 0x75,
    0x6c, 0x64, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20, 0x75, 0x6e, 0x69, 0x71, 0x75, 0x65, 0x20, 0x70,
    0x72, 0x69, 0x6e, 0x63, 0x69, 0x70, 0x61, 0x6c, 0x73, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1f, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd1, 0x06, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f,
    0x02, 0x00, 0x06, 0x12, 0x04, 0xd1, 0x06, 0x0b, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xd1, 0x06, 0x15, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xd1, 0x06, 0x1e, 0x1f, 0x0a, 0xc7, 0x01, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x01,
    0x12, 0x04, 0xd6, 0x06, 0x02, 0x2c, 0x1a, 0xb8, 0x01, 0x20, 0x41, 0x6c, 0x6c, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x73, 0x20, 0x6e, 0x6f, 0x74,
    0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x27, 0x6c,
    0x69, 0x6d, 0x69, 0x74, 0x73, 0x27, 0x20, 0x67, 0x65, 0x74, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20,
    0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20, 0x72, 0x61, 0x74, 0x65, 0x2e, 0x0a, 0x20, 0x54,
    0x68, 0x69, 0x73, 0x20, 0x72, 0x61, 0x74, 0x65, 0x20, 0x69, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x61,
    0x67, 0x67, 0x72, 0x65, 0x67, 0x61, 0x74, 0x65, 0x20, 0x72, 0x61, 0x74, 0x65, 0x20, 0x66, 0x6f,
    0x72, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x6d, 0x2c, 0x20, 0x69,
    0x2e, 0x65, 0x2e, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x69, 0x72, 0x20, 0x63, 0x6f, 0x6d, 0x62, 0x69,
    0x6e, 0x65, 0x64, 0x0a, 0x20, 0x74, 0x72, 0x61, 0x66, 0x66, 0x69, 0x63, 0x20, 0x69, 0x73, 0x20,
    0x74, 0x68, 0x72, 0x6f, 0x74, 0x74, 0x6c, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x67, 0x65, 0x74, 0x68,
    0x65, 0x72, 0x20, 0x61, 0x74, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x72, 0x61, 0x74, 0x65, 0x2e,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd6, 0x06, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x01, 0x05, 0x12, 0x04, 0xd6, 0x06, 0x0b, 0x11, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd6, 0x06, 0x12, 0x27, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1f, 0x02, 0x01, 0x03, 0x12, 0x04, 0xd6, 0x06, 0x2a, 0x2b, 0x0a, 0x98, 0x01,
    0x0a, 0x04, 0x04, 0x1f, 0x02, 0x02, 0x12, 0x04, 0xda, 0x06, 0x02, 0x31, 0x1a, 0x89, 0x01, 0x20,
    0x41, 0x6c, 0x6c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72,
    0x6b, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64,
    0x20, 0x69, 0x6e, 0x20, 0x27, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x73, 0x27, 0x20, 0x67, 0x65, 0x74,
    0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20, 0x63, 0x61,
    0x70, 0x61, 0x63, 0x69, 0x74, 0x79, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73,
    0x20, 0x61, 0x6e, 0x20, 0x61, 0x67, 0x67, 0x72, 0x65, 0x67, 0x61, 0x74, 0x65, 0x20, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x20, 0x73, 0x69, 0x6d, 0x69, 0x6c, 0x61, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x27,
    0x61, 0x67, 0x67, 0x72, 0x65, 0x67, 0x61, 0x74, 0x65, 0x5f, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c,
    0x74, 0x5f, 0x71, 0x70, 0x73, 0x27, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x02,
    0x04, 0x12, 0x04, 0xda, 0x06, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x02, 0x05,
    0x12, 0x04, 0xda, 0x06, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x02, 0x01, 0x12,
    0x04, 0xda, 0x06, 0x12, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x02, 0x03, 0x12, 0x04,
    0xda, 0x06, 0x2f, 0x30, 0x0a, 0x91, 0x01, 0x0a, 0x02, 0x04, 0x20, 0x12, 0x06, 0xe2, 0x06, 0x00,
    0xf0, 0x06, 0x01, 0x1a, 0x82, 0x01, 0x2a, 0x0a, 0x20, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62,
    0x65, 0x73, 0x20, 0x61, 0x20, 0x76, 0x6f, 0x6c, 0x75, 0x6d, 0x65, 0x20, 0x6d, 0x61, 0x70, 0x70,
    0x69, 0x6e, 0x67, 0x20, 0x65, 0x69, 0x74, 0x68, 0x65, 0x72, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20,
    0x68, 0x6f, 0x73, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65,
    0x72, 0x20, 0x6f, 0x72, 0x20, 0x76, 0x69, 0x63, 0x65, 0x0a, 0x20, 0x76, 0x65, 0x72, 0x73, 0x61,
    0x2e, 0x20, 0x42, 0x6f, 0x74, 0x68, 0x20, 0x70, 0x61, 0x74, 0x68, 0x73, 0x20, 0x63, 0x61, 0x6e,
    0x20, 0x65, 0x69, 0x74, 0x68, 0x65, 0x72, 0x20, 0x72, 0x65, 0x66, 0x65, 0x72, 0x20, 0x74, 0x6f,
    0x20, 0x61, 0x20, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x20, 0x6f, 0x72, 0x20,
    0x61, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x20, 0x01, 0x12,
    0x04, 0xe2, 0x06, 0x08, 0x0e, 0x0a, 0x4f, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x00, 0x12, 0x04, 0xe4,
    0x06, 0x02, 0x25, 0x1a, 0x41, 0x20, 0x41, 0x62, 0x73, 0x6f, 0x6c, 0x75, 0x74, 0x65, 0x20, 0x70,
    0x61, 0x74, 0x68, 0x20, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x6f, 0x20,
    0x61, 0x20, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x20, 0x6f, 0x72, 0x20, 0x66,
    0x69, 0x6c, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61,
    0x69, 0x6e, 0x65, 0x72, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xe4, 0x06, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x05, 0x12, 0x04,
    0xe4, 0x06, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe4,
    0x06, 0x12, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe4, 0x06,
    0x23, 0x24, 0x0a, 0x7e, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x01, 0x12, 0x04, 0xe8, 0x06, 0x02, 0x20,
    0x1a, 0x70, 0x20, 0x41, 0x62, 0x73, 0x6f, 0x6c, 0x75, 0x74, 0x65, 0x20, 0x70, 0x61, 0x74, 0x68,
    0x20, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x20, 0x64,
    0x69, 0x72, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x20, 0x6f, 0x72, 0x20, 0x66, 0x69, 0x6c, 0x65,
    0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x68, 0x6f, 0x73, 0x74, 0x20, 0x6f, 0x72, 0x20,
    0x61, 0x20, 0x70, 0x61, 0x74, 0x68, 0x0a, 0x20, 0x72, 0x65, 0x6c, 0x61, 0x74, 0x69, 0x76, 0x65,
    0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65,
    0x72, 0x20, 0x77, 0x6f, 0x72, 0x6b, 0x20, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x79,
    0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x04, 0x12, 0x04, 0xe8, 0x06, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x05, 0x12, 0x04, 0xe8, 0x06, 0x0b, 0x11,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x01, 0x12, 0x04, 0xe8, 0x06, 0x12, 0x1b, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x03, 0x12, 0x04, 0xe8, 0x06, 0x1e, 0x1f, 0x0a, 0x0e,
    0x0a, 0x04, 0x04, 0x20, 0x04, 0x00, 0x12, 0x06, 0xea, 0x06, 0x02, 0xed, 0x06, 0x03, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x20, 0x04, 0x00, 0x01, 0x12, 0x04, 0xea, 0x06, 0x07, 0x0b, 0x0a, 0x1d, 0x0a,
    0x06, 0x04, 0x20, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0xeb, 0x06, 0x04, 0x0b, 0x22, 0x0d, 0x20,
    0x72, 0x65, 0x61, 0x64, 0x2d, 0x77, 0x72, 0x69, 0x74, 0x65, 0x2e, 0x0a, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x20, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xeb, 0x06, 0x04, 0x06, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x20, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0xeb, 0x06, 0x09, 0x0a, 0x0a, 0x1c,
    0x0a, 0x06, 0x04, 0x20, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0xec, 0x06, 0x04, 0x0b, 0x22, 0x0c,
    0x20, 0x72, 0x65, 0x61, 0x64, 0x2d, 0x6f, 0x6e, 0x6c, 0x79, 0x2e, 0x0a, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x20, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xec, 0x06, 0x04, 0x06, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x20, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xec, 0x06, 0x09, 0x0a, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x20, 0x02, 0x02, 0x12, 0x04, 0xef, 0x06, 0x02, 0x19, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x20, 0x02, 0x02, 0x04, 0x12, 0x04, 0xef, 0x06, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x20, 0x02, 0x02, 0x06, 0x12, 0x04, 0xef, 0x06, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20,
    0x02, 0x02, 0x01, 0x12, 0x04, 0xef, 0x06, 0x10, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02,
    0x02, 0x03, 0x12, 0x04, 0xef, 0x06, 0x17, 0x18, 0x0a, 0x84, 0x01, 0x0a, 0x02, 0x04, 0x21, 0x12,
    0x06, 0xf7, 0x06, 0x00, 0xa2, 0x07, 0x01, 0x1a, 0x76, 0x2a, 0x0a, 0x20, 0x44, 0x65, 0x73, 0x63,
    0x72, 0x69, 0x62, 0x65, 0x73, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65,
    0x72, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20,
    0x61, 0x6e, 0x64, 0x20, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x73, 0x20, 0x65, 0x78, 0x74, 0x65, 0x6e,
    0x73, 0x69, 0x62, 0x6c, 0x65, 0x0a, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x64, 0x69, 0x66, 0x66, 0x65, 0x72,
    0x65, 0x6e, 0x74, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x20, 0x69, 0x6d,
    0x70, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x0a, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x21, 0x01, 0x12, 0x04, 0xf7, 0x06, 0x08, 0x15, 0x0a, 0x35, 0x0a, 0x04,
    0x04, 0x21, 0x04, 0x00, 0x12, 0x06, 0xf9, 0x06, 0x02, 0xfc, 0x06, 0x03, 0x1a, 0x25, 0x20, 0x41,
    0x6c, 0x6c, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x65, 0x72, 0x20, 0x69, 0x6d, 0x70,
    0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x74, 0x79, 0x70, 0x65,
    0x73, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x04, 0x00, 0x01, 0x12, 0x04, 0xf9, 0x06,
    0x07, 0x0b, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x21, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0xfa, 0x06,
    0x04, 0x0f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x21, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xfa,
    0x06, 0x04, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x21, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04,
    0xfa, 0x06, 0x0d, 0x0e, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x21, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04,
    0xfb, 0x06, 0x04, 0x0e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x21, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xfb, 0x06, 0x04, 0x09, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x21, 0x04, 0x00, 0x02, 0x01, 0x02,
    0x12, 0x04, 0xfb, 0x06, 0x0c, 0x0d, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x21, 0x03, 0x00, 0x12, 0x06,
    0xfe, 0x06, 0x02, 0x9b, 0x07, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x03, 0x00, 0x01, 0x12,
    0x04, 0xfe, 0x06, 0x0a, 0x14, 0x0a, 0x4e, 0x0a, 0x06, 0x04, 0x21, 0x03, 0x00, 0x02, 0x00, 0x12,
    0x04, 0x80, 0x07, 0x04, 0x1e, 0x1a, 0x3e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x64, 0x6f, 0x63, 0x6b,
    0x65, 0x72, 0x20, 0x69, 0x6d, 0x61, 0x67, 0x65, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x69, 0x73,
    0x20, 0x67, 0x6f, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x70, 0x61, 0x73,
    0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x67, 0x69, 0x73,
    0x74, 0x72, 0x79, 0x2e, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x21, 0x03, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x80, 0x07, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x21, 0x03, 0x00, 0x02, 0x00,
    0x05, 0x12, 0x04, 0x80, 0x07, 0x0d, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x21, 0x03, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x04, 0x80, 0x07, 0x14, 0x19, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x21, 0x03, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x04, 0x80, 0x07, 0x1c, 0x1d, 0x0a, 0x24, 0x0a, 0x06, 0x04, 0x21, 0x03,
    0x00, 0x04, 0x00, 0x12, 0x06, 0x83, 0x07, 0x04, 0x87, 0x07, 0x05, 0x1a, 0x12, 0x20, 0x4e, 0x65,
    0x74, 0x77, 0x6f, 0x72, 0x6b, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x0a, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x21, 0x03, 0x00, 0x04, 0x00, 0x01, 0x12, 0x04, 0x83, 0x07, 0x09, 0x10,
    0x0a, 0x10, 0x0a, 0x08, 0x04, 0x21, 0x03, 0x00, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0x84, 0x07,
    0x06, 0x0f, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x21, 0x03, 0x00, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x04, 0x84, 0x07, 0x06, 0x0a, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x21, 0x03, 0x00, 0x04, 0x00, 0x02,
    0x00, 0x02, 0x12, 0x04, 0x84, 0x07, 0x0d, 0x0e, 0x0a, 0x10, 0x0a, 0x08, 0x04, 0x21, 0x03, 0x00,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0x85, 0x07, 0x06, 0x11, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x21,
    0x03, 0x00, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0x85, 0x07, 0x06, 0x0c, 0x0a, 0x11, 0x0a,
    0x09, 0x04, 0x21, 0x03, 0x00, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0x85, 0x07, 0x0f, 0x10,
    0x0a, 0x10, 0x0a, 0x08, 0x04, 0x21, 0x03, 0x00, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0x86, 0x07,
    0x06, 0x0f, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x21, 0x03, 0x00, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12,
    0x04, 0x86, 0x07, 0x06, 0x0a, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x21, 0x03, 0x00, 0x04, 0x00, 0x02,
    0x02, 0x02, 0x12, 0x04, 0x86, 0x07, 0x0d, 0x0e, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x21, 0x03, 0x00,
    0x02, 0x01, 0x12, 0x04, 0x89, 0x07, 0x04, 0x32, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x21, 0x03, 0x00,
    0x02, 0x01, 0x04, 0x12, 0x04, 0x89, 0x07, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x21, 0x03,
    0x00, 0x02, 0x01, 0x06, 0x12, 0x04, 0x89, 0x07, 0x0d, 0x14, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x21,
    0x03, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0x89, 0x07, 0x15, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x21, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0x89, 0x07, 0x1f, 0x20, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x21, 0x03, 0x00, 0x02, 0x01, 0x08, 0x12, 0x04, 0x89, 0x07, 0x21, 0x31, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x21, 0x03, 0x00, 0x02, 0x01, 0x07, 0x12, 0x04, 0x89, 0x07, 0x2c, 0x30, 0x0a, 0x10,
    0x0a, 0x06, 0x04, 0x21, 0x03, 0x00, 0x03, 0x00, 0x12, 0x06, 0x8b, 0x07, 0x04, 0x90, 0x07, 0x05,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x21, 0x03, 0x00, 0x03, 0x00, 0x01, 0x12, 0x04, 0x8b, 0x07, 0x0c,
    0x17, 0x0a, 0x10, 0x0a, 0x08, 0x04, 0x21, 0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x12, 0x04, 0x8c,
    0x07, 0x06, 0x24, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x21, 0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x04, 0x8c, 0x07, 0x06, 0x0e, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x21, 0x03, 0x00, 0x03, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x04, 0x8c, 0x07, 0x0f, 0x15, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x21, 0x03,
    0x00, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8c, 0x07, 0x16, 0x1f, 0x0a, 0x11, 0x0a, 0x09,
    0x04, 0x21, 0x03, 0x00, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8c, 0x07, 0x22, 0x23, 0x0a,
    0x10, 0x0a, 0x08, 0x04, 0x21, 0x03, 0x00, 0x03, 0x00, 0x02, 0x01, 0x12, 0x04, 0x8d, 0x07, 0x06,
    0x29, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x21, 0x03, 0x00, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04,
    0x8d, 0x07, 0x06, 0x0e, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x21, 0x03, 0x00, 0x03, 0x00, 0x02, 0x01,
    0x05, 0x12, 0x04, 0x8d, 0x07, 0x0f, 0x15, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x21, 0x03, 0x00, 0x03,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8d, 0x07, 0x16, 0x24, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x21,
    0x03, 0x00, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8d, 0x07, 0x27, 0x28, 0x0a, 0x39, 0x0a,
    0x08, 0x04, 0x21, 0x03, 0x00, 0x03, 0x00, 0x02, 0x02, 0x12, 0x04, 0x8f, 0x07, 0x06, 0x23, 0x1a,
    0x27, 0x20, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x20, 0x74, 0x6f, 0x20, 0x65, 0x78,
    0x70, 0x6f, 0x73, 0x65, 0x20, 0x61, 0x73, 0x20, 0x28, 0x69, 0x65, 0x3a, 0x20, 0x74, 0x63, 0x70,
    0x2c, 0x20, 0x75, 0x64, 0x70, 0x29, 0x2e, 0x0a, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x21, 0x03, 0x00,
    0x03, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0x8f, 0x07, 0x06, 0x0e, 0x0a, 0x11, 0x0a, 0x09, 0x04,
    0x21, 0x03, 0x00, 0x03, 0x00, 0x02, 0x02, 0x05, 0x12, 0x04, 0x8f, 0x07, 0x0f, 0x15, 0x0a, 0x11,
    0x0a, 0x09, 0x04, 0x21, 0x03, 0x00, 0x03, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0x8f, 0x07, 0x16,
    0x1e, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x21, 0x03, 0x00, 0x03, 0x00, 0x02, 0x02, 0x03, 0x12, 0x04,
    0x8f, 0x07, 0x21, 0x22, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x21, 0x03, 0x00, 0x02, 0x02, 0x12, 0x04,
    0x92, 0x07, 0x04, 0x2b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x21, 0x03, 0x00, 0x02, 0x02, 0x04, 0x12,
    0x04, 0x92, 0x07, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x21, 0x03, 0x00, 0x02, 0x02, 0x06,
    0x12, 0x04, 0x92, 0x07, 0x0d, 0x18, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x21, 0x03, 0x00, 0x02, 0x02,
    0x01, 0x12, 0x04, 0x92, 0x07, 0x19, 0x26, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x21, 0x03, 0x00, 0x02,
    0x02, 0x03, 0x12, 0x04, 0x92, 0x07, 0x29, 0x2a, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x21, 0x03, 0x00,
    0x02, 0x03, 0x12, 0x04, 0x94, 0x07, 0x04, 0x33, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x21, 0x03, 0x00,
    0x02, 0x03, 0x04, 0x12, 0x04, 0x94, 0x07, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x21, 0x03,
    0x00, 0x02, 0x03, 0x05, 0x12, 0x04, 0x94, 0x07, 0x0d, 0x11, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x21,
    0x03, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0x94, 0x07, 0x12, 0x1c, 0x0a, 0x0f, 0x0a, 0x07, 0x04,
    0x21, 0x03, 0x00, 0x02, 0x03, 0x03, 0x12, 0x04, 0x94, 0x07, 0x1f, 0x20, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x21, 0x03, 0x00, 0x02, 0x03, 0x08, 0x12, 0x04, 0x94, 0x07, 0x21, 0x32, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x21, 0x03, 0x00, 0x02, 0x03, 0x07, 0x12, 0x04, 0x94, 0x07, 0x2c, 0x31, 0x0a, 0xd3,
    0x01, 0x0a, 0x06, 0x04, 0x21, 0x03, 0x00, 0x02, 0x04, 0x12, 0x04, 0x9a, 0x07, 0x04, 0x26, 0x1a,
    0xc2, 0x01, 0x20, 0x41, 0x6c, 0x6c, 0x6f, 0x77, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x72, 0x62, 0x69,
    0x74, 0x72, 0x61, 0x72, 0x79, 0x20, 0x70, 0x61, 0x72, 0x61, 0x6d, 0x65, 0x74, 0x65, 0x72, 0x73,
    0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x70, 0x61, 0x73, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f,
    0x20, 0x64, 0x6f, 0x63, 0x6b, 0x65, 0x72, 0x20, 0x43, 0x4c, 0x49, 0x2e, 0x0a, 0x20, 0x4e, 0x6f,
    0x74, 0x65, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x61, 0x6e, 0x79, 0x74, 0x68, 0x69, 0x6e, 0x67,
    0x20, 0x70, 0x61, 0x73, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20,
    0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x67, 0x75, 0x72,
    0x61, 0x6e, 0x74, 0x65, 0x65, 0x64, 0x0a, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x73, 0x75,
    0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x20, 0x6d, 0x6f, 0x76, 0x69, 0x6e, 0x67, 0x20, 0x66,
    0x6f, 0x72, 0x77, 0x61, 0x72, 0x64, 0x2c, 0x20, 0x61, 0x73, 0x20, 0x77, 0x65, 0x20, 0x6d, 0x69,
    0x67, 0x68, 0x74, 0x20, 0x6d, 0x6f, 0x76, 0x65, 0x20, 0x61, 0x77, 0x61, 0x79, 0x20, 0x66, 0x72,
    0x6f, 0x6d, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x6f, 0x63, 0x6b, 0x65, 0x72, 0x20, 0x43,
    0x4c, 0x49, 0x2e, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x21, 0x03, 0x00, 0x02, 0x04, 0x04, 0x12,
    0x04, 0x9a, 0x07, 0x04, 0x0c, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x21, 0x03, 0x00, 0x02, 0x04, 0x06,
    0x12, 0x04, 0x9a, 0x07, 0x0d, 0x16, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x21, 0x03, 0x00, 0x02, 0x04,
    0x01, 0x12, 0x04, 0x9a, 0x07, 0x17, 0x21, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x21, 0x03, 0x00, 0x02,
    0x04, 0x03, 0x12, 0x04, 0x9a, 0x07, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x21, 0x02, 0x00,
    0x12, 0x04, 0x9d, 0x07, 0x02, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x9d, 0x07, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00, 0x06, 0x12, 0x04,
    0x9d, 0x07, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9d,
    0x07, 0x10, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9d, 0x07,
    0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x21, 0x02, 0x01, 0x12, 0x04, 0x9e, 0x07, 0x02, 0x1e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x04, 0x12, 0x04, 0x9e, 0x07, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x06, 0x12, 0x04, 0x9e, 0x07, 0x0b, 0x11, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x01, 0x12, 0x04, 0x9e, 0x07, 0x12, 0x19, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x21, 0x02, 0x01, 0x03, 0x12, 0x04, 0x9e, 0x07, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x21, 0x02, 0x02, 0x12, 0x04, 0x9f, 0x07, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21,
    0x02, 0x02, 0x04, 0x12, 0x04, 0x9f, 0x07, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02,
    0x02, 0x05, 0x12, 0x04, 0x9f, 0x07, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x02,
    0x01, 0x12, 0x04, 0x9f, 0x07, 0x12, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x02, 0x03,
    0x12, 0x04, 0x9f, 0x07, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x21, 0x02, 0x03, 0x12, 0x04,
    0xa1, 0x07, 0x02, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x03, 0x04, 0x12, 0x04, 0xa1,
    0x07, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x03, 0x06, 0x12, 0x04, 0xa1, 0x07,
    0x0b, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x03, 0x01, 0x12, 0x04, 0xa1, 0x07, 0x16,
    0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x03, 0x03, 0x12, 0x04, 0xa1, 0x07, 0x1f, 0x20,
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
