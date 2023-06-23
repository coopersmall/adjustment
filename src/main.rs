use common::bitcoin::Bitcoin;
use common::currency::Currency;
use std::borrow::Cow;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use tokio;
use tokio::sync::{futures, mpsc};
use utils::adapters::http_client::*;
use utils::errors::{Error, ErrorCode, ErrorMeta};
use utils::json::Parse;
use utils::{async_main, spawn, spawn_async};

const NUM_THREADS: usize = 10;

async_main! {
    let url = Url::new("https://www.webhook.site", "/27acc74f-7f52-4cfc-abff-9ff4c3af5ca7")
        .build();

    let request = HttpRequest::new(
        url.as_ref(),
        HttpMethod::GET,
    ).build();

    let mut client_pool = ReqwestHttpClientPool::with_capacity(NUM_THREADS);
        let (tx, mut rx) = mpsc::channel::<Result<HttpResponse, Error>>(NUM_THREADS);


    for _ in 0..NUM_THREADS {
        let tx = tx.clone();
        let request = request.clone();

        let client = match client_pool.borrow_client().await {
            Ok(client) => client,
            Err(err) => {
                let request_err = Error::new("test", ErrorCode::Invalid).with_cause(err);
                eprintln!("Error getting client: {:?}", request_err);
                return;
            }
        };

             spawn_async! {
            let response = client.send_request(request).await;
            if let Err(err) = tx.send(response).await {
                let err = Error::new("test", ErrorCode::Invalid).with_cause(err);
                eprintln!("Error sending result: {:?}", err);
            }
        }
    }

    while let Some(result) = rx.recv().await {
        match result {
            Ok(response) => {
                let json = &response.body().marshal().unwrap();
                println!("{:?}", json)
            },
            Err(err) => println!("{:?}", err),
        }
    }
}
