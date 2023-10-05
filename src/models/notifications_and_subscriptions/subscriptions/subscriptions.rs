use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct NotificationSubscriptionRetrieveByNotificationTypeInputDetails {
    notification_type: String,
}

impl NotificationSubscriptionRetrieveByNotificationTypeInputDetails {
    pub fn new(notification_type: String) -> Result<Self, String> {
        if notification_type.is_empty() || notification_type.replace(" ", "").trim().len() == 0 {
            return Err(String::from("notification type is empty"));
        }
        // notification_type has possible values
        else if notification_type.eq_ignore_ascii_case(&String::from("account-status"))
            || notification_type.eq_ignore_ascii_case(&String::from("credit-debit-advice"))
            || notification_type.eq_ignore_ascii_case(&String::from("custody-status"))
            || notification_type.eq_ignore_ascii_case(&String::from("dda-status"))
            || notification_type.eq_ignore_ascii_case(&String::from("ddi-status"))
            || notification_type.eq_ignore_ascii_case(&String::from("newsflash"))
            || notification_type.eq_ignore_ascii_case(&String::from("payment-status"))
            || notification_type.eq_ignore_ascii_case(&String::from("payee-account-details"))
            || notification_type.eq_ignore_ascii_case(&String::from("payee-status"))
        {
            // notification_type is valid
        } else {
            return Err(String::from("notification type has invalid length"));
        }

        Ok(Self { notification_type })
    }

    pub fn get_notification_type(&self) -> String {
        let notification_type = &self.notification_type;
        notification_type.to_string()
    }
}

#[derive(Debug)]
pub struct NotificationSubscriptionCreateUpdateInputDetails {
    notification_type: String,
}

impl NotificationSubscriptionCreateUpdateInputDetails {
    pub fn new(notification_type: String) -> Result<Self, String> {
        if notification_type.is_empty() || notification_type.replace(" ", "").trim().len() == 0 {
            return Err(String::from("notification type is empty"));
        }
        // notification_type has possible values
        else if notification_type.eq_ignore_ascii_case(&String::from("account-status"))
            || notification_type.eq_ignore_ascii_case(&String::from("credit-debit-advice"))
            || notification_type.eq_ignore_ascii_case(&String::from("custody-status"))
            || notification_type.eq_ignore_ascii_case(&String::from("dda-status"))
            || notification_type.eq_ignore_ascii_case(&String::from("ddi-status"))
            || notification_type.eq_ignore_ascii_case(&String::from("newsflash"))
            || notification_type.eq_ignore_ascii_case(&String::from("payment-status"))
            || notification_type.eq_ignore_ascii_case(&String::from("payee-account-details"))
            || notification_type.eq_ignore_ascii_case(&String::from("payee-status"))
        {
            // notification_type is valid
        } else {
            return Err(String::from("notification type has invalid length"));
        }

        Ok(Self { notification_type })
    }

    pub fn get_notification_type(&self) -> String {
        let notification_type = &self.notification_type;
        notification_type.to_string()
    }
}

// response data

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct FilterData {
    pub fieldName: Option<String>,
    pub filterType: Option<String>,
    pub fieldValues: Option<Vec<String>>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct FilterAndOtherData {
    pub filters: Vec<FilterData>,
    pub webhookPath: Option<String>,
    pub deliveryType: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct NotificationSubscriptionRetrieveAllResponseData {
    #[serde(rename = "payment-status")]
    pub payment_status: FilterAndOtherData,
    #[serde(rename = "credit-debit-advice")]
    pub credit_debit_advice: FilterAndOtherData,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct NotificationSubscriptionRetrieveByNotificationTypeResponseData {
    pub filters: Vec<FilterData>,
    pub webhookPath: Option<String>,
    pub deliveryType: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct NotificationSubscriptionCreateUpdateResponseData {
    pub filters: Vec<FilterData>,
    pub webhookPath: Option<String>,
    pub deliveryType: Option<String>,
}
