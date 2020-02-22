install-deps:
	cargo install grpc-compiler

gen: gen-go gen-rust

gen-go:
	protoc --proto_path=proto --go_out=plugins=grpc:helloworld helloworld.proto

gen-rust:
	# Message
	( cd pbbuild && cargo run; )
	# grpc
	protoc --proto_path=proto --rust-grpc_out=src helloworld.proto

fmt:
	find . -name "*.rs" -exec rustfmt --edition 2018 {} ';'
	# nmp install -g prettier
	prettier --write --print-width 80 --prose-wrap preserve **/*.md
