use anyhow::{Context, Error, Result};
use log::info;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Default)]
pub struct ScraperRs {
    pub c: Option<Config>,
}

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
// The above has some commented out because they represent multiple values. However, the output for a given key needs to be one string.

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    definitions: HashMap<Key, Value>,
}

pub type Output = HashMap<Key, OutItem>;

pub type OutItem = String;
// TODO: figure out
// 1. a selector gets multiple elements
// 2. a user wants to access multiple items from an element (e.g. text + an attribute)
// pub enum OutItem {
//     String,
//     Vec<String>
// }

use scraper::{node::Element, ElementRef, Html, Selector};
// use quick_error::ResultExt;

// use itertools::Itertools;
use ElemOptions::*;

fn elem_to_out_item(em: ElementRef, opts: &ElemOptions) -> Result<OutItem> {
    match opts {
        HTML => Ok(em.html()),
        InnerHTML => Ok(em.inner_html()),
        Text => Ok(em.text().fold(String::new(), |a, b| a + b + "\n")),
        Attr(name) => em
            .value()
            .attr(name.as_str()) //Option
            // If there is something, convert type properly
            .map(|id| id.to_string())
            .ok_or_else(|| Error::msg(format!("failed to get attribute {}", name))),
        ID => em
            .value()
            .id()
            .map(|id| id.to_string())
            .ok_or_else(|| Error::msg("failed to get ID")),
    }
}

impl crate::plugin::Extractor for ScraperRs {
    // TODO: you need to implement this

    type Input = String;
    type Relevant = Output;

    // use anyhow::Result;

    // TODO: maybe think about streaming/batching if supported by extraction
    fn extract(&self, input: Self::Input) -> Result<Self::Relevant> {
        info!("extracting");
        // TODO: think about using fragment instead of Document here
        let doc = Html::parse_document(input.as_str());

        let mut out: Output = HashMap::new();

        let defs = &self
            .c
            .as_ref()
            .ok_or_else(|| Error::msg("failed to get defs"))?
            .definitions;
        for (key, val) in defs {
            let s = Selector::parse(val.selector.as_str())
                // .context("failed")?;
                // TODO: figure out this weird error handling
                .or_else(|e| Err(Error::msg("failed to parse selector")))?;

            let elem = doc.select(&s).next().ok_or_else(|| {
                Error::msg("failed to parse get first element")
            })?;

            info!("got elem {:#?}", elem);

            let o = elem_to_out_item(elem, &val.val)?;

            out.insert(key.clone(), o);
        }
        Ok(out)
    }
}

impl crate::plugin::BasicPlugin for ScraperRs {
    type Config = Config;
    fn configure(&mut self, c: Config) -> Result<()> {
        self.c = Some(c);
        Ok(())
    }
    fn get_default_config(&self) -> Config {
        Config::default()
    }

    fn parse_config(&self, input: serde_json::Value) -> Result<Self::Config> {
        serde_json::from_value(input.clone()).with_context(|| {
            format!("failed to parse configuration {}", input)
        })
    }
}
