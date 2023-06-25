use async_trait::async_trait;
use rand::Rng;
use reqwest::{Client, Method};
use thiserror::Error;

use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use std::time::Duration;

pub mod errors;
pub mod headers;
pub mod helpers;
pub mod request;
pub mod response;
pub mod url;

pub use self::headers::HttpHeaders;
pub use self::request::{HttpMethod, HttpRequest, HttpRequestBuilder};
pub use self::response::{HttpResponse, HttpResponseBuilder};
pub use self::url::Url;

use crate::errors::{Error, ErrorCode};

const DEFAULT_TIMEOUT_SECONDS: u64 = 30;
const DEFAULT_POOL_SIZE: usize = 10;

#[derive(Debug, Error)]
pub enum HttpClientError {
    #[error("Failed to connect: {0}")]
    ConnectError(#[from] reqwest::Error),

    #[error("Timeout occurred")]
    TimeoutError,

    #[error("Failed to parse response")]
    ParseError,
}

impl From<HttpClientError> for Error {
    fn from(err: HttpClientError) -> Self {
        match err {
            HttpClientError::ConnectError(err) => Error::new(
                format!("Failed to connect to server: {}", err).as_str(),
                ErrorCode::Internal,
            ),
            HttpClientError::TimeoutError => Error::new(
                "Timeout occurred while connecting to server",
                ErrorCode::Internal,
            ),
            HttpClientError::ParseError => {
                Error::new("Failed to parse response from server", ErrorCode::Internal)
            }
        }
    }
}

// Create a trait for the HTTP client.
#[async_trait]
pub trait HttpClient {
    async fn send_request(&self, request: Arc<HttpRequest>) -> Result<HttpResponse, Error>;
}

pub struct ReqwestHttpClient {
    client: Client,
    index: usize,
}

impl ReqwestHttpClient {
    pub fn new() -> ReqwestHttpClientBuilder {
        ReqwestHttpClientBuilder::new()
    }
}

#[async_trait]
impl HttpClient for ReqwestHttpClient {
    async fn send_request(&self, request: Arc<HttpRequest>) -> Result<HttpResponse, Error> {
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
        let body = response.text().await.or(Err(HttpClientError::ParseError))?;

        Ok(HttpResponseBuilder::new()
            .status_code(status_code)
            .headers(HttpHeaders::from(headers))
            .body(body)
            .build())
    }
}

pub struct ReqwestHttpClientBuilder {
    timeout: Option<Duration>,
}

impl ReqwestHttpClientBuilder {
    pub fn new() -> Self {
        Self { timeout: None }
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn build(self, index: usize) -> ReqwestHttpClient {
        let mut client_builder = Client::builder();
        match self.timeout {
            Some(timeout) => client_builder = client_builder.timeout(timeout),
            None => {
                client_builder =
                    client_builder.timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECONDS))
            }
        }

        let client = client_builder.build().unwrap();
        ReqwestHttpClient { client, index }
    }
}

pub struct ReqwestHttpClientPool {
    clients: Vec<Arc<ReqwestHttpClient>>,
    borrowed: Arc<RwLock<HashSet<usize>>>,
}

impl ReqwestHttpClientPool {
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_POOL_SIZE)
    }

    pub fn with_capacity(num_clients: usize) -> Self {
        let mut clients = Vec::with_capacity(num_clients);

        for i in 0..num_clients {
            clients.push(Arc::new(ReqwestHttpClient::new().build(i)));
        }

        let borrowed = Arc::new(RwLock::new(HashSet::new()));

        Self { clients, borrowed }
    }

    pub fn borrow_client(&mut self) -> Result<Arc<ReqwestHttpClient>, Error> {
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

            let client = ReqwestHttpClient::new().build(index);
            self.clients.push(Arc::new(client));
            index
        } else {
            let index = available_clients[rand::thread_rng().gen_range(0..available_clients.len())];
            borrowed_set.insert(index);
            index
        };

        Ok(self.clients[client_index].clone())
    }

    pub fn return_client(&mut self, client: Arc<ReqwestHttpClient>) {
        let mut borrowed = match self.borrowed.try_write() {
            Ok(borrowed) => borrowed,
            Err(_) => todo!(),
        };
        borrowed.remove(&client.index);
    }
}
