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
    async fn make_request(self, url: &str) -> AResult<Response> {
        // let parsed_url: Url = Url::parse(url)?;
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
