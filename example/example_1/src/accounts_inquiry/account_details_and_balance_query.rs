use scb_open_banking_sdk::models::accounts_inquiry::accounts_inquiry::{
    AccountDetailsAndBalanceQueryInputDetails, AccountDetailsAndBalanceResponseData,
    AccountNumberDataDetails, AsOfDataDetails,
};
use scb_open_banking_sdk::ScbGateway;

pub async fn test_enquire_account_details_and_balance_query(
    consumer_key: String,
    consumer_secret: String,
    _env: String,
) {
    let _result = ScbGateway::new(consumer_key, consumer_secret, _env);

    if let Ok(scb) = _result {
        let _date = String::from("04-10-2023");
        let _type = String::from("current");

        let _result = AsOfDataDetails::new(_date, _type);

        if let Ok(as_of_data_details) = _result {
            let account_number = String::from("***");
            let _result = AccountNumberDataDetails::new(account_number);
            if let Ok(account_number_data_details) = _result {
                let mut account_numbers: Vec<AccountNumberDataDetails> = Vec::new();
                account_numbers.push(account_number_data_details);
                let _result = AccountDetailsAndBalanceQueryInputDetails::new(
                    as_of_data_details,
                    account_numbers,
                );
                if let Ok(account_details) = _result {
                    let _output = scb.enquire_account_details_and_balance_query(account_details);
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
