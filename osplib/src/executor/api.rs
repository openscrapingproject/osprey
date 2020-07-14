
/// This represents the executor config.
pub struct Config {
    // TODO: top-level non-specific config
    
    // TODO: do we even need to abstract the executor out here
    // the only real difference should be in the data service and
    // how it's accessed
    // Also: how will CLI args be passed. Ah--> the config file
    // serialization from JSON means no CLI args
    // but 12FA means it should: https://github.com/softprops/envy
    pub executor: Box<dyn Executor>

}

#[typetag::serde(tag = "id", content = "config")]
#[async_trait]
pub trait Executor {
    /// The main run function will either poll the job service
    /// or wait on a pubsub system to get new jobs and spawn Agents
    async fn run() -> Result<()>;
    
    // TODO: think about adding custom logic for polling or pubsub
}