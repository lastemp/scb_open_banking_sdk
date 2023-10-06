use scb_open_banking_sdk::models::accounts_inquiry::accounts_inquiry::AccountsInquiryResponseData;
use scb_open_banking_sdk::ScbGateway;

pub async fn test_enquire_list_of_accounts(
    consumer_key: String,
    consumer_secret: String,
    _env: String,
) {
    let _result = ScbGateway::new(consumer_key, consumer_secret, _env);

    if let Ok(scb) = _result {
        let _output = scb.enquire_list_of_accounts();
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
}
