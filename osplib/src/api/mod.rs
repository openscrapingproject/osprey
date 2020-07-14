/*!
Defines traits for OSP Components and other core types.

In [plugin], we define the traits for each of the components of
OpenScraping. In our other internal modules, we define the types of
the standard CLI input file: [`JobCollection`], and
the intermediate plugin types that the plugins use as input and
output, and the agent passes to each other
*/

// TODO: think about making these public
pub mod http;
pub mod jobs;
pub mod plugin_types;

pub mod plugin;

pub use jobs::*;
pub use plugin::*;
pub use plugin_types::*;
