use super::plugin::*;
use anyhow::Error;

use async_trait::async_trait;
use crate::api::JobCollection as Config;

use log::info;

#[async_trait]
pub trait Agent<R: Requestor, M: Matcher, E: Extractor> {
    async fn run(self) -> Result<(), Error>;
    fn configure(&mut self, config: Config) -> AResult<()>;
}

use crate::builtin::requestor::Requestor as Reqr;
use crate::builtin::matchers::regex::RegexMatcher;
use crate::builtin::extractors::html::HTMLExtractor;

pub struct LocalAgent {
    c: Option<Config>,
    r: Reqr,
    m: RegexMatcher,
    e: HTMLExtractor,
}

impl LocalAgent {
    fn new() -> LocalAgent {
        LocalAgent {
            c: None,
            r: Reqr,
            m: RegexMatcher { c: None },
            e: HTMLExtractor { c: None },
        }
    }
}

#[async_trait]
impl Agent<Reqr, RegexMatcher, HTMLExtractor> for LocalAgent {
    fn configure(&mut self, config: Config) -> AResult<()> {
        self.c = Some(config);
        Ok(())
    }
    
    async fn run(self) -> Result<(), Error> {
        let c: Config = self.c.ok_or(Error::msg("no config provided"))?;
        for url in c.initial_urls {
            let resp = self.r.make_request(&url[..]).await?;
            info!("made request to {} and got response code {}", url, resp.status());
        }
        // self.m.
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::{env,fs};
    use std::path::PathBuf;
    
    #[tokio::test]
    async fn basic() -> AResult<()> {
        let mut a = LocalAgent::new();
        
        // let path = env::current_dir()?;
        // let mut path = PathBuf::from(file!());
        // println!("{}", path.to_str().ok_or(Error::msg("failed to convert path"))?);

        // println!("{}", std::env::current_exe()?.to_str().ok_or(Error::msg("failed to convert path"))?);
        // path.push("../../../data/clean.json");

        let data = include_str!("../../../tests/basic.json");
        println!("data: {}", data);

        // let data = fs::read_to_string(path)?;
        let conf: Config = serde_json::from_str(data)?;
        a.configure(conf)?;
        
        Ok(())
    }
}