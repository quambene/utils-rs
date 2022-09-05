mod item;
mod json;

use anyhow::Context;
use futures::StreamExt;
use item::Item;
use log::{error, info};
use std::{env, fs, path::Path};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    logger::start().context("Can't set up logging")?;

    let config_path =
        env::var("CONFIG_PATH").context("Missing environment variable 'CONFIG_PATH'")?;
    info!("Reading file '{}'", config_path);
    let config_path = Path::new(&config_path);
    let config_file = fs::read_to_string(config_path)?;
    let config = serde_json::from_str(&config_file)?;

    let mut stream = websocket_utils::connect(config).await?;

    while let Some(item) = stream.next().await {
        match item {
            Ok(msg) => println!("{}", Item::try_from(msg)?),
            Err(err) => error!("{}", err),
        }
    }

    Ok(())
}
