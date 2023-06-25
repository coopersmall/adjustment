use std::sync::Arc;
use tokio::sync::mpsc;
use utils::adapters::http_client::*;
use utils::errors::{Error, ErrorCode};
use utils::json::Parse;
use utils::{http_headers, http_request, spawn_async, url};

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

    let mut client_pool = ReqwestHttpClientPool::with_capacity(NUM_THREADS);
    let client = client_pool.borrow_client()?;

    let (tx, mut rx) = mpsc::channel::<Result<HttpResponse, Error>>(NUM_THREADS);

    for _ in 0..NUM_THREADS {
        let tx = tx.clone();
        let request = request.clone();
        let client = client.clone();

        spawn_async! {
            let response = client.send_request(request).await;
            if let Err(err) = tx.send(response).await {
                let err = Error::new("test", ErrorCode::Invalid).with_cause(err);
                eprintln!("Error sending result: {:?}", err);
            }
        };
    }

    drop(tx);
    client_pool.return_client(client);

    while let Some(result) = rx.recv().await {
        match result {
            Ok(response) => {
                let json = &response.body().marshal().unwrap();
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
