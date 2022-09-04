use anyhow::anyhow;
use futures::{SinkExt, Stream, StreamExt};
use log::info;
use serde::Deserialize;
use serde_json::Value;
pub use tungstenite::{Error, Message};
use url::Url;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    api_url: String,
    subscription_request: Value,
}

pub async fn connect(
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

        let subscription_message = Message::Text(config.subscription_request.to_string());

        info!(
            "Sending subscription request: {}",
            config.subscription_request
        );

        sink.send(subscription_message).await?;

        Ok(stream)
    } else {
        Err(anyhow!(format!(
            "Invalid status code {} for websocket response",
            response.status()
        )))
    }
}
