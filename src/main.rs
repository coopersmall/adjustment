use shared::utils::bitcoin::Bitcoin;
use shared::utils::currency::Currency;
use traits::errors::{Error, ErrorCode, ErrorMessage, ErrorMeta};
use traits::json::Parse;

fn main() {
    let result = Currency::new()
        .name("Dollars")
        .symbol("$")
        .code("USD")
        .build();

    let currency: Currency = match result {
        Ok(currency) => currency,
        Err(err) => {
            let invalid_error = Error::new()
                .message(ErrorMessage::from_str(err))
                .code(ErrorCode::Invalid)
                .build();

            let wrapped_error = Error::new()
                .message(ErrorMessage::from_str("Error parsing currency"))
                .code(ErrorCode::Empty)
                .source(Box::new(invalid_error))
                .build();

            println!("Error: {:?}", wrapped_error.marshal());

            let source = match wrapped_error.source {
                Some(source) => source,
                None => {
                    println!("Error: {:?}", wrapped_error.marshal());
                    return;
                }
            };

            let other_error = Error::new()
                .message(ErrorMessage::from_str("Error parsing currency"))
                .code(ErrorCode::Empty)
                .source(source)
                .add_meta("service".to_string(), "test".to_string())
                .add_meta("deployment".to_string(), "master".to_string())
                .add_meta("coorelation_id".to_string(), "123".to_string())
                .build();

            let other_error_json = match other_error.marshal() {
                Ok(json) => json,
                Err(err) => {
                    println!("Error: {:?}", err);
                    return;
                }
            };

            println!("Error: {:?}", other_error_json);

            let meta = match ErrorMeta::from_json(other_error_json.as_str()) {
                Ok(meta) => meta,
                Err(err) => {
                    println!("Error: {:?}", err);
                    return;
                }
            };

            println!("Error Meta: {:?}", meta);

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

fn bitcoin_marshalling<'a>(bitcoin: &Bitcoin) -> Result<String, Error<'a>> {
    Err(Error::new()
        .message(ErrorMessage::from_str("Error marshalling bitcoin"))
        .build())
}
