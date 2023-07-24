
use clap::{Parser, Subcommand};
use crate::config::ConfigCli;

use crate::state::State;

pub trait Runnable {
    type Args;
    fn run(&self, args: &Self::Args, state: &mut State) -> anyhow::Result<()>;
}

#[derive(Parser, Debug)]
#[clap(name = "day.rs", version = "0.1.0", author = "Varun Latthe (Var1337)", about, after_help = "Day.rs is a command line tool to help you maximise efficiency around an already packed day. To see what it can do, run `day config`")]
pub struct Cli {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

impl Cli {
    pub fn run(&self) -> anyhow::Result<()> {
        let mut state = State::load()?;

        match &self.subcmd {
            SubCommand::Config(config_args) => config_args.run(&self, &mut state),
        }
    }
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    #[clap(visible_aliases = &["cfg", "c"])]
    /// Show or change configuration values
    Config(ConfigCli),
}