pub mod errors;
mod json;

use json::JSON;
pub trait JSONParse<'a, T>: JSON<'a, T> {}
