use anyhow::{Error, Result};
use log::{debug, info};

use async_trait::async_trait;

use crate::api::JobCollection as Config;
use crate::plugin::*;

use crate::builtin::extractors::scraper_rs::ScraperRs;
use crate::builtin::matchers::regex::RegexMatcher;
use crate::builtin::requestor::Requestor as Reqr;

use crate::builtin::data_sinks::basic::BasicSink;

// This means that the generics need to be bounded by the Default trait, which
// BasicPlugins are
#[derive(Default)]
pub struct LocalAgent<R, M, E, S>
where
    R: Requestor,
    M: Matcher,
    E: Extractor,
    S: DataSink,
{
    c: Option<Config>,
    r: R,
    m: M,
    e: E,
    s: S,
}

use url::Url;

#[async_trait]
impl super::Agent<Reqr, RegexMatcher, ScraperRs, BasicSink>
    for LocalAgent<Reqr, RegexMatcher, ScraperRs, BasicSink>
{
    fn configure(&mut self, config: Config) -> Result<()> {
        debug!("configuration: {:#?}", config);
        self.c = Some(config.clone());

        self.r
            .configure(self.r.parse_config(config.requestor.config)?)?;

        // TODO: think about where to run matcher config.
        // Should it be here, thus requiring additional state, and maybe a
        // matcher per page set or should it be in run where config
        // errors shouldn't be caught

        // HACK: the primary purpose of this is to validate the config
        // Its true effect will be to overwrite the matchers config with that
        // of the last page set
        for (page_id, page) in config.pages {
            info!("Running pipeline for {}", page_id);
            self.m
                .configure(self.m.parse_config(page.matcher.config)?)?;
            self.e
                .configure(self.e.parse_config(page.extractor.config)?)?;
        }

        self.s.configure(self.s.parse_config(config.data.config)?)?;

        Ok(())
    }

    async fn run(self) -> Result<(), Error> {
        let c: Config =
            self.c.ok_or_else(|| Error::msg("no config provided"))?;
        for url in &c.initial_urls {
            // If the config provides a base_url, set the request URL to the
            // concatenation of the two
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
                "Made request to {} and got response code {}",
                url,
                resp.status()
            );
            let mdata = crate::builtin::matchers::MatchData {
                url: resp.url().clone(),
                headers: resp.headers().clone(),
            };
            let matched = self.m.run_match(mdata)?;
            info!("The matcher plugin resulted in {}", matched);

            if matched {
                info!("Starting extractor");

                let data = resp.text().await?;

                let out = self.e.extract(data)?;
                debug!("Extracted: {:#?}", out);

                self.s.consume(out)?;

                info!("Completed");
                // if out.contains_key("text") {
                //     println!(
                //         "Output: {}\n\n",
                //         // TODO: when to wrap?
                //         textwrap::fill(out.get("text").unwrap(), 50)
                //     )
                // }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::Agent;

    use anyhow::Context;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn configure() -> Result<()> {
        init();

        let mut a = LocalAgent::default();

        let data = include_str!("../../../tests/basic.json");

        let conf: Config = serde_json::from_str(data)
            .context("failed to deserialize configuration")?;
        a.configure(conf)?;

        Ok(())
    }

    #[tokio::test]
    async fn run() -> Result<()> {
        init();

        let mut a = LocalAgent::default();

        let data = include_str!("../../../tests/basic.json");

        let conf: Config = serde_json::from_str(data)?;
        a.configure(conf)?;

        a.run().await?;

        Ok(())
    }
}
