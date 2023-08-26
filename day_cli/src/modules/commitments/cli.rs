use clap::{Args, Subcommand};
use day_core::state::State;

use crate::cli::{Runnable, Cli};

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

    fn run(&self, args: &Self::Args, state: &mut State) -> anyhow::Result<()> {
        match &self.subcmd {
            CommitmentsSubcommand::Add => {
                println!("Adding a new commitment");
            }
            CommitmentsSubcommand::Remove => {
                println!("Removing a commitment");
            }
            CommitmentsSubcommand::List => {
                println!("Listing all commitments");
            }
        }
        Ok(())
    }
}