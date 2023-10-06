use scb_open_banking_sdk::models::notifications_and_subscriptions::subscriptions::subscriptions::NotificationSubscriptionRetrieveAllResponseData;
use scb_open_banking_sdk::ScbGateway;

pub async fn test_notification_subscription_retrieve_all(
    consumer_key: String,
    consumer_secret: String,
    _env: String,
) {
    let _result = ScbGateway::new(consumer_key, consumer_secret, _env);

    if let Ok(scb) = _result {
        let _output = scb.notification_subscription_retrieve_all();
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
