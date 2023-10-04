# scb_open_banking_sdk

This is an sdk that will be used by developers to seamlessly integrate with Standard Chartered Bank Open Banking APIs' Gateway.
Standard Chartered Plc (Standard Chartered) is a global financial services provider that provides a range of personal banking, wholesale banking, private banking, SME banking, and business banking solutions.
The API endpoints provided by Open Banking Gateway includes; Accounts Inquiry, Corporate Financial Markets, Move Money, {Notifications and Subscriptions}, Reporting, Securities Services, Trade and {Utilities and Tools} (https://axess.sc.com/). 

The sdk has below listed dependencies:
- [Serde](https://github.com/serde-rs/serde) for serializing and deserializing Rust data structures
- [Reqwest](https://github.com/seanmonstar/reqwest) Rust HTTP Client
- [serde_json](https://github.com/serde-rs/json) for serializing and deserializing Rust data structures
- [serde_urlencoded](https://github.com/nox/serde_urlencoded) for serialising to and deserialising from the application/x-www-form-urlencoded format
- [chrono](https://github.com/chronotope/chrono) provides all functionality needed to do correct operations on dates and times
- [base64](https://github.com/marshallpierce/rust-base64) Decode from Base64 format or encode into it
- [tokio](https://github.com/tokio-rs/tokio) A runtime for writing reliable, asynchronous applications

## installation

```
cargo install --git https://github.com/lastemp/scb_open_banking_sdk
```

## Usage

Please find below code samples and full working examples:

   - See [the code samples](./code_samples/) for more info.	
   - See [the examples](./examples/) for full working examples.
