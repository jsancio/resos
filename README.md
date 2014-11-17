# Resos

Rust language bindings for Apache Mesos.

## Building

This project is built using [cargo](http://doc.crates.io).

```
$ cargo build
```

## Generating the FFI Declarations

Using [cxx2rs](https://github.com/manuels/cxx2rs), run:

```
$ python cxx2rs.py libmesos src/mesos-c-api.h > src/mesos.rs
```

