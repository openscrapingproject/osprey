/*!
Osprey is a Rust implementation of the OpenScraping project.

It provides a library that contains trait definitions
for OSP Components, along with many builtin plugins.

It also provides a CLI frontend to this library, which
allows a user to input a simple JSON file to kick off
an entire OpenScraping pipeline.
*/

#![warn(clippy::all)]

pub mod api;

// Builtin provides all builtin plugin implementations
pub mod builtin;

pub mod remote_plugins;

pub mod agent;

pub mod prelude;

#[macro_use]
pub mod utils;
