/*!
 * The MongoDB Data Sink is a direct data sink to MongoDB
 */
use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct MongoDBSink {
    pub endpoint: String,
    pub database: String,
}

impl Default for MongoDBSink {
    fn default() -> Self {
        MongoDBSink {
            endpoint: "mongodb://localhost:27017".to_string(),
            database: "osprey".to_string(),
        }
    }
}

use async_trait::async_trait;
// use erased_serde::Serializer as ESerializer;

use mongodb::{options::ClientOptions, Client};

#[async_trait]
#[typetag::serde(name = "mongodb")]
impl crate::api::DataSink for MongoDBSink {
    // TODO: figure this out: dyn Any + Serialize
    async fn consume(&self, input: crate::api::Intermediate) -> Result<()> {
        info!("Running mongodb data sink");

        // Parse a connection string into an options struct.
        let mut client_options =
            ClientOptions::parse(self.endpoint.as_str()).await?;

        // Manually set an option.
        client_options.app_name = Some("osprey".to_string());

        // // Get a handle to the deployment.
        let client = Client::with_options(client_options)?;

        // List the names of the databases in that deployment.
        for db_name in client.list_database_names(None, None).await? {
            println!("{}", db_name);
        }

        let db = client.database(self.database.as_str());

        let col = db.collection("campaign_race_pages");

        let bs = bson::ser::to_bson(&input)?;

        match bs {
            bson::Bson::Document(d) => {
                let result = col.insert_one(d, None).await?;

                debug!("got result from insert {:#?}", result);

                info!("Wrote to sink");
            }
            _ => return Err(anyhow!("unknown bson type: {:#?}", bs)),
        }

        Ok(())
    }
}
