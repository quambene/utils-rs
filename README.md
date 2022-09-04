# utils-rs

Testing and utility tools written in Rust.

- [Install](#install)
- [Tools](#tools)
  - [websocket-client](#websocket-client)

## Install

``` bash
git clone git@github.com:quambene/utils-rs.git
cd utils-rs
```

## Tools

### websocket-client

Utility tool for testing websocket endpoints for streaming data.

#### Usage

1. Create config file, e.g. `config.json`:

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
