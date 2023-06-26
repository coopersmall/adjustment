/// Creates a hashmap of parameters with the given key-value pairs.
///
/// # Arguments
///
/// * `$key:expr => $value:expr` - The key-value pairs representing the parameters.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use crate::utils::params;
///
/// let params = params! {
///     "name" => "John",
///     "age" => "30",
/// };
///
/// assert_eq!(params[0], ("name".into(), "John".into()));
/// assert_eq!(params[1], ("age".into(), "30".into()));
/// ```
#[macro_export]
macro_rules! params {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {
        {
            let mut params: Vec<(Box<str>, Box<str>)> = Vec::new();
            $(
                params.push(($key.into(), $value.into()));
            )*
            params
        }
    };
}

/// Builds a URL using the provided base URL, path, and optional parameters.
///
/// # Arguments
///
/// * `$base_url:expr` - The base URL for the endpoint.
/// * `$path:expr` - The path to append to the base URL (optional).
/// * `$params:expr` - The parameters to include in the URL (optional).
///
/// # Examples
///
/// ```
/// use crate::utils::{params, url};
/// use crate::utils::http::Url;
///
/// pub fn test () -> Result<(), Box<dyn std::error::Error>> {
/// let url = url!("https://example.com");
/// assert_eq!(url, "https://example.com/".into());
///
/// let url = url!("https://example.com", "path");
/// assert_eq!(url, "https://example.com/path/".into());
///
/// let url = url!("https://example.com", "path", params! {
///     "name" => "John",
///     "age" => "30",
/// });
/// assert_eq!(url, "https://example.com/path/?name=John&age=30".into());
/// Ok(())
/// }
///
/// test();
/// ```
///
///
#[macro_export]
macro_rules! url {
    ( $base_url:expr ) => {{
        utils::http::Url::new($base_url)?.build()?
    }};

    ( $base_url:expr, $path:expr ) => {{
        utils::http::Url::new($base_url)?.add_path($path).build()?
    }};

    ( $base_url:expr, $path:expr, $params:expr ) => {{
        utils::http::Url::new($base_url)?
            .add_path($path)
            .set_params($params)
            .build()?
    }};
}

/// Creates an HTTP request with the specified method, URL, body, and optional headers.
///
/// # Arguments
///
/// * `GET, $url:expr $(, $headers:expr)?` - Constructs a GET request.
/// * `POST, $url:expr, $body:expr $(, $headers:expr)?` - Constructs a POST request.
/// * `PUT, $url:expr, $body:expr $(, $headers:expr)?` - Constructs a PUT request.
/// * `PATCH, $url:expr, $body:expr $(, $headers:expr)?` - Constructs a PATCH request.
/// * `DELETE, $url:expr $(, $headers:expr)?` - Constructs a DELETE request.
/// * `$url:expr` - The URL for the request.
/// * `$body:expr` - The body of the request (optional for POST, PUT, PATCH).
/// * `$headers:expr` - The headers for the request (optional).
///
/// # Examples
///
/// ```
/// use crate::utils::http_request;
/// use crate::utils::http::{HttpRequestBuilder, HttpMethod};
///
/// let get_request = http_request!(GET, "https://api.example.com/get");
///
/// let post_request = http_request!(POST, "https://api.example.com/post", "data");
/// ```
///
///
#[macro_export]
macro_rules! http_request {
    (GET, $url:expr $(, $headers:expr)?) => {{
        let mut builder = HttpRequestBuilder::new($url, HttpMethod::GET);
        $(builder = builder.headers($headers);)?
        builder.build()
    }};

    (POST, $url:expr, $body:expr $(, $headers:expr)?) => {{
        let mut builder = HttpRequestBuilder::new($url, HttpMethod::POST).body($body);
        $(builder = builder.headers($headers);)?
        builder.build()
    }};

    (PUT, $url:expr, $body:expr $(, $headers:expr)?) => {{
        let mut builder = HttpRequestBuilder::new($url, HttpMethod::PUT).body($body);
        $(builder = builder.headers($headers);)?
        builder.build()
    }};

    (PATCH, $url:expr, $body:expr $(, $headers:expr)?) => {{
        let mut builder = HttpRequestBuilder::new($url, HttpMethod::PATCH).body($body);
        $(builder = builder.headers($headers);)?
        builder.build()
    }};

    (DELETE, $url:expr $(, $headers:expr)?) => {{
        let mut builder = HttpRequestBuilder::new($url, HttpMethod::DELETE);
        $(builder = builder.headers($headers);)?
        builder.build()
    }};
}

/// Creates a hashmap of HTTP headers with the given key-value pairs.
///
/// # Arguments
///
/// * `$key:expr => $value:expr` - The key-value pairs representing the headers.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use crate::utils::http_headers;
///
/// let headers = http_headers! {
///     "Content-Type" => "application/json",
///     "Authorization" => "Bearer TOKEN",
/// };
///
/// assert_eq!(headers.get("Content-Type"), Some(&"application/json"));
/// assert_eq!(headers.get("Authorization"), Some(&"Bearer TOKEN"));
/// ```
///
///
#[macro_export]
macro_rules! http_headers {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {
        {
            let mut headers = std::collections::HashMap::new();
            $(
                headers.insert($key, $value);
            )*
            headers
        }
    };
}

/// Sends an HTTP request using the provided connection pool and request.
///
/// # Arguments
///
/// * `$pool:expr` - The connection pool to use for sending the request.
/// * `$request:expr` - The HTTP request to send.
///
/// # Examples
///
/// ```
/// use std::sync::{Arc, Mutex};
///
/// use crate::utils::{send_request, http_request, spawn};
/// use crate::utils::http::{HttpMethod, HttpRequest, HttpRequestBuilder};
/// use crate::utils::adapters::http_client::HttpClientPool;
/// use crate::utils::errors::{Error, ErrorCode};
///
/// let pool = Arc::new(Mutex::new(HttpClientPool::new()));
/// let request = Arc::new(http_request!(GET, "https://api.example.com"));
///
/// #[tokio::test]
/// async fn test() -> Result<(), Error> {
///    let response = send_request!(pool, request).await?;
///    Ok(())
/// }
///// ```
///
///
#[macro_export]
macro_rules! send_request {
    ($pool:expr, $request:expr) => {{
        let mut pool = match $pool.lock() {
            Ok(pool) => pool,
            Err(_) => return Err(Error::new("Failed to lock pool", ErrorCode::Internal)),
        };

        let client = match pool.borrow_client() {
            Ok(client) => client,
            Err(_) => return Err(Error::new("Failed to borrow client", ErrorCode::Internal)),
        };

        let (tx, rx) = tokio::sync::oneshot::channel();
        let request = $request.clone();
        let thread_client = client.clone();

        tokio::spawn(async move {
            let client = thread_client.clone();
            let response = client.send_request(request).await;
            let _ = tx.send(response);
        });

        pool.return_client(client);

        Box::pin(async move {
            rx.await
                .map_err(|_| Error::new("Failed to send request", ErrorCode::Internal))?
        })
    }};
}
