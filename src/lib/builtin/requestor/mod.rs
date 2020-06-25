use crate::plugin::AResult;
use log::info;
use reqwest;
use serde::{Deserialize, Serialize};

use anyhow::Error;
use reqwest::Response;

use async_trait::async_trait;

pub struct Requestor;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    version: String,
}
// use osprey;

#[async_trait]
impl crate::plugin::Requestor for Requestor {
    type Response = Response;
    async fn make_request(&self, url: &str) -> AResult<Response> {
        // let parsed_url: Url = Url::parse(url)?;
        // TODO: .context("failed to parse or fetch url")
        let resp = reqwest::get(url).await?;
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
    async fn get() -> AResult<()> {
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
            let res: reqwest::Response = r.make_request(url).await?;
            info!("requested {} and got {:#?}", url, res);
        }
        
        Ok(())
    }
}
