
use anyhow::{Error, Context};
use log::info;
use serde::{Deserialize, Serialize};
use crate::plugin::AResult;

use regex::Regex;
use std::collections::HashMap;

pub struct RegexMatcher {
    // TODO: refactor so the option is not there
    pub c: Option<Config>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    url: String,
    headers: HashMap<String, String>,
}

pub struct MatchData {
    url: http::Uri,
    headers: http::HeaderMap,
}

impl crate::plugin::Matcher for RegexMatcher {
    type MatchInput = MatchData;
    fn run_match(&self, data: Self::MatchInput) -> crate::plugin::AResult<bool> {
        let config = self.c.as_ref().ok_or(Error::msg("no configuration provided"))?;
        let re = Regex::new(config.url.as_str())?;
        
        let res = re.is_match(&data.url.to_string());
        info!("Match for URL {} is {}", data.url, res);
        // TODO match against the headers as well
        Ok(res)
    }
}

impl crate::plugin::BasicPlugin for RegexMatcher {
    type Config = Config;
    fn configure(&mut self, c: Config) -> AResult<()> {
        self.c = Some(c);
        Ok(())
    }
    fn get_default_config() -> Config {
        Config {
            url: "".to_string(),
            headers: HashMap::new(),
        }
    }
    
    fn parse_config(input: serde_json::Value) -> AResult<Self::Config> {
        serde_json::from_value(input.clone()).with_context(|| format!("failed to parse configuration {}", input))
    }
}
