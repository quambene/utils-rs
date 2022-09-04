use std::{
    fmt,
    str::{self, Utf8Error},
};
use thiserror::Error;
use websocket_utils::Message;

pub(crate) struct Item {
    content: String,
    kind: String,
}

#[derive(Error, Debug)]
pub(crate) enum ItemError {
    #[error("Can't convert to string: {0}")]
    Utf8Error(String),
}

impl Item {
    pub fn new(content: String, kind: String) -> Self {
        Self { content, kind }
    }
}

impl TryFrom<Message> for Item {
    type Error = ItemError;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        match message {
            Message::Text(data) => Ok(Item::new(data, "Text".to_string())),
            Message::Binary(ref data) => {
                let content = str::from_utf8(data)?;
                Ok(Item::new(content.to_string(), "Binary".to_string()))
            }
            Message::Ping(ref data) => {
                let content = str::from_utf8(data)?;
                Ok(Item::new(content.to_string(), "Ping".to_string()))
            }
            Message::Pong(ref data) => {
                let content = str::from_utf8(data)?;
                Ok(Item::new(content.to_string(), "Pong".to_string()))
            }
            Message::Frame(frame) => {
                let content = str::from_utf8(&frame.payload())?;
                Ok(Item::new(content.to_string(), "Frame".to_string()))
            }
            Message::Close(Some(frame)) => {
                let content = frame.reason;
                Ok(Item::new(content.to_string(), "Close".to_string()))
            }
            Message::Close(None) => Ok(Item::new("Close".to_string(), "Close".to_string())),
        }
    }
}

impl From<Utf8Error> for ItemError {
    fn from(err: Utf8Error) -> Self {
        Self::Utf8Error(err.to_string())
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.content)
    }
}
