<!-- markdownlint-disable MD024 -->

# utils-rs

Testing and utility tools written in Rust.

- [Install](#install)
- [Tools](#tools)
  - [websocket-client](#websocket-client)
  - [pretty-json](#pretty-json)

## Install

``` bash
git clone git@github.com:quambene/utils-rs.git
cd utils-rs
```

## Tools

### websocket-client

Utility tool for testing websocket endpoints for streaming data.

The client connects to websocket endpoint `apiUrl` and subscribes to a channel by sending the request `subscriptionRequest`.

#### Usage

1. Create config file, e.g. `config.json`, with content:

    ``` bash
    {
        "apiUrl": "wss://...",
        "subscriptionRequest": {
            ...
        }
    }
    ```

2. Run client with environment variable `CONFIG_PATH` for package `websocket-client`:

    ``` bash
    CONFIG_PATH=config.json cargo run -p websocket-client
    ```

### pretty-json

Provides pluggable and customized `Formatter` for [`serde_json::Serializer`](https://docs.rs/serde_json/latest/serde_json/struct.Serializer.html).

#### Usage

``` rust
let buf = Vec::new();
let formatter = pretty_json::Formatter::with_indent(b"  ");
let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
```
