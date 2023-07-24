#![allow(dead_code, unused_assignments)]

mod config;
mod state;
mod modules;
mod cli;
mod fs;
mod time;

use clap::Parser;
use human_panic::setup_panic;
use cli::Cli;

fn main() -> anyhow::Result<()> {
    setup_panic!();

    let cli = Cli::parse();
    cli.run()
}
