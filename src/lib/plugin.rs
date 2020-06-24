use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

use anyhow::Error;

use async_trait::async_trait;

pub type AResult<T> = Result<T, Error>;

pub type PluginID = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct Plugin<Config> {
    plugin: PluginID,
    config: Config
}

// Most of these plugins use associated types, so that our basic agent, which bakes the types of the plugins in for static dispatch, can be written with less duplication
pub trait BasicPlugin {
    type Config: Serialize + DeserializeOwned;
    fn configure(&mut self, config: Self::Config) -> AResult<()>;
    fn get_default_config() -> Self::Config;
}

#[async_trait]
pub trait Requestor: BasicPlugin {
    type Response;
    async fn make_request(self, url: &str) -> AResult<Self::Response>;
}

pub trait Matcher: BasicPlugin {
    type MatchInput;
    fn run_match(self, data: Self::MatchInput) -> AResult<bool>;
}

pub trait Extractor: BasicPlugin {
    type Input;
    type Relevant: Serialize;
    // I represents input type
    // R represents Relevant data type
    fn extract(self, input: Self::Input) -> AResult<Self::Relevant>;
}