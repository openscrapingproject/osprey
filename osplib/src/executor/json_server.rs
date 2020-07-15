use crate::prelude::*;
use async_trait::async_trait;

use crate::agent::DynamicAgent;

// TODO: have API server run a hash or checksum to make sure modified
// version contains latest changes

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerExecutor {
    #[serde(with = "humantime_serde")]
    pub poll_interval: Option<Duration>,

    pub num_polls: i32,

    /// Main endpoint URL
    pub base_url: String,
    /* Jobs endpoint (relative to base). Default: /jobs
     * pub jobs: String */

    /* pub config: String,
     * pub job_collections: String, */
}

const jobs: &str = "/jobs/";
const config: &str = "/configs/";
// const job_collections: &str = "/job_collections";

// use std::future::Future;
use crate::api::Config;
use url::Url;

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

use std::time::{Duration, Instant};
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

        let handle =
            tokio::task::spawn(run_poll(url, interval, self.num_polls));

        tokio::join!(handle).0?

        // Ok(())
    }
}

/// Fetches the config if it is remote, converts a DS Job into an API Job
async fn into_job(url: Url, j: Job) -> Result<crate::api::Job> {
    let cfg = match j.config {
        RemoteConfig::Embedded(c) => c,
        RemoteConfig::Remote(id) => {
            let u = url.join(config)?.join(id.as_str())?;
            info!("Making request to config url: {}", u);
            let res = reqwest::get(u).await?.json().await?;
            debug!("Got config for job {}: {:?}", j.name, res);
            // serde_json::from_str(res.as_str())?
            res
        }
    };

    Ok(crate::api::Job {
        url: j.url,
        config: cfg,
    })
}

use crate::agent::Agent;
use serde::de::DeserializeOwned;

/// Updates the state of a given Job by doing a GET, executing a closure,
/// then doing a PUT
async fn do_update<F, T>(url: Url, job_id: &String, mut func: F) -> Result<()>
where
    F: Fn(T) -> T,
    T: std::fmt::Debug + Serialize + DeserializeOwned,
{
    let c = reqwest::Client::new();
    let u = url.clone().join(jobs)?.join(job_id.as_str())?;
    let res: T = c.get(u.clone()).send().await?.json().await?;
    debug!("before update: {:?}", res);
    let updated = func(res);
    debug!("after update: {:?}", updated);

    let res = c.put(u).json(&updated).send().await?;
    debug!("response from update: {:?}", res);

    Ok(())
}

/// Runs the Agent on a Job configuration and updates the state of the Jobs API
async fn do_run(url: Url, j: Job) -> Result<()> {
    let start = Instant::now();

    let id = j.id.clone();
    do_update(url.clone(), &id, |mut j: Job| {
        j.state = State::Running;
        j
    })
    .await?;

    DynamicAgent::run(&into_job(url.clone(), j).await?).await?;

    let duration = start.elapsed();

    do_update(url.clone(), &id, |mut j: Job| {
        j.state = State::Done;
        j.elapsed = Some(duration);
        j
    })
    .await?;

    Ok(())
}

/// Runs a loop that polls the Jobs API on the provided interval for a number
/// of iterations
async fn run_poll(url: url::Url, interval: Duration, n: i32) -> Result<()> {
    // let mut handler = Box::pin(tokio::signal::ctrl_c());
    // let mut h = handler.as_mut();
    // h.poll(tokio);
    for i in 0..n {
        let res: Jobs = reqwest::get(url.join(jobs)?).await?.json().await?;
        debug!("Got response from polling iteration {}: {:?}", i, res);

        for job in res {
            info!("Got job: {}", job.name);
            match job.state {
                State::Waiting => {
                    info!("Job {} is waiting", job.name);
                    let handle = tokio::task::spawn(do_run(url.clone(), job));
                    tokio::join!(handle).0??;
                }
                State::Running => {
                    info!("Job {} is still running", job.name);
                }
                State::Done => {
                    info!("Job {} is done", job.name);
                }
            }
        }

        tokio::time::delay_for(interval).await;
    }

    Ok(())
}
