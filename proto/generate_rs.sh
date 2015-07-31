#!/bin/bash

brew install protobuf
git clone git@github.com:stepancheg/rust-protobuf
cd rust-protobuf
git pull
cargo build
PATH="`pwd`/target/debug:$PATH"

cd ../proto/mesos
curl -O https://raw.githubusercontent.com/apache/mesos/master/include/mesos/mesos.proto
cd ..
curl -O https://raw.githubusercontent.com/apache/mesos/master/src/messages/messages.proto
cd ..

protoc --rust_out src proto/mesos/mesos.proto
protoc --rust_out src --proto_path=proto proto/messages.proto
cd src
mv mesos.rs proto.rs
mv messages.rs internal.rs
sed -i "" 's/mesos:://g' internal.rs