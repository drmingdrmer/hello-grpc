use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_client::GreeterClient;
use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

use tokio::time::delay_for;

use std::time::{Duration};

pub mod hello_world {
    // If using build.rs:
    //
    // tonic::include_proto!("helloworld"); // The string specified here must match the proto package name

    // see: https://docs.rs/tonic/0.1.1/tonic/macro.include_proto.html
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/helloworld.rs"));
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<HelloReply>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

async fn cli() -> Result<(), Box<dyn std::error::Error>> {
    println!("client connecting");
    let mut client = GreeterClient::connect("http://[::1]:3334").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:3334".parse()?;
    let greeter = MyGreeter::default();

    let (tx, rx) = tokio::sync::oneshot::channel::<()>();
    tokio::spawn(async move {
        println!("spawned");
        Server::builder()
            .add_service(GreeterServer::new(greeter))
            // .serve(addr)
            .serve_with_shutdown(addr, async {
                rx.await.ok();
            })
            .await.unwrap();
    });

    println!("serving");

    delay_for(Duration::from_millis(1_000)).await;
    cli().await?;

    let _ = tx.send(());
    println!("sent killing");
    Ok(())
}
