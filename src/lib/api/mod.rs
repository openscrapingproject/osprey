// ## API
// The Osprey `api` module defines various core types.
// In api::plugin, we define the traits for each of the components of
// OpenScraping. In our other internal modules, we define the types of
// the standard CLI input file: `JobCollection`, and
// the intermediate plugin types that the plugins use as input and
// output, and the agent passes to each other

mod http;
mod job_collection;
pub mod plugin;
mod plugin_types;

pub use job_collection::*;
pub use plugin::*;
pub use plugin_types::*;
