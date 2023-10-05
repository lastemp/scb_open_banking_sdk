use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct RateQuotesBatchInputDetails {
    client_id: String,
    rate_category_id: String,
}

impl RateQuotesBatchInputDetails {
    pub fn new(client_id: String, rate_category_id: String) -> Result<Self, String> {
        if client_id.is_empty() || client_id.replace(" ", "").trim().len() == 0 {
            return Err(String::from("client id is empty"));
        }
        // client_id has a max length of 64 characters
        else if client_id.trim().len() > 0 && client_id.trim().len() == 64 {
            // client_id is valid
        } else {
            return Err(String::from("client id has invalid length"));
        }

        if rate_category_id.is_empty() || rate_category_id.replace(" ", "").trim().len() == 0 {
            return Err(String::from("rate category id is empty"));
        }
        // rate_category_id has a max length of 64 characters
        else if rate_category_id.trim().len() > 0 && rate_category_id.trim().len() == 64 {
            // rate_category_id is valid
        } else {
            return Err(String::from("rate category id has invalid length"));
        }

        Ok(Self {
            client_id,
            rate_category_id,
        })
    }

    pub fn get_client_id(&self) -> String {
        let client_id = &self.client_id;
        client_id.to_string()
    }

    pub fn get_rate_category_id(&self) -> String {
        let rate_category_id = &self.rate_category_id;
        rate_category_id.to_string()
    }
}

#[derive(Debug)]
pub struct RateQuotesByCurrencyPairInputDetails {
    client_id: String,
    rate_category_id: String,
    buy_currency: String,
    sell_currency: String,
    _tenor: u16,
}

impl RateQuotesByCurrencyPairInputDetails {
    pub fn new(
        client_id: String,
        rate_category_id: String,
        buy_currency: String,
        sell_currency: String,
        _tenor: u16,
    ) -> Result<Self, String> {
        if client_id.is_empty() || client_id.replace(" ", "").trim().len() == 0 {
            return Err(String::from("client id is empty"));
        }
        // client_id has a max length of 64 characters
        else if client_id.trim().len() > 0 && client_id.trim().len() == 64 {
            // client_id is valid
        } else {
            return Err(String::from("client id has invalid length"));
        }

        if rate_category_id.is_empty() || rate_category_id.replace(" ", "").trim().len() == 0 {
            return Err(String::from("rate category id is empty"));
        }
        // rate_category_id has a max length of 64 characters
        else if rate_category_id.trim().len() > 0 && rate_category_id.trim().len() == 64 {
            // rate_category_id is valid
        } else {
            return Err(String::from("rate category id has invalid length"));
        }

        if buy_currency.is_empty() || buy_currency.replace(" ", "").trim().len() == 0 {
            return Err(String::from("buy currency is empty"));
        }
        // buy_currency has a length of 3 characters
        else if buy_currency.trim().len() == 3 {
            // buy_currency is valid
        } else {
            return Err(String::from("buy currency has invalid length"));
        }

        if sell_currency.is_empty() || sell_currency.replace(" ", "").trim().len() == 0 {
            return Err(String::from("sell currency is empty"));
        }
        // sell_currency has a length of 3 characters
        else if sell_currency.trim().len() == 3 {
            // sell_currency is valid
        } else {
            return Err(String::from("sell currency has invalid length"));
        }

        if _tenor > 0 {
            // _tenor is valid
        } else {
            return Err(String::from("tenor has invalid value"));
        }

        Ok(Self {
            client_id,
            rate_category_id,
            buy_currency,
            sell_currency,
            _tenor,
        })
    }

    pub fn get_client_id(&self) -> String {
        let client_id = &self.client_id;
        client_id.to_string()
    }

    pub fn get_rate_category_id(&self) -> String {
        let rate_category_id = &self.rate_category_id;
        rate_category_id.to_string()
    }

    pub fn get_buy_currency(&self) -> String {
        let buy_currency = &self.buy_currency;
        buy_currency.to_string()
    }

    pub fn get_sell_currency(&self) -> String {
        let sell_currency = &self.sell_currency;
        sell_currency.to_string()
    }

    pub fn get_tenor(&self) -> u16 {
        let _tenor = &self._tenor;
        *_tenor
    }
}

#[derive(Debug)]
pub struct QuotesValidationInputDetails {
    client_id: String,
}

impl QuotesValidationInputDetails {
    pub fn new(client_id: String) -> Result<Self, String> {
        if client_id.is_empty() || client_id.replace(" ", "").trim().len() == 0 {
            return Err(String::from("client id is empty"));
        }
        // client_id has a max length of 64 characters
        else if client_id.trim().len() > 0 && client_id.trim().len() == 64 {
            // client_id is valid
        } else {
            return Err(String::from("client id has invalid length"));
        }

        Ok(Self { client_id })
    }

    pub fn get_client_id(&self) -> String {
        let client_id = &self.client_id;
        client_id.to_string()
    }
}

// response data

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum MixedTypeValue {
    IntegerValue(u32),
    FloatValue(f32),
    StringValue(String),
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct RateQuotesData {
    pub rateType: Option<String>,
    pub rateQuoteID: Option<String>,
    pub rateCategoryID: Option<String>,
    pub validFrom: Option<u32>,
    pub validTill: Option<u32>,
    pub buyCurrency: Option<String>,
    pub sellCurrency: Option<String>,
    pub tenor: MixedTypeValue, // Option<u32>,
    pub rate: MixedTypeValue,  // Option<f32>,
    pub clientRateType: Option<String>,
    pub status: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct RateQuotesResponseData {
    pub clientID: Option<String>,
    pub clientRequestID: Option<String>,
    pub status: Option<String>,
    pub rates: Vec<RateQuotesData>,
}
