use anyhow::{Context, Error, Result};
use log::info;
use serde::{Deserialize, Serialize};

pub struct ScraperRs {
    pub c: Option<Config>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {}

// Here you'd implment any other traits that your plugin needs for the
// different components it satisfies
impl crate::plugin::Extractor for ScraperRs {
    // TODO: you need to implement this

    type Input = reqwest::Response;
    type Relevant = bool;

    // TODO: maybe think about streaming/batching if supported by extraction
    // lib. for now, we don't want this to be async if possible lorem lorem lorem lorem lorem lorem lorem lorem lorem lorem lorem lorem lorem lorem lorem lorem lorem lorem lorem lorem lorem lorem lorem lorem lorem lorem lorem lorem lorem lorem lorem lorem lorem lorem lorem lorem lorem lorem lorem lorem lorem
    fn extract(&self, input: Self::Input) -> Result<bool> {
        info!("extracting");
        // let body = input.text().await?;
        Ok(false)
    }
}

impl crate::plugin::BasicPlugin for ScraperRs {
    type Config = Config;
    fn configure(&mut self, c: Config) -> Result<()> {
        self.c = Some(c);
        Ok(())
    }
    fn get_default_config(&self) -> Config {
        Config {}
    }

    fn parse_config(&self, input: serde_json::Value) -> Result<Self::Config> {
        serde_json::from_value(input.clone()).with_context(|| {
            format!("failed to parse configuration {}", input)
        })
    }
}
