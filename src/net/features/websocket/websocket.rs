#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebSocketState {
    Connecting,
    Open,
    Closing,
    Closed,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WebSocketMessage {
    pub text: Option<String>,
    pub binary: Option<Vec<u8>>,
}

impl WebSocketMessage {
    pub fn text(payload: &str) -> Self {
        WebSocketMessage {
            text: Some(payload.to_string()),
            binary: None,
        }
    }

    pub fn binary(payload: Vec<u8>) -> Self {
        WebSocketMessage {
            text: None,
            binary: Some(payload),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WebSocketClient {
    pub url: String,
    pub protocols: Vec<String>,
    pub state: WebSocketState,
    pub send_count: usize,
    pub receive_count: usize,
    pub close_code: Option<u16>,
}

impl WebSocketClient {
    pub fn new(url: &str) -> Self {
        WebSocketClient {
            url: url.to_string(),
            protocols: Vec::new(),
            state: WebSocketState::Connecting,
            send_count: 0,
            receive_count: 0,
            close_code: None,
        }
    }

    pub fn connect(&mut self) -> Result<(), String> {
        if !(self.url.starts_with("ws://") || self.url.starts_with("wss://")) {
            return Err(format!("invalid websocket url: {}", self.url));
        }
        self.state = WebSocketState::Connecting;
        self.state = WebSocketState::Open;
        Ok(())
    }

    pub fn add_protocol(&mut self, protocol: &str) {
        if !self.protocols.contains(&protocol.to_string()) {
            self.protocols.push(protocol.to_string());
        }
    }

    pub fn is_open(&self) -> bool {
        self.state == WebSocketState::Open
    }

    pub fn send_text(&mut self, _payload: &str) -> Result<(), String> {
        if !self.is_open() {
            return Err("websocket not open".to_string());
        }
        self.send_count += 1;
        Ok(())
    }

    pub fn send_binary(&mut self, _payload: Vec<u8>) -> Result<(), String> {
        if !self.is_open() {
            return Err("websocket not open".to_string());
        }
        self.send_count += 1;
        Ok(())
    }

    pub fn receive(&mut self) -> Option<WebSocketMessage> {
        if !self.is_open() {
            return None;
        }
        self.receive_count += 1;
        Some(WebSocketMessage::text("stub"))
    }

    pub fn close(&mut self, code: u16) {
        self.state = WebSocketState::Closing;
        self.close_code = Some(code);
        self.state = WebSocketState::Closed;
    }

    pub fn negotiation_headers(&self) -> String {
        let protocols = if self.protocols.is_empty() {
            String::new()
        } else {
            format!("Sec-WebSocket-Protocol: {}\r\n", self.protocols.join(", "))
        };
        format!("Upgrade: websocket\r\nConnection: Upgrade\r\n{}", protocols)
    }
}

impl Default for WebSocketClient {
    fn default() -> Self {
        WebSocketClient {
            url: String::new(),
            protocols: Vec::new(),
            state: WebSocketState::Closed,
            send_count: 0,
            receive_count: 0,
            close_code: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connects_and_opens() {
        let mut ws = WebSocketClient::new("wss://example.com/socket");
        ws.connect().unwrap();
        assert!(ws.is_open());
        assert_eq!(ws.state, WebSocketState::Open);
    }

    #[test]
    fn rejects_bad_scheme() {
        let mut ws = WebSocketClient::new("https://example.com");
        assert!(ws.connect().is_err());
    }

    #[test]
    fn send_requires_open() {
        let mut ws = WebSocketClient::new("ws://example.com");
        assert!(ws.send_text("hi").is_err());
        ws.connect().unwrap();
        ws.send_text("hi").unwrap();
        assert_eq!(ws.send_count, 1);
    }

    #[test]
    fn close_transitions() {
        let mut ws = WebSocketClient::new("ws://example.com");
        ws.connect().unwrap();
        ws.close(1000);
        assert_eq!(ws.state, WebSocketState::Closed);
        assert_eq!(ws.close_code, Some(1000));
    }

    #[test]
    fn adds_protocols_once() {
        let mut ws = WebSocketClient::new("ws://example.com");
        ws.add_protocol("chat");
        ws.add_protocol("chat");
        assert_eq!(ws.protocols.len(), 1);
        assert!(ws.negotiation_headers().contains("chat"));
    }

    #[test]
    fn receive_increments_counter() {
        let mut ws = WebSocketClient::new("ws://example.com");
        ws.connect().unwrap();
        let msg = ws.receive().unwrap();
        assert_eq!(msg.text, Some("stub".to_string()));
        assert_eq!(ws.receive_count, 1);
    }
}
