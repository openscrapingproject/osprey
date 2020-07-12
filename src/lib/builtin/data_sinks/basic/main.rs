use anyhow::{Context, Error, Result};
use log::{info, warn};
use serde::{Deserialize, Serialize};

use super::format::Format;

#[derive(Serialize, Deserialize, Debug)]
pub struct BasicSink {
    format: Format,
    location: OutputLocation,

    // when trying to add this, we get: deserialization of generic impls is
    // not supported yet; use #[typetag::serialize] to generate serialization
    // only #[serde(skip)]
    // tmpOut: std::cell::RefCell<String>,
    #[serde(skip)]
    sr: Option<Box<dyn Any>>,
}

impl Default for BasicSink {
    fn default() -> Self {
        BasicSink {
            format: Format::Json,
            location: OutputLocation::StdOut,
            // tmpOut: Box::new("".to_string())
            sr: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OutputLocation {
    StdOut,
    StdErr,
    File(PathBuf),
}

use std::io::Write;
use std::path::PathBuf;
use std::{any::Any, fs::File, io};

use erased_serde::Serialize as ESerialize;
use erased_serde::Serializer as ESerializer;

impl BasicSink {
    // TODO: make more DRY, could put this inline the consume function
    fn get_output(&self) -> Result<Box<dyn Write>> {
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

#[typetag::serde(name = "output")]
impl crate::api::DataSink for BasicSink {
    // TODO: figure this out: dyn Any + Serialize
    fn consume(&self, input: crate::api::Intermediate) -> Result<()> {
        info!("Running basic data sink");

        // TODO: make this dynamic
        let ser = &mut serde_json::Serializer::pretty(self.get_output()?);

        input.erased_serialize(&mut ESerializer::erase(ser))?;

        if self.location == OutputLocation::StdOut {
            println!()
        }

        info!("Wrote to sink");

        Ok(())
    }
}
