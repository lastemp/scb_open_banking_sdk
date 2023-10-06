# account_details_and_balance

The Account Details & Balance API upon successful user authentication and entitlement checks will return the Account details and balances applicable for the specified Account Number for the given date..

## main.rs

This should contain below code:

```rust
mod accounts_inquiry {
    pub mod account_details_and_balance;
}

// SANDBOX
const CONSUMER_KEY_SANDBOX: &str = "***";
const CONSUMER_SECRET_SANDBOX: &str = "***";

const ENVIRONMENT: &str = "sandbox";

#[tokio::main]
async fn main() {
    let consumer_key = CONSUMER_KEY_SANDBOX.to_string();
    let consumer_secret = CONSUMER_SECRET_SANDBOX.to_string();
    let _env = ENVIRONMENT.to_string();

    let x = accounts_inquiry::account_details_and_balance::test_enquire_account_details_and_balance(
        consumer_key,
        consumer_secret,
        _env,
    );
	
    x.await;
}
```

## account_details_and_balance.rs

This module contains the function test_enquire_account_details_and_balance:

```rust
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
        let account_number = String::from("1450160649886");
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

```
