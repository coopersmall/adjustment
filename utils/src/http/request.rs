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
            self.headers = Some(Self::get_default_headers());
        } else {
            let mut headers = self.headers.take().unwrap();
            let default_headers = Self::get_default_headers();
            for (k, v) in default_headers {
                if !headers.contains_key(&k) {
                    headers.insert(k, v);
                }
            }
            self.headers = Some(headers);
        }
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        if self.headers.is_none() {
            self.headers = Some(Self::get_default_headers());
        }
        let mut headers = self.headers.take().unwrap();
        headers.insert(key.into(), value.into());
        self.headers = Some(headers);
    }

    pub fn body(mut self, body: &str) -> Self {
        self.body = Some(body.into());
        self
    }

    pub fn build(self) -> HttpRequest {
        let headers = match self.headers {
            Some(headers) => headers,
            None => Self::get_default_headers(),
        };

        HttpRequest {
            method: self.method,
            url: self.url,
            agent: self.agent,
            headers: Some(headers),
            body: self.body,
        }
    }

    fn get_default_headers() -> HashMap<Box<str>, Box<str>> {
        let mut headers = HashMap::new();
        headers.insert("User-Agent".into(), DEFAULT_USER_AGENT.into());
        headers.insert("Content-Type".into(), DEFAULT_CONTENT_TYPE.into());
        headers
    }
}
