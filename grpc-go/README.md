grpc server and client in go

# Directory layout

- `/c` for go client.
- `/s` for go server.
- `/helloworld` for generated go-package `helloworld`.

## Start server

`cd s && go run server.go`

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

## Call it with go:

`cd c && go run client.go`
