use tungstenite::{connect, Message};
use tungstenite::stream::MaybeTlsStream;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

pub struct WebSocketClient {
    stream: Arc<Mutex<Option<WebSocketStream>>>,
    url: String,
    connected: bool,
}

type WebSocketStream = tungstenite::WebSocket<MaybeTlsStream<TcpStream>>;

impl WebSocketClient {
    pub fn new() -> Self {
        Self {
            stream: Arc::new(Mutex::new(None)),
            url: String::new(),
            connected: false,
        }
    }

    pub fn connect(&mut self, url: &str) -> Result<(), tungstenite::Error> {
        match connect(url) {
            Ok((stream, _)) => {
                *self.stream.lock().unwrap() = Some(stream);
                self.url = url.to_string();
                self.connected = true;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn send(&mut self, message: &str) -> Result<(), tungstenite::Error> {
        let mut guard = self.stream.lock().unwrap();
        if let Some(stream) = guard.as_mut() {
            stream.send(Message::Text(message.to_string()))?;
            Ok(())
        } else {
            Err(tungstenite::Error::ConnectionClosed)
        }
    }

    pub fn send_binary(&mut self, data: &[u8]) -> Result<(), tungstenite::Error> {
        let mut guard = self.stream.lock().unwrap();
        if let Some(stream) = guard.as_mut() {
            stream.send(Message::Binary(data.to_vec()))?;
            Ok(())
        } else {
            Err(tungstenite::Error::ConnectionClosed)
        }
    }

    pub fn receive(&mut self) -> Result<Option<String>, tungstenite::Error> {
        let mut guard = self.stream.lock().unwrap();
        if let Some(stream) = guard.as_mut() {
            match stream.read() {
                Ok(msg) => {
                    if msg.is_text() {
                        Ok(Some(msg.into_text().unwrap()))
                    } else if msg.is_binary() {
                        Ok(Some(format!("{:?}", msg.into_data())))
                    } else if msg.is_close() {
                        self.connected = false;
                        Ok(None)
                    } else {
                        Ok(None)
                    }
                }
                Err(e) => {
                    self.connected = false;
                    Err(e)
                }
            }
        } else {
            Ok(None)
        }
    }

    pub fn receive_binary(&mut self) -> Result<Option<Vec<u8>>, tungstenite::Error> {
        let mut guard = self.stream.lock().unwrap();
        if let Some(stream) = guard.as_mut() {
            match stream.read() {
                Ok(msg) => {
                    if msg.is_binary() {
                        Ok(Some(msg.into_data()))
                    } else if msg.is_text() {
                        Ok(Some(msg.into_text().unwrap().into_bytes()))
                    } else if msg.is_close() {
                        self.connected = false;
                        Ok(None)
                    } else {
                        Ok(None)
                    }
                }
                Err(e) => {
                    self.connected = false;
                    Err(e)
                }
            }
        } else {
            Ok(None)
        }
    }

    pub fn close(&mut self) -> Result<(), tungstenite::Error> {
        let mut guard = self.stream.lock().unwrap();
        if let Some(stream) = guard.as_mut() {
            stream.close(None)?;
            self.connected = false;
        }
        *guard = None;
        Ok(())
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub fn get_url(&self) -> &str {
        &self.url
    }
}

impl Default for WebSocketClient {
    fn default() -> Self {
        Self::new()
    }
}
