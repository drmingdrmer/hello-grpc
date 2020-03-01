gen: gen-go gen-rust

gen-go:
	protoc --proto_path=proto --go_out=plugins=grpc:grpc-go/helloworld helloworld.proto

gen-rust:
	( cd pbbuild && OUT_DIR=../grpc-rust-tonic/src cargo run; )

fmt:
	find . -name "*.rs" -exec rustfmt --edition 2018 {} ';'
	# nmp install -g prettier
	prettier --write --print-width 80 --prose-wrap preserve **/*.md
