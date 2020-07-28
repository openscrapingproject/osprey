use anyhow::Result;

use async_trait::async_trait;

#[allow(unused_imports)]
use super::jobs::PageSet;
use crate::prelude::*;
/// The Super trait represents shared bounds on all of the component traits
/// Right now, since all component traits get a readonly immutable reference
/// to self, components can be both Send and Sync. An Agent calling
/// make_request(url1) and make_request(url2) from different threads
/// will work because they only need immutable access to self.
pub trait Super: std::fmt::Debug + Send + Sync + JsonSchema {}
impl<T> Super for T where T: std::fmt::Debug + Send + Sync + JsonSchema {}

/// Fetches pages or other data
#[typetag::serde(tag = "plugin", content = "config")]
#[async_trait]
pub trait Requestor: Super {
    /// Makes an HTTP get request to the given URL
    /// using any configuration the plugin was created with.
    // TODO: add another function for a HEAD request?
    // TODO: think about user interaction sequences
    async fn make_request(&self, url: &str) -> Result<crate::api::Response>;
}

/// Determines if responses match a given configuration.
/// Used for distinguishing [PageSet]s.
#[typetag::serde(tag = "plugin", content = "config")]
pub trait Matcher: Super {
    fn run_match(&self, data: crate::api::MatchData) -> Result<bool>;
}

pub trait SerDebug: erased_serde::Serialize + std::fmt::Debug {}
impl<T> SerDebug for T where T: erased_serde::Serialize + std::fmt::Debug {}

/// Any type that can be [std::fmt::Debug] ed and also serialized
/// using erased_serde upon arrival.
pub type Intermediate = Box<dyn SerDebug>;

/// Extracts relevant data from the page
#[typetag::serde(tag = "plugin", content = "config")]
pub trait Extractor: Super {
    /// TODO: in the future, as we think about standardizing Scraping
    /// Definitions, we might modify this signature. However, for now, they
    /// can go directly into the plugin's Config.
    fn extract(&self, input: &crate::api::Response) -> Result<Intermediate>;
}

/// Outputs relevant data to a data sink
#[typetag::serde(tag = "plugin", content = "config")]
pub trait DataSink: Super {
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

        println!("{:#?}", parsed.config.requestor);

        Ok(())
    }
}
