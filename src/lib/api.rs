// mod api {
    use serde::{Deserialize, Serialize};
    
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Agent {
        name: String,
        config: AgentConfiguration,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct AgentConfiguration {
        afterRequest: Option<AfterRequestConfig>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct AfterRequestConfig {
        storeRequest: bool,
        storeResponseHeaders: bool,
        storeResponse: bool,
        storeHash: bool,
    }
// }
