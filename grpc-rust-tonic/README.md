grpc impl in rust with tonic.

# Try it!

## Start server on port 3334:

- `cargo run --bin server`

> Ctrl-C to quit

## Call it from CLI:

```
# brew install grpcurl

grpcurl \
    -plaintext                         \
    -import-path   ../proto            \
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

## Or start server and client in one process:

`cargo run --bin oneshot`

# Re-generate protobuf:

`make gen`
