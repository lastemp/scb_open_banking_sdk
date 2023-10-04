pub mod models {
    pub mod accounts_inquiry {
        pub mod accounts_inquiry;
    }
    pub mod authorization {
        pub mod auth_token;
    }
}
mod util {
    pub mod util;
}
mod authorization {
    pub mod generate_auth_token;
}
mod accounts_inquiry {
    pub mod account_details_and_balance;
    pub mod account_details_and_balance_query;
    pub mod list_of_accounts;
    //pub mod account_transactions;
    //pub mod account_validation;
}
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use models::accounts_inquiry::accounts_inquiry::{
    AccountDetailsAndBalanceAsOfInputDetails, AccountDetailsAndBalanceByCurrencyInputDetails,
    AccountDetailsAndBalanceInputDetails, AccountDetailsAndBalanceQueryInputDetails,
    AccountDetailsAndBalanceResponseData, AccountsInquiryResponseData,
};

const AUTHORISATION_BEARER: &str = "Bearer";
const GRANT_TYPE: &str = "client_credentials";

const AUTH_TOKEN_URL_SANDBOX: &str = "https://";
const AUTH_TOKEN_URL_PROD: &str = "https://";

const LIST_OF_ACCOUNTS_URL_SANDBOX: &str =
    "https://api.standardchartered.com/openapi/accounts/v2/list";
const LIST_OF_ACCOUNTS_URL_PROD: &str =
    "https://api.standardchartered.com/openapi/accounts/v2/list";

const ACCOUNT_DETAILS_AND_BALANCE_URL_SANDBOX: &str =
    "https://api.standardchartered.com/openapi/accounts/v2/";
const ACCOUNT_DETAILS_AND_BALANCE_URL_PROD: &str =
    "https://api.standardchartered.com/openapi/accounts/v2/";

#[derive(Debug)]
pub struct ScbGateway {
    grant_type: String,
    consumer_key: String,
    consumer_secret: String,
    auth_token_url: String,
    list_of_accounts_url: String,
    account_details_and_balance_url: String,
}

impl ScbGateway {
    pub fn new(
        consumer_key: String,
        consumer_secret: String,
        _env: String,
    ) -> Result<Self, String> {
        if consumer_key.is_empty() || consumer_key.replace(" ", "").trim().len() == 0 {
            return Err(String::from("consumer key is empty"));
        }

        if consumer_secret.is_empty() || consumer_secret.replace(" ", "").trim().len() == 0 {
            return Err(String::from("consumer secret is empty"));
        }

        if _env.is_empty() || _env.replace(" ", "").trim().len() == 0 {
            return Err(String::from("_env is empty"));
        }

        if _env.eq_ignore_ascii_case(&String::from("sandbox"))
            || _env.eq_ignore_ascii_case(&String::from("prod"))
        {
            // valid _env
        } else {
            return Err(String::from("invalid env"));
        }

        let grant_type = GRANT_TYPE.to_string();

        let auth_token_url = if _env.eq_ignore_ascii_case(&String::from("prod")) {
            AUTH_TOKEN_URL_PROD.to_string()
        } else {
            AUTH_TOKEN_URL_SANDBOX.to_string()
        };

        let list_of_accounts_url = if _env.eq_ignore_ascii_case(&String::from("prod")) {
            LIST_OF_ACCOUNTS_URL_PROD.to_string()
        } else {
            LIST_OF_ACCOUNTS_URL_SANDBOX.to_string()
        };

        let account_details_and_balance_url = if _env.eq_ignore_ascii_case(&String::from("prod")) {
            ACCOUNT_DETAILS_AND_BALANCE_URL_PROD.to_string()
        } else {
            ACCOUNT_DETAILS_AND_BALANCE_URL_SANDBOX.to_string()
        };

        Ok(Self {
            grant_type,
            consumer_key,
            consumer_secret,
            auth_token_url,
            list_of_accounts_url,
            account_details_and_balance_url,
        })
    }

    fn get_api_key(&self) -> String {
        let consumer_key = &self.consumer_key;
        let consumer_secret = &self.consumer_secret;
        let mut password: String = consumer_key.to_string();
        let k = ":"; // Separator
        password.push_str(k);
        password.push_str(&consumer_secret);
        let encodedpassword = general_purpose::STANDARD.encode(password);

        let mut api_key = String::from("Basic");
        let k = " "; // Separator
        api_key.push_str(k);
        api_key.push_str(&encodedpassword);

        api_key
    }

    fn parse_auth_token(&self, access_token_result: String) -> String {
        let access_token: String = if !access_token_result.is_empty()
            && access_token_result.replace(" ", "").trim().len() > 0
        {
            let mut access_token = AUTHORISATION_BEARER.to_string();
            let k = " "; // Separator
            access_token.push_str(k);
            access_token.push_str(&access_token_result);

            access_token
        } else {
            String::from("")
        };

        access_token
    }

    async fn get_auth_token(&self) -> std::result::Result<String, String> {
        let grant_type = &self.grant_type;
        let api_key = &self.get_api_key();
        let api_url = &self.auth_token_url;

        let _result = authorization::generate_auth_token::get_auth_token(
            grant_type.to_string(),
            api_key.to_string(),
            api_url.to_string(),
        )
        .await;

        _result
    }

