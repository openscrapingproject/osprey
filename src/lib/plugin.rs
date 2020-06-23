use serde::{Deserialize, Serialize};
use http::Result as HRes;
use http::Response;
// use std::future::Future;


use async_trait::async_trait;

#[derive(Serialize, Deserialize, Debug)]
struct Plugin<ComponentConfigType> {
    plugin: String,
    config: ComponentConfigType
}

// type DataRepresentation<R> = dyn Serialize;
type InMemResult<R> = Result<R, simple_error::SimpleError>;

type PluginID = String;

pub trait BasicPlugin<ComponentConfig, Err> {
    fn configure(config: ComponentConfig) -> Result<(), Err>;
}


#[async_trait]
pub trait Requestor<C, E>: BasicPlugin<C, E> {
    async fn make_request(url: String) -> HRes<Response<()>>;
}

trait Matcher<C, E>: BasicPlugin<C, E> {
    fn run_parse_match(headers: String) -> PluginID;
    fn run_extractor_match(headers: String) -> PluginID;
}

// trait Parser<C, E, R>: BasicPlugin<C, E> {
//     fn parse(response: String) -> InMemResult<R>;
// }

trait Extractor<C, E, R>: BasicPlugin<C, E> {
    // , definition: String should be encoded in config??
    fn extract(repres: InMemResult<R>) -> InMemResult<R>;
}

trait Generator<C, E, R>: BasicPlugin<C, E> {
    fn generate(repres: InMemResult<R>) -> InMemResult<R>;
}