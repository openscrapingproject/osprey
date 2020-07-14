use crate::prelude::*;

use async_trait::async_trait;

use crate::api::jobs::{Job, JobCollection};

// TODO: are there cases where the agent has state and
// needs to access it
#[async_trait]
pub trait Agent {
    // TODO: think about these move semantics

    async fn run(job: &Job) -> Result<()>;

    // this is mainly for debugging/lgeacy purposes right now
    async fn run_job_collection(jc: &JobCollection) -> Result<()>;
}
