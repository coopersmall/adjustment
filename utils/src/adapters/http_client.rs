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
const DEFAULT_POOL_SIZE: usize = 10;

pub struct HttpClient {
    client: Client,
    index: usize,
}

impl HttpClient {
    pub fn new() -> ReqwestHttpClientBuilder {
        ReqwestHttpClientBuilder::new()
    }
}

impl HttpClient {
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
        let headers = headers
            .iter()
            .fold(HashMap::new(), |mut headers, (key, value)| {
                let value = match value.to_str() {
                    Ok(value) => value,
                    Err(_) => return headers,
                };
                headers.insert(key.as_str(), value);
                headers
            });

        let body = match response.text().await {
            Ok(body) => body,
            Err(err) => {
                return Err(Error::new(
                    format!("Failed to read response body: {}", err).as_str(),
                    ErrorCode::Internal,
                ));
            }
        };

        Ok(HttpResponse::new(
            status_code,
            body.as_str(),
            headers.to_owned(),
        ))
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

pub struct HttpClientPool {
    clients: Vec<Arc<HttpClient>>,
    borrowed: Arc<RwLock<HashSet<usize>>>,
}

impl HttpClientPool {
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_POOL_SIZE)
    }

    pub fn with_capacity(num_clients: usize) -> Self {
        let mut clients = Vec::with_capacity(num_clients);

        for i in 0..num_clients {
            clients.push(Arc::new(HttpClient::new().build(i)));
        }

        let borrowed = Arc::new(RwLock::new(HashSet::new()));

        Self { clients, borrowed }
    }

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

    pub fn return_client(&mut self, client: Arc<HttpClient>) {
        let mut borrowed = match self.borrowed.try_write() {
            Ok(borrowed) => borrowed,
            Err(_) => todo!(),
        };
        borrowed.remove(&client.index);
    }
}
