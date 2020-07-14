#![warn(clippy::all)]
use clap::{load_yaml, App};

// use fs_extra::file::read_to_string;
use serde::de::DeserializeOwned;

use osplib::prelude::*;

use log::info;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use osplib::{
    agent::{Agent, DynamicAgent},
    api::JobCollection,
};

// TODO: think about separating the launching of the Tokio runtime
// from potential CLI commands like verify the config file
// that don't need async functionality
#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    // The YAML file is found relative to the current file, similar to how
    // modules are found
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    if let Some(sub_m) = matches.subcommand_matches("run") {
        let filename = sub_m
            .value_of("INPUT")
            .ok_or_else(|| Error::msg("Failed to get file"))?;

        let input: JobCollection = read_json_from_file(filename).unwrap();
        info!("Got input: {:#?}", input);

        // DynamicAgent::into()
        DynamicAgent::run_job_collection(&input).await?
    }
    println!("Try running a subcommand, or adding --help to see the options");
    return Ok(());
}

fn read_json_from_file<P, T>(path: P) -> Result<T>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // TODO: think about if this is a different format
    let u = serde_json::from_reader(reader)?;

    Ok(u)
}
