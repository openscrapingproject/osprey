use anyhow::{Context, Error, Result};
use log::info;
use serde::{Deserialize, Serialize};

use super::format::Format;

#[derive(Default)]
pub struct BasicSink {
    // TODO: refactor so the option is not there
    pub c: Option<Config>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    format: Format,
    location: OutputLocation,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            format: Format::Json,
            location: OutputLocation::StdOut,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OutputLocation {
    StdOut,
    StdErr,
    File(PathBuf),
}

use std::io::Write;
use std::path::PathBuf;
use std::{fs::File, io};

impl BasicSink {
    // TODO: make more DRY, could put this inline the consume function
    fn get_output(&self) -> Result<Box<dyn Write>, Error> {
        let loc = &self
            .c
            .as_ref()
            .ok_or_else(|| Error::msg("no config"))?
            .location;
        match loc {
            OutputLocation::StdOut => Ok(Box::new(io::stdout())),
            OutputLocation::StdErr => Ok(Box::new(io::stderr())),
            OutputLocation::File(ref path) => {
                Ok(File::open(path).map(|f| Box::new(f) as Box<dyn Write>)?)
            }
        }
    }
}

impl crate::plugin::DataSink for BasicSink {
    type Input = crate::builtin::extractors::scraper_rs::Output;

    fn consume(&self, input: Self::Input) -> Result<()> {
        info!("Running basic data sink");

        let ff = self
            .c
            .as_ref()
            .ok_or_else(|| Error::msg("no config"))?
            .format;

        let s = serde_any::to_string(&input, ff.into()).or_else(|_| {
            Err(Error::msg(format!(
                "failed to serialize input into {:?}",
                ff
            )))
        })?;

        let mut sink = self.get_output()?;

        let nbytes = sink.write(s.as_bytes())?;

        info!("Wrote {} bytes to sink", nbytes);

        Ok(())
    }
}

impl crate::plugin::BasicPlugin for BasicSink {
    type Config = Config;
    fn configure(&mut self, c: Config) -> Result<()> {
        self.c = Some(c);
        Ok(())
    }
    fn get_default_config(&self) -> Config {
        Config::default()
    }

    fn parse_config(&self, input: serde_json::Value) -> Result<Self::Config> {
        serde_json::from_value(input.clone()).with_context(|| {
            format!("failed to parse configuration {}", input)
        })
    }
}
