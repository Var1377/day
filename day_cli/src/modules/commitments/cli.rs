use clap::{Args, Subcommand};
use day_core::{modules::commitments::Commitment, state::State};

use crate::{
    autocomplete::TextAutocompleter,
    cli::{Cli, Runnable},
    config::Configurable, table::TableFmt,
};

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
    /// Manage ICALs
    Ical {
        #[clap(subcommand)]
        subcmd: IcalSubcommand,
    },
}

#[derive(Subcommand, Debug)]
/// Manage ICAL links
enum IcalSubcommand {
    /// Add a new ICAL
    Add,
    /// Remove an ICAL
    Remove,
    /// List all ICALs
    List,
}

enum CommitmentTypeToRemove {
    Custom,
    Ical,
}

impl std::fmt::Display for CommitmentTypeToRemove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommitmentTypeToRemove::Custom => write!(f, "Custom"),
            CommitmentTypeToRemove::Ical => write!(f, "Ical"),
        }
    }
}

impl Runnable for CommitmentCli {
    type Args = Cli;

    fn run(&self, _args: &Self::Args, state: &mut State) -> anyhow::Result<()> {
        match &self.subcmd {
            CommitmentsSubcommand::Add => {
                let mut commitment = Commitment::default();
                commitment.run_configurator()?;
                println!("\"{}\" saved!", commitment.details.title);
                state.commitments.commitments.push(commitment);
            }
            CommitmentsSubcommand::Remove => {
                let autocompleter = TextAutocompleter::new(
                    |commitment| commitment.details.title.clone(),
                    state.commitments.commitments.clone(),
                );
                let to_remove =
                    inquire::Text::new("What is the title of the commitment you want to remove?")
                        .with_autocomplete(autocompleter)
                        .prompt()?;

                let idx_to_remove = state
                    .commitments
                    .commitments
                    .iter()
                    .position(|commitment| commitment.details.title == to_remove)
                    .expect("No commitment with that title found");

                state.commitments.commitments.remove(idx_to_remove);
                println!("\"{}\" removed!", to_remove);
            }
            CommitmentsSubcommand::List => {
                TableFmt::print_iter(state.commitments.commitments.clone())
            }
            CommitmentsSubcommand::Ical { subcmd } => match subcmd {
                IcalSubcommand::Add => {
                    unimplemented!()
                }
                IcalSubcommand::List => {
                    unimplemented!()
                }
                IcalSubcommand::Remove => {
                    unimplemented!()
                }
            },
        }
        Ok(())
    }
}
