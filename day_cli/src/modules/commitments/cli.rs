use clap::{Args, Subcommand};
use day_core::{modules::commitments::{Commitment, CustomEvent}, state::State};

use crate::{cli::{Cli, Runnable}, config::Configurable};

#[derive(Args, Debug)]
pub struct CommitmentCli {
    #[clap(subcommand)]
    subcmd: CommitmentsSubcommand,
}

#[derive(Subcommand, Debug)]
/// Manage commitments
enum CommitmentsSubcommand {
    /// Add a new commitment
    Add,
    /// Remove a commitment
    Remove,
    /// List all commitments
    List,
}

impl Runnable for CommitmentCli {
    type Args = Cli;

    fn run(&self, _args: &Self::Args, state: &mut State) -> anyhow::Result<()> {
        match &self.subcmd {
            CommitmentsSubcommand::Add => {
                let mut commitment = CustomEvent::default();
                commitment.run_configurator()?;
                println!("\"{}\" saved!", commitment.details.title);
                state.commitments.commitments.push(Commitment::Event(commitment));
            }
            CommitmentsSubcommand::Remove => {}
            CommitmentsSubcommand::List => {}
        }
        Ok(())
    }
}
