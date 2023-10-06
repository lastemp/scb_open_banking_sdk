use scb_open_banking_sdk::models::notifications_and_subscriptions::subscriptions::subscriptions::{
    NotificationSubscriptionRetrieveByNotificationTypeInputDetails,
    NotificationSubscriptionRetrieveByNotificationTypeResponseData,
};
use scb_open_banking_sdk::ScbGateway;

pub async fn test_notification_subscription_retrieve_by_notification_type(
    consumer_key: String,
    consumer_secret: String,
    _env: String,
) {
    let _result = ScbGateway::new(consumer_key, consumer_secret, _env);

    if let Ok(scb) = _result {
        let notification_type = String::from("account-status");
        let _result =
            NotificationSubscriptionRetrieveByNotificationTypeInputDetails::new(notification_type);

        if let Ok(account_details) = _result {
            let _output =
                scb.notification_subscription_retrieve_by_notification_type(account_details);
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
