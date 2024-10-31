mod tree;

use clap::Subcommand;
use crate::cli::Runnable;
use crate::Cli;
use day_core::state::State;

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    #[clap(visible_aliases = &["t"])]
    /// Visualise your slice tree
    Tree(tree::Args),

    // #[clap(visible_aliases = &["cfg"])]
    // /// Show or change configuration values
    // Config(ConfigCli),

    // #[clap(visible_aliases = &["c"])]
    // /// Manage your commitments
    // Commitments(CommitmentCli),

    // #[clap(visible_aliases = &["d"])]
    // /// Show and edit all data stored by day
    // Data(StateArgs),

    // #[clap(visible_aliases = &["t"])]
    // /// Manage your todo list
    // Todo(TodoArgs),
}

impl Runnable for SubCommand {
    type Args = Cli;

    fn run(&self, _cli: &Self::Args, state: &mut State) -> anyhow::Result<()> {
        match self {
            SubCommand::Tree(args) => args.run(&(), state),
            // SubCommand::Config(args) => args.run(args, state),
            // SubCommand::Commitments(args) => args.run(args, state),
            // SubCommand::Data(args) => args.run(args, state),
            // SubCommand::Todo(args) => args.run(args, state),
        }
    }
}