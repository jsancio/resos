#!/bin/bash

brew install protobuf
if [ ! -d rust-protobuf ]; then
    git clone https://github.com/stepancheg/rust-protobuf.git
fi
cd rust-protobuf
git pull
cargo build
PATH="`pwd`/target/debug:$PATH"
VERSION=0.26.0

cd ../proto/mesos/v1
curl -O https://raw.githubusercontent.com/apache/mesos/$VERSION/include/mesos/v1/mesos.proto
cd executor
curl -O https://raw.githubusercontent.com/apache/mesos/$VERSION/include/mesos/v1/executor/executor.proto
cd ../scheduler
curl -O https://raw.githubusercontent.com/apache/mesos/$VERSION/include/mesos/v1/scheduler/scheduler.proto
cd ../../../..

protoc --rust_out src proto/mesos/v1/mesos.proto
protoc --rust_out src --proto_path=proto proto/mesos/v1/executor/executor.proto
protoc --rust_out src --proto_path=proto proto/mesos/v1/scheduler/scheduler.proto
