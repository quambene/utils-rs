use anyhow::{anyhow, Context};
use futures::{SinkExt, Stream, StreamExt};
use log::info;
use std::{env, fs, path::Path};
use tungstenite::{Error, Message};
use url::Url;

pub async fn connect() -> Result<impl Stream<Item = Result<Message, Error>>, anyhow::Error> {
    let api_url = env::var("API_URL").context("Missing environment variable 'API_URL'")?;
    let websocket_url = Url::parse(&api_url)?;
    let subscription_path = env::var("SUBSCRIPTION_PATH")
        .context("Missing environment variable 'SUBSCRIPTION_PATH'")?;
    let subscription_path = Path::new(&subscription_path);

    info!("Reading file '{}'", subscription_path.display());
    let subscription_file = fs::read_to_string(subscription_path)?;

    info!("Connecting to websocket at url '{}'", api_url);
    let (socket, response) = tokio_tungstenite::connect_async(websocket_url).await?;

    // Check if protocol was changed to websocket protocol (see
    // https://www.iana.org/assignments/http-status-codes/http-status-codes.xhtml)
    if response.status() == 101 {
        info!("Switching to websocket protocol");
        info!("Connection to websocket established");

        let (mut sink, stream) = socket.split();

        let subscription_message = Message::Text(subscription_file);

        info!("Subscribing to channel");

        sink.send(subscription_message).await?;

        Ok(stream)
    } else {
        Err(anyhow!(format!(
            "Invalid status code {} for websocket response",
            response.status()
        )))
    }
}
