// (Full example with detailed comments in examples/17_yaml.rs)
//
// This example demonstrates clap's building from YAML style of creating arguments which is far
// more clean, but takes a very small performance hit compared to the other two methods.
use clap::{App, load_yaml};

// use fs_extra::file::read_to_string;
use serde::de::DeserializeOwned;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use osp::api::Agent;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    // if let Some(sub_m) = matches.subcommand_matches("test") {
    //     // Use the struct like normal
    //     assert_eq!(sub_m.value_of("debug"), Some("false"));
    // }
    // Same as previous examples...
    let filename = matches.value_of("INPUT").unwrap();

    let u: Agent = read_json_from_file(filename).unwrap();
    println!("{:#?}", u);
    return Ok(())
}



fn read_json_from_file<P, T>(path: P) -> Result<T, Box<dyn Error>> where P: AsRef<Path>, T: DeserializeOwned {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}