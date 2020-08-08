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

/// Value represents one property extracted from one HTML element
#[derive(Serialize, Deserialize, Debug)]
pub struct Value {
    /// Selector is a CSS string to select an element
    pub selector: String,

    /// Val is which property of the HTML element to extract
    pub val: ElemOptions,
}

// pub type Attr(String);

/// This enum represents which property of an HTML element to extract
///
/// First 3: Representation: these correspond to [`ElementRef`]
/// Next 2: Logical: these correspond to [`scraper::node::Element`]
#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ScraperRs {
    pub definitions: HashMap<Key, Value>,
}

pub type Output = HashMap<Key, MultipleElements<OutItem>>;

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
        input: &crate::api::Response,
    ) -> Result<crate::api::Intermediate> {
        info!("extracting");
        // TODO: think about using fragment instead of Document here
        let doc = Html::parse_document(input.body.as_str());

        debug!("document {:#?}", doc);

        let mut out: Output = HashMap::new();

        let defs = &self.definitions;
        for (key, val) in defs {
            let s = Selector::parse(val.selector.as_str())
                // .context("failed")?;
                // TODO: figure out this weird error handling
                // Selector library uses other error handling lib that
                // is incompatible with anyhow Context
                .or_else(|_| Err(Error::msg("failed to parse selector")))?;

            let sr = doc.select(&s);

            let mut multiple: Vec<OutItem> = Vec::new();
            for elem in sr {
                debug!("got elem {:#?}", elem.value().name());

                let o = elem_to_out_item(elem, &val.val)?;
                multiple.push(o);
            }

            let n = key.clone();

            debug!("key: {}, value (output): {:?}", n, multiple);

            out.insert(n, multiple);
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
        let _ = ScraperRs {
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
        let e = ScraperRs {
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
            definitions: map!(
                "headerText".to_string() => Value {
                    selector:".Headline1".to_string(),
                    val: ElemOptions::Text
                }
            ),
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
}
