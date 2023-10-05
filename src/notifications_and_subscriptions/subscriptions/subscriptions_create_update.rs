use reqwest::StatusCode;

use crate::{
    models::notifications_and_subscriptions::subscriptions::subscriptions::NotificationSubscriptionCreateUpdateResponseData,
    util::util::build_headers,
};

pub async fn create_update(
    access_token: String,
    api_url: String,
) -> std::result::Result<NotificationSubscriptionCreateUpdateResponseData, String> {
    let client = reqwest::Client::new();

    let res = client.post(api_url).headers(build_headers()).send().await;

    match res {
        Err(_err) => {
            return Err(_err.to_string());
        }
        Ok(response) => match response.status() {
            StatusCode::OK => {
                // 200
                match response
                    .json::<NotificationSubscriptionCreateUpdateResponseData>()
                    .await
                {
                    Ok(account_response_data) => {
                        // Handle success case

                        return Ok(account_response_data);
                    }
                    Err(_err) => {
                        // Handle error case
                        return Err(_err.to_string());
                    }
                }
            }
            s => {
                let mut _x = String::from("Request failed processing, status code: ");
                _x.push_str(&s.to_string());
                return Err(_x.to_string());
            }
        },
    };
}
