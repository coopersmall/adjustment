use shared::utils::bitcoin::Bitcoin;
use shared::utils::currency::Currency;

fn main() {
    let result = Currency::new()
        .code("BTC")
        .name("Bitcoin")
        .symbol("₿")
        .build();

    if let Err(e) = result {
        println!("Error: {}", e);
    }

    let currency = result.unwrap();

    println!("Currency: {:?}", currency.marshal());

    let bitcoin = Bitcoin::new().name("Bitcoin").price(currency).build();

    println!("Currency: {:?}", bitcoin.marshal());
}
