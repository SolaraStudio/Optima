pub struct MediaStream {
    pub url: String,
    pub mime_type: String,
    pub is_live: bool,
}

impl MediaStream {
    pub fn new(url: &str, mime_type: &str) -> Self {
        MediaStream {
            url: url.to_string(),
            mime_type: mime_type.to_string(),
            is_live: false,
        }
    }

    pub fn live(url: &str, mime_type: &str) -> Self {
        MediaStream {
            url: url.to_string(),
            mime_type: mime_type.to_string(),
            is_live: true,
        }
    }
}
