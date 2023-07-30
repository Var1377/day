mod cli;
pub use cli::*;

use std::path::PathBuf;
use day_core::modules::todos::{TodoState, TodoConfig, Todo};
use once_cell::sync::Lazy;
use crate::{fs::DATA_DIR, config::Configurable};

static TODO_STATE_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = DATA_DIR.clone();
    path.push("todo.json");
    path
});

#[extension_trait]
pub impl TodoStateLoad for TodoState {
    fn load() -> anyhow::Result<Self> where Self: Sized {
        let contents = crate::fs::file_contents(&TODO_STATE_PATH)?;
        if let Some(contents) = contents {
            Ok(serde_json::from_str(&contents)?)
        } else {
            Ok(Self::default())
        }
    }

    fn save(&self) -> anyhow::Result<()> {
        std::fs::write(&*TODO_STATE_PATH, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }
}

impl Configurable for TodoConfig {
    fn run_configurator(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

impl Configurable for Todo {
    fn run_configurator(&mut self) -> anyhow::Result<()> {
        self.name = inquire::Text::new("Todo Name:")
            .with_default(&self.name)
            .prompt()?;

        self.description = inquire::Text::new("Todo Description:")
            .with_default(&self.description)
            .prompt()?;

        if inquire::Confirm::new("Does this todo have a deadline?")
            .with_default(false)
            .prompt()?
        {
            let mut date_picker = inquire::DateSelect::new("Todo Deadline:");
            if let Some(deadline) = &self.deadline {
                date_picker = date_picker.with_default(*deadline);
            }
            self.deadline = Some(date_picker.prompt()?);
        }
        
        Ok(())
    }
}