use serde::{ser::SerializeStruct, Deserialize, Serialize};
use thiserror::Error;

use std::{
    collections::HashMap,
    error::Error as StdError,
    fmt::{Display, Formatter},
};

pub mod codes;
pub mod message;
pub mod meta;

pub use codes::ErrorCode;
pub use message::ErrorMessage;
pub use meta::ErrorMeta;

#[derive(Debug, Error)]
pub struct Error<'a> {
    pub message: ErrorMessage<'a>,
    pub code: ErrorCode,
    pub meta: HashMap<String, String>,
    pub is_transient: bool,
    pub source: Option<Box<dyn StdError>>,
}

impl<'a> Error<'a> {
    pub fn new() -> ErrorBuilder<'a> {
        ErrorBuilder::new()
    }
}

impl<'a> Error<'a> {
    fn is_transient_error(&self) -> bool {
        self.is_transient
    }

    fn is_known_error(&self) -> bool {
        self.code != ErrorCode::Unknown
    }

    fn get_stack(&self) -> Option<String> {
        self.source.as_ref().map(|e| format!("{:?}", e))
    }

    fn set_code(&mut self, code: ErrorCode) -> &mut Self {
        self.code = code;
        self
    }
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref())
    }
}

impl<'a> Error<'a> {
    fn from(source: impl Into<Box<dyn std::error::Error + Send + Sync>>) -> Self {
        Error::new().source(source.into()).build()
    }
}

impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "App Error: {}", self.message)
    }
}

impl Default for Error<'_> {
    fn default() -> Self {
        Self {
            message: ErrorMessage::from_str(
                "Application error occurred (please add more details to assist with debugging)",
            ),
            code: ErrorCode::Unknown,
            meta: HashMap::new(),
            is_transient: true,
            source: None,
        }
    }
}

impl<'a> From<ErrorBuilder<'a>> for Error<'a> {
    fn from(builder: ErrorBuilder<'a>) -> Self {
        builder.build()
    }
}

impl<'a> From<Box<dyn std::error::Error>> for Error<'a> {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        Self {
            message: ErrorMessage::from_str(""),
            code: ErrorCode::Unknown,
            meta: HashMap::new(),
            is_transient: true,
            source: Some(error),
        }
    }
}

impl<'a> Serialize for Error<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Error", 5)?;
        state.serialize_field("message", &self.message)?;
        state.serialize_field("code", &self.code)?;
        state.serialize_field("meta", &self.meta)?;
        state.serialize_field("is_transient", &self.is_transient)?;
        state.end()
    }
}

impl<'de, 'a> Deserialize<'de> for Error<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            Message,
            Code,
            Meta,
            IsTransient,
        }

        struct AppErrorVisitor<'a> {
            marker: std::marker::PhantomData<&'a ()>,
        }

        impl<'de, 'a> serde::de::Visitor<'de> for AppErrorVisitor<'a> {
            type Value = Error<'a>;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("struct Error")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: serde::de::SeqAccess<'de>,
            {
                let message = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
                let code = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                let meta = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(2, &self))?;
                let is_transient = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(3, &self))?;

                Ok(Error {
                    message,
                    code,
                    meta,
                    is_transient,
                    source: None,
                })
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut message = None;
                let mut code = None;
                let mut meta = None;
                let mut is_transient = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Message => {
                            if message.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message = Some(map.next_value()?);
                        }
                        Field::Code => {
                            if code.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code = Some(map.next_value()?);
                        }
                        Field::Meta => {
                            if meta.is_some() {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            meta = Some(map.next_value()?);
                        }
                        Field::IsTransient => {
                            if is_transient.is_some() {
                                return Err(serde::de::Error::duplicate_field("is_transient"));
                            }
                            is_transient = Some(map.next_value()?);
                        }
                    }
                }

                let message = message.ok_or_else(|| serde::de::Error::missing_field("message"))?;
                let code = code.ok_or_else(|| serde::de::Error::missing_field("code"))?;
                let meta = meta.ok_or_else(|| serde::de::Error::missing_field("meta"))?;
                let is_transient =
                    is_transient.ok_or_else(|| serde::de::Error::missing_field("is_transient"))?;

                Ok(Error {
                    message,
                    code,
                    meta,
                    is_transient,
                    source: None,
                })
            }
        }

        const FIELDS: &[&str] = &["message", "code", "meta", "is_transient"];
        deserializer.deserialize_struct(
            "Error",
            FIELDS,
            AppErrorVisitor {
                marker: std::marker::PhantomData,
            },
        )
    }
}

pub struct ErrorBuilder<'a> {
    message: Option<ErrorMessage<'a>>,
    code: Option<ErrorCode>,
    meta: Option<HashMap<String, String>>,
    is_transient: Option<bool>,
    source: Option<Box<dyn StdError>>,
}

impl<'a> ErrorBuilder<'a> {
    pub fn new() -> Self {
        Self {
            message: None,
            code: None,
            meta: None,
            is_transient: None,
            source: None,
        }
    }

    pub fn message(mut self, message: ErrorMessage<'a>) -> Self {
        self.message = Some(message);
        self
    }

    pub fn code(mut self, code: ErrorCode) -> Self {
        self.code = Some(code);
        self
    }

    pub fn add_meta(mut self, key: String, value: String) -> Self {
        if self.meta.is_none() {
            self.meta = Some(HashMap::new());
        }

        if let Some(meta) = self.meta.as_mut() {
            meta.insert(key, value);
        }

        self
    }

    pub fn set_meta(mut self, meta: HashMap<String, String>) -> Self {
        self.meta = Some(meta);
        self
    }

    pub fn is_transient(mut self, is_transient: bool) -> Self {
        self.is_transient = Some(is_transient);
        self
    }

    pub fn source(mut self, source: Box<dyn StdError>) -> Self {
        self.source = Some(source);
        self
    }

    pub fn build(self) -> Error<'a> {
        Error {
            message: self.message.unwrap_or_default(),
            code: self.code.unwrap_or_default(),
            meta: self.meta.unwrap_or_default(),
            is_transient: self.is_transient.unwrap_or(true),
            source: self.source,
        }
    }
}

pub fn new_error<'a>() -> ErrorBuilder<'a> {
    ErrorBuilder::new()
}

pub fn from_string(
    message: String,
    code: Option<ErrorCode>,
    meta: Option<HashMap<String, String>>,
    cause: Option<Box<dyn StdError>>,
) -> Error<'static> {
    Error {
        message: ErrorMessage::from_string(message),
        code: code.unwrap_or_default(),
        meta: meta.unwrap_or_default(),
        is_transient: true,
        source: cause,
    }
}

pub fn from_str<'a>(
    message: &'a str,
    code: Option<ErrorCode>,
    meta: Option<HashMap<String, String>>,
    cause: Option<Box<dyn StdError>>,
) -> Error<'a> {
    Error {
        message: ErrorMessage::from_str(message),
        code: code.unwrap_or_default(),
        meta: meta.unwrap_or_default(),
        is_transient: true,
        source: cause,
    }
}

mod errors {
    pub use crate::errors::Error;
    pub use crate::errors::{ErrorCode, ErrorMessage, ErrorMeta};
}
