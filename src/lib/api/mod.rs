/*!
Defines traits for OSP Components and other core types.

In [plugin], we define the traits for each of the components of
OpenScraping. In our other internal modules, we define the types of
the standard CLI input file: [`JobCollection`], and
the intermediate plugin types that the plugins use as input and
output, and the agent passes to each other
*/

// TODO: think about making these public
mod http;
mod job_collection;
mod plugin_types;

pub mod plugin;

pub use job_collection::*;
pub use plugin::*;
pub use plugin_types::*;
