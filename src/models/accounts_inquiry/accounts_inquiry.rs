use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct AccountDetailsAndBalanceAsOfInputDetails {
    account_number: String,
    as_of_date: String,
}

impl AccountDetailsAndBalanceAsOfInputDetails {
    pub fn new(account_number: String, as_of_date: String) -> Result<Self, String> {
        if account_number.is_empty() || account_number.replace(" ", "").trim().len() == 0 {
            return Err(String::from("account number is empty"));
        }
        // account_number has a max length of 20 characters
        else if account_number.trim().len() > 5 && account_number.trim().len() == 20 {
            // account_number is valid
        } else {
            return Err(String::from("account number has invalid length"));
        }

        if as_of_date.is_empty() || as_of_date.replace(" ", "").trim().len() == 0 {
            return Err(String::from("as_of date is empty"));
        }
        // as_of_date has a length of 10 characters i.e 04-10-2023
        else if as_of_date.trim().len() == 10 {
            // as_of_date is valid
        } else {
            return Err(String::from("as_of date has invalid length"));
        }

        Ok(Self {
            account_number,
            as_of_date,
        })
    }

    pub fn get_account_number(&self) -> String {
        let account_number = &self.account_number;
        account_number.to_string()
    }

    pub fn get_as_of_date(&self) -> String {
        let as_of_date = &self.as_of_date;
        as_of_date.to_string()
    }
}

#[derive(Debug)]
pub struct AccountDetailsAndBalanceInputDetails {
    account_number: String,
}

impl AccountDetailsAndBalanceInputDetails {
    pub fn new(account_number: String) -> Result<Self, String> {
        if account_number.is_empty() || account_number.replace(" ", "").trim().len() == 0 {
            return Err(String::from("account number is empty"));
        }
        // account_number has a max length of 20 characters
        else if account_number.trim().len() > 5 && account_number.trim().len() == 20 {
            // account_number is valid
        } else {
            return Err(String::from("account number has invalid length"));
        }

        Ok(Self { account_number })
    }

    pub fn get_account_number(&self) -> String {
        let account_number = &self.account_number;
        account_number.to_string()
    }
}

#[derive(Debug)]
pub struct AccountDetailsAndBalanceByCurrencyInputDetails {
    account_number: String,
    _currency: String,
    as_of_date: String,
}

impl AccountDetailsAndBalanceByCurrencyInputDetails {
    pub fn new(
        account_number: String,
        _currency: String,
        as_of_date: String,
    ) -> Result<Self, String> {
        if account_number.is_empty() || account_number.replace(" ", "").trim().len() == 0 {
            return Err(String::from("account number is empty"));
        }
        // account_number has a max length of 20 characters
        else if account_number.trim().len() > 5 && account_number.trim().len() == 20 {
            // account_number is valid
        } else {
            return Err(String::from("account number has invalid length"));
        }

        if _currency.is_empty() || _currency.replace(" ", "").trim().len() == 0 {
            return Err(String::from("currency is empty"));
        }
        // _currency has a length of 3 characters i.e KES
        else if _currency.trim().len() == 3 {
            // _currency is valid
        } else {
            return Err(String::from("currency has invalid length"));
        }

        if as_of_date.is_empty() || as_of_date.replace(" ", "").trim().len() == 0 {
            return Err(String::from("as_of date is empty"));
        }
        // as_of_date has a length of 10 characters i.e 04-10-2023
        else if as_of_date.trim().len() == 10 {
            // as_of_date is valid
        } else {
            return Err(String::from("as_of date has invalid length"));
        }

        Ok(Self {
            account_number,
            _currency,
            as_of_date,
        })
    }

    pub fn get_account_number(&self) -> String {
        let account_number = &self.account_number;
        account_number.to_string()
    }

    pub fn get_currency(&self) -> String {
        let _currency = &self._currency;
        _currency.to_string()
    }

    pub fn get_as_of_date(&self) -> String {
        let as_of_date = &self.as_of_date;
        as_of_date.to_string()
    }
}

#[derive(Debug)]
pub struct AsOfDataDetails {
    _date: String,
    _type: String,
}

