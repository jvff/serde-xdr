#[macro_use]
extern crate error_chain;

mod errors;

pub use errors::{Error, ErrorKind, Result};
