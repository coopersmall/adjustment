use async_trait::async_trait;
use rand::Rng;
use reqwest::{Client, Method};
use thiserror::Error;

use std::collections::HashSet;
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub mod errors;
pub mod headers;
pub mod request;
pub mod response;
pub mod url;

pub use self::headers::HttpHeaders;
pub use self::request::{HttpMethod, HttpRequest, HttpRequestBuilder};
pub use self::response::{HttpResponse, HttpResponseBuilder};
pub use self::url::Url;

use crate::errors::codes::InternalErrorCode;
use crate::errors::{Error, ErrorCode, ErrorMessage};

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

// Create a trait for the HTTP client.
#[async_trait]
pub trait HttpClient {
    async fn send_request(&self, request: HttpRequest) -> Result<HttpResponse, HttpClientError>;
}

pub struct ReqwestHttpClient {
    client: Client,
    pool: Arc<Box<ReqwestHttpClientPool>>,
    index: usize,
}

impl ReqwestHttpClient {
    pub fn new() -> ReqwestHttpClientBuilder {
        ReqwestHttpClientBuilder::new()
    }
}

#[async_trait]
impl HttpClient for ReqwestHttpClient {
    async fn send_request(&self, request: HttpRequest) -> Result<HttpResponse, HttpClientError> {
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

        let response = request_builder.send().await?;
        let status_code = response.status().as_u16();
        let headers = response.headers().to_owned();
        let body = response.text().await?;

        Ok(HttpResponseBuilder::new()
            .status_code(status_code)
            .headers(HttpHeaders::from(headers))
            .body(body)
            .build())
    }
}

impl Drop for ReqwestHttpClient {
    fn drop(&mut self) {
        let borrowed = self.pool.borrowed.lock();
        if let Err(_) = borrowed {
            return;
        }
        let mut set = borrowed.unwrap();

        set.remove(&self.index);
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

    pub fn build(self, pool: Box<ReqwestHttpClientPool>, index: usize) -> ReqwestHttpClient {
        let mut client_builder = Client::builder();
        match self.timeout {
            Some(timeout) => client_builder = client_builder.timeout(timeout),
            None => {
                client_builder =
                    client_builder.timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECONDS))
            }
        }

        let client = client_builder.build().unwrap();
        let pool = Arc::new(pool);
        ReqwestHttpClient {
            client,
            pool,
            index,
        }
    }
}

#[derive(Clone)]
pub struct ReqwestHttpClientPool {
    clients: Vec<Arc<ReqwestHttpClient>>,
    borrowed: Arc<Mutex<HashSet<usize>>>,
}

impl ReqwestHttpClientPool {
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_POOL_SIZE)
    }

    pub fn with_capacity(num_clients: usize) -> Self {
        let clients = Vec::with_capacity(num_clients);
        let borrowed = Arc::new(Mutex::new(HashSet::new()));

        Self { clients, borrowed }
    }

    pub async fn borrow_client(&mut self) -> Result<Arc<ReqwestHttpClient>, Arc<Error>> {
        let borrowed = self.borrowed.lock();
        let mut borrowed_set = match borrowed {
            Ok(set) => set,
            Err(err) => {
                return Err(Arc::new(
                    Error::new()
                        .code(ErrorCode::Internal(InternalErrorCode::Cache(
                            crate::errors::codes::Cache::Query,
                        )))
                        .message(ErrorMessage::from_str("Failed to borrow client client"))
                        .source(Box::new(
                            Error::new()
                                .message(ErrorMessage::from_string(format!("{}", err)))
                                .build(),
                        ))
                        .build(),
                ));
            }
        };

        let available_clients: Vec<usize> = self
            .clients
            .iter()
            .enumerate()
            .filter(|(index, _)| !borrowed_set.contains(index))
            .map(|(index, _)| index)
            .collect();

        if available_clients.is_empty() {
            let client_index = self.clients.len();
            let client =
                Arc::new(ReqwestHttpClient::new().build(Box::new(self.clone()), client_index));

            borrowed_set.insert(client_index);

            self.clients.push(client);

            Ok(self.clients[client_index].clone())
        } else {
            let client_index =
                available_clients[rand::thread_rng().gen_range(0..available_clients.len())];
            borrowed_set.insert(client_index);

            match self.clients.get(client_index) {
                Some(client) => Ok(Arc::clone(client)),
                None => Err(Arc::new(
                    Error::new()
                        .code(ErrorCode::Internal(InternalErrorCode::Cache(
                            crate::errors::codes::Cache::Query,
                        )))
                        .message(ErrorMessage::from_str("Failed to borrow client client"))
                        .build(),
                )),
            }
        }
    }

    pub async fn return_client(&mut self, index: usize) {
        let borrowed = self.borrowed.lock();
        if let Err(_) = borrowed {
            self.clients[index] =
                Arc::new(ReqwestHttpClient::new().build(Box::new(self.clone()), index));
            return;
        }

        let mut borrowed_set = borrowed.unwrap();
        borrowed_set.remove(&index);
    }
}
