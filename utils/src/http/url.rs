use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::errors::{Error, ErrorCode};

const BASE_URL_PATTERN: &str = r"^(https?://)?(www\.)?([\w-]+\.[a-z]+)(/)?$";
const URL_PATTERN: &str = r"^(https?://)?(www\.)?([\w-]+\.[a-z]+)(/.*)?(\?.*)?$";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Url {
    base_url: Box<str>,
    path: Option<Box<str>>,
    params: Option<Vec<(Box<str>, Box<str>)>>,
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
            self.params = Some(Vec::new());
        }

        let params = self.params.as_mut().unwrap();
        params.push((Self::encode_url(key).into(), Self::encode_url(value).into()));

        self
    }

    pub fn set_params(mut self, params: Vec<(Box<str>, Box<str>)>) -> Self {
        self.params = Some(params);
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

    fn encode_url(url: &str) -> String {
        utf8_percent_encode(url, NON_ALPHANUMERIC).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url() {
        let url = Url::new("https://www.google.com").unwrap();
        assert_eq!(url.build().unwrap(), "https://www.google.com/".into());

        let url = Url::new("https://www.google.com").unwrap();
        let url = url.add_path("search").build().unwrap();
        assert_eq!(url, "https://www.google.com/search/".into());

        let url = Url::new("https://www.google.com").unwrap();
        let url = url
            .add_path("search")
            .add_param("q", "rust")
            .build()
            .unwrap();

        assert_eq!(url, "https://www.google.com/search/?q=rust".into());

        let url = Url::new("https://www.google.com").unwrap();
        let url = url
            .add_path("search")
            .add_param("q", "rust")
            .add_param("oq", "rust")
            .add_param("aqs", "chrome..69i57j69i60l3j69i65.1053j0j7")
            .build()
            .unwrap();

        assert_eq!(
            url,
            "https://www.google.com/search/?q=rust&oq=rust&aqs=chrome%2E%2E69i57j69i60l3j69i65%2E1053j0j7"
                .into()
        );
    }
}
