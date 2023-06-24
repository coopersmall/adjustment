use regex::Regex;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::errors::{Error, ErrorCode};

const BASE_URL_PATTERN: &str = r"^(https?://)?(www\.)?([\w-]+\.[a-z]+)(/)?$";
const URL_PATTERN: &str = r"^(https?://)?(www\.)?([\w-]+\.[a-z]+)(/.*)?(\?.*)?$";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Url {
    base_url: Box<str>,
    path: Option<Box<str>>,
    params: Option<HashMap<Box<str>, Box<str>>>,
}

impl Url {
    pub fn new(base_url: &str) -> Result<Self, Error> {
        let re = Regex::new(BASE_URL_PATTERN).unwrap();
        if !re.is_match(base_url) {
            return Err(Error::new(
                format!("Invalid base URL: {}", base_url).as_str(),
                ErrorCode::Invalid,
            ));
        }

        let mut base_url = base_url.to_string();
        if !base_url.ends_with('/') {
            base_url.push('/');
        }

        Ok(Self {
            base_url: base_url.into(),
            path: None,
            params: None,
        })
    }

    pub fn add_path(mut self, path: &str) -> Self {
        let mut path = path.to_string();

        if path.starts_with('/') {
            path.remove(0);
        }

        if !path.ends_with('/') {
            path.push('/');
        }

        self.path = Some(path.into());
        self
    }

    pub fn add_param(mut self, key: &str, value: &str) -> Self {
        if self.params.is_none() {
            self.params = Some(HashMap::new());
        }

        let params = self.params.as_mut().unwrap();
        params.insert(key.into(), value.into());

        self
    }

    pub fn build(self) -> Result<Box<str>, Error> {
        let mut url = self.base_url.to_string();

        if let Some(path) = self.path {
            url.push_str(path.as_ref());
        }

        if let Some(params) = self.params {
            let mut first = true;
            for (key, value) in params {
                if first {
                    url.push('?');
                    first = false;
                } else {
                    url.push('&');
                }

                url.push_str(key.as_ref());
                url.push('=');
                url.push_str(value.as_ref());
            }
        }

        let re = Regex::new(URL_PATTERN).unwrap();
        if !re.is_match(&url) {
            return Err(Error::new(
                format!("Invalid URL: {}", url).as_str(),
                ErrorCode::Invalid,
            ));
        }

        Ok(url.into())
    }
}
