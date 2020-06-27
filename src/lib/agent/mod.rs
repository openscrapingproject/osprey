use super::plugin::*;
use anyhow::Error;

use crate::api::JobCollection as Config;
use async_trait::async_trait;

use log::info;

#[async_trait]
pub trait Agent<R: Requestor, M: Matcher, E: Extractor> {
    async fn run(self) -> AResult<()>;
    fn configure(&mut self, config: Config) -> AResult<()>;
}

use crate::builtin::extractors::html::HTMLExtractor;
use crate::builtin::matchers::regex::RegexMatcher;
use crate::builtin::requestor::Requestor as Reqr;

pub struct LocalAgent {
    c: Option<Config>,
    r: Reqr,
    m: RegexMatcher,
    e: HTMLExtractor,
}

impl LocalAgent {
    fn new() -> LocalAgent {
        LocalAgent {
            c: None,
            r: Reqr,
            m: RegexMatcher { c: None },
            e: HTMLExtractor { c: None },
        }
    }
}

use url::Url;

#[async_trait]
impl Agent<Reqr, RegexMatcher, HTMLExtractor> for LocalAgent {
    fn configure(&mut self, config: Config) -> AResult<()> {
        self.c = Some(config);
        Ok(())
    }

    async fn run(self) -> Result<(), Error> {
        let c: Config = self.c.ok_or(Error::msg("no config provided"))?;
        for url in &c.initial_urls {
            // If the config provides a base_url, set the request URL to the concatenation of the two
            let req_url = if c.base_url.is_some() {
                info!("base_url provided");
                let base = Url::parse(c.base_url.as_ref().unwrap().as_str())?;
                base.join(url.as_str())?
            } else {
                // otherwise just use url provided
                Url::parse(url.as_str())?
            };
            let resp = self.r.make_request(req_url.as_str()).await?;
            info!(
                "made request to {} and got response code {}",
                url,
                resp.status()
            );
        }
        // self.m.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::{env, fs};

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[tokio::test]
    async fn configure() -> AResult<()> {
        init();

        let mut a = LocalAgent::new();

        let data = include_str!("../../../tests/basic.json");

        let conf: Config = serde_json::from_str(data)?;
        info!("configuration: {:#?}", conf);
        a.configure(conf)?;

        Ok(())
    }

    #[tokio::test]
    async fn run() -> AResult<()> {
        init();

        let mut a = LocalAgent::new();

        let data = include_str!("../../../tests/basic.json");

        let conf: Config = serde_json::from_str(data)?;
        info!("configuration: {:#?}", conf);
        a.configure(conf)?;

        a.run().await?;

        Ok(())
    }
}
