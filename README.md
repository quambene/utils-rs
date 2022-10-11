<!-- markdownlint-disable MD024 -->

# utils-rs

Testing and utility tools written in Rust.

- [pretty-json](#pretty-json)
- [any-client](#any-client)

## pretty-json

Provides pluggable and customized `Formatter` for [`serde_json::Serializer`](https://docs.rs/serde_json/latest/serde_json/struct.Serializer.html).

### Usage

``` rust
let buf = Vec::new();
let formatter = pretty_json::Formatter::with_indent(b"  ");
let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
```

## any-client

Utility tool for testing http, websocket, and grpc endpoints.

### Install any-client

``` bash
git clone git@github.com:quambene/utils-rs.git
cd utils-rs/any-client
cargo install --path .
```

### Usage

``` bash
CONFIG_PATH=config.json any-client
```

### Development

``` bash
cd utils-rs/any-client
CONFIG_PATH=config.json cargo run
```

### Configure HTTP client

Create config file, e.g. `config.json`, with content:

``` json
{
    "client": "http",
    "api": {
        "url": "https://...",
        "endpoint": "/my_endpoint"
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

### Configure websocket client

Create config file, e.g. `config.json`, with content:

``` json
{
    "client": "websocket",
    "api": {
        "url": "wss://my_url.com",
        "endpoint": "/my_endpoint"
    },
    "subscription": {
        "query_string": {
            "myParam1": "my_value1",
            "myParam2": "my_value2"
        },
        "request": {
            "myKey": "my_value"
        }
    }
}
```

where `query_string` and `request` are optional. The `query_string` is leading to the url `wss://my_url.com/my_endoint?myParam1=my_value1&myParam2=my_value2`.

### Configure grpc client

Create config file, e.g. `config.json`, with content:

``` json
{
    "client": "grpc",
    "api": {
        "url": "https://...",
    },
    "proto": {
        "path": "my_proto_dir",
        "file": "my_proto_file.proto",
        "package": "my_package",
        "service": "MyService",
        "method": "MyMethod",
        "message": "MyMessage",
        "request": {
            "myKey": "my_value"
        }
    }
}
```
