mod item;

use anyhow::Context;
use futures::StreamExt;
use item::Item;
use log::error;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    logger::start().context("Can't set up logging")?;

    let mut stream = websocket_utils::connect().await?;

    while let Some(item) = stream.next().await {
        match item {
            Ok(msg) => println!("{}", Item::try_from(msg)?),
            Err(err) => error!("{}", err),
        }
    }

    Ok(())
}
