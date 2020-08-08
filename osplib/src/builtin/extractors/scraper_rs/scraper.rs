use crate::prelude::*;

use std::collections::HashMap;

pub type Key = String;

use scraper;
use scraper::{ElementRef, Html, Selector};
use ElemOptions::*;

/// SelectorValue represents one property extracted from one HTML element
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SelectorValue {
    /// Selector is a CSS string to select an element
    pub selector: String,

    /// Val is which property of the HTML element to extract
    pub val: ElemOptions,

    pub transform: Option<Transform>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Transform {
    TrimWhitespace,
    RemoveNewlines,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Value {
    Literal(String),
    Selector(SelectorValue),
    Metadata(MetadataExtractor),
}


// pub type Attr(String);

/// This enum represents which property of an HTML element to extract
///
/// First 3: Representation: these correspond to [`ElementRef`]
/// Next 2: Logical: these correspond to [`scraper::node::Element`]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ElemOptions {
    HTML,
    InnerHTML,
    Text,

    ID,
    Attr(String),
    // TODO: Classes,
}
// The above has some commented out because they represent multiple values.
// However, the output for a given key needs to be one string.

use serde_json::{Map, Value as SVal};
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ScraperRs {
    pub definitions: HashMap<Key, Value>,
}

pub type Output = Map<Key, SVal>;

pub type MultipleElements<T> = Vec<T>;

pub type OutItem = String;


fn elem_to_out_item(em: ElementRef, opts: &SelectorValue) -> Result<OutItem> {
    let intermediate = match &opts.val {
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
    }?;

    // TODO: think about user-provided transforms
    // Also think about using semantic information (e.g. xsd:DateTime) to parse
    // certain inputs to normalize them Also maybe have an option that is
    // original: boolean, which disables all transforms and outputs direct out
    if let Some(t) = &opts.transform {
        Ok(match t {
            Transform::TrimWhitespace => intermediate.trim().to_string(),
            Transform::RemoveNewlines => intermediate.replace('\n', ""),
        })
    } else {
        Ok(intermediate)
    }
}

#[typetag::serde(name = "scraper_rs")]
impl crate::api::Extractor for ScraperRs {
    // TODO: maybe think about streaming/batching if supported by extraction
    fn extract(
        &self,
        input: &crate::api::Response,
    ) -> Result<crate::api::Intermediate> {
        info!("extracting");
        // TODO: think about using fragment instead of Document here
        let doc = Html::parse_document(input.body.as_str());

        // debug!("document {:#?}", doc);

        let mut out: Output = Map::new();

        let defs = &self.definitions;
        for (key, val) in defs {
            match val {
                Value::Literal(l) => {
                    out.insert(key.clone(), SVal::String(l.clone()));
                    continue;
                }
                Value::Selector(sel) => {
                    let mut slc = sel.clone();
                    if slc.transform.is_none() {
                        slc.transform = Some(Transform::TrimWhitespace)
                    }
                    let s = Selector::parse(slc.selector.as_str())
                        // .context("failed")?;
                        // TODO: figure out this weird error handling
                        // Selector library uses other error handling lib that
                        // is incompatible with anyhow Context
                        .or_else(|_| {
                            Err(Error::msg("failed to parse selector"))
                        })?;

                    let sr = doc.select(&s);

                    let mut multiple: Vec<SVal> = Vec::new();
                    for elem in sr {
                        debug!("got elem {:#?}", elem.value().name());

                        let o = elem_to_out_item(elem, &slc)?;
                        multiple.push(SVal::String(o));
                    }

                    let n = key.clone();

                    debug!("key: {}, value (output): {:?}", n, multiple);

                    if multiple.len() == 1 {
                        let s = &multiple[0];
                        out.insert(n, s.clone());
                        continue;
                    }

                    out.insert(n, SVal::Array(multiple));
                }
            }
        }
        Ok(Box::new(out))
    }
}
