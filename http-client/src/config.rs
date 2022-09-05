use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub api_url: String,
    pub api_endpoint: String,
    pub request_method: String,
    pub request_body: Option<Value>,
}
