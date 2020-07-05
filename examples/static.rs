use anyhow::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::any::Any;
use std::env;
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    plugin: String,
    config: Value,
}

#[derive(Serialize, Deserialize, Debug)]
struct C1 {
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct C2 {
    papaya: String,
}

trait Plugin: Any {
    type Config;
    fn print(&self, c: Self::Config) -> bool;
}

struct Foo;

impl Plugin for Foo {
    type Config = C1;
    fn print(&self, c: Self::Config) -> bool {
        println!("{:#?}", c);
        true
    }
}

struct Bar;

impl Plugin for Bar {
    type Config = C2;
    fn print(&self, c: Self::Config) -> bool {
        println!("{:#?}", c);
        true
    }
}

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    println!("Starting static dynamic test");

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let jsonpath: &String = args
        .get(1)
        .ok_or_else(|| Error::msg("failed to get path"))?;

    let file = File::open(jsonpath)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u: Config = serde_json::from_reader(reader)?;

    println!("{:#?}", u);

    // let res = Result::Err("");
    // res?;
    // The following fails:
    // It appears there is no fix without dyn

    // let got = u.plugin.as_str();
    // let p: Any = if got == "foo" {
    //     let c1: C1 = serde_json::from_value(u.config).unwrap();
    //     &Foo {}
    // // true
    // } else if got == "bar" {
    //     &Bar {}
    // } else {
    //     return Err(Error::msg("unknown plugin"));
    // };

    // let p: dyn Plugin;

    Ok(())
}
