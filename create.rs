#!/usr/bin/env cargo +nightly -Zscript

//! ```cargo
//! [package]
//! edition = "2021"
//!
//! [dependencies]
//! inquire = { version = "0.6.2", features = ["date"] }
//! chrono = "0.4.31"
//! reqwest = { version = "0.11.22", features=["blocking"] }
//! ```

use inquire::{error::InquireResult, DateSelect};

fn main() -> InquireResult<()> {
    let date = DateSelect::new("Date:").prompt()?;

    Ok(())
}
