use anyhow::{Context, Error, Result};
use log::info;
// use serde::{Deserialize, Serialize};

use super::format::Format;

#[derive(Serialize, Deserialize, Debug)]
pub struct BasicSink {
    format: Format,
    location: OutputLocation,
}

impl Default for BasicSink {
    fn default() -> Self {
        BasicSink {
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
use std::{any::Any, fs::File, io};

use erased_serde::Serialize;

impl BasicSink {
    // TODO: make more DRY, could put this inline the consume function
    fn get_output(&self) -> Result<Box<dyn Write>, Error> {
        let loc = &self.location;
        match loc {
            OutputLocation::StdOut => Ok(Box::new(io::stdout())),
            OutputLocation::StdErr => Ok(Box::new(io::stderr())),
            OutputLocation::File(ref path) => {
                Ok(File::open(path).map(|f| Box::new(f) as Box<dyn Write>)?)
            }
        }
    }
}

type Input = crate::builtin::extractors::scraper_rs::Output;

#[typetag::serde(name = "output")]
impl crate::api::DataSink for BasicSink {
    // TODO: figure this out: dyn Any + Serialize
    fn consume(&self, input: Box<dyn Serialize>) -> Result<()> {
        info!("Running basic data sink");

        // let ff = self
        //     .c
        //     .as_ref()
        //     .ok_or_else(|| Error::msg("no config"))?
        //     .format;

        // Here we need to go into a concrete type that implements
        // Serialize
        // let output = input
        // .downcast::<dyn Serialize>()
        // .or_else(|_| Err(Error::msg("failed to downcast")))?;

        let output = input.unwrap();
        let s = serde_any::to_string(&output, self.format.into()).or_else(
            |_| {
                Err(Error::msg(format!(
                    "failed to serialize input into {:?}",
                    self.format
                )))
            },
        )?;

        let mut sink = self.get_output()?;

        let nbytes = sink.write(s.as_bytes())?;

        info!("Wrote {} bytes to sink", nbytes);

        Ok(())
    }
}
