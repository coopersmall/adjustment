use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Url {
    base_url: Rc<str>,
    path: Rc<str>,
    params: HashMap<Rc<str>, Rc<str>>,
}

impl Url {
    pub fn new(base_url: &str, path: &str) -> Self {
        Self {
            base_url: base_url.into(),
            path: path.into(),
            params: HashMap::new(),
        }
    }

    pub fn add_param(mut self, key: &str, value: &str) -> Self {
        self.params.insert(key.into(), value.into());
        self
    }

    pub fn build(self) -> Arc<str> {
        let mut url = String::from(self.base_url.as_ref());
        url.push_str(self.path.as_ref());

        if !self.params.is_empty() {
            url.push('?');
            for (key, value) in self.params {
                url.push_str(key.as_ref());
                url.push('=');
                url.push_str(value.as_ref());
                url.push('&');
            }
            url.pop();
        }

        url.into()
    }
}
