use serde::{Deserialize, Serialize};

// The following is from https://docs.rs/serde_any/0.5.0/src/serde_any/format.rs.html#8-21
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    /// TOML (Tom's Obvious, Minimal Language), enabled by the `toml` feature, implemented using [`toml`](https://docs.rs/toml).
    Toml,
    /// JSON (JavaScript Object Notation), enabled by the `json` feature, implemented using [`serde_json`](https://docs.rs/serde_json).
    Json,
    /// YAML (YAML Ain't Markup Language), enabled by the `yaml` feature, implemented using [`serde_yaml`](https://docs.rs/serde_yaml).
    Yaml,
    /// RON (Rusty Object Notation), enabled by the `ron` feature, implemented using [`ron`](https://docs.rs/ron).
    Ron,
    /// XML (Rusty Object Notation), enabled by the `xml` feature, implemented using [`serde-xml-rs`](https://docs.rs/serde-xml-rs).
    Xml,
    /// Url encoding (also known as percent encoding), enabled by the `url` feature, implemented using [`serde_urlencode`](https://docs.rs/serde_urlencode).
    Url,
}

use serde_any::format::Format as Fmt;

impl From<Format> for Fmt {
    fn from(item: Format) -> Self {
        match item {
            Format::Toml => Fmt::Toml,
            Format::Json => Fmt::Json,
            Format::Yaml => Fmt::Yaml,
            Format::Ron => Fmt::Ron,
            Format::Xml => Fmt::Xml,
            Format::Url => Fmt::Url
        }
    }
}