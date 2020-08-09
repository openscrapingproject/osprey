use crate::prelude::*;

use super::api::Agent;
use async_trait::async_trait;

// TODO: maybe reintroduce the basic functionality of Agents as a trait
// use async_trait::async_trait;

use crate::api::{Config, JobCollection};
pub struct DynamicAgent {}

use url::Url;

use std::collections::HashMap;

#[async_trait]
impl Agent for DynamicAgent {
    // pub async fn run(jo)

    /// This is effectively a tiny mini agent web crawler right here
    /// We use a stack for managing the URLs: this results effectively in a DFS
    /// crawling agent We also have a hashmap of urls to ignore already
    /// visited ones.
    async fn run_job_collection(
        collection: &JobCollection,
    ) -> Result<(), Error> {
        let c: &Config = &collection.config;
        let data_sink = c
            .data
            .as_ref()
            .ok_or_else(|| Error::msg("failed to get data plugin"))?;

        let us = &collection.initial_urls;
        let mut urls = us.clone();
        let mut visited: HashMap<String, bool> = HashMap::new();
        let mut i: usize = 0;
        loop {
            let u = urls.pop();
            if u.is_none() {
                return Ok(());
            }
            let url = u.unwrap();
            if visited.get(&url).is_none() {
                visited.insert(url.clone(), true);
            } else {
                continue;
            }
            debug!("Starting iteration {}. Remaining: {}", i, urls.len());
            // If the config provides a base_url, set the request URL to the
            // concatenation of the two
            let req_url = if collection.base_url.is_some() {
                info!("base_url provided");
                let base = Url::parse(
                    collection.base_url.as_ref().unwrap().as_str(),
                )?;
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

                    // Push the new items onto our list
                    // for item in out.generated {
                    //     urls.push(item);
                    // }
                    urls.extend(out.generated.iter().cloned());

                    // TODO: should we maybe record the generated links in the
                    // store as well? No, we do want to
                    // keep the raw data output product separate from metadata
                    // tracking However it is valuable to
                    // put the URL of the item inside the data object
                    data_sink.consume(out.data).await?;

                    info!("Completed");
                }
            }
            i += 1;
        }
        Ok(())
    }
    async fn run(job: &crate::api::Job) -> Result<()> {
        let c = &job.config;
        let resp = c.requestor.make_request(job.url.as_str()).await?;
        info!(
            "Made request to {} and got response code {}",
            job.url, resp.status
        );

        let data_sink = c
            .data
            .as_ref()
            .ok_or_else(|| Error::msg("failed to get data plugin"))?;

        // TODO: for each of the following Try ? operators: think about
        // only skipping a faulty page set instead of stopping the whole job
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

                data_sink.consume(out.data).await?;

                info!("Completed");
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

        // let data = include_str!("../../../tests/basic.json");
        // let conf: Config = serde_json::from_str(data)
        //     .context("failed to deserialize configuration")?;

        // let _ = DynamicAgent::new(conf);

        Ok(())
    }

    #[tokio::test]
    async fn run() -> Result<()> {
        init();

        let data = include_str!("../../../tests/basic.json");
        let conf: JobCollection = serde_json::from_str(data)
            .context("failed to deserialize configuration")?;

        DynamicAgent::run_job_collection(&conf).await
    }
}
