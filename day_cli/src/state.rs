use clap::{Args, Subcommand};
use day_core::{
    config::Config,
    state::State, modules::todos::TodoState,
};

use crate::cli::{Cli, Runnable};
use crate::config::ConfigLoad;
use crate::fs::DATA_DIR;
use crate::modules::todo::TodoStateLoad;

#[extension_trait]
pub impl StateLoad for State {
    fn load() -> anyhow::Result<Self> where Self : Sized {
        Ok(
            Self { config: Config::load()?, todo: TodoState::load()? }
        )
    }

    fn save(&mut self) -> anyhow::Result<()> {
        self.config.save()?;
        self.todo.save()?;
        Ok(())
    }
}

#[derive(Debug, Args)]
pub struct StateArgs {
    #[clap(subcommand)]
    subcmd: StateSubCommand,
}

#[derive(Subcommand, Debug)]
enum StateSubCommand {
    #[clap(visible_aliases = &["p"])]
    /// Display the path to the data directory
    Path,

    #[clap(visible_aliases = &["r"])]
    /// Reset all data to defaults
    Reset,
}

impl Runnable for StateArgs {
    type Args = Cli;
    fn run(&self, _args: &Self::Args, state: &mut State) -> anyhow::Result<()> {
        match &self.subcmd {
            StateSubCommand::Path => {
                println!("{}", DATA_DIR.display());
                Ok(())
            }
            StateSubCommand::Reset => {
                if inquire::Confirm::new("Are you sure you want to reset all data?").with_default(false).prompt()? {
                    let mut new = State::default();
                    new.config = state.config.clone();
                    *state = new;
                    println!("Data reset");
                }
                Ok(())
            }
        }
    }
}