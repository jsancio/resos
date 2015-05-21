#![allow(dead_code)]

extern crate libc;

use native::ProtobufObj;
use protobuf::error::ProtobufError;
use protobuf::Message;

use std::raw::{Repr, Slice};
use std::vec::Vec;

/// Returns the result of serializing the supplied protobuf message
pub fn serialize(proto: &Message) -> Result<ProtobufObj, ProtobufError> {
    proto.write_to_bytes().map(|buffer: Vec<u8>| {
        slice_to_pb(&buffer[..])
    })
}

/// Returns the result of deserializing the supplied protobuf data
/// into the supplied message.
pub fn deserialize<'a, T: Message>(
    obj: &ProtobufObj,
    proto: &'a mut T) -> Result<&'a mut T, ProtobufError> {
    let slice = pb_to_slice(obj);
    try!(proto.merge_from_bytes(slice));
    Ok(proto)
}

/// Returns a wrapped represtation of the supplied vector as protobuf
/// data.
fn slice_to_pb(buffer: &[u8]) -> ProtobufObj {
    let repr = buffer.repr();
    ProtobufObj {
        data: repr.data as *mut libc::c_void,
        size: repr.len as u64
    }
}

/// Returns an unwrapped represtation of the supplied protobuf data
/// as a vector of bytes.
fn pb_to_slice(pb: &ProtobufObj) -> &[u8] {
    unsafe {
        // std::raw::Slice is internal representation of a slice,
        // so we just construct it manually and reinterpret it as an &[u8]
        ::std::mem::transmute(
            Slice {
                data: pb.data as *const u8,
                len: pb.size as usize
            }
        )
    }
}

