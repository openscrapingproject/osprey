use anyhow::{Error, Result};
use log::{debug, info};

// TODO: maybe reintroduce the basic functionality of Agents as a trait
// use async_trait::async_trait;

use crate::api::JobCollection as Config;
pub struct DynamicAgent {
    c: Config,
}

use url::Url;

impl DynamicAgent {
    pub fn new(config: Config) -> Self {
        DynamicAgent { c: config }
    }

    pub async fn run(self) -> Result<(), Error> {
        let c: Config = self.c;
        let data_sink = c
            .data
            .ok_or_else(|| Error::msg("failed to get data plugin"))?;
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
            let resp = c.requestor.make_request(req_url.as_str()).await?;
            info!(
                "Made request to {} and got response code {}",
                url, resp.status
            );

            for (page_id, page_set) in &c.pages {
                info!("Runnning pipeline on {}", page_id);
                // TODO: optimize this?
                // Nah, shouldn't be a bottleneck for now
                let mdata = crate::api::MatchData {
                    url: resp.url.clone(),
                    status: resp.status,
                    version: resp.version.clone(),
                    headers: resp.headers.clone(),
                };
                let matched = page_set.matcher.run_match(mdata)?;
                info!("The matcher plugin resulted in {}", matched);

                if matched {
                    info!("Starting extractor");

                    // let data = resp.body;

                    let out = page_set.extractor.extract(&resp)?;
                    debug!("Extracted: {:#?}", out);

                    data_sink.consume(out)?;

                    info!("Completed");
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use anyhow::Context;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn configure() -> Result<()> {
        init();

        let data = include_str!("../../../tests/basic.json");
        let conf: Config = serde_json::from_str(data)
            .context("failed to deserialize configuration")?;

        let _ = DynamicAgent::new(conf);

        Ok(())
    }

    #[tokio::test]
    async fn run() -> Result<()> {
        init();

        let data = include_str!("../../../tests/basic.json");
        let conf: Config = serde_json::from_str(data)
            .context("failed to deserialize configuration")?;

        let a = DynamicAgent::new(conf);

        a.run().await
    }
}
