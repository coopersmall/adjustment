use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum ErrorCode {
    Invalid,
    Empty,
    NotFound,
    Unauthorized(UnauthorizedErrorCode),
    Internal(InternalErrorCode),
    Network(NetworkErrorCode),
    Unknown,
    Timeout,
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCode::Invalid => write!(f, "Invalid Error"),
            ErrorCode::Empty => write!(f, "Empty Error"),
            ErrorCode::NotFound => write!(f, "Not Found"),
            ErrorCode::Unauthorized(code) => UnauthorizedErrorCode::fmt(code, f),
            ErrorCode::Internal(code) => InternalErrorCode::fmt(code, f),
            ErrorCode::Network(code) => NetworkErrorCode::fmt(code, f),
            ErrorCode::Unknown => write!(f, "Unknown Error"),
            ErrorCode::Timeout => write!(f, "Timeout Error"),
        }
    }
}

impl Default for ErrorCode {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Error, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum NetworkErrorCode {
    Connection,
    Timeout,
}

impl NetworkErrorCode {
    pub fn get_code_str(&self) -> &str {
        match self {
            NetworkErrorCode::Connection => "CONNECTION_ERROR",
            NetworkErrorCode::Timeout => "TIMEOUT_ERROR",
        }
    }

    pub fn is_network_error(code: &ErrorCode) -> bool {
        match code {
            ErrorCode::Network(_) => true,
            _ => false,
        }
    }
}

impl Display for NetworkErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NetworkErrorCode::Connection => write!(f, "Connection Error"),
            NetworkErrorCode::Timeout => write!(f, "Timeout Error"),
        }
    }
}

#[derive(Error, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum UnauthorizedErrorCode {
    InvalidToken,
    ExpiredToken,
    InvalidCredentials,
}

impl UnauthorizedErrorCode {
    pub fn is_unauthorized_error(code: &ErrorCode) -> bool {
        match code {
            ErrorCode::Unauthorized(_) => true,
            _ => false,
        }
    }
}

impl Display for UnauthorizedErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UnauthorizedErrorCode::InvalidToken => write!(f, "Invalid Token"),
            UnauthorizedErrorCode::ExpiredToken => write!(f, "Expired Token"),
            UnauthorizedErrorCode::InvalidCredentials => write!(f, "Invalid Credentials"),
        }
    }
}

#[derive(Error, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum InternalErrorCode {
    Database(Database),
    Cache(Cache),
    MessageQueue(MessageQueue),
}

impl InternalErrorCode {
    pub fn is_internal_error(code: &ErrorCode) -> bool {
        match code {
            ErrorCode::Internal(_) => true,
            _ => false,
        }
    }
}

impl Display for InternalErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InternalErrorCode::Database(code) => Database::fmt(code, f),
            InternalErrorCode::Cache(code) => Cache::fmt(code, f),
            InternalErrorCode::MessageQueue(code) => MessageQueue::fmt(code, f),
        }
    }
}

#[derive(Error, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Database {
    Connection,
    Query,
    Transaction,
}

impl Display for Database {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Database::Connection => write!(f, "Connection Error"),
            Database::Query => write!(f, "Query Error"),
            Database::Transaction => write!(f, "Transaction Error"),
        }
    }
}

#[derive(Error, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Cache {
    Connection,
    Query,
    Transaction,
}

impl Display for Cache {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Cache::Connection => write!(f, "Connection Error"),
            Cache::Query => write!(f, "Query Error"),
            Cache::Transaction => write!(f, "Transaction Error"),
        }
    }
}

#[derive(Error, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum MessageQueue {
    Connection,
    Query,
    Transaction,
}

impl Display for MessageQueue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageQueue::Connection => write!(f, "Connection Error"),
            MessageQueue::Query => write!(f, "Query Error"),
            MessageQueue::Transaction => write!(f, "Transaction Error"),
        }
    }
}