impl AsOfDataDetails {
    pub fn new(_date: String, _type: String) -> Result<Self, String> {
        if _date.is_empty() || _date.replace(" ", "").trim().len() == 0 {
            return Err(String::from("date is empty"));
        }
        // _date has a length of 10 characters i.e 04-10-2023
        else if _date.trim().len() == 10 {
            // _date is valid
        } else {
            return Err(String::from("date has invalid length"));
        }

        if _type.is_empty() || _type.replace(" ", "").trim().len() == 0 {
            return Err(String::from("type is empty"));
        }
        // _type has possible values
        else if _type.eq_ignore_ascii_case(&String::from("current"))
            || _type.eq_ignore_ascii_case(&String::from("historical"))
        {
            // _type is valid
        } else {
            return Err(String::from("type has invalid length"));
        }

        Ok(Self { _date, _type })
    }

    pub fn get_date(&self) -> String {
        let _date = &self._date;
        _date.to_string()
    }

    pub fn get_type(&self) -> String {
        let _type = &self._type;
        _type.to_string()
    }
}

#[derive(Debug)]
pub struct AccountNumberDataDetails {
    account_number: String,
}

impl AccountNumberDataDetails {
    pub fn new(account_number: String) -> Result<Self, String> {
        if account_number.is_empty() || account_number.replace(" ", "").trim().len() == 0 {
            return Err(String::from("account number is empty"));
        }
        // account_number has a max length of 20 characters
        else if account_number.trim().len() > 5 && account_number.trim().len() == 20 {
            // account_number is valid
        } else {
            return Err(String::from("account number has invalid length"));
        }

        Ok(Self { account_number })
    }

    pub fn get_account_number(&self) -> String {
        let account_number = &self.account_number;
        account_number.to_string()
    }
}

#[derive(Debug)]
pub struct AccountDetailsAndBalanceQueryInputDetails {
    as_of: AsOfDataDetails,
    account_numbers: Vec<AccountNumberDataDetails>,
}

impl AccountDetailsAndBalanceQueryInputDetails {
    pub fn new(
        as_of: AsOfDataDetails,
        account_numbers: Vec<AccountNumberDataDetails>,
    ) -> Result<Self, String> {
        if account_numbers.len() > 0 {
            // account_number is valid
        } else {
            return Err(String::from("account numbers has invalid length"));
        }

        Ok(Self {
            as_of,
            account_numbers,
        })
    }

    pub fn get_as_of(&self) -> &AsOfDataDetails {
        let as_of = &self.as_of;
        as_of
    }

    pub fn get_account_number(&self) -> &Vec<AccountNumberDataDetails> {
        let account_numbers = &self.account_numbers;
        account_numbers
    }
}

// request data

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct AsOfData {
    pub date: String,
    pub r#type: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct AccountIdData {
    pub accountId: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct AccountDetailsAndBalanceQueryData {
    pub asOf: AsOfData,
    pub accountIds: Vec<AccountIdData>,
}

// response data

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct OwnerDetailsData {
    pub id: Option<String>,
    pub provider: Option<String>,
    pub display_name: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct AccountDetailsData {
    pub id: Option<String>,
    pub label: Option<String>,
    pub number: Option<String>,
    pub owners: Vec<OwnerDetailsData>,
    pub r#type: Option<String>,
    pub swiftBic: Option<String>,
    pub bankId: Option<String>,
    pub currency: Option<String>,
    pub groupId: Option<String>,
    pub country: Option<String>,
    pub iban: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct AccountsInquiryResponseData {
    pub accounts: Vec<AccountDetailsData>,
}

//
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct BalanceDetailsData {
    pub currency: Option<String>,
    pub asOf: Option<String>,
    pub amount: Option<f32>,
    pub opening: Option<f32>,
    pub closing: Option<f32>,
    pub openingLedger: Option<f32>,
    pub closingLedger: Option<f32>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct AccountDetailsAndBalanceData {
    pub id: Option<String>,
    pub label: Option<String>,
    pub number: Option<String>,
    pub owners: Vec<OwnerDetailsData>,
    pub r#type: Option<String>,
    pub balance: BalanceDetailsData,
    pub iban: Option<String>,
    pub swiftBic: Option<String>,
    pub bankId: Option<String>,
    pub currency: Option<String>,
    pub groupId: Option<String>,
    pub country: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct AccountDetailsAndBalanceResponseData {
    pub accounts: Vec<AccountDetailsAndBalanceData>,
}
