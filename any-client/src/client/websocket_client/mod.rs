pub mod config;
pub mod item;

use self::{config::Config, item::Item};
use crate::client::prettify_json;
use anyhow::anyhow;
use futures::{SinkExt, Stream, StreamExt};
use log::{error, info};
pub use tungstenite::{Error, Message};
use url::Url;

pub async fn use_websocket_client(config_file: String) -> Result<(), anyhow::Error> {
    let config = serde_json::from_str(&config_file)?;

    let mut stream = connect(config).await?;

    info!("Receiving stream");

    while let Some(item) = stream.next().await {
        match item {
            Ok(msg) => println!("{}", Item::try_from(msg)?),
            Err(err) => error!("{}", err),
        }
    }

    Ok(())
}

async fn connect(
    config: Config,
) -> Result<impl Stream<Item = Result<Message, Error>>, anyhow::Error> {
    info!("Connecting to websocket at url '{}'", config.api_url);
    let api_url = Url::parse(&config.api_url)?;

    let (socket, response) = tokio_tungstenite::connect_async(api_url).await?;

    // Check if protocol was changed to websocket protocol (see
    // https://www.iana.org/assignments/http-status-codes/http-status-codes.xhtml)
    if response.status() == 101 {
        info!("Switching to websocket protocol");
        info!("Connection to websocket established");

        let (mut sink, stream) = socket.split();

        if let Some(subscription_request) = config.subscription_request {
            let subscription_message = Message::Text(subscription_request.to_string());

            info!(
                "Sending subscription message\nMessage: {}",
                prettify_json(subscription_request)?
            );

            sink.send(subscription_message).await?;
        }

        Ok(stream)
    } else {
        Err(anyhow!(format!(
            "Invalid status code {} for websocket response",
            response.status()
        )))
    }
}
