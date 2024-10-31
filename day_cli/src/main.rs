#![allow(dead_code, unused_assignments)]

#[macro_use] extern crate extension_trait;

mod config;
mod cli;
mod fs;
mod table;
mod autocomplete;
mod configurable;
mod subcommands;

use configurable::Configurable;

use clap::Parser;
use day_core::state::State;
use human_panic::setup_panic;
use cli::Cli;

use crate::{cli::Runnable, fs::LoadAndSave};

fn main() -> anyhow::Result<()> {
    setup_panic!(
        Metadata {
            name: env!("CARGO_PKG_NAME").into(),
            version: env!("CARGO_PKG_VERSION").into(),
            authors: "Varun L (vl331@cam.ac.uk)".into(),
            homepage: "https://var.cx".into(),
        }
    );

    let cli = Cli::parse();
    let mut state = State::load()?;
    cli.run(&(), &mut state)?;

    state.save()?;

    Ok(())
}
