use anyhow::{Error, Result};

use crate::api::JobCollection as Config;
use crate::api::*;

use async_trait::async_trait;

// TODO: do we even need this trait anymore
// #[async_trait]
// pub trait Agent<R, M, E, S>
// where
//     R: Requestor,
//     M: Matcher,
//     E: Extractor,
//     S: DataSink,
// {
//     async fn run(self) -> Result<()>;
//     fn configure(&mut self, config: Config) -> Result<()>;
// }
