//! # HTTP Client
//!
//! The `HttpClient` module provides a flexible and easy-to-use HTTP client for making HTTP requests and handling responses.
//!
//! ## Usage
//!
//! To create a new `HttpClient` instance, use the `new` method of the `HttpClientBuilder`. The builder allows you to customize the timeout duration for the client:
//!
//! ```rust
//! use utils::adapters::http_client::HttpClient;
//! use std::time::Duration;
//!
//! let client = HttpClient::new()
//!     .timeout(Duration::from_secs(10))
//!     .build(0);
//! ```
//!
//! Once you have an `HttpClient` instance, you can use the `send_request` method to send HTTP requests and receive responses. The `send_request` method takes an `HttpRequest` object as a parameter and returns an `HttpResponse` object:
//!
//! ```rust
//! use utils::adapters::http_client::HttpClient;
//! use utils::http::{HttpRequest, HttpMethod};
//! use utils::errors::{Error, ErrorCode};
//!
//! let client = HttpClient::new().build(0);
//!
//! // Create an HTTP request
//! let request = HttpRequest::new("https://api.example.com", HttpMethod::GET)
//!     .add_header("Authorization", "Bearer <access_token>");
//!
//! // Send the request and receive the response
//! #[tokio::test]
//! async fn send_request_example() {
//!     let response = client.send_request(request).await;
//!
//!     // Process the response
//!     match response {
//!         Ok(response) => {
//!             println!("Status Code: {}", response.status_code());
//!             println!("Headers: {:?}", response.headers());
//!             println!("Body: {:?}", response.body());
//!         }
//!         Err(error) => {
//!             eprintln!("Error: {}", error);
//!         }
//!     }
//! }
//! ```
//!
//! ## Client Pooling
//!
//! The `HttpClientPool` struct provides a pool of `HttpClient` instances for efficient handling of concurrent requests. The pool allows borrowing and returning clients from the pool, ensuring safe and concurrent access to the clients.
//!
//! To create an `HttpClientPool` with a specific number of clients, use the `with_capacity` method. You can then borrow clients from the pool using the `borrow_client` method and return them using the `return_client` method:
//!
//! ```rust
//! use utils::adapters::http_client::HttpClientPool;
//!
//! let mut pool = HttpClientPool::with_capacity(5);
//!
//! // Borrow a client from the pool
//! let client = pool.borrow_client().unwrap();
//!
//! // Use the client for making requests
//!
//! // Return the client back to the pool
//! pool.return_client(client);
//! ```
//!
//! ## Error Handling
//!
//! The HTTP client provides error handling for various scenarios, such as failed request sending, reading response bodies, or acquiring and releasing clients from the pool. Errors are represented by the `Error` struct and can be inspected for error codes and detailed error messages.
//!
//! ```rust
//! use utils::adapters::http_client::HttpClient;
//! use utils::http::{HttpRequest, HttpMethod};
//! use utils::errors::{Error, ErrorCode};
//!
//! let client = HttpClient::new().build(0);
//!
//! let request = HttpRequest::new("https://api.example.com", HttpMethod::GET).build();
//!
//! // Send the request and handle errors
//! #[tokio::test]
//! async fn send_request_error_handling() {
//!     let response = client.send_request(request).await;
//!
//!     match response {
//!         Ok(response) => {
//!             // Process the response
//!         }
//!         Err(error) => match error.code() {
//!             ErrorCode::Timeout => {
//!                 eprintln!("Request timed out");
//!             }
//!             ErrorCode::Internal => {
//!                 eprintln!("Internal error: {}", error);
//!             }
//!             // Handle other error codes
//!             _ => {
//!                 eprintln!("Error: {}", error);
//!             }
//!         },
//!     }
//! }
//! ```
//!
//! ## Dependencies
//!
//! This module depends on the `reqwest` crate for handling HTTP requests and responses. Make sure to include it in your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! reqwest = "0.11"
//! ```
//!
//! ## Examples
//!
//! Additional examples and usage instructions can be found in the documentation of each struct and method.
//! ```

use rand::Rng;
use reqwest::{Client, Method};

use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, RwLock},
    time::Duration,
};

pub use crate::http::{
    request::{HttpMethod, HttpRequest, HttpRequestBuilder},
    response::HttpResponse,
    url::Url,
};

use crate::errors::{Error, ErrorCode};

const DEFAULT_TIMEOUT_SECONDS: u64 = 30;

pub struct HttpClient {
    client: Client,
    index: usize,
}

impl HttpClient {
    pub fn new() -> HttpClientBuilder {
        HttpClientBuilder::new()
    }
}

