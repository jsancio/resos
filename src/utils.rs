#![allow(dead_code)]
use protobuf::error::ProtobufError;
use protobuf::Message;

use std::vec::Vec;

/// Returns the result of serializing the supplied protobuf message
pub fn serialize(proto: &Message) -> Result<Vec<u8>, ProtobufError> {
    proto.write_to_bytes()
}

/// Returns the result of deserializing the supplied protobuf data
/// into the supplied message.
pub fn deserialize<'a, T: Message>(
    obj: &Vec<u8>,
    proto: &'a mut T) -> Result<&'a mut T, ProtobufError> {
    try!(proto.merge_from_bytes(obj));
    Ok(proto)
}
