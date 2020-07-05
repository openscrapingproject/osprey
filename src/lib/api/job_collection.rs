use super::plugin::*;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobCollection {
    pub name: String,
    pub base_url: Option<String>,
    pub initial_urls: Vec<String>,
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
