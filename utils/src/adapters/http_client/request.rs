use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub url: Arc<str>,
    pub agent: Arc<str>,
    pub headers: Option<HashMap<Arc<str>, Arc<str>>>,
    pub body: Option<Arc<str>>,
}

impl HttpRequest {
    pub fn new(url: &str, method: HttpMethod) -> HttpRequestBuilder {
        HttpRequestBuilder::new(url, method)
    }
}

pub struct HttpRequestBuilder {
    method: HttpMethod,
    url: Arc<str>,
    agent: Arc<str>,
    headers: Option<HashMap<Arc<str>, Arc<str>>>,
    body: Option<Arc<str>>,
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