impl HttpClient {
    /// Sends an HTTP request and returns the corresponding response.
    ///
    /// # Arguments
    ///
    /// * `request` - An `Arc<HttpRequest>` representing the request to be sent.
    ///
    /// # Examples
    ///
    /// ```
    /// #[tokio::test]
    /// async fn test_send_request() {
    ///     let client = HttpClient::new().build(0);
    ///     let request = Arc::new(HttpRequest::new("https://api.example.com", HttpMethod::GET).build());
    ///
    ///     let response = client.send_request(request).await;
    /// }
    /// ```
    ///
    ///
    pub async fn send_request(&self, request: Arc<HttpRequest>) -> Result<HttpResponse, Error> {
        let method = match request.method {
            HttpMethod::GET => Method::GET,
            HttpMethod::POST => Method::POST,
            HttpMethod::PUT => Method::PUT,
            HttpMethod::DELETE => Method::DELETE,
        };

        let mut request_builder = self
            .client
            .request(method, request.url.to_string())
            .header("User-Agent".to_string(), request.agent.to_string());

        if let Some(headers) = &request.headers {
            for (key, value) in headers {
                request_builder = request_builder.header(key.to_string(), value.to_string());
            }
        }

        let request_builder = if let Some(body) = &request.body {
            request_builder.body(body.to_string())
        } else {
            request_builder
        };

        drop(request);

        let response = match request_builder.send().await {
            Ok(response) => response,
            Err(err) => {
                return Err(Error::new(
                    format!("Failed to send request: {}", err).as_str(),
                    ErrorCode::Internal,
                ));
            }
        };

        let status_code = response.status().as_u16();

        let headers = response.headers().to_owned();
        let headers = if headers.len() == 0 {
            None
        } else {
            Some(
                headers
                    .iter()
                    .fold(HashMap::new(), |mut headers, (key, value)| {
                        let value = match value.to_str() {
                            Ok(value) => value,
                            Err(_) => return headers,
                        };
                        headers.insert(key.as_str(), value);
                        headers
                    }),
            )
        };

        let body = match response.text().await {
            Ok(body) => body,
            Err(err) => {
                return Err(Error::new(
                    format!("Failed to read response body: {}", err).as_str(),
                    ErrorCode::Internal,
                ));
            }
        };

        if body.is_empty() {
            return Ok(HttpResponse::new(status_code, None, headers));
        }

        Ok(HttpResponse::new(status_code, Some(body.as_str()), headers))
    }
}

/// Builder pattern implementation for creating an `HttpClient`.
pub struct HttpClientBuilder {
    timeout: Option<Duration>,
}

impl HttpClientBuilder {
    /// Creates a new instance of `HttpClientBuilder`.
    pub fn new() -> Self {
        Self { timeout: None }
    }

    /// Sets the timeout duration for the HTTP client.
    ///
    /// # Arguments
    ///
    /// * `timeout` - The duration after which the client request will time out.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::utils::adapters::http_client::HttpClientBuilder;
    /// use std::time::Duration;
    ///
    /// let builder = HttpClientBuilder::new().timeout(Duration::from_secs(10));
    /// ```
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Constructs and returns an `HttpClient` instance.
    ///
    /// # Arguments
    ///
    /// * `index` - The index used to identify the client within a client pool.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::utils::adapters::http_client::HttpClientBuilder;
    ///
    /// let builder = HttpClientBuilder::new();
    /// let client = builder.build(0);
    /// ```
    pub fn build(self, index: usize) -> HttpClient {
        let mut client_builder = Client::builder();
        match self.timeout {
            Some(timeout) => client_builder = client_builder.timeout(timeout),
            None => {
                client_builder =
                    client_builder.timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECONDS))
            }
        }

        let client = client_builder.build().unwrap();
        HttpClient { client, index }
    }
}

const DEFAULT_POOL_SIZE: usize = 10;

/// Represents a pool of `HttpClient` instances.
pub struct HttpClientPool {
    clients: Vec<Arc<HttpClient>>,
    borrowed: Arc<RwLock<HashSet<usize>>>,
}

impl HttpClientPool {
    /// Creates a new `HttpClientPool` with the default pool size.
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_POOL_SIZE)
    }

    /// Creates a new `HttpClientPool` with the specified capacity.
    ///
    /// # Arguments
    ///
    /// * `num_clients` - The number of clients to create in the pool.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::utils::adapters::http_client::HttpClientPool;
    ///
    /// let pool = HttpClientPool::with_capacity(5);
    /// ```
    pub fn with_capacity(num_clients: usize) -> Self {
        let mut clients = Vec::with_capacity(num_clients);

        for i in 0..num_clients {
            clients.push(Arc::new(HttpClient::new().build(i)));
        }

        let borrowed = Arc::new(RwLock::new(HashSet::new()));

        Self { clients, borrowed }
    }

    /// Borrows an `HttpClient` from the pool.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::utils::adapters::http_client::HttpClientPool;
    ///
    /// let mut pool = HttpClientPool::new();
    ///
    /// let client = pool.borrow_client().unwrap();
    ///
    /// // Use the client for making requests
    ///
    /// pool.return_client(client);
    /// ```
    pub fn borrow_client<'a>(&'a mut self) -> Result<Arc<HttpClient>, Error> {
        let mut borrowed_set = self.borrowed.try_write().map_err(|err| {
            let poisoned_err = err.to_string();
            Error::new(
                format!("Failed to borrow client: {}", poisoned_err).as_str(),
                ErrorCode::Internal,
            )
        })?;

        let available_clients: Vec<usize> = self
            .clients
            .iter()
            .enumerate()
            .filter(|(index, _)| !borrowed_set.contains(index))
            .map(|(index, _)| index)
            .collect();

        let client_index = if available_clients.is_empty() {
            let index = self.clients.len();
            borrowed_set.insert(index);

            let client = HttpClient::new().build(index);
            self.clients.push(Arc::new(client));
            index
        } else {
            let index = available_clients[rand::thread_rng().gen_range(0..available_clients.len())];
            borrowed_set.insert(index);
            index
        };

        Ok(self.clients[client_index].clone())
    }

    /// Returns a borrowed `HttpClient` back to the pool.
    ///
    /// # Arguments
    ///
    /// * `client` - The borrowed `HttpClient` to return to the pool.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::utils::adapters::http_client::{HttpClient, HttpClientPool};
    /// use std::sync::Arc;
    ///
    /// let mut pool = HttpClientPool::new();
    ///
    /// let client = pool.borrow_client().unwrap();
    ///
    /// // Use the client for making requests
    ///
    /// pool.return_client(client);
    /// ```
    pub fn return_client(&mut self, client: Arc<HttpClient>) {
        let mut borrowed = match self.borrowed.try_write() {
            Ok(borrowed) => borrowed,
            Err(_) => todo!(),
        };
        borrowed.remove(&client.index);
    }
}
