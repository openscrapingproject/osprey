/*!
This crate implements a basic XPath 1.0 extractor

It uses the [sxd-xpath] library.
!*/
use crate::prelude::*;

use std::collections::HashMap;

pub type Key = String;

use sxd_document::parser;
use sxd_xpath::{Context, Factory, Value, XPath};

/// Value represents one property extracted from one HTML element
// #[derive(Serialize, Deserialize, Debug)]
// pub struct Value {
//     /// Selector is an XPATH selector string string to select an element (or
// a value)     pub selector: String,

//     // TODO: will we support getting values
//     // Val is which property of the HTML element to extract
//     // pub val: ElemOptions,
// }

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct XPathExtractor {
    // unfortunately we can't do advanced parsing of this item here, e.g. at
    // config/creation time b/c some limits on the Expression trait. (its
    // not sync?)
    pub definitions: HashMap<Key, String>,
}

pub type Output = HashMap<Key, MultipleElements<OutItem>>;

pub type MultipleElements<T> = Vec<T>;

pub type OutItem = String;

use html5ever::serialize::{serialize, SerializeOpts};
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::{parse_document, ParseOpts};
use markup5ever_rcdom as rcdom;
use rcdom::{RcDom, SerializableHandle};

fn parse(input: String) -> sxd_document::Package {
    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let dom = parse_document(RcDom::default(), opts)
        .from_utf8()
        .read_from(&mut input.as_bytes())
        .unwrap();

    // let document = dom.document.borrow();
    // let html = document.children[0].borrow();
    // let body = html.children[1].borrow(); // Implicit head element at
    // children[0].

    // {
    //     let mut a = body.children[0].borrow_mut();
    //     if let Element(_, _, ref mut attributes) = a.node {
    //         attributes[0].value.push_tendril(&From::from("#anchor"));
    //     }
    // }

    let doc: SerializableHandle = dom.document.clone().into();
    let mut bytes = vec![];
    serialize(&mut bytes, &doc, SerializeOpts::default()).unwrap();
    let result = String::from_utf8(bytes).unwrap();
    info!("output: {}", result);

    let package = parser::parse(result.as_str()).expect("failed to parse XML");
    package
}

#[typetag::serde(name = "xpath")]
impl crate::api::Extractor for XPathExtractor {
    // TODO: maybe think about streaming/batching if supported by extraction
    fn extract(
        &self,
        input: &crate::api::Response,
    ) -> Result<crate::api::Intermediate> {
        info!("extracting");

        debug!("parsing");
        let package = parse(input.body.clone());
        let document = package.as_document();

        info!("doc: {:#?}", document.root().children());

        let mut out: Output = HashMap::new();

        let defs = &self.definitions;
        for (key, val) in defs {
            let factory = Factory::new();
            let xpath = factory.build(val).or_else(|e| {
                Err(Error::new(e)
                    .context(format!("Invalid XPath selector: {}", val)))
            })?;

            debug!("xpath: {:#?}", xpath);

            let context = Context::new();

            let value = xpath
                .evaluate(&context, document.root())
                .expect("XPath evaluation failed");

            let n = key.clone();
            debug!("val: {:#?}", value);
            let o = vec![value.string()];

            info!("key: {}, value (output): {:?}", n, o);

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
        let _ = XPathExtractor {
            definitions: map!(
                "charset".to_string() => "//meta@charset".to_string()
            ),
        };

        Ok(())
    }

    // Cargo runs tests in the workspace dir (e.g. osplib), so we do this
    const HTML_PREFIX: &'static str = "../tests/html";

    use crate::api::plugin::Extractor;
    use anyhow::Context;
    use std::fs::read_to_string;
    use std::path::Path;

    #[test]
    fn run_extract() -> Result<()> {
        init();

        // We get this: 'failed to parse XML: Error { location: 151, errors:
        // {UnclosedElement} }' with the html
        // let html = r#"
        // <!DOCTYPE html>
        // <meta charset="utf-8">
        // <title>Hello, world!</title>
        // <h1 class="foo">Hello, <i>world!</i></h1>
        // "#;
        let p = Path::new(HTML_PREFIX).join("example.html");
        let html = read_to_string(&p).context(format!(
            "failed to open path {:?} from {:?}",
            p,
            std::env::current_dir()?
        ))?;

        let italic = r#"//*[@class="foo"]/i"#;

        // TODO: maybe have this map include the expected values
        let e = XPathExtractor {
            definitions: map!(
                "charset".to_string() => "//meta/@charset".to_string();
                "italicText".to_string() => italic.to_string();
                "headers".to_string() => "//h1".to_string();
                "func".to_string() => "substring-after(//meta/@charset, \"-\")".to_string()
            ),
        };

        let r = e.extract(&crate::api::Response {
            url: "localhost:8080/hello".to_string(),
            status: 200,
            version: "HTTP/1.1".to_string(),
            headers: HashMap::new(),
            body: html.to_string(),
        })?;

        assert!(r.is::<Output>());
        let fin = r.downcast_ref::<Output>().unwrap();
        assert_eq!(
            fin,
            &map!(
                "charset".to_string() => vec!["UTF-8".to_string()];
                "italicText".to_string() => vec!["world!".to_string()];
                "headers".to_string() => vec!["Heading".to_string()];
                "func".to_string() => vec!["8".to_string()]
            )
        );

        println!("{:#?}", r);

        Ok(())
    }

    #[test]
    fn run_complex_extract() -> Result<()> {
        init();

        // We get this: 'failed to parse XML: Error { location: 151, errors:
        // {UnclosedElement} }' with the html
        // let html = r#"
        // <!DOCTYPE html>
        // <meta charset="utf-8">
        // <title>Hello, world!</title>
        // <h1 class="foo">Hello, <i>world!</i></h1>
        // "#;

        let html = read_to_string(Path::new(HTML_PREFIX).join("race.html"))?;

        let italic = r#"//*[@class="foo"]/i"#;

        // TODO: maybe have this map include the expected values
        let e = XPathExtractor {
            definitions: map!(
                "name".to_string() => "(//table[@class=\"infotable\"])[1]/tbody/tr[2]/text()".to_string()
            ),
        };

        let r = e.extract(&crate::api::Response {
            url: "localhost:8080/hello".to_string(),
            status: 200,
            version: "HTTP/1.1".to_string(),
            headers: HashMap::new(),
            body: html,
        })?;

        assert!(r.is::<Output>());
        // let fin = r.downcast_ref::<Output>().unwrap();
        // assert_eq!(
        //     fin,
        //     &map!(
        //         "charset".to_string() => vec!["UTF-8".to_string()];
        //         "italicText".to_string() => vec!["world!".to_string()];
        //         "headers".to_string() => vec!["Heading".to_string()];
        //         "func".to_string() => vec!["8".to_string()]
        //     )
        // );

        println!("{:#?}", r);

        Ok(())
    }
}
