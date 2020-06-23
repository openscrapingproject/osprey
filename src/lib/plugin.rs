use serde::{Deserialize, Serialize};
use anyhow::Error;

use async_trait::async_trait;

pub type PluginID = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct Plugin<Config> {
    plugin: PluginID,
    config: Config
}

pub trait BasicPlugin<Config> {
    fn configure(config: Config) -> Result<(), Error>;
    fn get_default_config() -> Config;
}

#[async_trait]
pub trait Requestor<C, R>: BasicPlugin<C> {
    async fn make_request(url: &str) -> Result<R, Error>;
}

pub struct MatchData {
    url: http::Uri,
    headers: http::HeaderMap,
}

trait Matcher<C>: BasicPlugin<C> {
    fn run_match(data: MatchData) -> PluginID;
}

trait Extractor<C, I, R>: BasicPlugin<C> where R: Serialize {
    // I represents input type
    // R represents Relevant data type
    fn extract(input: I) -> Result<R, Error>;
}