use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

use utils::adapters::http_client::*;
use utils::errors::{Error, ErrorCode};
use utils::json::Parse;
use utils::*;

const NUM_THREADS: usize = 10;
const BASE_URL: &str = "https://webhook.site";
const PATH: &str = "27acc74f-7f52-4cfc-abff-9ff4c3af5ca7";

#[macros::async_main]
pub async fn main() -> Result<(), Error> {
    let url = url!(BASE_URL, PATH);
    let headers = http_headers! {
        "Content-Type" => "application/json",
    };
    let request = Arc::new(http_request!(GET, &url, headers));

    let client_pool = Arc::new(Mutex::new(HttpClientPool::with_capacity(NUM_THREADS)));
    let (tx, mut rx) = mpsc::channel::<Result<HttpResponse, Error>>(NUM_THREADS);

    for _ in 0..NUM_THREADS {
        let pool = client_pool.clone();
        let request = request.clone();
        let tx = tx.clone();

        spawn!(async move {
            let response = send_request!(pool, request).await;
            if let Err(err) = tx.send(response).await {
                return Err(Error::new(
                    format!("Failed to send response: {}", err).as_str(),
                    ErrorCode::Internal,
                )
                .with_cause(err));
            }

            Ok::<(), Error>(())
        });
    }

    drop(tx);

    while let Some(result) = rx.recv().await {
        match result {
            Ok(response) => {
                let json = &response.marshal().unwrap();
                println!("{:?}", json)
            }
            Err(err) => {
                println!("{:?}", err);
                return Err(err);
            }
        }
    }

    Ok(())
}
