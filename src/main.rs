use std::net::SocketAddr;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    // Multiuser chat

    let listener: TcpListener = TcpListener::bind("localhost:8080").await.unwrap();
    let (tx, _rx) = broadcast::channel::<(String, SocketAddr)>(10);

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        let tx: broadcast::Sender<(String, SocketAddr)> = tx.clone();
        let mut rx: broadcast::Receiver<(String, SocketAddr)> = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader: BufReader<tokio::net::tcp::ReadHalf> = BufReader::new(reader);
            let mut line: String = String::new();

            loop {
                tokio::select! {
                    _ = reader.read_line(&mut line) => {
                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }
                    result = rx.recv() => {
                        let (msg, msg_addr) = result.unwrap();
                        if addr != msg_addr {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
