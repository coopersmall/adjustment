use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HttpResponse {
    status_code: u16,
    headers: HashMap<Box<str>, Box<str>>,
    body: Box<str>,
}

impl HttpResponse {
    pub fn new(status_code: u16, body: &str, headers: HashMap<&str, &str>) -> Self {
        let headers = headers
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();

        Self {
            status_code,
            headers,
            body: body.into(),
        }
    }

    pub fn status_code(&self) -> u16 {
        self.status_code
    }

    pub fn headers(&self) -> &HashMap<Box<str>, Box<str>> {
        &self.headers
    }

    pub fn body(&self) -> &Box<str> {
        &self.body
    }

    pub fn is_successful(&self) -> bool {
        self.status_code >= 200 && self.status_code < 300
    }
}
