# Resos 

<!--
[![Build Status](https://travis-ci.org/ConnorDoyle/resos.png?branch=master)](https://travis-ci.org/ConnorDoyle/resos)
-->

Rust language bindings for [Apache Mesos](http://mesos.apache.org).

## Prerequisites

- [Rust](http://rust-lang.org)
- `libmesos.{so, dylib}` on your shared library search path

## Building

This project is built using [cargo](http://doc.crates.io) against the 0.13 nightlies.

```
$ cargo build
```

The initial FFI declarations were generated using
[cxx2rs](https://github.com/manuels/cxx2rs).

The protobuf code was generated using
[rust-protobuf](https://github.com/stepancheg/rust-protobuf).
