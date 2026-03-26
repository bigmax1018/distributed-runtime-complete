use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn start_server(addr: &str) {
    let listener = TcpListener::bind(addr).await.unwrap();

    loop {
        let (mut stream, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            let n = stream.read(&mut buf).await.unwrap();

            println!("Received {} bytes from {}", n, stream.peer_addr().unwrap());

            stream.write_all(&buf[0..n]).await.unwrap();
        });
    }
}

pub async fn send(addr: &str, data: &[u8]) {
    let mut stream = TcpStream::connect(addr).await.unwrap();
    stream.write_all(data).await.unwrap();
}