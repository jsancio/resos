#!/bin/bash

brew install protobuf
git clone git@github.com:stepancheg/rust-protobuf
cd rust-protobuf
cargo build
PATH="`pwd`/target/debug:$PATH"
cd ..
# scheduler.proto from the mesos distribution is mesos/mesos.proto
protoc --rust_out src proto/mesos/mesos.proto
protoc --rust_out src --proto_path=proto proto/messages.proto
cd src
mv mesos.rs proto.rs
mv messages.rs internal.rs
sed -i "" 's/mesos:://g' internal.rs