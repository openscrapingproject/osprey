use crate::plugin::AResult;
use log::info;
use reqwest;
use serde::{Deserialize, Serialize};

use crate::utils;
use anyhow::{anyhow, Context, Error};
use reqwest::Response;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::time::Duration;

use async_trait::async_trait;

pub struct Requestor {
    c: Option<Config>,
}

impl Requestor {
    pub fn new() -> Requestor {
        Requestor { c: None }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    // version: String,
    timeout: Option<Duration>,
    headers: HashMap<String, String>,
}

#[async_trait]
impl crate::plugin::Requestor for Requestor {
    type Response = Response;
    async fn make_request(&self, url: &str) -> AResult<Response> {
        // TODO: reuse clients, think about pooling?
        let config = self.c.as_ref().ok_or(Error::msg("failed to get config"))?;
        let builder = reqwest::ClientBuilder::new().timeout(
            config
                .timeout
                .unwrap_or(Duration::from_secs(5)),
        );

        let client = builder.build()?;

        let req = client
            .get(url)
            .headers(utils::hash2headers(&config.headers)?)
            .build()?;

        info!("req = {:#?}", req);

        let resp = client.execute(req).await?;
        // let resp = reqwest::get(url)
        //     .await
        //     .with_context(|| format!("failed to parse or fetch URL from {}", url))?;
        info!("resp = {:#?}", resp);
        Ok(resp)
    }
}

impl crate::plugin::BasicPlugin for Requestor {
    type Config = Config;
    fn configure(&mut self, config: Config) -> AResult<()> {
        self.c = Some(config);
        Ok(())
    }
    fn get_default_config() -> Config {
        Config {
            // version: "".to_string(),
            timeout: None,
            headers: HashMap::new(),
        }
    }

    fn parse_config(input: serde_json::Value) -> AResult<Self::Config> {
        serde_json::from_value(input.clone())
            .with_context(|| format!("failed to parse configuration {}", input))
    }
}

#[cfg(test)]
mod tests {
    use super::Error;

    use super::Requestor as BasicRequestor;
    use crate::plugin::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[tokio::test]
    async fn invalid_urls() -> AResult<()> {
        init();

        let mut r = BasicRequestor::new();
        r.configure(BasicRequestor::get_default_config())?;

        let urls = vec![
            "/product/:ID1",
            "/product/:ID2",
            "each URL generates a new Job",
            "with an ID of collection_name_0 ... collection_name_N",
        ];

        for url in urls {
            let result = Requestor::make_request(&r, url).await;
            match result {
                Err(_) => continue,
                Ok(_) => return Err(Error::msg("these should have failed")),
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn valid_urls() -> AResult<()> {
        init();

        let mut r = BasicRequestor::new();
        r.configure(BasicRequestor::get_default_config())?;

        let urls = vec![
            "https://google.com",
            "https://www.cnn.com/",
            "https://www.nytimes.com/",
        ];

        for url in urls {
            let result = r.make_request(url).await;
            match result {
                Err(e) => return Err(e),
                Ok(_) => continue,
            }
        }

        Ok(())
    }
}
