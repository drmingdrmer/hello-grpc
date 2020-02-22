[![Gitpod Ready-to-Code](https://img.shields.io/badge/Gitpod-Ready--to--Code-blue?logo=gitpod)](https://gitpod.io/#https://github.com/drmingdrmer/hello-grpc) 

# hello-grpc

A minimized `grpc` experiment with `go` and `rust`
for fun, to learn.

It builds a hello-world grpc service from protobuf definition in `go` and `rust`
and a grpc client in `go`.

# Directory layout

- `/c` for go client.
- `/s` for go server.
- `/src` for rust server and client.
- `/proto` for proto file .
- `/helloworld` for generated go-package `helloworld`.
- `/pbbuild` is a standalone cargo to generate rust code from proto.

# Generate codes

- `make install-deps` to install CLI command for generating rust code.
- `make gen` to generate `go` and `rust` code with grpc support.

# Try it!

## Start server on port 3334: choose one of go or rust:

- go: `cd s && go run server.go`
- rust: `cargo run --bin server`

> Ctrl-C to quit

## Call it from CLI:

```
# brew install grpcurl

grpcurl \
    -plaintext                         \
    -import-path   ./proto             \
    -proto         helloworld.proto    \
    -d             '{"name": "Tonic"}' \
    localhost:3334                     \
    helloworld.Greeter/SayHello

# {
#   "message": "Hello Tonic!"
# }
```

## Call it with rust:

`cargo run --bin client`

## Call it with go:

`cd c && go run client.go`

# Notes for rust

Crate `protoc-rust` and `protobuf` are from a same git repo:
`github.com/stepancheg/rust-protobuf`.

Using different version for these two crates may have potential compatibility
issues. (The git history is a mess.)

Currently we use v`2.8.2` for both of them.
The latest is v`2.10.1` and v`2.11.x`.

---

TODO need grpc support in `build.rs`
