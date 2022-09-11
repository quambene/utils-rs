mod config;

use crate::config::RequestMethod;
use anyhow::Context;
use config::Config;
use log::info;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client,
};
use serde::Serialize;
use serde_json::Value;
use std::{env, fs, path::Path, str::FromStr};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    logger::start().context("Can't set up logging")?;

    let config_path =
        env::var("CONFIG_PATH").context("Missing environment variable 'CONFIG_PATH'")?;
    info!("Reading file '{}'", config_path);
    let config_path = Path::new(&config_path);
    let config_file = fs::read_to_string(config_path).context("Can't read file")?;
    let config: Config = serde_json::from_str(&config_file).context("Can't deserialize json")?;
    let url = format!("{}{}", config.api.url, config.api.endpoint);

    info!(
        "Sending {} request to api endpoint '{}'",
        config.request.method, url
    );

    let client = Client::new();

    let mut header_map = HeaderMap::new();

    if let Some(headers) = config.request.headers {
        for header in headers {
            let key = HeaderName::from_str(&header.key)?;
            let value = HeaderValue::from_str(&header.value)?;
            header_map.insert(key, value);
        }
    }

    let mut request = match config.request.method {
        RequestMethod::Get => client.get(url),
        RequestMethod::Post => client.post(url),
        RequestMethod::Put => client.put(url),
        RequestMethod::Delete => client.delete(url),
    };

    request = request.headers(header_map);

    if let Some(value) = &config.request.query_string {
        request = request.query(value)
    };

    if let Some(value) = &config.request.body {
        request = request.json(value)
    };

    let response = request.send().await?;
    let response_status = response.status();
    let response_body = response.text().await?;

    println!("Status code: {}", response_status);

    match serde_json::from_str::<Value>(&response_body) {
        Ok(value) => println!("Response body: {}", prettify(value)?),
        Err(_) => println!("Response body: {}", response_body),
    }

    Ok(())
}

pub fn prettify(obj: Value) -> Result<String, anyhow::Error> {
    let buf = Vec::new();
    let formatter = pretty_json::Formatter::with_indent(b"  ");
    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
    obj.serialize(&mut ser)?;
    Ok(String::from_utf8(ser.into_inner())?)
}
