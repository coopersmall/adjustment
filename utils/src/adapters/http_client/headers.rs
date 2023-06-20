use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::TryInto, sync::Arc};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HttpHeaders {
    headers: HashMap<Arc<str>, Arc<str>>,
}

impl<'a> HttpHeaders {
    pub fn new() -> Self {
        Self {
            headers: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: &str, value: &str) {
        self.headers.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.headers.get(key).map(|s| s.as_ref())
    }
}

impl<'a> From<reqwest::header::HeaderMap> for HttpHeaders {
    fn from(header_map: reqwest::header::HeaderMap) -> Self {
        let mut headers = Self::new();

        for (key, value) in header_map.iter() {
            if let Ok(key) = key.as_str().try_into() {
                if let Ok(value) = value.to_str().map(|s| s) {
                    headers.insert(key, value);
                }
            }
        }
        headers
    }
}

impl<'a> std::ops::Deref for HttpHeaders {
    type Target = HashMap<Arc<str>, Arc<str>>;

    fn deref(&self) -> &Self::Target {
        &self.headers
    }
}
