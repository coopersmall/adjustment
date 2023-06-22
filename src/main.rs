use common::bitcoin::Bitcoin;
use common::currency::Currency;
use utils::errors::{Error, ErrorCode, ErrorMeta};
use utils::json::Parse;

fn main() {
    let error_meta = ErrorMeta::new()
        .add("Invalid bitcoin", "help")
        .add("Invalid eth", "help")
        .build();

    let result = Currency::new()
        .name("Dollars")
        .symbol("$")
        .code("USD")
        .build();

    let currency: Currency = match result {
        Ok(currency) => currency,
        Err(err) => {
            let invalid_error = Error::new(err, ErrorCode::Invalid).with_meta(error_meta.clone());
            let wrapped_error = Error::new("Something", ErrorCode::Invalid)
                .with_cause(Box::new(invalid_error))
                .with_meta(error_meta.clone());
            println!("Error: {:?}", wrapped_error.source());
            return;
        }
    };

    println!("Currency: {:?}", currency.marshal());

    let currency_code = currency.code();
    println!("Currency Code: {:?}", currency_code.marshal());

    let mut bitcoin = Bitcoin::new().name("Bitcoin").price(currency).build();

    let bitcoin_data = match bitcoin_marshalling(&bitcoin) {
        Ok(data) => data,
        Err(err) => {
            println!("Error: {:?}", err.marshal());
            return;
        }
    };

    println!("Bitcoin: {:?}", bitcoin_data);

    bitcoin = match Bitcoin::unmarshal(bitcoin_data.as_str()) {
        Ok(bitcoin) => bitcoin,
        Err(err) => {
            println!("Error: {:?}", err);
            return;
        }
    };

    println!("Bitcoin: {:?}", bitcoin.marshal());
}

fn bitcoin_marshalling<'a>(bitcoin: &Bitcoin) -> Result<String, Error> {
    let invalid_error = Error::new("Invalid bitcoin", ErrorCode::Invalid);
    return Err(invalid_error);
}
