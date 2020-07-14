use crate::prelude::*;


#[derive(Serialize, Deserialize, Debug)]
pub struct ServerExecutor {
    /// Main endpoint URL
    pub base_url: String,

    /// Jobs endpoint (relative to base). Default: /jobs
    // pub jobs: String

    // pub config: String,
    // pub job_collections: String,
}


const jobs: &str = "/jobs";
const config: &str = "/config";
const job_collections: &str = "/job_collections";

#[typetag::serde(name = "regex")]
impl crate::executor::Executor for ServerExecutor {
    
}