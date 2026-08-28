use tungstenite::{connect, Message};
use tungstenite::stream::MaybeTlsStream;
use std::net::TcpStream;

pub struct WebSocketClient {
    stream: Option<WebSocket<MaybeTlsStream<TcpStream>>>,
}

impl WebSocketClient {
    pub fn new() -> Self {
        Self { stream: None }
    }

    pub fn connect(&mut self, url: &str) -> bool {
        match connect(url) {
            Ok((stream, _)) => {
                self.stream = Some(stream);
                true
            }
            Err(_) => false,
        }
    }

    pub fn send(&mut self, message: &str) -> bool {
        if let Some(stream) = &mut self.stream {
            stream.send(Message::Text(message.to_string())).is_ok()
        } else {
            false
        }
    }

    pub fn send_binary(&mut self, data: &[u8]) -> bool {
        if let Some(stream) = &mut self.stream {
            stream.send(Message::Binary(data.to_vec())).is_ok()
        } else {
            false
        }
    }

    pub fn receive(&mut self) -> Option<String> {
        if let Some(stream) = &mut self.stream {
            if let Ok(msg) = stream.read() {
                if msg.is_text() {
                    return Some(msg.into_text().unwrap());
                }
            }
        }
        None
    }

    pub fn close(&mut self) {
        if let Some(stream) = &mut self.stream {
            let _ = stream.close(None);
        }
        self.stream = None;
    }
}
