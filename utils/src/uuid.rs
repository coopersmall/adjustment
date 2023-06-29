//!
//! ```md
//!  _   _ _   _ ___ ____  
//! | | | | | | |_ _|  _ \
//! | | | | | | || || | | |
//! | |_| | |_| || || |_| |
//!  \___/ \___/|___|____/
//! ```
//!
//! # UUID
//!
//! This module provides ways to generate and validate UUIDs.
//! Module includes:
//! - UUID Type
//! - Generate a UUID
//! - Validate a UUID
//! - UUID Generate Macro
//!
//! ## UUID Type
//! A UUID is a 36 character string
//! and is in the format of xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
//!
//! ## Generate a UUID
//! ```
//! use utils::uuid;
//! let uuid = uuid::generate();
//! assert!(uuid.len() > 0);
//! assert!(uuid::is_valid(&uuid));
//! ```
//!
//! ## Validate a UUID
//! ```
//! use utils::uuid;
//! assert!(uuid::is_valid("00000000-0000-0000-0000-000000000000"));
//! assert!(!uuid::is_valid("00000000-0000-0000-0000-00000000000"));
//! ```
//!
//! ## UUID Generate Macro
//! ```
//! use utils::uuid;
//! let uuid = uuid!();
//! assert!(uuid.len() > 0);
//! assert!(uuid::is_valid(&uuid));
//! ```

use uuid::Uuid;

/// A UUID is a 36 character string
/// and is in the format of xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
pub type UUID = Box<str>;

/// Generate a new UUID
///
/// # Example
/// ```
/// use utils::uuid;
///
/// let uuid = uuid!();
/// assert!(uuid.len() > 0);
/// assert!(uuid::is_valid(&uuid));
/// ```
#[macro_export]
macro_rules! uuid {
    () => {
        $crate::uuid::generate()
    };
}

/// Generates a new UUID
///
/// # Example
/// ```
/// use utils::uuid;
///
/// let uuid = uuid::generate();
/// assert!(uuid.len() > 0);
/// assert!(uuid::is_valid(&uuid));
/// ```
pub fn generate() -> Box<str> {
    let uuid = Uuid::new_v4();
    uuid.to_string().as_str().into()
}

/// Checks if a given string is a valid UUID
///
/// A UUID is valid if it is a 36 character string
/// and is in the format of xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
///
/// # Example
/// ```
/// use utils::uuid;
///
/// assert!(uuid::is_valid("00000000-0000-0000-0000-000000000000"));
/// assert!(!uuid::is_valid("00000000-0000-0000-0000-00000000000"));
/// ```
pub fn is_valid(uuid: &str) -> bool {
    Uuid::parse_str(uuid).is_ok()
}
