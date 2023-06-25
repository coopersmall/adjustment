#[macro_export]
macro_rules! params {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {
        {
            let mut params = std::collections::HashMap::new();
            $(
                params.insert($key, $value);
            )*
            params
        }
    };
}

#[macro_export]
macro_rules! url {
    ( $base_url:expr ) => {{
        crate::Url::new($base_url)?.build()?
    }};

    ( $base_url:expr, $path:expr ) => {{
        crate::Url::new($base_url)?.add_path($path).build()?
    }};

    ( $base_url:expr, $path:expr, $params:expr ) => {{
        crate::Url::new($base_url)?
            .add_path($path)
            .set_params($params)
            .build()?
    }};
}

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

#[macro_export]
macro_rules! send_request {
    ($request:expr) => {{
        let client = HttpClient::new().build(0);
        let future = client.send_request($request);
        tokio::runtime::Runtime::new().unwrap().block_on(future)
    }};

    ($request:expr, $pool:expr) => {{
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