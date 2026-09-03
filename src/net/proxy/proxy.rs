#[derive(Debug, Clone)]
pub struct Proxy {
    pub host: String,
    pub port: u16,
    pub proxy_type: ProxyType,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProxyType {
    HTTP,
    HTTPS,
    SOCKS5,
}

impl Proxy {
    pub fn http(host: &str, port: u16) -> Self {
        Proxy { host: host.to_string(), port, proxy_type: ProxyType::HTTP, username: None, password: None }
    }

    pub fn socks5(host: &str, port: u16) -> Self {
        Proxy { host: host.to_string(), port, proxy_type: ProxyType::SOCKS5, username: None, password: None }
    }

    pub fn with_auth(mut self, user: &str, pass: &str) -> Self {
        self.username = Some(user.to_string());
        self.password = Some(pass.to_string());
        self
    }

    pub fn url(&self) -> String {
        let scheme = match self.proxy_type {
            ProxyType::HTTP => "http",
            ProxyType::HTTPS => "https",
            ProxyType::SOCKS5 => "socks5",
        };
        format!("{}://{}:{}", scheme, self.host, self.port)
    }
}
