use anyhow::Result;

use async_trait::async_trait;

pub type PluginID = String;

#[typetag::serde(tag = "plugin", content = "config")]
#[async_trait]
pub trait Requestor: std::fmt::Debug {
    /// Makes an HTTP get request to the given URL
    /// using any configuration the plugin was created with.
    /// TODO: add another function for a HEAD request?
    /// TODO: think about user interaction sequences
    async fn make_request(&self, url: &str) -> Result<crate::api::Response>;
}

#[typetag::serde(tag = "plugin", content = "config")]
pub trait Matcher: std::fmt::Debug {
    fn run_match(&self, data: crate::api::MatchData) -> Result<bool>;
}

use std::any::Any;

pub trait SerDebug: erased_serde::Serialize + std::fmt::Debug {}
impl<T> SerDebug for T where T: erased_serde::Serialize + std::fmt::Debug {}

pub type Intermediate = Box<dyn SerDebug>;

#[typetag::serde(tag = "plugin", content = "config")]
pub trait Extractor: std::fmt::Debug {
    /// TODO: in the future, as we think about standardizing Scraping
    /// Definitions, we might modify this signature. However, for now, they
    /// can go directly into the plugin's Config.
    fn extract(&self, input: &crate::api::Response) -> Result<Intermediate>;
}

#[typetag::serde(tag = "plugin", content = "config")]
pub trait DataSink: std::fmt::Debug {
    fn consume(&self, input: Intermediate) -> Result<()>;
}

#[cfg(test)]
mod tests {
    // use super::Requestor;
    use super::*;

    #[test]
    fn test_dyn_requestor() -> Result<()> {
        let data = include_str!("../../../tests/basic.json");

        let parsed: crate::api::JobCollection = serde_json::from_str(data)?;

        println!("{:#?}", parsed.requestor);

        Ok(())
    }
}
