/*!
A bootstrapping library so users can build consistent remote plugins.

So far it only supports HTTP as the transport. It allows for
registration of environment variables for configuration.
*/

#![warn(clippy::all)]

pub mod plugin;

pub mod settings;

pub use plugin::Plugin;
pub use settings::Settings;
