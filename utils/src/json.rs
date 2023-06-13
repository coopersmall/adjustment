use serde::{Deserialize, Serialize};
use serde_json::Error;

pub trait Parse<'a, T>: Serialize + Deserialize<'a> {
    fn unmarshal(data: &'a str) -> Result<Self, Error>
    where
        Self: Sized,
    {
        serde_json::from_str(data)
    }

    fn marshal(&'a self) -> Result<String, Error> {
        serde_json::to_string(self)
    }
}

impl<'a, T> Parse<'a, T> for T where T: Serialize + Deserialize<'a> {}
