use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Plugin<ComponentConfigType> {
    plugin: String,
    config: ComponentConfigType
}

type DataRepresentation Serialize;
type InMemResult Result<DataRepresentation, _>;

type PluginID String;

trait BasicPlugin<ComponentConfigType> {
    fn configure(ComponentConfigType) -> Result;
}

trait Requestor: BasicPlugin {
    fn makeRequest(url: String) -> Result;
}

trait Matcher: BasicPlugin {
    fn runParseMatch(headers: String) -> PluginID;
    fn runExtractorMatch(headers: String) -> PluginID;
}

trait Parser: BasicPlugin {
    fn parse(response: String) -> InMemResult;
}

trait Extractor: BasicPlugin {
    // , definition: String should be encoded in config??
    fn extract(repres: InMemResult) -> InMemResult;
}

trait Generator: BasicPlugin {
    fn generate(repres: InMemResult) -> InMemResult;
}