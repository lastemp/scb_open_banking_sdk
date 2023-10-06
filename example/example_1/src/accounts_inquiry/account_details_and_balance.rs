use scb_open_banking_sdk::models::accounts_inquiry::accounts_inquiry::{
    AccountDetailsAndBalanceInputDetails, AccountDetailsAndBalanceResponseData,
};
use scb_open_banking_sdk::ScbGateway;

pub async fn test_enquire_account_details_and_balance(
    consumer_key: String,
    consumer_secret: String,
    _env: String,
) {
    let _result = ScbGateway::new(consumer_key, consumer_secret, _env);

    if let Ok(scb) = _result {
        let account_number = String::from("***");
        let _result = AccountDetailsAndBalanceInputDetails::new(account_number);

        if let Ok(account_details) = _result {
            let _output = scb.enquire_account_details_and_balance(account_details);
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
