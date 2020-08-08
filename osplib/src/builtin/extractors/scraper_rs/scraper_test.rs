use anyhow::Result;
use crate::prelude::*;

use std::collections::HashMap;

use crate::map;
use super::scraper::*;

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
