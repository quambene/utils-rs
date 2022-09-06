mod request_method;

pub(crate) use self::request_method::RequestMethod;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct Header {
    pub key: String,
    pub value: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub api_url: String,
    pub api_endpoint: String,
    pub headers: Option<Vec<Header>>,
    pub request_method: RequestMethod,
    pub request_body: Option<Value>,
}
