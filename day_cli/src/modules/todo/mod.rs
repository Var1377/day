mod cli;
mod fmt;
mod configurator;

pub use cli::*;

use crate::fs::DATA_DIR;
use day_core::modules::todos::TodoState;
use once_cell::sync::Lazy;
use std::path::PathBuf;

static TODO_STATE_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = DATA_DIR.clone();
    path.push("todo.json");
    path
});

#[extension_trait]
pub impl TodoStateLoad for TodoState {
    fn load() -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let contents = crate::fs::file_contents(&TODO_STATE_PATH)?;
        if let Some(contents) = contents {
            Ok(serde_json::from_str(&contents)?)
        } else {
            Ok(Self::default())
        }
    }

    fn save(&mut self) -> anyhow::Result<()> {
        // Sort todos by id descending, this is the same as ordering by creation date
        self.normalize();
        std::fs::write(&*TODO_STATE_PATH, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }
}
