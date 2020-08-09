use anyhow::Result;

use async_trait::async_trait;

#[allow(unused_imports)]
use super::jobs::PageSet;

/// The Super trait represents shared bounds on all of the component traits
/// Right now, since all component traits get a readonly immutable reference
/// to self, components can be both Send and Sync. An Agent calling
/// make_request(url1) and make_request(url2) from different threads
/// will work because they only need immutable access to self.
pub trait Super: std::fmt::Debug + Send + Sync {}
impl<T> Super for T where T: std::fmt::Debug + Send + Sync {}

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

use mopa::{mopafy, Any};

/// We use the mopa crate to allow using the Any trait in addition to our
/// SerDebug trait. This allows code that knows the original concrete type of
/// the trait to cast it back to that type The main use case for this is to run
/// tests that validate the output of extractors.
pub trait SerDebug:
    erased_serde::Serialize + std::fmt::Debug + Any + Send
{
}
impl<T> SerDebug for T where
    T: erased_serde::Serialize + std::fmt::Debug + Any + Send
{
}

// Now this trait automatically implements serde::Serialize for any type needed
// No more jiggering with weird Serializer's manually
erased_serde::serialize_trait_object!(SerDebug);

mopafy!(SerDebug);

/// Any type that can be [std::fmt::Debug] ed and also serialized
/// using erased_serde upon arrival.
pub type Intermediate = Box<dyn SerDebug>;

pub type Link = String;

use crate::prelude::*;

// We can't derive deserialize because data can literally be any type that
// impls serialize also there's no real use to want to deserialize this
#[derive(Debug, Serialize)]
pub struct ExtractOutput {
    pub data: Intermediate,
    pub generated: Vec<Link>,
}

/// Extracts relevant data from the page
#[typetag::serde(tag = "plugin", content = "config")]
pub trait Extractor: Super {
    /// TODO: in the future, as we think about standardizing Scraping
    /// Definitions, we might modify this signature. However, for now, they
    /// can go directly into the plugin's Config.
    fn extract(&self, input: &crate::api::Response) -> Result<ExtractOutput>;
}

/// Outputs relevant data to a data sink
#[typetag::serde(tag = "plugin", content = "config")]
#[async_trait]
pub trait DataSink: Super {
    async fn consume(&self, input: Intermediate) -> Result<()>;
}

// In the future, we may add a separate Generator trait which could have
// different dynamics But for now, the extractor trait is all we need really to
// generate new links to visit

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
