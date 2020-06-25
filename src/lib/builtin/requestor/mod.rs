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

    #[tokio::test]
    async fn get() -> AResult<()> {
        let mut r = super::Requestor{};
        r.configure(Config {
            version: "".to_string(),
        })?;
        
        let url = "https://google.com";
        println!("calling {}", url);
        let res: reqwest::Response = r.make_request(url).await?;
        println!("{:#?}", res);
        // res.headers;
        Ok(())
    }
}
