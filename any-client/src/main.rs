mod client;

use anyhow::Context;
use client::{
    client_type::ClientType, http_client::use_http_client, websocket_client::use_websocket_client,
    Client,
};
use log::info;
use std::{env, fs, path::Path};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    logger::start().context("Can't set up logging")?;

    let config_path =
        env::var("CONFIG_PATH").context("Missing environment variable 'CONFIG_PATH'")?;
    info!("Reading file '{}'", config_path);
    let config_path = Path::new(&config_path);
    let config_file = fs::read_to_string(config_path).context("Can't read file")?;

    let client: Client =
        serde_json::from_str(&config_file).context("Can't deserialize client value")?;

    match client.client_type {
        ClientType::Http => use_http_client(config_file).await?,
        ClientType::Websocket => use_websocket_client(config_file).await?,
        ClientType::Grpc => todo!(),
    }

    Ok(())
}
