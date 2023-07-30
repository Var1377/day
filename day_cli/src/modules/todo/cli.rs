use crate::{cli::{Runnable, Cli}, fs::JsonEditable, config::Configurable};
use clap::{Args, Subcommand};
use day_core::modules::todos::Todo;

use super::TODO_STATE_PATH;

#[derive(Debug, Args)]
pub struct TodoArgs {
    #[clap(subcommand)]
    pub subcommand: TodoSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum TodoSubcommand {
    #[clap(visible_aliases = &["a"])]
    /// Add a new todo
    Add(TodoAddArgs),
    #[clap(visible_aliases = &["l"])]
    /// List todos
    List(TodoListArgs),
    #[clap(visible_aliases = &["d"])]
    /// Mark a todo as done
    Done(TodoDoneArgs),
    #[clap(visible_aliases = &["r"])]
    /// Remove a todo
    Remove(TodoRemoveArgs),
    /// Manage your todo list in your editor
    Edit,
}

#[derive(Debug, Args)]
pub struct TodoAddArgs;

#[derive(Debug, Args)]
pub struct TodoListArgs {
    #[clap(short, long)]
    /// list all todos
    pub all: bool,
    #[clap(short, long)]
    /// list n todos
    pub num: Option<usize>,
    #[clap(short, long)]
    /// list only done todos
    pub done: bool,
}

#[derive(Debug, Args)]
pub struct TodoDoneArgs;

#[derive(Debug, Args)]
pub struct TodoRemoveArgs;

impl Runnable for TodoArgs {
    type Args = Cli;

    fn run(&self, _args: &Self::Args, state: &mut day_core::state::State) -> anyhow::Result<()> {
        match &self.subcommand {
            TodoSubcommand::Add(_) => {
                let mut default = Todo::default();
                default.run_configurator()?;
                println!("{} added to todo list", &default.name);
                state.todo.todos.push(default);
            }
            TodoSubcommand::List(_) => {
                println!("List");
                unimplemented!();
            }
            TodoSubcommand::Done(_) => {
                println!("Done");
                unimplemented!();
            }
            TodoSubcommand::Remove(_) => {
                println!("Remove");
                unimplemented!();
            }
            TodoSubcommand::Edit => {
                state.todo.run_editor(&format!("Starting editor at {}", TODO_STATE_PATH.display()))?;
            }
        }

        Ok(())
    }
}