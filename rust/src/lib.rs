//! The Rust port of paceghost.
//!
//! The landed Python implementation is the behavioral spec of this
//! crate, under the port obligations PORT-1 to PORT-6 in CONTRACT.md.
//! The library carries the loaders, the readings, and the command
//! dispatch as pure functions, and the binary is a thin shell over
//! the dispatch.

pub mod cli;
pub mod fieldday;
pub mod json;
pub mod ladder;
pub mod pynum;
pub mod pytext;
pub mod rematch;
pub mod runs;
pub mod scan;
pub mod season;
