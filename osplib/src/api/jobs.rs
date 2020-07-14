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

// TODO: make this private or depracate
/// Represents a format that creates several jobs from one representation
#[derive(Debug, Serialize, Deserialize)]
pub struct JobCollection {
    pub name: String,
    pub base_url: Option<String>,
    pub initial_urls: Vec<String>,
    pub config: Config,
}

/// This struct represents the minimal information needed
/// to be passed to an Agent. However, often, Executor implementations
/// will need a structure with much more metadata, such as the state
/// of the job.
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
