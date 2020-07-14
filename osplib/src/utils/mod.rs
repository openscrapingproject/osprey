/*!
Provides utilities for converting HTTP types, dealing with files

- Converting HTTP [HeaderMap]s to [HashMap]s
- Macro for easily creating HashMaps (optionally with [`String`])
- Various file reading/deseralization utilities
*/

mod http_conv;
mod read_files;

pub use http_conv::*;
pub use read_files::*;
