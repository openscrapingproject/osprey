#![warn(clippy::all)]
use osplib::prelude::*;

use clap::{load_yaml, App};

use osplib::{
    agent::{Agent, DynamicAgent},
    api::JobCollection,
    executor::Config as EConfig,
    utils::read_from_file,
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

    match matches.subcommand() {
        ("run", Some(sub_m)) => {
            let filename = sub_m
                .value_of("INPUT")
                .ok_or_else(|| Error::msg("Failed to get file"))?;

            let input: JobCollection = read_from_file(filename)?;
            info!("Got input: {:#?}", input);

            DynamicAgent::run_job_collection(&input).await?
        }
        ("validate", Some(sub_m)) => {
            let filename = sub_m
                .value_of("INPUT")
                .ok_or_else(|| Error::msg("Failed to get file"))?;

            println!("Validating");
            let input: JobCollection = read_from_file(filename)?;

            println!("Done!\n\n");
            println!("Got input: {:#?}", input);
        }
        ("executor", Some(sub_m)) => {
            let filename = sub_m
                .value_of("CONFIG")
                .ok_or_else(|| Error::msg("Failed to get config file"))?;

            println!("Starting executor...");
            let input: EConfig = read_from_file(filename)?;

            input.executor.run().await?;

            println!("Completed!");
        }
        _ => {
            println!("Try running a subcommand, or adding --help to see the options");
        }
    }

    return Ok(());
}
