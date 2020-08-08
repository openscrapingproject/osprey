/*!
This crate implements a basic extractor using CSS selectors.

It uses the [scraper] library.
!*/
// TODO: why are the docs rendering differently here?
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
    // We don't really want this transform enum to become a subset of XPath's
    // function library However, for interim purposes, this is here.
    // TODO: maybe find a declarative function library we can just plug in
    // to.
    SubstringAfter(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Value {
    Literal(String),
    Selector(SelectorValue),
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

// TODO: figure out Multiple Extraction stuff
// 1. a selector gets multiple elements
// 2. a user wants to access multiple items from an element (e.g. text + an
// attribute)

// In that case, OutItem could be a variety of types
// pub enum OutItem {
//     String,
//     Vec<String>
// }

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
            Transform::SubstringAfter(s) => intermediate
                .splitn(2, s)
                .last()
                .ok_or_else(|| {
                    anyhow!(
                        "failed to create substring after {} using {}",
                        intermediate,
                        s
                    )
                })?
                .to_string(),
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

#[cfg(test)]
mod tests {

    use super::*;
    use anyhow::Result;

    use crate::map;
    // use crate::utils::map;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn convert(hm: HashMap<String, SelectorValue>) -> HashMap<String, Value> {
        let mut out = HashMap::new();
        for (k, v) in hm.iter() {
            out.insert(k.clone(), Value::Selector(v.to_owned()));
        }
        out
    }

    #[test]
    fn configure_extract() -> Result<()> {
        init();
        let _ = ScraperRs {
            definitions: map!(
                "charset".to_string() => Value::Selector(SelectorValue {
                    selector:"meta[charset]".to_string(),
                    val: ElemOptions::HTML,
                    transform: None
                })
            ),
        };

        Ok(())
    }

    #[test]
    fn configure_extract_literals() -> Result<()> {
        init();
        let _ = ScraperRs {
            definitions: map!(
                "charset".to_string() => Value::Selector(SelectorValue {
                    selector:"meta[charset]".to_string(),
                    val: ElemOptions::HTML,
                    transform: None
                });
                "literal".to_string() => Value::Literal("an IRI".to_string())
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

        let mut hm = map!(
            "charset".to_string() => SelectorValue {
                selector:"meta[charset]".to_string(),
                val: ElemOptions::HTML,
                transform: None
            };
            "charset2".to_string() => SelectorValue {
                selector:"meta[charset]".to_string(),
                val: ElemOptions::InnerHTML,
                transform: None
            };
            "charset3".to_string() => SelectorValue {
                selector:"meta[charset]".to_string(),
                val: ElemOptions::Text,
                transform: None
            };
            "headerText".to_string() => SelectorValue {
                selector:"h1".to_string(),
                val: ElemOptions::Text,
                transform: None
            };
            "italicText".to_string() => SelectorValue {
                selector:".foo>i".to_string(),
                val: ElemOptions::Text,
                transform: None
            }
        );

        // TODO: maybe have this map include the expected values
        let e = ScraperRs {
            definitions: convert(hm),
        };

        e.extract(&crate::api::Response {
            url: "localhost:8080/hello".to_string(),
            status: 200,
            version: "HTTP/1.1".to_string(),
            headers: HashMap::new(),
            body: html.to_string(),
        })?;

        Ok(())
    }

    use std::fs::read_to_string;
    use std::path::Path;

    // Cargo runs tests in the workspace dir (e.g. osplib), so we do this
    const HTML_PREFIX: &'static str = "../tests/html";
    const officeSelector: &'static str = "body:nth-child(2) td.Text1 td.Text1 td:nth-child(1) table.infotable:nth-child(1) tbody:nth-child(1) tr:nth-child(3) > td.Switch0";

    #[test]
    fn run_realistic_extract() -> Result<()> {
        init();

        let p = Path::new(HTML_PREFIX).join("race.html");
        let html = read_to_string(&p).context(format!(
            "failed to open path {:?} from {:?}",
            p,
            std::env::current_dir()?
        ))?;

        // TODO: maybe have this map include the expected values
        let e = ScraperRs {
            definitions: convert(map!(
                "headerText".to_string() => SelectorValue {
                    selector:".Headline1".to_string(),
                    val: ElemOptions::Text,
                    transform: None
                };
                "office".to_string() => SelectorValue {
                    selector: officeSelector.to_string(),
                    val: ElemOptions::Text,
                    transform: None
                }
            )),
        };

        let result = e.extract(&crate::api::Response {
            url: "localhost:8080/hello".to_string(),
            status: 200,
            version: "HTTP/1.1".to_string(),
            headers: HashMap::new(),
            body: html.to_string(),
        })?;

        println!("got result: {:#?}", result);

        Ok(())
    }

    #[test]
    fn use_transform() -> Result<()> {
        init();

        let html = r#"
        <!DOCTYPE html>
        <meta charset="utf-8">
        <title>Hello, world!</title>
        <h1 class="foo">Hello, <i>world!</i></h1>
        <form name="dateForm" method="post" action="RaceDetail.html?SetNow=Y&amp;SetNow=Y&amp;SetNow=Y&amp;SetNow=Y&amp;RaceID=79753" xpath="1" style=""></form>
        "#;

        // TODO: maybe have this map include the expected values
        let e = ScraperRs {
            definitions: convert(map!(
                "raceID".to_string() => SelectorValue {
                    selector:"form".to_string(),
                    val: ElemOptions::Attr("action".to_string()),
                    transform: Some(Transform::SubstringAfter("RaceID=".to_string()))
                }
            )),
        };

        let result = e.extract(&crate::api::Response {
            url: "localhost:8080/hello".to_string(),
            status: 200,
            version: "HTTP/1.1".to_string(),
            headers: HashMap::new(),
            body: html.to_string(),
        })?;

        println!("got result: {:#?}", result);

        // TODO: figure out a way to assert this is actually what we expect.
        // Maybe use Any

        Ok(())
    }
}
