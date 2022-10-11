<!-- markdownlint-disable MD024 -->

# utils-rs

Testing and utility tools written in Rust.

- [pretty-json](#pretty-json)

## pretty-json

Provides pluggable and customized `Formatter` for [`serde_json::Serializer`](https://docs.rs/serde_json/latest/serde_json/struct.Serializer.html).

### Usage

``` rust
let buf = Vec::new();
let formatter = pretty_json::Formatter::with_indent(b"  ");
let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
```
