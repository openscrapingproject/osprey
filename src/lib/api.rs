use super::plugin::*;
use serde::{Deserialize, Serialize};

use serde_json::value::RawValue;
use std::collections::HashMap;

use crate::http;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobCollection {
    pub name: String,
    pub base_url: Option<String>,
    pub initial_urls: Vec<String>,
    pub requestor: Box<dyn Requestor>,
    pub pages: HashMap<PageSetID, PageSet>,
    pub data: Option<Box<dyn DataSink>>,
}

// PluginConfig represents generic plugin configuration read from JSON. It
// could then be converted to a specific plugin's configuration type
// pub type PluginWithConfig<'a> = Plugin<&'a RawValue>;

pub type PageSetID = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct PageSet {
    pub matcher: Box<dyn Matcher>,
    pub extractor: Box<dyn Extractor>,
}

// TODO: how to add additional field???
// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct Extractor<'a> {
//     pub plugin: String,
//     pub config: &'a RawValue,
//     pub definition: &'a RawValue,
// }

type HeaderMap = HashMap<String, String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub status: http::StatusCode,
    pub version: http::Version,
    pub headers: HeaderMap,
    pub body: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchData {
    pub url: http::URL,
    pub headers: HeaderMap,
    pub status: http::StatusCode,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct ExtractorConfig {
//     pub extra: String,
// }

// // TODO: implement this
// #[derive(Debug, Serialize, Deserialize)]
// pub struct Definition {
//     pub scraping: String,
// }
