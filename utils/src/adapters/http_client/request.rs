use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

const DEFAULT_USER_AGENT: &str = "Coop";
const DEFAULT_CONTENT_TYPE: &str = "application/json";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub url: Box<str>,
    pub agent: Box<str>,
    pub headers: Option<HashMap<Box<str>, Box<str>>>,
    pub body: Option<Box<str>>,
}

impl HttpRequest {
    pub fn new(url: &str, method: HttpMethod) -> HttpRequestBuilder {
        HttpRequestBuilder::new(url, method)
    }
}

pub struct HttpRequestBuilder {
    method: HttpMethod,
    url: Box<str>,
    agent: Box<str>,
    headers: Option<HashMap<Box<str>, Box<str>>>,
    body: Option<Box<str>>,
}

impl HttpRequestBuilder {
    pub fn new(url: &str, method: HttpMethod) -> Self {
        Self {
            method,
            url: url.into(),
            agent: "".into(),
            headers: None,
            body: None,
        }
    }

    pub fn agent(mut self, agent: &str) -> Self {
        self.agent = agent.into();
        self
    }

    pub fn headers(mut self, headers: HashMap<&str, &str>) -> Self {
        self.headers = Some(
            headers
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        );
        self
    }

    pub fn add_default_headers(&mut self) {
        if self.headers.is_none() {
            self.headers = Some(HashMap::new());
        }

        let headers = self.headers.as_mut().unwrap();

        if !headers.contains_key("User-Agent") {
            headers.insert("User-Agent".into(), DEFAULT_USER_AGENT.into());
        }

        if !headers.contains_key("Content-Type") {
            headers.insert("Content-Type".into(), DEFAULT_CONTENT_TYPE.into());
        }
    }

    pub fn body(mut self, body: &str) -> Self {
        self.body = Some(body.into());
        self
    }

    pub fn build(self) -> HttpRequest {
        HttpRequest {
            method: self.method,
            url: self.url,
            agent: self.agent,
            headers: self.headers,
            body: self.body,
        }
    }
}
