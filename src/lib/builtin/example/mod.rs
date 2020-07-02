// WARNING: this is example code

use serde::{Deserialize, Serialize};
use log::info;
use anyhow::{Error, Context, Result};

pub struct PluginName {
    pub c: Option<Config>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
}

// Here you'd implment any other traits that your plugin is for the different components it satisfies
impl crate::plugin::Extractor for PluginName {
    type Input = reqwest::Response;
    type Relevant = bool;
    fn extract(&self, input: Self::Input) -> Result<bool> {
        Ok(false)
    }
}

impl crate::plugin::BasicPlugin for PluginName {
    type Config = Config;
    fn configure(&mut self, c: Config) -> Result<()> {
        self.c = Some(c);
        Ok(())
    }
    fn get_default_config(&self) -> Config {
        Config {}
    }
    
    fn parse_config(&self, input: serde_json::Value) -> Result<Self::Config> {
        serde_json::from_value(input.clone()).with_context(|| format!("failed to parse configuration {}", input))
    }
}
