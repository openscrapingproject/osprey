use crate::prelude::*;

use crate::api::Config;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoteConfig {
    // TODO: should this be a data-service neutral URL
    // or a well-defined ID that requires knowledge of the
    // Server API
    /// This is either a data-service specific ID that needs to be
    /// resolved with DS-specific knowledge, or a URL that is
    /// fully qualified and returns the appropriate config
    Remote(String),
    Embedded(Config),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum State {
    Waiting,
    Running,
    Done,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
    /// Human readable name for job. Often generated from collection_name
    pub name: String,

    /// Internal ID
    pub id: String,

    /// Current state of Job
    pub state: State,

    /// URL to request
    pub url: String,

    #[serde(with = "humantime_serde")]
    pub elapsed: Option<Duration>,

    // TODO: since we know the structure of this API, we can just make
    // a string?
    /// Configuration to provide to components during request
    pub config: RemoteConfig,
}

// #[derive(Debug, Serialize, Deserialize)]
pub type Jobs = Vec<Job>;
