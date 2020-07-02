use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use anyhow::Error;

use async_trait::async_trait;

pub type AResult<T> = Result<T, Error>;

pub type PluginID = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Plugin<Config> {
    pub plugin: PluginID,
    pub config: Config,
}

pub trait BasicPlugin {
    type Config: Serialize + DeserializeOwned;

    // associated functions: not a good idea right here
    // Let's go with non modifying object funcs
    fn get_default_config(&self) -> Self::Config;
    fn parse_config(&self, input: serde_json::Value) -> AResult<Self::Config>;

    fn configure(&mut self, config: Self::Config) -> AResult<()>;
}

#[async_trait]
pub trait Requestor: BasicPlugin {
    type Response;
    async fn make_request(&self, url: &str) -> AResult<Self::Response>;
}

pub trait Matcher: BasicPlugin {
    type MatchInput;
    fn run_match(&self, data: Self::MatchInput) -> AResult<bool>;
}

pub trait Extractor: BasicPlugin {
    type Input;
    type Relevant: Serialize;

    fn extract(&self, input: Self::Input) -> AResult<Self::Relevant>;
    // TODO: in the future, as we think about standardizing Scraping Definitions, we might modify this signature
    // However, for now, they can go directly into the plugin's Config
}
