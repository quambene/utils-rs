use anyhow::Context;
use futures::StreamExt;
use log::error;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    logger::start().context("Can't set up logging")?;

    let mut stream = websocket_utils::connect().await?;

    while let Some(item) = stream.next().await {
        match item {
            Ok(item) => println!("{}", item),
            Err(err) => error!("{}", err),
        }
    }

    Ok(())
}
