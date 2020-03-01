grpc server in rust with crate protobuf and grpc.

# Generate codes

- `make install-deps` to install CLI command for generating rust code.
- `make gen` to generate `go` and `rust` code with grpc support.

# Try it!

## Start server on port 3334: choose one of go or rust:

`cargo run --bin server`

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

# Notes

Crate `protoc-rust` and `protobuf` are from a same git repo:
`github.com/stepancheg/rust-protobuf`.

Using different version for these two crates may have potential compatibility
issues. (The git history is a mess.)

Currently we use v`2.8.2` for both of them.
The latest is v`2.10.1` and v`2.11.x`.
