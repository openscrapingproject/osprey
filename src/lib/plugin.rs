use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use anyhow::Result;

use async_trait::async_trait;

pub type PluginID = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Plugin<Config> {
    pub plugin: PluginID,
    pub config: Config,
}

pub trait BasicPlugin: Default {
    type Config: Serialize + DeserializeOwned;

    // associated functions: not a good idea right here
    // Let's go with non modifying object funcs
    fn get_default_config(&self) -> Self::Config;
    fn parse_config(&self, input: serde_json::Value) -> Result<Self::Config>;

    fn configure(&mut self, config: Self::Config) -> Result<()>;
}

#[async_trait]
pub trait Requestor: BasicPlugin {
    type Response;
    async fn make_request(&self, url: &str) -> Result<Self::Response>;
}

pub trait Matcher: BasicPlugin {
    type MatchInput;
    fn run_match(&self, data: Self::MatchInput) -> Result<bool>;
}

pub trait Extractor: BasicPlugin {
    type Input;
    type Relevant: Serialize;

    fn extract(&self, input: Self::Input) -> Result<Self::Relevant>;
    // TODO: in the future, as we think about standardizing Scraping
    // Definitions, we might modify this signature However, for now, they
    // can go directly into the plugin's Config
}
