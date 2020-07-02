use anyhow::{Context, Error, Result};
use log::info;
use serde::{Deserialize, Serialize};

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

impl crate::plugin::Matcher for RegexMatcher {
    type MatchInput = super::MatchData;
    fn run_match(&self, data: Self::MatchInput) -> Result<bool> {
        let config = self
            .c
            .as_ref()
            .ok_or(Error::msg("no configuration provided"))?;
        let reg = config.url.as_str();
        info!("Trying to match with regex {}", reg);

        let re = Regex::new(reg)?;

        let res = re.is_match(&data.url.to_string());
        info!("Match for URL {} is {}", data.url, res);
        // TODO match against the headers as well
        Ok(res)
    }
}

impl crate::plugin::BasicPlugin for RegexMatcher {
    type Config = Config;
    fn configure(&mut self, c: Config) -> Result<()> {
        self.c = Some(c);
        Ok(())
    }
    fn get_default_config(&self) -> Config {
        Config {
            url: "".to_string(),
            headers: HashMap::new(),
        }
    }

    fn parse_config(&self, input: serde_json::Value) -> Result<Self::Config> {
        serde_json::from_value(input.clone()).with_context(|| {
            format!("failed to parse configuration {}", input)
        })
    }
}
