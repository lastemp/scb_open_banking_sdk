use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct AuthTokenResponseData {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
    pub token_type: Option<String>,
    pub expires_in: Option<u32>,
}
