/*!
Utilities for converting HTTP types, dealing with files

- Converting HTTP [HeaderMap]s to [HashMap]s
- Macro for easily creating HashMaps (optionally with [`String`])
- Various file reading/deseralization utilities
*/
#[allow(unused_imports)]
use reqwest::header::HeaderMap;
#[allow(unused_imports)]
use std::collections::HashMap;

mod http_conv;
mod read_files;

pub use http_conv::*;
pub use read_files::*;
