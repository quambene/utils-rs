use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub api_url: String,
    pub subscription_request: Option<Value>,
}
