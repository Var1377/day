pub mod time;
pub mod state;
pub mod weekly;
pub mod config;
pub mod error;
pub mod maybe_set;

pub use error::Error;

#[macro_use]
extern crate serde;
extern crate serde_inline_default;