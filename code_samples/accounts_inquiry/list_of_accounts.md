# list_of_accounts

The List of Accounts API provides an ability to retrieve a list of available bank accounts which client is authorized to access(no balance information).

## main.rs

This should contain below code:

```rust
mod accounts_inquiry {
    pub mod list_of_accounts;
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

    let x = accounts_inquiry::list_of_accounts::test_enquire_list_of_accounts(
        consumer_key,
        consumer_secret,
        _env,
    );
	
    x.await;
}
```

## list_of_accounts.rs

This module contains the function test_enquire_list_of_accounts:

```rust
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
```
