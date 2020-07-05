// ## remote_plugins
// This module defines various shared utilities that allow for users
// to build uniform remote plugins. So far it only supports HTTP as
// the transport

pub mod plugin;

pub mod settings;

pub use plugin::Plugin;
pub use settings::Settings;
