use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt};

fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(process());
}

async fn process() {
    let thread_count = std::env::args()
        .nth(1)
        .unwrap_or("10240".to_string())
        .parse::<usize>()
        .unwrap_or(10240);

    let counter = Arc::new(std::sync::atomic::AtomicUsize::new(0));

    for _ in 0..thread_count {
        tokio::spawn(connection(counter.clone()));
    }

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

async fn connection(counter: Arc<std::sync::atomic::AtomicUsize>) {
    let addr = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "kr-1.esukmean.com:8080".to_string());

    let mut stream = match tokio::net::TcpStream::connect(&addr).await {
        Ok(stream) => stream,
        Err(_e) => {
            return;
        }
    };

    counter
        .as_ref()
        .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    stream.write_all(b"GET /mychat HTTP/1.1\r\nHost: server.example.com\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Key: x3JJHMbDL1EzLkh9GBhXDw==\r\nSec-WebSocket-Protocol: chat\r\nSec-WebSocket-Version: 13\r\nOrigin: http://example.com\r\n\r\n").await.unwrap();

    let mut buf = [0; 1024];
    loop {
        let n = stream.read(&mut buf).await.unwrap();
        if n == 0 {
            break;
        }
    }
}
