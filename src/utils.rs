extern crate libc;

use mesos;
use native::ProtobufObj;
use protobuf::error::ProtobufError;
use protobuf::Message;
use protobuf::stream::CodedInputStream;

use std::ops::Deref;
use std::path::BytesContainer;
use std::raw::{Repr, Slice};
use std::vec::Vec;

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
                len: pb.size as uint
            }
        )
    }
}

/// Returns an input stream wrapping the supplied vector.
fn slice_to_stream(buffer: &[u8]) -> CodedInputStream {
    CodedInputStream::from_bytes(buffer)
}

/// Returns the result of serializing the supplied protobuf message
pub fn serialize(proto: &Message) -> Result<ProtobufObj, ProtobufError> {
    proto.write_to_bytes().map(|buffer: Vec<u8>| {
        slice_to_pb(buffer.as_slice())
    })
}

/*
/// Returns...
pub fn deserialize<T: Message>(
    obj: &ProtobufObj,
    proto: &T) -> Result<T, ProtobufError> {
    let slice = pb_to_slice(obj);
    let stream = slice_to_stream(slice);
    proto.merge_from(stream).map(|_| proto)
}
*/
