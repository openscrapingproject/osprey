use serde::{Deserialize, Serialize};
use log::info;
use anyhow::{Error, Context};
use crate::plugin::AResult;

use regex::Regex;
use std::collections::HashMap;

pub struct HTMLExtractor {
    pub c: Option<Config>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
}

impl crate::plugin::Extractor for HTMLExtractor {
    type Input = reqwest::Response;
    type Relevant = bool;
    fn extract(&self, input: Self::Input) -> AResult<bool> {
        Ok(false)
    }
}

impl crate::plugin::BasicPlugin for HTMLExtractor {
    type Config = Config;
    fn configure(&mut self, c: Config) -> AResult<()> {
        self.c = Some(c);
        Ok(())
    }
    fn get_default_config() -> Config {
        Config {}
    }
    
    fn parse_config(input: serde_json::Value) -> AResult<Self::Config> {
        serde_json::from_value(input.clone()).with_context(|| format!("failed to parse configuration {}", input))
    }
}
