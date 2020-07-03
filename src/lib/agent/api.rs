use anyhow::{Error, Result};

use crate::api::JobCollection as Config;
use crate::plugin::*;

use async_trait::async_trait;

#[async_trait]
pub trait Agent<R, M, E, S>
where
    R: Requestor,
    M: Matcher,
    E: Extractor,
    S: DataSink,
{
    async fn run(self) -> Result<()>;
    fn configure(&mut self, config: Config) -> Result<()>;
}
