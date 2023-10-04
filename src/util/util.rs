use reqwest::header::HeaderMap;
use reqwest::header::{ACCEPT, CONTENT_TYPE};

use crate::models::accounts_inquiry::accounts_inquiry::{
    AccountDetailsAndBalanceQueryData, AccountIdData, AccountNumberDataDetails, AsOfData,
    AsOfDataDetails,
};

pub fn build_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();

    //headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(ACCEPT, "application/json".parse().unwrap());

    headers
}

pub fn build_headers_data() -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(ACCEPT, "application/json".parse().unwrap());

    headers
}

pub fn build_headers_generate_auth_token(api_key: String) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(
        CONTENT_TYPE,
        "application/x-www-form-urlencoded".parse().unwrap(),
    );
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert("Authorization", api_key.parse().unwrap());

    headers
}

pub fn build_account_details_and_balance_query_data(
    as_of: &AsOfDataDetails,
    account_numbers: &Vec<AccountNumberDataDetails>,
) -> AccountDetailsAndBalanceQueryData {
    let as_of_data = AsOfData {
        date: as_of.get_date(),
        r#type: as_of.get_type(),
    };

    let mut account_number_data: Vec<AccountIdData> = Vec::new();

    for account_number in account_numbers.iter() {
        let account_data = AccountIdData {
            accountId: account_number.get_account_number(),
        };

        account_number_data.push(account_data)
    }

    AccountDetailsAndBalanceQueryData {
        asOf: as_of_data,
        accountIds: account_number_data,
    }
}
