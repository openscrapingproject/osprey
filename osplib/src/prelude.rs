/*!
Provides a custom prelude for commonly used utilities.

Provides anyhow overrides of basic error and [`Result`] types,
exports various log macros, and Serde derive macros.
*/

// Error handling and logging
pub use anyhow::{Context, Error, Result};
pub use log::{debug, info, warn};

// Serde
pub use serde::{Deserialize, Serialize};