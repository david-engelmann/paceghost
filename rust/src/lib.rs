//! Paceghost scores recorded agent runs against timed budget windows using
//! exact rational projection math and renders twenty seven deterministic
//! readings across single runs, seasons, field days, ladders, rematches,
//! and multi day meets.
//!
//! Every input loader in this crate is a pure offline function that
//! returns structured defect lists rather than panicking on malformed
//! JSON, and report renderers execute deterministically once their
//! documented run or meet preconditions hold. The command line binary
//! in `cli` is a thin dispatch shell over these modules.

pub mod cli;
pub mod fieldday;
pub mod json;
pub mod ladder;
pub mod meet;
pub mod pynum;
pub mod pytext;
pub mod rematch;
pub mod runs;
pub mod scan;
pub mod season;
