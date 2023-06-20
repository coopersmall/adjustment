use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::adapters::http_client::headers::HttpHeaders;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HttpResponse {
    status_code: u16,
    headers: HttpHeaders,
    body: Arc<str>,
}

impl HttpResponse {
    pub fn builder() -> HttpResponseBuilder {
        HttpResponseBuilder::new()
    }

    pub fn status_code(&self) -> u16 {
        self.status_code
    }

    pub fn headers(&self) -> &HttpHeaders {
        &self.headers
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn is_success(&self) -> bool {
        self.status_code >= 200 && self.status_code < 300
    }
}

pub struct HttpResponseBuilder {
    status_code: u16,
    headers: HttpHeaders,
    body: Arc<str>,
}

impl HttpResponseBuilder {
    pub fn new() -> Self {
        Self {
            status_code: 0,
            headers: HttpHeaders::new(),
            body: "".into(),
        }
    }

    pub fn status_code(mut self, status_code: u16) -> Self {
        self.status_code = status_code;
        self
    }

    pub fn headers(mut self, headers: HttpHeaders) -> Self {
        self.headers = headers;
        self
    }

    pub fn body(mut self, body: String) -> Self {
        self.body = body.into();
        self
    }

    pub fn build(self) -> HttpResponse {
        HttpResponse {
            status_code: self.status_code,
            headers: self.headers,
            body: self.body,
        }
    }
}
