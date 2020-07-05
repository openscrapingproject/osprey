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

// pub trait BasicPlugin: Default {
//     // We used to manually configure stuff here, but now we're trying
// typetag     // fn configure(&mut self, config: serde_json::value::RawValue)
// -> Result<()>; }

// TODO: figure out what to do about default?
// Actually NVM b/c they have to be instantiated directly with their config!!

// pub trait BasicPlugin: std::fmt::Debug + std::clone::Clone + ?Sized {}

#[typetag::serde(tag = "plugin", content = "config")]
#[async_trait]
pub trait Requestor: std::fmt::Debug {
    async fn make_request(&self, url: &str) -> Result<crate::api::Response>;
}

#[typetag::serde(tag = "plugin", content = "config")]
pub trait Matcher: std::fmt::Debug {
    fn run_match(&self, data: crate::api::MatchData) -> Result<bool>;
}

use std::any::Any;

#[typetag::serde(tag = "plugin", content = "config")]
pub trait Extractor: std::fmt::Debug {
    fn extract(&self, input: crate::api::Response) -> Result<Box<dyn Any>>;
    // TODO: in the future, as we think about standardizing Scraping
    // Definitions, we might modify this signature However, for now, they
    // can go directly into the plugin's Config
}

#[typetag::serde(tag = "plugin", content = "config")]
pub trait DataSink: std::fmt::Debug {
    fn consume(&self, input: Box<dyn Any>) -> Result<()>;
}

#[cfg(test)]
mod tests {
    // use super::Requestor;
    use super::*;

    #[test]
    fn test_dyn_requestor() -> Result<()> {
        let data = include_str!("../../tests/basic.json");

        let parsed: crate::api::JobCollection = serde_json::from_str(data)?;

        println!("{:#?}", parsed.requestor);

        Ok(())
    }
}
