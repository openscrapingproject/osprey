/*!
Defines Job related types.

Includes Job, Config, and JobCollection.

TODO: think about changing URL type to impose more validation
requirements when deserializing with sede. E.g. Into<Url>?
TODO: think about remote config
TODO: think about whether we need to add Clone to these traits
*/

use super::plugin::*;
use crate::prelude::*;

use std::collections::HashMap;

/// Represents a format that creates several jobs from one representation
#[derive(Debug, Serialize, Deserialize)]
pub struct JobCollection {
    pub name: String,
    pub base_url: Option<String>,
    pub initial_urls: Vec<String>,
    pub config: Config,
}

// pub enum RemoteConfig {
//     // TODO: should this be a data-service neutral URL
//     // or a well-defined ID that requires knowledge of the
//     // Server API
//     Remote(String),
//     Embedded(Config)
// }
#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    pub url: String,
    pub config: Config,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub requestor: Box<dyn Requestor>,
    pub pages: HashMap<PageSetID, PageSet>,
    pub data: Option<Box<dyn DataSink>>,
}

pub type PageSetID = String;

#[derive(Debug, Serialize, Deserialize)]
pub struct PageSet {
    pub matcher: Box<dyn Matcher>,
    pub extractor: Box<dyn Extractor>,
}
