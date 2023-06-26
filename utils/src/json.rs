//!
//!      _ ____   ___  _   _
//!     | / ___| / _ \| \ | |
//!  _  | \___ \| | | |  \| |
//! | |_| |___) | |_| | |\  |
//!  \___/|____/ \___/|_| \_|
//!
//! # JSON
//!
//! This module provides a macro for defining JSON literals in Rust code and a trait for parsing JSON strings into structs and serializing structs into JSON strings
//!
//! ## JSON Macro
//! The `json!` macro is a wrapper around the serde_json::json! macro
//! ```
//! use utils::json;
//!
//! let data = json!({
//!    "name": "John Doe",
//!    "age": 30,
//!    "city": "New York"
//! });
//!
//! assert_eq!(data["name"], "John Doe");
//! assert_eq!(data["age"], 30);
//! assert_eq!(data["city"], "New York");
//!    ```
//!
//! ## JSON Trait
//! The `JSON` trait is implemented for all structs that use the `#[json_parse]` macro from the macros crate
//! ```
//! use serde::{Deserialize, Serialize};
//!
//! use utils::json::JSON;
//! use utils::json;
//!
//! #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
//! pub struct Person {
//!   name: String,
//!   age: u8,
//!   city: String,
//! }
//!
//! impl Person {
//!    pub fn new(name: String, age: u8, city: String) -> Self {
//!      Self { name, age, city }
//!    }
//! }
//!
//! // Create a JSON object from a struct
//! let data = json!({
//!    "name": "John Doe",
//!    "age": 30,
//!    "city": "New York"
//! });
//!
//! let test: Person = Person::from_json(&data.to_string()).unwrap();
//! let expected = Person::new("John Doe".to_string(), 30, "New York".to_string());
//!
//! assert_eq!(test, expected);
//!
//! let test = test.to_json().unwrap();
//! let expected = r#"{"name":"John Doe","age":30,"city":"New York"}"#;
//! assert_eq!(test, expected);
//!
//! // Error handling
//! // Returns an error with JsonParse error code if the JSON string cannot be parsed
//! use utils::errors::{Error, ErrorCode};
//!
//! let data = json!({
//!   "name": "John Doe",
//!   "age": "30",
//!   "city": "New York"
//! });
//!
//! let test = match Person::from_json(&data.to_string()) {
//!   Ok(_) => !unreachable!(),
//!   Err(e) => e,
//! };
//!
//! let expected = Error::new(
//!   "Failed to parse JSON",
//!   ErrorCode::JsonParse
//! );
//!
//! assert_eq!(test.message(), expected.message());
//! assert_eq!(test.code(), expected.code());
//! ```
//!

use serde::{Deserialize, Serialize};

use crate::errors::{Error, ErrorCode};

/// Macro for defining JSON literals in Rust code
/// This macro is a wrapper around the serde_json::json! macro
///
/// # Arguments
/// * `tokens` - JSON tokens
///
/// # Example
/// ```
/// fn main() {
/// use utils::json;
///
/// let data = json!({
///     "name": "John Doe",
///     "age": 30,
///     "city": "New York"
/// });
///
/// assert_eq!(data["name"], "John Doe");
/// assert_eq!(data["age"], 30);
/// assert_eq!(data["city"], "New York");
/// }
/// ```
///
#[macro_export]
macro_rules! json {
    ($($tokens:tt)*) => {
        serde_json::json!($($tokens)*)
    };
}

/// Trait for parsing JSON strings into structs and serializing structs into JSON strings
///
/// This trait is implemented for all structs that use the `#[json_parse]` macro from the macros crate
/// Import this trait to use the `from_json` and `to_json` methods on structs
///
pub trait JSON<'a, T>: Serialize + Deserialize<'a> {
    /// Parse a JSON string into a struct
    ///
    /// # Arguments
    /// * `data` - JSON string
    ///
    /// # Example
    /// ```
    /// use serde::{Deserialize, Serialize};
    /// use utils::json::JSON;
    /// use utils::json;
    ///
    /// // Any object outside of the utils crate SHOULD NOT USE `#[derive(Serialize, Deserialize)]`
    /// // Instead objects should use `#[json_parse]` from the macros crate
    /// #[derive(Serialize, Deserialize, PartialEq, Debug)]
    /// struct Person {
    ///    name: String,
    ///    phones: Vec<String>,
    /// }
    ///
    /// let data = json!({
    ///    "name": "John Doe",
    ///    "phones": [
    ///    "+44 1234567",
    ///    "+44 2345678"
    ///    ]
    /// });
    ///
    /// let test: Person = Person::from_json(&data.to_string()).unwrap();
    /// let expected = Person {
    ///   name: "John Doe".to_string(),
    ///   phones: vec!["+44 1234567".to_string(), "+44 2345678".to_string()],
    /// };
    ///
    /// assert_eq!(test, expected);
    /// ```
    ///
    /// # Errors
    /// Returns an error with JsonParse error code if the JSON string cannot be parsed
    ///
    fn from_json(data: &'a str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match serde_json::from_str(data) {
            Ok(v) => Ok(v),
            Err(err) => {
                Err(Error::new("Failed to parse JSON", ErrorCode::JsonParse).with_cause(err))
            }
        }
    }

    /// Convert a struct into a JSON string
    ///
    /// # Example
    /// ```
    /// use serde::{Deserialize, Serialize};
    /// use utils::json::JSON;
    /// use utils::json;
    ///
    /// // Any object outside of the utils crate SHOULD NOT USE `#[derive(Serialize, Deserialize)]`
    /// // Instead objects should use `#[json_parse]` from the macros crate
    /// #[derive(Serialize, Deserialize, Debug)]
    /// struct Person {
    ///   name: String,
    ///   phones: Vec<String>,
    /// }
    ///
    /// let test = Person {
    ///   name: "John Doe".to_string(),
    ///   phones: vec!["+44 1234567".to_string(), "+44 2345678".to_string()],
    /// };
    ///
    /// let data = test.to_json().unwrap();
    /// let expected = json!({
    ///   "name": "John Doe",
    ///   "phones": [
    ///   "+44 1234567",
    ///   "+44 2345678"
    /// ]
    /// });
    ///
    /// assert_eq!(data, expected.to_string());
    /// ```
    ///
    /// # Errors
    /// Returns an error with JsonSerialize error code if the struct cannot be serialized
    ///
    fn to_json(&'a self) -> Result<String, Error> {
        match serde_json::to_string(self) {
            Ok(v) => Ok(v),
            Err(err) => Err(
                Error::new("Failed to serialize JSON", ErrorCode::JsonSerialize).with_cause(err),
            ),
        }
    }
}

impl<'a, T> JSON<'a, T> for T where T: Serialize + Deserialize<'a> {}
