//!
//!  _____                         
//! | ____|_ __ _ __ ___  _ __ ___
//! |  _| | '__| '__/ _ \| '__/ __|
//! | |___| |  | | | (_) | |  \__ \
//! |_____|_|  |_|  \___/|_|  |___/
//!
//!
//! # Error Handling
//!
//! The `errors` module provides error handling functionality for the "Adjustment Mining" project. It defines
//! the `Error` struct, which represents an error in the application. The struct contains fields such as
//! `message`, `code`, `meta`, `is_transient`, and `source`, allowing for detailed error reporting and handling.
//!
//! ## Usage
//!
//! ```
//! use utils::errors::{Error, ErrorCode, ErrorMeta};
//!
//! fn main() {
//!     // Creating a new error
//!     let mut error = Error::new("Something went wrong", ErrorCode::Internal);
//!
//!     // Attaching a cause to the error
//!     let cause = std::io::Error::new(std::io::ErrorKind::Other, "I/O error");
//!
//!     // Adding the cause to the error
//!     error = error.with_cause(cause);
//!
//!     // Creating metadata to attach to the error
//!     let mut meta = ErrorMeta::new()
//!     .add("key1", "value1")
//!     .add("key2", "value2")
//!     .build();
//!
//!     // Adding the metadata to the error
//!     error = error.with_meta(meta);
//!
//!     let error = &error;
//!
//!     assert_eq!(error.message(), "Something went wrong");
//!     assert_eq!(error.code(), ErrorCode::Internal);
//!     assert!(error.source().is_some());
//!
//!     assert_eq!(error.meta().unwrap().get("key1"), Some(&Box::from("value1")));
//!     assert_eq!(error.meta().unwrap().get("key2"), Some(&Box::from("value2")));
//!
//!     // New errors are transient by default
//!     assert!(error.is_transient());
//!
//!     // Fatal errors should be created using `permanent`
//!     let error = Error::permanent("Something went wrong", ErrorCode::Unknown);
//!     assert!(!error.is_transient());
//! }
//! ```
//!
//! ## Error Codes
//!
//! The `ErrorCode` enum represents different error codes that can occur in the application. It provides
//! a human-readable string representation of each error code.
//!
//! ```
//! use utils::errors::ErrorCode;
//!
//! fn main() {
//!     let code = ErrorCode::Invalid;
//!
//!     assert_eq!(code.to_string(), "invalid");
//! }
//! ```
//!
//! ## Error Metadata
//!
//! The `ErrorMeta` struct allows attaching additional metadata to an error. Metadata is stored as key-value
//! pairs in a `HashMap`. The `ErrorMetaBuilder` is used to construct the metadata with a fluent API.
//!
//! ```
//! use utils::errors::{ErrorMeta, ErrorMetaBuilder};
//!
//! fn main() {
//!     let mut meta = ErrorMeta::new()
//!     .add("key1", "value1")
//!     .add("key2", "value2")
//!     .build();
//!
//!     assert_eq!(meta.get("key1"), Some(&"value1".into()));
//!     assert_eq!(meta.get("key2"), Some(&"value2".into()));
//! }
//! ```
//!
//! ## Error Handling
//!
//! The `Error` struct represents an error in the application. It contains information such as the error
//! message, error code, metadata, whether the error is transient, and the underlying cause of the error.
//!
//! ```
//! use utils::errors::{Error, ErrorCode};
//!
//! fn main() {
//!     // Creating a new error
//!     let error = Error::new("Something went wrong", ErrorCode::Internal);
//!
//!     assert_eq!(&error.message(), &"Something went wrong");
//!     assert_eq!(&error.code(), &ErrorCode::Internal);
//!     assert!(&error.meta().is_none());
//!     assert!(&error.is_transient());
//!     assert!(&error.source().is_none());
//! }
//! ```

use serde::{ser::SerializeStruct, Deserialize, Serialize};
use thiserror::Error;

