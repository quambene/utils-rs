<!-- markdownlint-disable MD024 -->

# utils-rs

Testing and utility tools written in Rust.

- [Install](#install)
- [Tools](#tools)
  - [pretty-json](#pretty-json)
  - [websocket-client](#websocket-client)
  - [http-client](#http-client)

## Install

``` bash
git clone git@github.com:quambene/utils-rs.git
cd utils-rs
```

## Tools

### pretty-json

Provides pluggable and customized `Formatter` for [`serde_json::Serializer`](https://docs.rs/serde_json/latest/serde_json/struct.Serializer.html).

#### Usage

``` rust
let buf = Vec::new();
let formatter = pretty_json::Formatter::with_indent(b"  ");
let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
```

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

### http-client

Utility tool for testing http endpoints.

#### Usage

1. Create config file, e.g. `config.json`, with content:

    ``` bash
    {
        "api": {
            "url": "https://...",
            "endpoint": "/my_endoint"
        },
        "request": {
            "method": "POST",
            "headers": [{"key": "MY_KEY", "value": "my_value"}],
            "body": {
                "myKey": "my_value"
            },
            "queryString": {
                "myKey": "my_value"
            }
        }
    }
    ```

2. Run client with environment variable `CONFIG_PATH` for package
   `http-client`:

    ``` bash
    CONFIG_PATH=config.json cargo run -p http-client
    ```
