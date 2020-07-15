/*!
 * Executors listen for new Jobs via their internal
 * Data Service and spawn Agents to run them.
 */

mod api;

pub mod hja;

pub use api::*;
pub use hja::ServerExecutor;
