// (Full example with detailed comments in examples/17_yaml.rs)
//
// This example demonstrates clap's building from YAML style of creating
// arguments which is far more clean, but takes a very small performance hit
// compared to the other two methods.
use clap::{load_yaml, App};

// use fs_extra::file::read_to_string;
use serde::de::DeserializeOwned;

use log::info;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use osplib::api::JobCollection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // The YAML file is found relative to the current file, similar to how
    // modules are found
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    if let Some(sub_m) = matches.subcommand_matches("run") {
        let filename = sub_m.value_of("INPUT").unwrap();

        let input: JobCollection = read_json_from_file(filename).unwrap();
        info!("{:#?}", input);
    }
    println!("Try running a subcommand, or adding --help to see the options");
    return Ok(());
}

fn read_json_from_file<P, T>(path: P) -> Result<T, Box<dyn Error>>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}
