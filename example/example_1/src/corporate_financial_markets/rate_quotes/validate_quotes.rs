use scb_open_banking_sdk::models::corporate_financial_markets::rate_quotes::rate_quotes::{
    QuotesValidationInputDetails, RateQuotesResponseData,
};
use scb_open_banking_sdk::ScbGateway;

pub async fn test_validate_quotes(consumer_key: String, consumer_secret: String, _env: String) {
    let _result = ScbGateway::new(consumer_key, consumer_secret, _env);

    if let Ok(scb) = _result {
        let client_id = String::from("***");

        let _result = QuotesValidationInputDetails::new(client_id);

        if let Ok(account_details) = _result {
            let _output = scb.validate_quotes(account_details);
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
