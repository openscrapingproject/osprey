use serde::{Deserialize, Serialize};
use super::plugin::Plugin;

use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobCollection {
    pub name: String,
    pub initial_urls: Vec<String>,
    pub requestor: PluginWithConfig,
    pub pages: HashMap<PageSetID, PageSet>,
    pub data: PluginWithConfig,
}

// PluginConfig represents generic plugin configuration read from JSON. It could then be converted to a specific plugin's configuration type
pub type PluginWithConfig = Plugin<HashMap<String, Value>>;

pub type PageSetID = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct PageSet {
    pub matcher: PluginWithConfig,
    pub extractor: Extractor,
}

// TODO: make this generic?
#[derive(Debug, Serialize, Deserialize)]
pub struct Extractor {
    pub plugin: String,
    pub config: ExtractorConfig,
    pub definition: Definition,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtractorConfig {
    pub extra: String,
}

// TODO: implement this
#[derive(Debug, Serialize, Deserialize)]
pub struct Definition {
    pub scraping: String,
}
