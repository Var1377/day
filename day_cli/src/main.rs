#![allow(dead_code, unused_assignments)]

#[macro_use] extern crate extension_trait;

mod config;
mod state;
mod modules;
mod cli;
mod fs;
mod table;

use clap::Parser;
use human_panic::setup_panic;
use cli::Cli;

fn main() -> anyhow::Result<()> {
    setup_panic!();

    let cli = Cli::parse();
    cli.run()
}
