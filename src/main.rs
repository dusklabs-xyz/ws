use futures_util::{SinkExt, StreamExt};
use rand::Rng;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{sleep, Duration};
use tokio_tungstenite::tungstenite::Message;

async fn handle_message(msg: String) -> String {
    let delay_ms = rand::thread_rng().gen_range(300..=500);
    sleep(Duration::from_millis(delay_ms)).await;

    let mut response = msg;
    if delay_ms > 480 {
        response.pop();
    }

    format!("echo: {response}")
}

async fn handle_connection(stream: TcpStream, addr: SocketAddr) {
    let ws_stream = match tokio_tungstenite::accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("WebSocket handshake failed for {addr}: {e}");
            return;
        }
    };

    println!("Connected: {addr}");
    let (mut sink, mut stream) = ws_stream.split();

    while let Some(Ok(msg)) = stream.next().await {
        match msg {
            Message::Text(text) => {
                let response = handle_message(text.to_string()).await;
                if sink.send(Message::Text(response.into())).await.is_err() {
                    break;
                }
            }
            Message::Close(_) => break,
            _ => {}
        }
    }

    println!("Disconnected: {addr}");
}

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(addr).await.expect("failed to bind");
    println!("WebSocket server listening on ws://{addr}");

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, addr));
    }
}
