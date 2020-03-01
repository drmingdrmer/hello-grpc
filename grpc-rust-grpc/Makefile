install-deps:
	cargo install grpc-compiler

gen:
	# Message
	cargo run --bin build-pb
	# grpc service
	protoc --proto_path=../proto --rust-grpc_out=src helloworld.proto
