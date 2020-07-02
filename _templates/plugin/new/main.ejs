---
to: src/lib/builtin/<%= h.changeCase.snake(type) %>/<%= h.changeCase.snake(name) %>/mod.rs
---
use serde::{Deserialize, Serialize};
use log::info;
use anyhow::{Error, Context, Result};

pub struct <%= h.changeCase.pascal(name) %> {
    pub c: Option<Config>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
}

// Here you'd implment any other traits that your plugin is for the different components it satisfies
impl crate::plugin::<%= h.changeCase.pascal(type) %> for <%= h.changeCase.pascal(name) %> {
    // TODO: you need to implement this
    
}

impl crate::plugin::BasicPlugin for <%= h.changeCase.pascal(name) %> {
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

