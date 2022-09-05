mod config;

use anyhow::Context;
use config::Config;
use log::info;
use reqwest::get;
use serde::Serialize;
use serde_json::Value;
use std::{env, fs, path::Path};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    logger::start().context("Can't set up logging")?;

    let config_path =
        env::var("CONFIG_PATH").context("Missing environment variable 'CONFIG_PATH'")?;
    info!("Reading file '{}'", config_path);
    let config_path = Path::new(&config_path);
    let config_file = fs::read_to_string(config_path).context("Can't read file")?;
    let config: Config = serde_json::from_str(&config_file).context("Can't deserialize json")?;
    let url = format!("{}{}", config.api_url, config.api_endpoint);

    info!(
        "Sending {} request to api endpoint '{}'",
        config.request_method, url
    );
    let response = get(url).await?.text().await?;

    println!("Response: {}", prettify(&response)?);

    Ok(())
}

pub fn prettify(obj: &str) -> Result<String, anyhow::Error> {
    let obj: Value = serde_json::from_str(obj)?;
    let buf = Vec::new();
    let formatter = pretty_json::Formatter::with_indent(b"  ");
    let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
    obj.serialize(&mut ser)?;
    Ok(String::from_utf8(ser.into_inner())?)
}