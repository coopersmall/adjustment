use std::sync::Arc;
use tokio::sync::mpsc;
use utils::adapters::http_client::*;
use utils::errors::{Error, ErrorCode};
use utils::json::Parse;
use utils::{http_request, spawn_async};

const NUM_THREADS: usize = 10;

#[macros::async_main]
pub async fn main() -> Result<(), Error> {
    let url = Url::new("https://webhook.site")?
        .add_path("27acc74f-7f52-4cfc-abff-9ff4c3af5ca7")
        .build()?;

    let request = Arc::new(http_request!(GET, url.as_ref()).build());

    let mut client_pool = ReqwestHttpClientPool::with_capacity(NUM_THREADS);
    let (tx, mut rx) = mpsc::channel::<Result<HttpResponse, Error>>(NUM_THREADS);

    let client = match client_pool.borrow_client() {
        Ok(client) => client,
        Err(_) => {
            let err = Error::new("test", ErrorCode::Invalid);
            eprintln!("Error getting client: {:?}", err);
            return Err(err);
        }
    };

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
