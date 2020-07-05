use anyhow::{Context, Error, Result};
use log::{debug, info};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

pub type Key = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct Value {
    selector: String,
    val: ElemOptions,
}

// pub type Attr(String);

#[derive(Serialize, Deserialize, Debug)]
pub enum ElemOptions {
    // Representation: these correspond to scraper::element_ref::ElementRef
    HTML,
    InnerHTML,
    Text,

    // Logical: these correspond to scraper::node::Element
    ID,
    // Classes,
    Attr(String),
    // Attributes
}
// The above has some commented out because they represent multiple values.
// However, the output for a given key needs to be one string.

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ScraperRs {
    definitions: HashMap<Key, Value>,
}

pub type Output = HashMap<Key, OutItem>;

pub type OutItem = String;
// TODO: figure out Multiple Extraction stuff
// 1. a selector gets multiple elements
// 2. a user wants to access multiple items from an element (e.g. text + an
// attribute)

// In that case, OutItem could be a variety of types
// pub enum OutItem {
//     String,
//     Vec<String>
// }

use scraper::{ElementRef, Html, Selector};
// use quick_error::ResultExt;

// use itertools::Itertools;
use ElemOptions::*;

fn elem_to_out_item(em: ElementRef, opts: &ElemOptions) -> Result<OutItem> {
    match opts {
        HTML => Ok(em.html()),
        InnerHTML => Ok(em.inner_html()),
        // TODO: figure out what this separator should be
        // TODO: when we implement Multiple Extraction,
        // this could just be a list
        Text => Ok(em.text().fold(String::new(), |a, b| a + b + " ")),
        Attr(name) => em
            .value()
            .attr(name.as_str()) //Option
            // If there is something, convert type properly
            .map(|id| id.to_string())
            .ok_or_else(|| {
                Error::msg(format!("failed to get attribute {}", name))
            }),
        ID => em
            .value()
            .id()
            .map(|id| id.to_string())
            .ok_or_else(|| Error::msg("failed to get ID")),
    }
}

#[typetag::serde(name = "scraper_rs")]
impl crate::api::Extractor for ScraperRs {
    // TODO: maybe think about streaming/batching if supported by extraction
    fn extract(
        &self,
        input: crate::api::Response,
    ) -> Result<Box<dyn std::any::Any>> {
        info!("extracting");
        // TODO: think about using fragment instead of Document here
        let doc = Html::parse_document(input.body.as_str());

        let mut out: Output = HashMap::new();

        let defs = &self.definitions;
        for (key, val) in defs {
            let s = Selector::parse(val.selector.as_str())
                // .context("failed")?;
                // TODO: figure out this weird error handling
                .or_else(|_| Err(Error::msg("failed to parse selector")))?;

            let elem = doc.select(&s).next().ok_or_else(|| {
                Error::msg("failed to parse get first element")
            })?;

            debug!("got elem {:#?}", elem);

            let o = elem_to_out_item(elem, &val.val)?;

            let n = key.clone();

            debug!("key \"{}\", value (output) \"{}\"", n, o);

            out.insert(n, o);
        }
        Ok(Box::new(out))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use anyhow::Result;

    use crate::map;
    // use crate::utils::map;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn configure_extract() -> Result<()> {
        init();
        let e = ScraperRs {
            definitions: map!(
                "charset".to_string() => Value {
                    selector:"meta[charset]".to_string(),
                    val: ElemOptions::HTML
                }
            ),
        };

        Ok(())
    }

    use crate::api::plugin::Extractor;
    #[test]
    fn run_extract() -> Result<()> {
        init();

        let html = r#"
        <!DOCTYPE html>
        <meta charset="utf-8">
        <title>Hello, world!</title>
        <h1 class="foo">Hello, <i>world!</i></h1>
        "#;

        // TODO: maybe have this map include the expected values
        let mut e = ScraperRs {
            definitions: map!(
                "charset".to_string() => Value {
                    selector:"meta[charset]".to_string(),
                    val: ElemOptions::HTML
                };
                "charset2".to_string() => Value {
                    selector:"meta[charset]".to_string(),
                    val: ElemOptions::InnerHTML
                };
                "charset3".to_string() => Value {
                    selector:"meta[charset]".to_string(),
                    val: ElemOptions::Text
                };
                "headerText".to_string() => Value {
                    selector:"h1".to_string(),
                    val: ElemOptions::Text
                };
                "italicText".to_string() => Value {
                    selector:".foo>i".to_string(),
                    val: ElemOptions::Text
                }
            ),
        };

        e.extract(crate::api::Response {
            status: 200,
            version: "HTTP/1.1".to_string(),
            body: html.to_string(),
            headers: HashMap::new(),
        })?;

        Ok(())
    }
}
