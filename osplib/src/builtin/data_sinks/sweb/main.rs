/*!
 * The Oxigraph Data Sink is a direct data sink to the Oxigraph
 * triplestore/graph database It simply takes in the output JSON-LD, may
 * convert it into a different acceptable format, and POSTs it to an
 * Oxigraph endpoint
 */
use crate::prelude::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct SemanticWebServerSink {
    pub base_url: String,
    pub mapping: HashMap<String, String>,
}

impl Default for SemanticWebServerSink {
    fn default() -> Self {
        SemanticWebServerSink {
            base_url: "0.0.0.0:9000".to_string(),
            mapping: HashMap::new(),
        }
    }
}

use async_trait::async_trait;
use erased_serde::Serializer as ESerializer;

#[async_trait]
#[typetag::serde(name = "semantic-web")]
impl crate::api::DataSink for SemanticWebServerSink {
    // TODO: figure this out: dyn Any + Serialize
    async fn consume(&self, input: crate::api::Intermediate) -> Result<()> {
        info!("Running basic data sink");

        let c = reqwest::Client::builder().build()?;

        let mut writer = Vec::with_capacity(128);
        // TODO: make this dynamic
        let ser = &mut serde_json::Serializer::pretty(&mut writer);

        input.erased_serialize(&mut ESerializer::erase(ser))?;

        // We may need to access the @type field to determine where to post via
        // regular HTTP Also, at some point we will need to support at
        // least parsing the context to get the full type
        // How do we access on an Any/Erased serialize type
        // Maybe we should just natively use the Serde_json Value type no
        // matter what, and through the traits it would certainly
        // simplify things also if I write the JSON-LD parser
        // supporting that

        let post_location = format!("{}{}", &self.base_url, "/person");

        debug!("sending request to: {}", post_location);

        let res = &c
            .post(&post_location)
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
