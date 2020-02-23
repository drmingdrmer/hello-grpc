#![warn(rust_2018_idioms)]

use tokio;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

async fn proc(mut listener: TcpListener) {
    loop {
        let (mut sock, _) = listener.accept().await.expect("accept failure");
        println!("new connection");

        tokio::spawn(async move {
            let mut buf = [0; 32];
            let n = sock
                .read(&mut buf)
                .await
                .expect("failed to read data from socket");

            println!("read: {:?}", &buf);

            if n == 0 {
                println!("closed");
                return;
            }

            sock.write_all(&buf[0..n])
                .await
                .expect("failed to write data to socket");
        });
    }
}

#[tokio::main]
async fn mm() {

    let listener1 = TcpListener::bind(&"127.0.0.1:3334").await.unwrap();
    println!("listened 3334");

    let join_handle_1 = tokio::spawn(async move {
        proc(listener1).await;
    });

    let listener2 = TcpListener::bind(&"127.0.0.1:3335").await.unwrap();
    println!("listened 3335");

    let join_handle_2 = tokio::spawn(async move {
        proc(listener2).await;
    });

    let _ = tokio::join!(join_handle_1, join_handle_2);
}

fn main() {
    // test:
    // cd try-future
    // cargo run
    //
    // telnet localhost 333[45]
    mm();
}
