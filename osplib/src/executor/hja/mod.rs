/*!
 * HJA is the HTTP Jobs API.
 *
 * It is an Executor that provides all external APIs over HTTP.
 * It also stores all internal data about the Jobs over HTTP as well.
 */

pub mod api;
pub mod executor;

pub use executor::ServerExecutor;
