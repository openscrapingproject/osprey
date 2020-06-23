use serde::{Deserialize, Serialize};
use reqwest;
use log::info;

use anyhow::Error;
use reqwest::Response;

use async_trait::async_trait;

pub struct Requestor;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    version: String,
}

#[async_trait]
impl osprey::plugin::Requestor<Config, Response> for Requestor {
    async fn make_request(url: &str) -> Result<Response, Error> {
        // let parsed_url: Url = Url::parse(url)?;
        let resp = reqwest::get(url).await?;
        info!("resp = {:#?}", resp);
        Ok(resp)
    }
}

impl osprey::plugin::BasicPlugin<Config> for Requestor {
    fn configure(_: Config) -> Result<(), Error> {
        Ok(())
    }
}
