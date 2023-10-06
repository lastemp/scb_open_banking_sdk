use scb_open_banking_sdk::models::reporting::reporting::{
    DownloadSingleMultipleReportsInputDetails, DownloadSingleMultipleReportsResponseData,
};
use scb_open_banking_sdk::ScbGateway;

pub async fn test_download_single_multiple_reports(
    consumer_key: String,
    consumer_secret: String,
    _env: String,
) {
    let _result = ScbGateway::new(consumer_key, consumer_secret, _env);

    if let Ok(scb) = _result {
        let _product = String::from("payments");
        let _type = String::from("statement");
        let report_name = String::from("MT940");
        let as_of_date = String::from("2023-10-06"); // i.e format yyyy-mm-dd
        let fetch_report_type = String::from("LATEST");
        let _result = DownloadSingleMultipleReportsInputDetails::new(
            _product,
            _type,
            report_name,
            as_of_date,
            fetch_report_type,
        );

        if let Ok(account_details) = _result {
            let _output = scb.download_single_multiple_reports(account_details);
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
