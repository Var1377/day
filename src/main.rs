#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;

mod config;
mod state;
mod modules;

use clap::{Parser, Subcommand};
use config::ConfigCli;
use human_panic::setup_panic;

#[derive(Parser, Debug)]
#[clap(name = "day.rs", version = "0.1.0", author = "Varun Latthe (Var1337)", about, after_help = "Day.rs is a command line tool to help you maximise efficiency around an already packed day. To see what it can do, run `day config`0")]
pub struct Cli {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

impl Cli {
    pub fn run(&self) -> anyhow::Result<()> {
        match &self.subcmd {
            SubCommand::Config(config_args) => config_args.run(),
        }
    }
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    #[clap(visible_aliases = &["cfg", "c"])]
    /// Show or change configuration values
    Config(ConfigCli),
}

fn main() -> anyhow::Result<()> {
    setup_panic!();

    let cli = Cli::parse();
    cli.run()
}
