mod accounts_inquiry {
    pub mod account_details_and_balance;
    pub mod account_details_and_balance_as_of;
    pub mod account_details_and_balance_by_currency;
    pub mod account_details_and_balance_query;
    pub mod list_of_accounts;
}
pub mod corporate_financial_markets {
    pub mod rate_quotes {
        pub mod rate_quotes_batch;
        pub mod rate_quotes_by_currency_pair;
        pub mod validate_quotes;
    }
}
pub mod notifications_and_subscriptions {
    pub mod subscriptions {
        pub mod subscriptions_create_update;
        pub mod subscriptions_retrieve_all;
        pub mod subscriptions_retrieve_by_notification_type;
    }
}
pub mod reporting {
    pub mod download_single_multiple_reports;
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

    // accounts_inquiry
    let x = accounts_inquiry::list_of_accounts::test_enquire_list_of_accounts(
        consumer_key,
        consumer_secret,
        _env,
    );
    /*
    let x = accounts_inquiry::account_details_and_balance::test_enquire_account_details_and_balance(
        consumer_key,
        consumer_secret,
        _env,
    );

    let x = accounts_inquiry::account_details_and_balance_as_of::test_enquire_account_details_and_balance_as_of(
        consumer_key,
        consumer_secret,
        _env,
    );

    let x = accounts_inquiry::account_details_and_balance_by_currency::test_enquire_account_details_and_balance_by_currency(
        consumer_key,
        consumer_secret,
        _env,
    );

    let x =
        accounts_inquiry::account_details_and_balance_query::test_enquire_account_details_and_balance_query(
            consumer_key,
            consumer_secret,
            _env,
        );

    // corporate_financial_markets

    let x =
        corporate_financial_markets::rate_quotes::rate_quotes_batch::test_enquire_rate_quotes_batch(
            consumer_key,
            consumer_secret,
            _env,
        );

    let x = corporate_financial_markets::rate_quotes::rate_quotes_by_currency_pair::test_enquire_rate_quotes_by_currency_pair(
        consumer_key,
        consumer_secret,
        _env,
    );

    let x = corporate_financial_markets::rate_quotes::validate_quotes::test_validate_quotes(
        consumer_key,
        consumer_secret,
        _env,
    );

    // notifications_and_subscriptions

    let x = notifications_and_subscriptions::subscriptions::subscriptions_create_update::test_notification_subscriptions_create_update(
        consumer_key,
        consumer_secret,
        _env,
    );

    let x =
        notifications_and_subscriptions::subscriptions::subscriptions_retrieve_all::test_notification_subscription_retrieve_all(
            consumer_key,
            consumer_secret,
            _env,
        );

    let x = notifications_and_subscriptions::subscriptions::subscriptions_retrieve_by_notification_type::test_notification_subscription_retrieve_by_notification_type(
        consumer_key,
        consumer_secret,
        _env,
    );

    // reporting

    let x = reporting::download_single_multiple_reports::test_download_single_multiple_reports(
        consumer_key,
        consumer_secret,
        _env,
    );
     */

    x.await;
}