    pub async fn enquire_list_of_accounts(
        &self,
    ) -> std::result::Result<AccountsInquiryResponseData, String> {
        let _output = self.get_auth_token();

        let _result = _output.await;

        match _result {
            Ok(access_token_result) => {
                // Handle success case
                let access_token: String = self.parse_auth_token(access_token_result);
                let api_url = &self.list_of_accounts_url;

                let _result =
                    accounts_inquiry::list_of_accounts::enquire(access_token, api_url.to_string())
                        .await;

                return _result;
            }
            Err(_err) => {
                // Handle error case
                return Err(_err.to_string());
            }
        }
    }

    pub async fn enquire_account_details_and_balance_as_of(
        &self,
        account_details: AccountDetailsAndBalanceAsOfInputDetails,
    ) -> std::result::Result<AccountDetailsAndBalanceResponseData, String> {
        let _output = self.get_auth_token();

        let _result = _output.await;

        match _result {
            Ok(access_token_result) => {
                // Handle success case
                let account_id: String = account_details.get_account_number(); // mandatory for this request
                                                                               //let _currency: Option<String> = None;
                let as_of_date: String = account_details.get_as_of_date(); // mandatory for this request
                let access_token: String = self.parse_auth_token(access_token_result);
                let api_url = &self.account_details_and_balance_url;
                let mut api_url = api_url.to_string();
                let _a = "/";

                api_url.push_str(&account_id);
                api_url.push_str(&_a);
                api_url.push_str(&as_of_date);

                let _result = accounts_inquiry::account_details_and_balance::enquire(
                    access_token,
                    api_url.to_string(),
                )
                .await;

                return _result;
            }
            Err(_err) => {
                // Handle error case
                return Err(_err.to_string());
            }
        }
    }

    pub async fn enquire_account_details_and_balance(
        &self,
        account_details: AccountDetailsAndBalanceInputDetails,
    ) -> std::result::Result<AccountDetailsAndBalanceResponseData, String> {
        let _output = self.get_auth_token();

        let _result = _output.await;

        match _result {
            Ok(access_token_result) => {
                // Handle success case
                let account_id: String = account_details.get_account_number(); // mandatory for this request
                                                                               //let _currency: Option<String> = None;
                                                                               //let as_of_date: Option<String> = None;

                let access_token: String = self.parse_auth_token(access_token_result);
                let api_url = &self.account_details_and_balance_url;
                let mut api_url = api_url.to_string();

                api_url.push_str(&account_id);

                let _result = accounts_inquiry::account_details_and_balance::enquire(
                    access_token,
                    api_url.to_string(),
                )
                .await;

                return _result;
            }
            Err(_err) => {
                // Handle error case
                return Err(_err.to_string());
            }
        }
    }

    pub async fn enquire_account_details_and_balance_by_currency(
        &self,
        account_details: AccountDetailsAndBalanceByCurrencyInputDetails,
    ) -> std::result::Result<AccountDetailsAndBalanceResponseData, String> {
        let _output = self.get_auth_token();

        let _result = _output.await;

        match _result {
            Ok(access_token_result) => {
                // Handle success case
                let account_id: String = account_details.get_account_number(); // mandatory for this request
                let _currency: String = account_details.get_currency();
                let as_of_date: String = account_details.get_as_of_date(); // mandatory for this request
                let access_token: String = self.parse_auth_token(access_token_result);
                let api_url = &self.account_details_and_balance_url;
                let mut api_url = api_url.to_string();
                let _a = "/";

                api_url.push_str(&account_id);
                api_url.push_str(&_a);
                api_url.push_str(&_currency);
                api_url.push_str(&_a);
                api_url.push_str(&as_of_date);

                let _result = accounts_inquiry::account_details_and_balance::enquire(
                    access_token,
                    api_url.to_string(),
                )
                .await;

                return _result;
            }
            Err(_err) => {
                // Handle error case
                return Err(_err.to_string());
            }
        }
    }

    pub async fn enquire_account_details_and_balance_query(
        &self,
        account_details: AccountDetailsAndBalanceQueryInputDetails,
    ) -> std::result::Result<AccountDetailsAndBalanceResponseData, String> {
        let _output = self.get_auth_token();

        let _result = _output.await;

        match _result {
            Ok(access_token_result) => {
                // Handle success case
                let access_token: String = self.parse_auth_token(access_token_result);
                let api_url = &self.account_details_and_balance_url;

                let _result = accounts_inquiry::account_details_and_balance_query::enquire(
                    account_details,
                    access_token,
                    api_url.to_string(),
                )
                .await;

                return _result;
            }
            Err(_err) => {
                // Handle error case
                return Err(_err.to_string());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scb_gateway() {
        let consumer_key = String::from("***");
        let consumer_secret = String::from("***");
        let _env = String::from("sandbox");

        let _result = ScbGateway::new(consumer_key, consumer_secret, _env);
        assert_eq!(_result.is_ok(), true);
    }
}
