use scb_open_banking_sdk::models::corporate_financial_markets::rate_quotes::rate_quotes::{
    RateQuotesByCurrencyPairInputDetails, RateQuotesResponseData,
};
use scb_open_banking_sdk::ScbGateway;

pub async fn test_enquire_rate_quotes_by_currency_pair(
    consumer_key: String,
    consumer_secret: String,
    _env: String,
) {
    let _result = ScbGateway::new(consumer_key, consumer_secret, _env);

    if let Ok(scb) = _result {
        let client_id = String::from("***");
        let rate_category_id = String::from("***");
        let buy_currency = String::from("KES");
        let sell_currency = String::from("KES");
        let _tenor = 1;
        let _result = RateQuotesByCurrencyPairInputDetails::new(
            client_id,
            rate_category_id,
            buy_currency,
            sell_currency,
            _tenor,
        );

        if let Ok(account_details) = _result {
            let _output = scb.enquire_rate_quotes_by_currency_pair(account_details);
            let _result = _output.await;
            if let Ok(result_message) = _result {
                println!("result_message: {:?}", result_message);
            } else if let Err(e) = _result {
                println!("{:?}", e);
            } else {
                println!("Unexpected error occured during processing");
            }
        } else if let Err(e) = _result {
            println!("{:?}", e);
        } else {
            println!("Unexpected error occured during processing");
        }
    } else if let Err(e) = _result {
        println!("{:?}", e);
    } else {
        println!("Unexpected error occured during processing");
    }
}