use std::{
    collections::HashMap,
    error::Error as StdError,
    fmt::{Display, Formatter},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ErrorCode {
    /// The provided input is invalid.
    Invalid,
    /// The requested resource was not found.
    NotFound,
    /// The user is not authorized to perform the requested action.
    Unauthorized,
    /// The requested action is forbidden.
    Forbidden,
    /// The request parameters are unprocessable.
    Unprocessable,
    /// An internal server error occurred.
    Internal,
    /// The requested service is currently unavailable.
    Unavailable,
    /// There was a conflict with the requested resource.
    Conflict,
    /// The operation timed out.
    Timeout,
    /// An unknown error occurred.
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

pub struct ErrorMeta(HashMap<Box<str>, Box<str>>);

impl ErrorMeta {
    /// Creates metadata that can be used to attach to an error.
    /// The metadata is empty by default.
    ///
    /// # Returns
    /// A new `ErrorMetaBuilder` instance.
    ///
    ///
    /// # Example
    /// ```
    /// use utils::errors::ErrorMeta;
    /// let meta = ErrorMeta::new().build();
    /// assert_eq!(meta.len(), 0);
    /// ```
    ///
    pub fn new() -> ErrorMetaBuilder {
        ErrorMetaBuilder::new()
    }
}

impl Default for ErrorMeta {
    fn default() -> Self {
        ErrorMeta(HashMap::new())
    }
}

impl From<HashMap<Box<str>, Box<str>>> for ErrorMeta {
    fn from(meta: HashMap<Box<str>, Box<str>>) -> Self {
        ErrorMeta(meta)
    }
}

impl Into<HashMap<Box<str>, Box<str>>> for ErrorMeta {
    fn into(self) -> HashMap<Box<str>, Box<str>> {
        self.0
    }
}

pub struct ErrorMetaBuilder(HashMap<Box<str>, Box<str>>);

impl ErrorMetaBuilder {
    fn new() -> Self {
        ErrorMetaBuilder(HashMap::new())
    }

    /// Adds a key-value pair to the metadata.
    /// If the key already exists, the value is overwritten.
    ///
    /// # Arguments
    /// * `key` - The key.
    /// * `value` - The value.
    ///
    /// # Returns
    /// The builder instance.
    ///
    /// # Example
    /// ```
    /// use utils::errors::ErrorMeta;
    /// let meta = ErrorMeta::new().add("key", "value").build();
    /// assert_eq!(meta.get("key"), Some(&"value".into()));
    /// ```
    ///
    pub fn add(&mut self, key: &str, value: &str) -> &mut Self {
        self.0.insert(key.into(), value.into());
        self
    }

    /// Creates a new Boxed HashMap of Boxed strs from the builder.
    /// The builder is emptied after the call.
    ///
    /// # Returns
    /// A new Boxed HashMap of Boxed strs.
    ///
    /// # Example
    /// ```
    /// use utils::errors::ErrorMeta;
    /// let meta = ErrorMeta::new().add("key", "value").build();
    /// assert_eq!(meta.get("key"), Some(&"value".into()));
    /// ```
    ///
    pub fn build(&mut self) -> HashMap<Box<str>, Box<str>> {
        std::mem::take(&mut self.0)
    }
}

/// An error that can occur in the application.
/// The error contains information such as the error message, error code, metadata, whether the error is transient,
/// and the underlying cause of the error.
///
/// # Example
/// ```
/// use utils::errors::{Error, ErrorCode};
///
/// let error = Error::new("error", ErrorCode::Internal);
/// assert_eq!(&error.message(), &"error");
/// assert_eq!(&error.code(), &ErrorCode::Internal);
/// ```
///
#[derive(Debug, Error)]
pub struct Error {
    message: Box<str>,
    code: ErrorCode,
    meta: Option<HashMap<Box<str>, Box<str>>>,
    is_transient: bool,
    source: Option<Box<dyn StdError + Send + Sync>>,
}

impl Error {
    /// Creates a new `Error` instance with the given message and error code.
    /// The error is marked as transient.
    ///
    /// # Arguments
    ///
    /// * `message` - The error message.
    /// * `code` - The error code.
    ///
    /// # Returns
    ///
    /// A new `Error` instance.
    ///
    /// # Example
    /// ```
    /// use utils::errors::{Error, ErrorCode};
    /// let error = Error::new("error", ErrorCode::Internal);
    /// assert_eq!(error.is_transient(), true);
    /// ```
    ///
    pub fn new(message: &str, code: ErrorCode) -> Error {
        Error {
            message: message.into(),
            code,
            meta: None,
            is_transient: true,
            source: None,
        }
    }

    /// Creates a new `Error` instance with the given message and error code.
    /// The error is marked as permanent.
    ///
    /// # Arguments
    ///
    /// * `message` - The error message.
    /// * `code` - The error code.
    ///
    /// # Returns
    ///
    /// A new `Error` instance.
    ///
    /// # Example
    /// ```
    /// use utils::errors::{Error, ErrorCode};
    /// let error = Error::permanent("error", ErrorCode::Internal);
    /// assert_eq!(error.is_transient(), false);
    /// ```
    ///
    pub fn permanent(message: &str, code: ErrorCode) -> Error {
        Error {
            message: message.into(),
            code,
            meta: None,
            is_transient: false,
            source: None,
        }
    }

    /// Attaches a cause to the error.
    ///
    /// # Arguments
    ///
    /// * `cause` - The underlying cause of the error.
    ///
    /// # Returns
    ///
    /// The error instance with the cause attached.
    ///
    /// # Example
    /// ```
    /// use utils::errors::{Error, ErrorCode};
    /// use std::io;
    /// let io_error = io::Error::new(io::ErrorKind::Other, "io error");
    /// let error = Error::new("error", ErrorCode::Internal).with_cause(io_error);
    /// ```
    ///
    /// # Note
    /// The cause must implement `std::error::Error`.
    ///
    pub fn with_cause<T>(mut self, cause: T) -> Self
    where
        T: StdError + Send + Sync + 'static,
    {
        self.source = Some(Box::new(cause));
        self
    }

    /// Attaches metadata to the error.
    ///
    /// # Arguments
    ///
    /// * `meta` - The metadata to attach to the error.
    ///
    /// # Returns
    ///
    /// The error instance with the metadata attached.
    ///
    /// # Example
    /// ```
    ///
    /// use utils::errors::{Error, ErrorMeta, ErrorCode};
    /// let mut meta = ErrorMeta::new()
    /// .add("key", "value").build();
    /// let error = Error::new("error", ErrorCode::Internal).with_meta(meta);
    /// ```
    ///
    pub fn with_meta(mut self, meta: HashMap<Box<str>, Box<str>>) -> Self {
        self.meta = Some(meta);
        self
    }

    /// Returns the error message.
    ///
    /// # Returns
    /// The error message.
    ///
    /// # Example
    /// ```
    /// use utils::errors::{Error, ErrorCode};
    /// let error = Error::new("error", ErrorCode::Internal);
    /// assert_eq!(error.message(), "error");
    /// ```
    ///
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the error code.
    ///
    /// # Returns
    /// The error code.
    ///
    /// # Example
    /// ```
    ///
    /// use utils::errors::{Error, ErrorCode};
    /// let error = Error::new("error", ErrorCode::Internal);
    /// assert_eq!(error.code(), ErrorCode::Internal);
    /// ```
    ///
    pub fn code(&self) -> ErrorCode {
        self.code
    }

    /// Returns the metadata attached to the error.
    /// The metadata is a HashMap of Boxed strs.
    ///
    /// # Returns
    /// The metadata attached to the error.
    ///
    /// # Example
    /// ```
    /// use utils::errors::{Error, ErrorMeta, ErrorCode};
    ///
    /// let mut meta = ErrorMeta::new()
    /// .add("key", "value")
    /// .build();
    ///
    /// let error = Error::new("error", ErrorCode::Internal).with_meta(meta);
    ///
    /// let meta = error.meta().unwrap();
    /// assert_eq!(meta.get("key").unwrap(), &Box::from("value"));
    /// ```
    ///
    pub fn meta(&self) -> Option<&HashMap<Box<str>, Box<str>>> {
        self.meta.as_ref()
    }

    /// Returns the metadata value for the given key.
    ///
    /// # Arguments
    /// * `key` - The key of the metadata value to return.
    ///
    /// # Returns
    /// The metadata value for the given key.
    ///
    /// # Example
    /// ```
    /// use utils::errors::{Error, ErrorMeta, ErrorCode};
    ///
    /// let mut meta = ErrorMeta::new()
    /// .add("key", "value")
    /// .build();
    ///
    /// let error = Error::new("error", ErrorCode::Internal).with_meta(meta);
    ///
    /// assert_eq!(error.meta_value("key").unwrap(), "value");
    /// ```
    ///
    pub fn meta_value(&self, key: &str) -> Option<&str> {
        self.meta
            .as_ref()
            .and_then(|m| m.get(key).map(|v| v.as_ref()))
    }

    /// Returns the underlying cause of the error.
    /// This is the error that was passed to `with_cause`.
    ///
    /// # Returns
    ///
    /// The underlying cause of the error.
    ///
    /// # Example
    /// ```
    /// use utils::errors::{Error, ErrorCode};
    /// use std::io;
    ///
    /// let io_error = io::Error::new(io::ErrorKind::Other, "io error");
    /// let error = Error::new("error", ErrorCode::Internal).with_cause(io_error);
    ///
    /// assert_eq!(error.source().unwrap().to_string(), "io error");
    /// ```
    ///
    pub fn source(&self) -> Option<&(dyn std::error::Error + Send + Sync + 'static)> {
        self.source.as_ref().map(|e| e.as_ref())
    }

    /// This is the stack trace of the error that was passed to `with_cause`.
    ///
    /// # Returns
    /// The stack trace of the error.
    ///
    /// # Example
    /// ```
    /// use utils::errors::{Error, ErrorCode};
    /// use std::io;
    ///
    /// let io_error = io::Error::new(io::ErrorKind::Other, "io error");
    /// let error = Error::new("error", ErrorCode::Internal).with_cause(io_error);
    ///
    /// assert!(error.get_stack().is_some());
    /// assert!(error.get_stack().unwrap().contains("io error"));
    /// ```
    ///
    pub fn get_stack(&self) -> Option<String> {
        self.source.as_ref().map(|e| format!("{:?}", e))
    }

    /// Indicates whether the error is transient or not.
    ///
    /// # Returns
    /// A boolean indicating whether the error is transient or not.
    ///
    /// # Example
    /// ```
    /// use utils::errors::{Error, ErrorCode};
    /// let error = Error::new("error", ErrorCode::Internal);
    /// assert_eq!(error.is_transient(), true);
    /// ```
    ///
    /// ```
    /// use utils::errors::{Error, ErrorCode};
    /// let error = Error::permanent("error", ErrorCode::Internal);
    /// assert_eq!(error.is_transient(), false);
    /// ```
    ///
    pub fn is_transient(&self) -> bool {
        self.is_transient
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

impl AsRef<Self> for Error {
    fn as_ref(&self) -> &Self {
        self
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
