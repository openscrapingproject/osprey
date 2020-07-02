use super::plugin::*;
use anyhow::{Error, Context};

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

pub struct LocalAgent<R, M, E> where R: Requestor, M: Matcher, E: Extractor {
    c: Option<Config>,
    r: R,
    m: M,
    e: E,
}

impl LocalAgent<Reqr, RegexMatcher, HTMLExtractor> {
    fn new() -> LocalAgent<Reqr, RegexMatcher, HTMLExtractor> {
        LocalAgent {
            c: None,
            r: Reqr::new(),
            m: RegexMatcher { c: None },
            e: HTMLExtractor { c: None },
        }
    }
    // Reqr, RegexMatcher, HTMLExtractor

    // r: Reqr,
    // m: RegexMatcher { c: None },
    // e: HTMLExtractor { c: None },
}

use url::Url;

#[async_trait]
impl Agent<Reqr, RegexMatcher, HTMLExtractor> for LocalAgent<Reqr, RegexMatcher, HTMLExtractor> {
    fn configure(&mut self, config: Config) -> AResult<()> {
        info!("configuration: {:#?}", config);
        self.c = Some(config.clone());
        
        self.r.configure(self.r.parse_config(config.requestor.config)?)?;

        // TODO: think about where to run matcher config.
        // Should it be here, thus requiring additional state, and maybe a matcher per page set
        // or should it be in run where config errors shouldn't be caught

        // HACK: the primary purpose of this is to validate the config
        // Its true effect will be to overwrite the matchers config with that of the last page set
        for (pageID, page) in config.pages {
            self.m.configure(self.m.parse_config(page.matcher.config)?)?;
        }


        Ok(())
    }

    async fn run(self) -> Result<(), Error> {
        let c: Config = self.c.ok_or(Error::msg("no config provided"))?;
        // self.r.
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
            let mdata = crate::builtin::matchers::MatchData {
                url: resp.url().clone(),
                headers: resp.headers().clone()
            };
            let matched = self.m.run_match(mdata)?;
            // let mm: &'a str = {if m == true {"Matched"} else {"Didn't match"}};
            info!("The matcher plugin resulted in {}", matched);

            if matched {
                info!("Starting extractor");
                
            }
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

    #[test]
    fn configure() -> AResult<()> {
        init();

        let mut a = LocalAgent::new();

        let data = include_str!("../../../tests/basic.json");

        let conf: Config = serde_json::from_str(data).context("failed to deserialize configuration")?;
        // info!("configuration: {:#?}", conf);
        a.configure(conf)?;

        Ok(())
    }

    #[tokio::test]
    async fn run() -> AResult<()> {
        init();

        let mut a = LocalAgent::new();

        let data = include_str!("../../../tests/basic.json");

        let conf: Config = serde_json::from_str(data)?;
        // info!("configuration: {:#?}", conf);
        a.configure(conf)?;

        a.run().await?;

        Ok(())
    }
}
