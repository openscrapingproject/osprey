/*!
A custom prelude for commonly used utilities.

Provides anyhow overrides of basic error and [`Result`] types,
exports various log macros, and Serde derive macros.
*/

// Error handling and logging
pub use anyhow::{anyhow, Context, Error, Result};
pub use log::{debug, error, info, warn};

// Serde
pub use serde::{Deserialize, Serialize};
