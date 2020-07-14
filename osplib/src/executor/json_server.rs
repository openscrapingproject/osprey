use crate::prelude::*;
use async_trait::async_trait;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerExecutor {
    #[serde(with = "humantime_serde")]
    pub poll_interval: Option<Duration>,

    /// Main endpoint URL
    pub base_url: String,
    /* Jobs endpoint (relative to base). Default: /jobs
     * pub jobs: String */

    /* pub config: String,
     * pub job_collections: String, */
}

const jobs: &str = "/jobs";
const config: &str = "/config";
const job_collections: &str = "/job_collections";

// use std::future::Future;

#[typetag::serde(name = "server")]
#[async_trait]
impl crate::executor::Executor for ServerExecutor {
    async fn run(&self) -> Result<()> {
        println!("Started");
        let interval =
            self.poll_interval.unwrap_or_else(|| Duration::from_secs(3));

        let url: url::Url = self.base_url.parse().or_else(|e| {
            Err(Error::msg(format!("failed parsing {}", self.base_url)))
        })?;

        let handle = tokio::task::spawn(run_poll(url, interval));

        tokio::join!(handle).0?

        // Ok(())
    }
}

async fn run_poll(url: url::Url, interval: Duration) -> Result<()> {
    // let mut handler = Box::pin(tokio::signal::ctrl_c());
    // let mut h = handler.as_mut();
    // h.poll(tokio);
    for i in 0..100 {
        tokio::time::delay_for(interval).await;
        let res = reqwest::get(url.clone()).await?.text().await?;
        println!("res {}", res);
    }

    Ok(())
}
