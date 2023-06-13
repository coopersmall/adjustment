use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorMessage<'a> {
    message: Cow<'a, str>,
}

impl<'a> ErrorMessage<'a> {
    pub fn from_string(message: String) -> Self {
        Self {
            message: Cow::Owned(message),
        }
    }

    pub fn from_static(message: &'static str) -> Self {
        Self {
            message: Cow::Borrowed(message),
        }
    }

    pub fn from_str(message: &'a str) -> Self {
        Self {
            message: Cow::Borrowed(message),
        }
    }

    pub fn to_string(&self) -> String {
        self.message.to_string()
    }

    pub fn to_str(&self) -> &str {
        self.message.as_ref()
    }
}

impl<'a> Display for ErrorMessage<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl<'a> Default for ErrorMessage<'a> {
    fn default() -> Self {
        Self {
            message: Cow::Borrowed(""),
        }
    }
}
