use serde::{ser::SerializeStruct, Deserialize, Serialize};
use thiserror::Error;

use std::{
    collections::HashMap,
    error::Error as StdError,
    fmt::{Display, Formatter},
    rc::Rc,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ErrorCode {
    Invalid,
    NotFound,
    Unauthorized,
    Forbidden,
    Unprocessable,
    Internal,
    Unavailable,
    Conflict,
    Timeout,
    Unknown,
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCode::Invalid => write!(f, "invalid"),
            ErrorCode::NotFound => write!(f, "not_found"),
            ErrorCode::Unauthorized => write!(f, "unauthorized"),
            ErrorCode::Forbidden => write!(f, "forbidden"),
            ErrorCode::Unprocessable => write!(f, "unprocessable"),
            ErrorCode::Internal => write!(f, "internal"),
            ErrorCode::Unavailable => write!(f, "unavailable"),
            ErrorCode::Conflict => write!(f, "conflict"),
            ErrorCode::Timeout => write!(f, "timeout"),
            ErrorCode::Unknown => write!(f, "unknown"),
        }
    }
}

pub struct ErrorMeta(HashMap<Rc<str>, Rc<str>>);

impl ErrorMeta {
    pub fn new() -> ErrorMetaBuilder {
        ErrorMetaBuilder::new()
    }
}

impl Default for ErrorMeta {
    fn default() -> Self {
        ErrorMeta(HashMap::new())
    }
}

impl From<HashMap<Rc<str>, Rc<str>>> for ErrorMeta {
    fn from(meta: HashMap<Rc<str>, Rc<str>>) -> Self {
        ErrorMeta(meta)
    }
}

impl Into<HashMap<Rc<str>, Rc<str>>> for ErrorMeta {
    fn into(self) -> HashMap<Rc<str>, Rc<str>> {
        self.0
    }
}

pub struct ErrorMetaBuilder(HashMap<Rc<str>, Rc<str>>);

impl ErrorMetaBuilder {
    fn new() -> Self {
        ErrorMetaBuilder(HashMap::new())
    }

    pub fn add(&mut self, key: &str, value: &str) -> &mut Self {
        self.0.insert(key.into(), value.into());
        self
    }

    pub fn build(&mut self) -> Rc<HashMap<Rc<str>, Rc<str>>> {
        Rc::new(std::mem::take(&mut self.0))
    }
}

#[derive(Debug, Error)]
pub struct Error {
    message: Rc<str>,
    code: ErrorCode,
    pub meta: Option<Rc<HashMap<Rc<str>, Rc<str>>>>,
    is_transient: bool,
    source: Option<Box<dyn StdError>>,
}

impl Error {
    pub fn new(message: &str, code: ErrorCode) -> Error {
        Error {
            message: message.into(),
            code,
            meta: None,
            is_transient: true,
            source: None,
        }
    }

    pub fn permanent(message: &str, code: ErrorCode) -> Error {
        Error {
            message: message.into(),
            code,
            meta: None,
            is_transient: false,
            source: None,
        }
    }
}

impl Error {
    pub fn with_cause(mut self, cause: Box<dyn StdError>) -> Self {
        self.source = Some(cause);
        self
    }

    pub fn with_meta(mut self, meta: Rc<HashMap<Rc<str>, Rc<str>>>) -> Self {
        self.meta = Some(meta.clone());
        self
    }

    pub fn get_stack(&self) -> Option<String> {
        self.source.as_ref().map(|e| format!("{:?}", e))
    }

    pub fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "App Error: {}", self.message)
    }
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Error", 5)?;
        state.serialize_field("message", &self.message.to_string())?;
        state.serialize_field("code", &self.code)?;
        state.serialize_field("meta", &self.meta)?;
        state.serialize_field("is_transient", &self.is_transient)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Error {
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

        impl<'de> serde::de::Visitor<'de> for AppErrorVisitor<'_> {
            type Value = Error;

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
