use reqwest::StatusCode;

use crate::{
    models::accounts_inquiry::accounts_inquiry::{
        AccountDetailsAndBalanceQueryInputDetails, AccountDetailsAndBalanceResponseData,
    },
    util::util::{build_account_details_and_balance_query_data, build_headers_data},
};

pub async fn enquire(
    account_details: AccountDetailsAndBalanceQueryInputDetails,
    access_token: String,
    api_url: String,
) -> std::result::Result<AccountDetailsAndBalanceResponseData, String> {
    let as_of = account_details.get_as_of();
    let account_numbers = account_details.get_account_number();

    // Lets build the request params as a struct
    let account_data = build_account_details_and_balance_query_data(as_of, account_numbers);

    let client = reqwest::Client::new();

    let res = client
        .post(api_url)
        .headers(build_headers_data())
        .json(&account_data)
        .send()
        .await;

    match res {
        Err(_err) => {
            return Err(_err.to_string());
        }
        Ok(response) => match response.status() {
            StatusCode::OK => {
                // 200
                match response
                    .json::<AccountDetailsAndBalanceResponseData>()
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
            StatusCode::CREATED | StatusCode::ACCEPTED | StatusCode::NO_CONTENT => {
                match response.status() {
                    StatusCode::CREATED => {
                        // 201
                        let mut _x = String::from(
                            "Request processed successfully and data created, status code: ",
                        );
                        _x.push_str(&response.status().to_string());
                        return Err(_x.to_string());
                    }
                    StatusCode::ACCEPTED => {
                        // 202
                        let mut _x = String::from(
                            "Request processed successfully and data accepted, status code: ",
                        );
                        _x.push_str(&response.status().to_string());
                        return Err(_x.to_string());
                    }
                    StatusCode::NO_CONTENT => {
                        // 204
                        let mut _x = String::from(
                            "Request processed successfully and no data returned, status code: ",
                        );
                        _x.push_str(&response.status().to_string());
                        return Err(_x.to_string());
                    }
                    s => {
                        let mut _x = String::from("Request failed processing, status code: ");
                        _x.push_str(&s.to_string());
                        return Err(_x.to_string());
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
