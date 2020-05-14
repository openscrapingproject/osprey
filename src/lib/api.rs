use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Agent {
    name: String,
    initial_urls: Vec<String>,
    requestor: Option<Requestor>,
    pages: Option<Pages>,
    parsers: Option<Parsers>,
    data: Option<Data>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    plugin: Option<String>,
    config: Option<DataConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataConfig {
    serialize: Option<String>,
    out: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pages {
    #[serde(rename = "<page_ID>")]
    page_id: Option<PageId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageId {
    matcher: Option<Matcher>,
    data: Option<PageIdData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageIdData {
    #[serde(rename = "<data_layer>")]
    data_layer: Option<DataLayer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataLayer {
    plugin: Option<String>,
    config: Option<DataLayerConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataLayerConfig {
    #[serde(rename = "econfKey")]
    econf_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Matcher {
    url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parsers {
    html: Option<Html>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Html {
    plugin: Option<String>,
    headers: Option<Headers>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Headers {
    #[serde(rename = "Content-Type")]
    content_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Requestor {
    plugin: Option<String>,
    config: Option<RequestorConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestorConfig {
    browser: Option<Vec<String>>,
    template: Option<String>,
}
