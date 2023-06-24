#[macro_export]
macro_rules! http_request {
    (GET, $url:expr) => {
        HttpRequestBuilder::new($url, HttpMethod::GET)
    };

    (POST, $url:expr) => {
        HttpRequestBuilder::new($url, HttpMethod::POST)
    };

    (PUT, $url:expr) => {
        HttpRequestBuilder::new($url, HttpMethod::PUT)
    };

    (DELETE, $url:expr) => {
        HttpRequestBuilder::new($url, HttpMethod::DELETE)
    };
}

#[macro_export]
macro_rules! http_request_builder {
    () => {
        HttpRequestBuilder::new("", HttpMethod::GET)
    };
}
