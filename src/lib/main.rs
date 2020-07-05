// #![warn(clippy::all)]

pub mod api;

// Builtin provides all builtin plugin implementations
pub mod builtin;

pub mod remote_plugins;

#[macro_use]
pub mod utils;
