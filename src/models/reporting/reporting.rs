use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct DownloadSingleMultipleReportsInputDetails {
    _product: String,
    _type: String,
    report_name: String,
    as_of_date: String,
    fetch_report_type: String,
}

impl DownloadSingleMultipleReportsInputDetails {
    pub fn new(
        _product: String,
        _type: String,
        report_name: String,
        as_of_date: String,
        fetch_report_type: String,
    ) -> Result<Self, String> {
        if _product.is_empty() || _product.replace(" ", "").trim().len() == 0 {
            return Err(String::from("product is empty"));
        }
        // _product has possible values
        else if _product.eq_ignore_ascii_case(&String::from("collections"))
            || _product.eq_ignore_ascii_case(&String::from("payments"))
            || _product.eq_ignore_ascii_case(&String::from("account"))
        {
            // _product is valid
        } else {
            return Err(String::from("product has invalid value"));
        }

        if _type.is_empty() || _type.replace(" ", "").trim().len() == 0 {
            return Err(String::from("type is empty"));
        }
        // _type has possible values
        else if _type.eq_ignore_ascii_case(&String::from("DDA"))
            || _type.eq_ignore_ascii_case(&String::from("DDI"))
            || _type.eq_ignore_ascii_case(&String::from("payment status"))
            || _type.eq_ignore_ascii_case(&String::from("statement"))
        {
            // _type is valid
        } else {
            return Err(String::from("type has invalid value"));
        }

        if report_name.is_empty() || report_name.replace(" ", "").trim().len() == 0 {
            return Err(String::from("report name is empty"));
        }
        // report_name has possible values
        else if report_name.eq_ignore_ascii_case(&String::from("QLDS0010R"))
            || report_name.eq_ignore_ascii_case(&String::from("E_140"))
            || report_name.eq_ignore_ascii_case(&String::from("E_139"))
            || report_name.eq_ignore_ascii_case(&String::from("MT940"))
            || report_name.eq_ignore_ascii_case(&String::from("MT940A"))
            || report_name.eq_ignore_ascii_case(&String::from("MT942"))
            || report_name.eq_ignore_ascii_case(&String::from("CAMT52"))
            || report_name.eq_ignore_ascii_case(&String::from("CAMT53"))
            || report_name.eq_ignore_ascii_case(&String::from("MT940-UTF8"))
            || report_name.eq_ignore_ascii_case(&String::from("MT942-UTF8"))
        {
            // report_name is valid
        } else {
            return Err(String::from("report name has invalid value"));
        }

        if as_of_date.is_empty() || as_of_date.replace(" ", "").trim().len() == 0 {
            return Err(String::from("as of date is empty"));
        }
        // as_of_date has a length of 10 characters i.e format yyyy-mm-dd
        else if as_of_date.trim().len() == 10 {
            // as_of_date is valid
        } else {
            return Err(String::from("as of date has invalid length"));
        }

        if fetch_report_type.is_empty() || fetch_report_type.replace(" ", "").trim().len() == 0 {
            return Err(String::from("fetch report type is empty"));
        }
        // fetch_report_type has possible values
        else if fetch_report_type.eq_ignore_ascii_case(&String::from("LATEST"))
            || fetch_report_type.eq_ignore_ascii_case(&String::from("PENDING"))
            || fetch_report_type.eq_ignore_ascii_case(&String::from("DEFAULT"))
        {
            // fetch_report_type is valid
        } else {
            return Err(String::from("fetch report type has invalid value"));
        }

        Ok(Self {
            _product,
            _type,
            report_name,
            as_of_date,
            fetch_report_type,
        })
    }

    pub fn get_product(&self) -> String {
        let _product = &self._product;
        _product.to_string()
    }

    pub fn get_type(&self) -> String {
        let _type = &self._type;
        _type.to_string()
    }

    pub fn get_report_name(&self) -> String {
        let report_name = &self.report_name;
        report_name.to_string()
    }

    pub fn get_as_of_date(&self) -> String {
        let as_of_date = &self.as_of_date;
        as_of_date.to_string()
    }

    pub fn get_fetch_report_type(&self) -> String {
        let fetch_report_type = &self.fetch_report_type;
        fetch_report_type.to_string()
    }
}

// response data

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct ReportData {
    pub id: Option<String>,
    pub content: Option<String>,
    pub dateTimeStamp: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct DownloadSingleMultipleReportsResponseData {
    pub groupId: Option<String>,
    pub reports: Vec<ReportData>,
    pub reportName: Option<String>,
}
