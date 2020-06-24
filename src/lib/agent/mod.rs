use super::plugin::*;
use anyhow::Error;

pub trait Agent<R: Requestor, M: Matcher, E: Extractor> {
    fn run(self) -> Result<(), Error>;
}

// impl dyn Agent {
//     fn new<R, M, E>() {

//     }
// }

use crate::builtin::requestor::Requestor as Reqr;
use crate::builtin::matchers::regex::RegexMatcher;
use crate::builtin::extractors::html::HTMLExtractor;

pub struct LocalAgent {
    r: Reqr,
    m: RegexMatcher,
    e: HTMLExtractor,
}

impl LocalAgent {
    fn new() -> LocalAgent {
        LocalAgent {
            r: Reqr,
            m: RegexMatcher { c: None },
            e: HTMLExtractor { c: None },
        }
    }
}

impl Agent<Reqr, RegexMatcher, HTMLExtractor> for LocalAgent {
    fn run(self) -> Result<(), Error> {
        // self.r.make_request();
        // self.m.
        Ok(())
    }
}
