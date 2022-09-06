use serde::{de, Deserialize, Deserializer};
use std::fmt;

const GET: &str = "GET";
const POST: &str = "POST";

pub enum RequestMethod {
    Get,
    Post,
}

impl<'de> Deserialize<'de> for RequestMethod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let request_method = match <&str>::deserialize(deserializer)? {
            GET => RequestMethod::Get,
            POST => RequestMethod::Post,
            others => return Err(de::Error::unknown_variant(others, &[GET, POST])),
        };

        Ok(request_method)
    }
}

impl fmt::Display for RequestMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let request_method = match self {
            RequestMethod::Get => GET,
            RequestMethod::Post => POST,
        };

        write!(f, "{}", request_method)
    }
}
