use super::plugin::Plugin;
use serde::{Deserialize, Serialize};

use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JobCollection {
    pub name: String,
    pub base_url: Option<String>,
    pub initial_urls: Vec<String>,
    pub requestor: PluginWithConfig,
    pub pages: HashMap<PageSetID, PageSet>,
    pub data: PluginWithConfig,
}

// PluginConfig represents generic plugin configuration read from JSON. It could then be converted to a specific plugin's configuration type
pub type PluginWithConfig = Plugin<Value>;

pub type PageSetID = String;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageSet {
    pub matcher: PluginWithConfig,
    pub extractor: Extractor,
}

// TODO: make this generic?
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Extractor {
    pub plugin: String,
    pub config: Value,
    pub definition: Value,
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
