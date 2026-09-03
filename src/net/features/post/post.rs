use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum PostBodyKind {
    Form,
    Json,
    Multipart,
}

#[derive(Debug, Clone)]
pub struct PostField {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct FilePart {
    pub field_name: String,
    pub file_name: String,
    pub content: Vec<u8>,
    pub content_type: String,
}

#[derive(Debug, Clone)]
pub struct PostBody {
    pub kind: PostBodyKind,
    pub fields: Vec<PostField>,
    pub json: Option<serde_json::Value>,
    pub files: Vec<FilePart>,
}

impl PostBody {
    pub fn new_form() -> Self {
        PostBody {
            kind: PostBodyKind::Form,
            fields: Vec::new(),
            json: None,
            files: Vec::new(),
        }
    }

    pub fn new_json(value: serde_json::Value) -> Self {
        PostBody {
            kind: PostBodyKind::Json,
            fields: Vec::new(),
            json: Some(value),
            files: Vec::new(),
        }
    }

    pub fn new_multipart() -> Self {
        PostBody {
            kind: PostBodyKind::Multipart,
            fields: Vec::new(),
            json: None,
            files: Vec::new(),
        }
    }

    pub fn field(mut self, name: &str, value: &str) -> Self {
        self.fields.push(PostField {
            name: name.to_string(),
            value: value.to_string(),
        });
        self
    }

    pub fn file(
        mut self,
        field: &str,
        file_name: &str,
        content: Vec<u8>,
        content_type: &str,
    ) -> Self {
        self.files.push(FilePart {
            field_name: field.to_string(),
            file_name: file_name.to_string(),
            content,
            content_type: content_type.to_string(),
        });
        self
    }

    pub fn to_form_encoded(&self) -> String {
        self.fields
            .iter()
            .map(|f| format!("{}={}", f.name, f.value))
            .collect::<Vec<_>>()
            .join("&")
    }

    pub fn to_json_bytes(&self) -> Result<Vec<u8>, String> {
        match &self.json {
            Some(value) => serde_json::to_vec(value).map_err(|e| e.to_string()),
            None => serde_json::to_vec(&serde_json::Map::new()).map_err(|e| e.to_string()),
        }
    }

    pub fn to_multipart_boundary(&self, boundary: &str) -> Vec<u8> {
        let mut out = Vec::new();
        for f in &self.fields {
            out.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
            out.extend_from_slice(
                format!(
                    "Content-Disposition: form-data; name=\"{}\"\r\n\r\n{}\r\n",
                    f.name, f.value
                )
                .as_bytes(),
            );
        }
        for file in &self.files {
            out.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
            out.extend_from_slice(
                format!(
                    "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
                    file.field_name, file.file_name
                )
                .as_bytes(),
            );
            out.extend_from_slice(
                format!("Content-Type: {}\r\n\r\n", file.content_type).as_bytes(),
            );
            out.extend_from_slice(&file.content);
            out.extend_from_slice(b"\r\n");
        }
        out.extend_from_slice(format!("--{}--\r\n", boundary).as_bytes());
        out
    }

    pub fn content_type(&self, _boundary: &str) -> &'static str {
        match self.kind {
            PostBodyKind::Form => "application/x-www-form-urlencoded",
            PostBodyKind::Json => "application/json",
            PostBodyKind::Multipart => "multipart/form-data",
        }
    }
}

pub struct PostBuilder {
    pub url: String,
    pub body: PostBody,
    pub headers: HashMap<String, String>,
    pub custom_boundary: Option<String>,
}

impl PostBuilder {
    pub fn new(url: &str) -> Self {
        PostBuilder {
            url: url.to_string(),
            body: PostBody::new_form(),
            headers: HashMap::new(),
            custom_boundary: None,
        }
    }

    pub fn url(mut self, url: &str) -> Self {
        self.url = url.to_string();
        self
    }

    pub fn form(mut self) -> Self {
        self.body = PostBody::new_form();
        self
    }

    pub fn json(mut self, value: serde_json::Value) -> Self {
        self.body = PostBody::new_json(value);
        self
    }

    pub fn multipart(mut self) -> Self {
        self.body = PostBody::new_multipart();
        self
    }

    pub fn field(mut self, name: &str, value: &str) -> Self {
        self.body = self.body.field(name, value);
        self
    }

    pub fn file(
        mut self,
        field: &str,
        file_name: &str,
        content: Vec<u8>,
        content_type: &str,
    ) -> Self {
        self.body = self.body.file(field, file_name, content, content_type);
        self
    }

    pub fn header(mut self, name: &str, value: &str) -> Self {
        self.headers.insert(name.to_string(), value.to_string());
        self
    }

    pub fn boundary(mut self, boundary: &str) -> Self {
        self.custom_boundary = Some(boundary.to_string());
        self
    }

    pub fn build(&self) -> Result<(String, PostBody, HashMap<String, String>, Vec<u8>), String> {
        let mut headers = self.headers.clone();
        let boundary = self
            .custom_boundary
            .clone()
            .unwrap_or_else(|| "OptimaBoundary".to_string());
        let content_type = if self.body.kind == PostBodyKind::Multipart {
            format!("multipart/form-data; boundary={}", boundary)
        } else {
            self.body.content_type(&boundary).to_string()
        };
        headers.insert("content-type".to_string(), content_type);

        let body_bytes = match self.body.kind {
            PostBodyKind::Form => self.body.to_form_encoded().into_bytes(),
            PostBodyKind::Json => self.body.to_json_bytes().unwrap_or_default(),
            PostBodyKind::Multipart => self.body.to_multipart_boundary(&boundary),
        };

        Ok((self.url.clone(), self.body.clone(), headers, body_bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn form_encoding() {
        let body = PostBody::new_form()
            .field("user", "alice")
            .field("pwd", "secret");
        assert_eq!(body.to_form_encoded(), "user=alice&pwd=secret");
        assert_eq!(body.kind, PostBodyKind::Form);
    }

    #[test]
    fn json_body() {
        let value = serde_json::json!({ "name": "optima" });
        let body = PostBody::new_json(value);
        let bytes = body.to_json_bytes().unwrap();
        let decoded: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(decoded["name"], "optima");
    }

    #[test]
    fn multipart_builds_boundary() {
        let body = PostBody::new_multipart().field("title", "hello").file(
            "upload",
            "f.txt",
            Vec::from("data"),
            "text/plain",
        );
        let bytes = body.to_multipart_boundary("B0");
        let text = String::from_utf8(bytes).unwrap();
        assert!(text.contains("--B0\r\n"));
        assert!(text.contains("name=\"upload\""));
        assert!(text.contains("filename=\"f.txt\""));
    }

    #[test]
    fn builder_returns_tuple() {
        let result = PostBuilder::new("https://example.com")
            .json(serde_json::json!({ "a": 1 }))
            .build();
        let (url, body, headers, _bytes) = result.unwrap();
        assert_eq!(url, "https://example.com");
        assert_eq!(body.kind, PostBodyKind::Json);
        assert_eq!(headers["content-type"], "application/json");
    }

    #[test]
    fn builder_multipart_sets_boundary() {
        let result = PostBuilder::new("https://x.com")
            .multipart()
            .boundary("xyz")
            .field("a", "1")
            .build();
        let (_url, _body, headers, bytes) = result.unwrap();
        assert!(headers["content-type"].contains("boundary=xyz"));
        assert!(String::from_utf8(bytes).unwrap().contains("--xyz"));
    }
}
