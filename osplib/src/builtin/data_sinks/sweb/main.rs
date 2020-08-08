/*!
 * The Oxigraph Data Sink is a direct data sink to the Oxigraph
 * triplestore/graph database It simply takes in the output JSON-LD, may
 * convert it into a different acceptable format, and POSTs it to an
 * Oxigraph endpoint
 */
use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct SemanticWebServerSink {
    pub baseURL: String,
}

impl Default for SemanticWebServerSink {
    fn default() -> Self {
        SemanticWebServerSink {
            baseURL: "0.0.0.0:9000".to_string(),
        }
    }
}

use async_trait::async_trait;
use erased_serde::Serializer as ESerializer;

#[async_trait]
#[typetag::serde(name = "oxigraph")]
impl crate::api::DataSink for SemanticWebServerSink {
    // TODO: figure this out: dyn Any + Serialize
    async fn consume(&self, input: crate::api::Intermediate) -> Result<()> {
        info!("Running basic data sink");

        let c = reqwest::Client::builder().build()?;

        let mut writer = Vec::with_capacity(128);
        // TODO: make this dynamic
        let ser = &mut serde_json::Serializer::pretty(&mut writer);

        input.erased_serialize(&mut ESerializer::erase(ser))?;
        // input.eras

        // Right now, Oxigraph allows POSTing directly to the / endpoint
        let postLocation = &self.baseURL;

        let res = &c
            .post(postLocation)
            .header("Content-Type", "application/ld+json")
            .body(writer)
            .send()
            .await?;

        res.error_for_status_ref()?;

        debug!("response {:#?}", res);

        info!("Wrote to sink");

        Ok(())
    }
}
