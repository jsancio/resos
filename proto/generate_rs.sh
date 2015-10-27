#!/bin/bash

brew install protobuf
if [ ! -d rust-protobuf ]; then
    git clone https://github.com/stepancheg/rust-protobuf.git
fi
cd rust-protobuf
git pull
cargo build
PATH="`pwd`/target/debug:$PATH"

cd ../proto/mesos/v1
curl -O https://raw.githubusercontent.com/apache/mesos/master/include/mesos/v1/mesos.proto
cd scheduler
curl -O https://raw.githubusercontent.com/apache/mesos/master/include/mesos/v1/scheduler/scheduler.proto
cd ../../../..

protoc --rust_out src proto/mesos/v1/mesos.proto
protoc --rust_out src --proto_path=proto proto/mesos/v1/scheduler/scheduler.proto

# TODO This is probably a bug in rust-protobuf that I have to workaround
cd src
sed -i "" 's/Operation/Offer_Operation/g' scheduler.rs
sed -i "" 's/use super::mesos::Offer;/use super::mesos::Offer;\
use super::mesos::Offer_Operation;/g' scheduler.rs