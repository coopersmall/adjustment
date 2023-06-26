use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HttpResponse {
    status_code: u16,
    headers: Option<HashMap<Box<str>, Box<str>>>,
    body: Option<Box<str>>,
}

impl HttpResponse {
    /// Creates a new `HttpResponse` instance.
    ///
    /// # Arguments
    ///
    /// * `status_code` - The HTTP status code.
    /// * `body` - The response body as a string.
    /// * `headers` - A HashMap of headers, where the keys and values are strings.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::utils::http::HttpResponse;
    /// use std::collections::HashMap;
    ///
    /// let mut headers = HashMap::new();
    /// headers.insert("Content-Type", "application/json");
    ///
    /// let response = HttpResponse::new(200, Some("{\"name\":\"John\"}"), Some(headers));
    /// ```
    pub fn new(status_code: u16, body: Option<&str>, headers: Option<HashMap<&str, &str>>) -> Self {
        let headers = match headers {
            Some(headers) => Some(
                headers
                    .into_iter()
                    .map(|(k, v)| (k.into(), v.into()))
                    .collect(),
            ),
            None => None,
        };

        Self {
            status_code,
            headers,
            body: body.map(|s| s.into()),
        }
    }

    /// Returns the HTTP status code.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::utils::http::HttpResponse;
    ///
    /// let response = HttpResponse::new(200, Some("OK"), None);
    /// assert_eq!(response.status_code(), 200);
    /// ```
    ///
    ///
    pub fn status_code(&self) -> u16 {
        self.status_code
    }

    /// Returns a reference to the headers HashMap.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::utils::http::HttpResponse;
    /// use std::collections::HashMap;
    ///
    /// let mut headers = HashMap::new();
    /// headers.insert("Content-Type", "application/json");
    /// let response = HttpResponse::new(200, Some("OK"), Some(headers));
    /// ```
    ///
    ///
    pub fn headers(&self) -> &Option<HashMap<Box<str>, Box<str>>> {
        &self.headers
    }

    /// Returns a reference to the response body.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::utils::http::HttpResponse;
    ///
    /// let response = HttpResponse::new(200, Some("OK"), None);
    /// assert_eq!(response.body(), &Some(Box::from("OK")));
    /// ```
    ///
    ///
    pub fn body(&self) -> &Option<Box<str>> {
        &self.body
    }

    /// Checks if the response is successful.
    ///
    /// A response is considered successful if the status code is in the range 200-299.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::utils::http::HttpResponse;
    ///
    /// let response = HttpResponse::new(200, Some("OK"), None);
    /// assert_eq!(response.is_successful(), true);
    /// ```
    ///
    ///
    pub fn is_successful(&self) -> bool {
        self.status_code >= 200 && self.status_code < 300
    }
}
