[![Gitpod Ready-to-Code](https://img.shields.io/badge/Gitpod-Ready--to--Code-blue?logo=gitpod)](https://gitpod.io/#https://github.com/drmingdrmer/hello-grpc) 

# hello-grpc

A minimized `grpc` experiment with `go` and `rust`
for fun, to learn.

It builds a hello-world grpc service from protobuf definition in `go` and `rust`
and a grpc client in `go`.

# Directory layout

- `/proto` for proto file .
- `/pbbuild` is a standalone cargo to generate rust code from proto.

- `grpc-go` grpc impl in go.
- `grpc-rust-tonic` grpc impl in rust with tonic.

# Re-Generate proto codes

- `make install-deps` to install CLI command for generating rust code.
- `make gen` to generate `go` and `rust` code with grpc support.

# Notes for rust

Crate `protoc-rust` and `protobuf` are from a same git repo:
`github.com/stepancheg/rust-protobuf`.

Using different version for these two crates may have potential compatibility
issues. (The git history is a mess.)

Currently we use v`2.8.2` for both of them.
The latest is v`2.10.1` and v`2.11.x`.

---

TODO need grpc support in `build.rs`
