#![warn(rust_2018_idioms)]

use tokio;
use tokio::time::{self, Duration};

use futures::Future;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

// for boxed()
use futures::future::FutureExt;

async fn proc<F:Future + Send>(
    mut listener: TcpListener,
    sig: F)
{
    let mut delay = time::delay_for(Duration::from_millis(1_000));

    // select! requires Unpin trait. async {} does not impl it.
    let mut f = sig.boxed();

    loop {
        let mut sock;
        tokio::select!{
            v = listener.accept() => { let x = v.unwrap(); sock = x.0; },
            u = (&mut f) => { println!("stopping"); break; },
            v = (&mut delay) => { println!("stopping"); break; },
        };

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

    println!("stopped");
}

#[tokio::main]
async fn mm() {

    let listener1 = TcpListener::bind(&"127.0.0.1:3334").await.unwrap();
    println!("listened 3334");

    let (tx1, rx1) = tokio::sync::oneshot::channel::<()>();
    let join_handle_1 = tokio::spawn(async move {
        // using a Future to signal
        proc(listener1, async {rx1.await.ok();}).await;
        // proc(listener1, delay).await;
    });

    let (tx2, rx2) = tokio::sync::oneshot::channel::<()>();

    let listener2 = TcpListener::bind(&"127.0.0.1:3335").await.unwrap();
    println!("listened 3335");

    let join_handle_2 = tokio::spawn(async move {
        // using a channel::Receiver to signal
        proc(listener2, rx2).await;
    });

    let mut delay = time::delay_for(Duration::from_millis(1_000));
    delay.await;
    println!("killing-1");
    tx1.send(()).unwrap();

    let mut delay = time::delay_for(Duration::from_millis(1_000));
    delay.await;
    println!("killing-2");
    tx2.send(()).unwrap();

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
