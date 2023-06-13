use serde_json::Value;

use std::collections::HashMap;

use super::codes::ErrorCode;
use super::message::ErrorMessage;
use super::Error;

pub struct ErrorMeta {}

impl ErrorMeta {
    pub fn new() -> ErrorMetaBuilder {
        ErrorMetaBuilder::new()
    }

    pub fn from_json(json: &str) -> Result<HashMap<String, String>, Error> {
        let json_value = match serde_json::from_str::<Value>(json) {
            Ok(json_value) => json_value,
            Err(error) => Err(Error::new()
                .message(ErrorMessage::from_string(format!(
                    "Error parsing json: {}",
                    error
                )))
                .code(ErrorCode::Invalid)
                .source(error.into())
                .build())?,
        };

        let meta_value = json_value["meta"].clone();

        if let Value::Object(meta_object) = meta_value {
            let mut meta = HashMap::new();
            for (key, value) in meta_object {
                if let Value::String(value) = value {
                    meta.insert(key, value);
                }
            }
            return Ok(meta);
        }

        Err(Error::new()
            .message(ErrorMessage::from_string(format!(
                "Error parsing json: {}",
                "Invalid meta"
            )))
            .code(ErrorCode::Invalid)
            .build())
    }
}

pub struct ErrorMetaBuilder {
    meta: Option<HashMap<String, String>>,
}

impl ErrorMetaBuilder {
    pub fn new() -> Self {
        Self { meta: None }
    }

    pub fn add_meta(mut self, key: String, value: String) -> Self {
        let mut meta = self.meta.unwrap_or_default();
        meta.insert(key, value);
        self.meta = Some(meta);
        self
    }

    pub fn set_meta(mut self, meta: HashMap<String, String>) -> Self {
        self.meta = Some(meta);
        self
    }

    pub fn build(self) -> HashMap<String, String> {
        self.meta.unwrap_or_default()
    }
}
