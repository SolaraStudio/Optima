use reqwest::blocking::Client;

pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub fn get(&self, url: &str) -> Result<String, reqwest::Error> {
        let resp = self.client.get(url).send()?;
        resp.text()
    }
}
