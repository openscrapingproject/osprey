use crate::prelude::*;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde::de::DeserializeOwned;

pub fn read_json_from_file<P, T>(path: P) -> Result<T>
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

pub fn read_from_file<P, T>(path: P) -> Result<T>
where
    P: AsRef<Path> + std::fmt::Debug,
    T: DeserializeOwned,
{
    let ext = path
        .as_ref()
        .extension()
        .ok_or_else(|| Error::msg("failed to convert path"))?;
    if ext != "json" {
        error!("path: {:?}", path);
        return Err(Error::msg("not a JSON file"));
    }
    read_json_from_file(path)

    // TODO: add more file types??
}
