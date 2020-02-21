// extern crate futures;
extern crate grpc;
extern crate protobuf;

mod helloworld;
mod helloworld_grpc;

use grpc::Server;
use helloworld::*;
use helloworld_grpc::*;
use std::thread;

struct Hello;

impl Greeter for Hello {
    fn say_hello(
        &self,
        _: ::grpc::RequestOptions,
        p: HelloRequest,
    ) -> ::grpc::SingleResponse<HelloReply> {
        let mut resp = HelloReply::new();
        let req = p.get_name();
        let res = format!("response to req:{}", req);
        resp.set_message(res);
        grpc::SingleResponse::completed(resp)
    }
}

fn main() {
    println!("Rust gRPC demo.");
    let mut server = grpc::ServerBuilder::new_plain();

    server.http.set_addr("0.0.0.0:3334").unwrap();
    server.http.set_cpu_pool_threads(4);
    server.add_service(GreeterServer::new_service_def(Hello));
    let _server: Server = server.build().expect("server");

    loop {
        thread::park();
    }
}
