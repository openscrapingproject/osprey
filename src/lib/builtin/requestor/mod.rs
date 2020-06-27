use crate::plugin::AResult;
use log::info;
use reqwest;
use serde::{Deserialize, Serialize};

use anyhow::{Error,anyhow};
use reqwest::Response;

use async_trait::async_trait;

pub struct Requestor;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    version: String,
}
// use osprey;
use anyhow::{Context};

#[async_trait]
impl crate::plugin::Requestor for Requestor {
    type Response = Response;
    async fn make_request(&self, url: &str) -> AResult<Response> {
        // let parsed_url: Url = Url::parse(url)?;
        // TODO: .context("failed to parse or fetch url")
        let resp = reqwest::get(url).await.with_context(|| format!("failed to parse or fetch URL from {}", url))?;
        info!("resp = {:#?}", resp);
        Ok(resp)
    }
}

impl crate::plugin::BasicPlugin for Requestor {
    type Config = Config;
    fn configure(&mut self, _: Config) -> AResult<()> {
        Ok(())
    }
    fn get_default_config() -> Config {
        Config {
            version: "".to_string(),
        }
    }
    
    fn parse_config(input: serde_json::Value) -> AResult<Self::Config> {
        serde_json::from_value(input.clone()).with_context(|| format!("failed to parse configuration {}", input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin::Requestor;
    use crate::plugin::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[tokio::test]
    async fn invalid_urls() -> AResult<()> {
        init();

        let mut r = super::Requestor{};
        r.configure(Config {
            version: "".to_string(),
        })?;

        let urls = vec![
            "/product/:ID1",
            "/product/:ID2",
            "each URL generates a new Job",
            "with an ID of collection_name_0 ... collection_name_N"
        ];

        for url in urls {
            let result = r.make_request(url).await;
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

        let mut r = super::Requestor{};
        r.configure(Config {
            version: "".to_string(),
        })?;

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
