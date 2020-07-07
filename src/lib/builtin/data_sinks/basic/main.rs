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

use erased_serde::Serialize as ESerialize;
use erased_serde::Serializer;
use std::ops::DerefMut;

/// The biggest issue with the system right now is that erased_serde
/// requires us to only serialize to a Serializer, and some output
/// formats support serializers that go directly to writers, but others
/// don't. This means we either have to serialize them all to in-memory
/// or implement logic that determines whether it was already written to
/// the writer, or needs to be written from in-memory state.
///
/// For now, it seems the best option is to serialize everything to
/// a string and dump that out on our own.
///
/// Ok, never mind. Right now there are too many inconsistencies between
/// the different serialization format libraries.
/// Some allow creating Serializer with a Writer, some have custom
/// to_writer functions
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
    // fn get_serializer(&self) -> Result<Box<dyn Serializer>> {
    //     let sr = &mut serde_json::Serializer::new(self.get_output()?);
    //     let r = Box::new(Serializer::erase(sr));
    //     Ok(r)
    // Ok(match self.format {
    //     Format::Json => {
    //         sr
    //     },
    //     _ => {
    //         warn!("right now the other serialization formats are
    // unimplemented");

    //         sr

    //         // let sr = &mut toml::Serializer::new(self.tmpOut.deref_mut());
    //         // Ok(Box::new(Serializer::erase(sr)))
    //     }
    // })
    // Ok(Box::new(Serializer::erase(sr)))
    // }
}

type Input = crate::builtin::extractors::scraper_rs::Output;

#[feature(type_name_of_val)]
#[typetag::serde(name = "output")]
impl crate::api::DataSink for BasicSink {
    // TODO: figure this out: dyn Any + Serialize
    fn consume(&self, input: crate::api::Intermediate) -> Result<()> {
        info!("Running basic data sink");

        // let output =
        // input.erased_serialize(self.get_serializer()?.deref_mut())?;

        // info!("{:#?}", output.type_id());
        // info!("{}", std::any::type_name_of_val(&output));

        info!("Wrote to sink");

        Ok(())
    }
}
