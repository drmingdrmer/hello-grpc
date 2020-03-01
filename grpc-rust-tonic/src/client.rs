use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;

pub mod hello_world {
    // If using build.rs:
    //
    // tonic::include_proto!("helloworld"); // The string specified here must match the proto package name

    // see: https://docs.rs/tonic/0.1.1/tonic/macro.include_proto.html
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/helloworld.rs"));
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:3334").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
