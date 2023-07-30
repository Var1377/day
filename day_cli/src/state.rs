use day_core::{
    config::Config,
    state::State, modules::todos::TodoState,
};

use crate::config::ConfigLoad;
use crate::modules::todo::TodoStateLoad;

#[extension_trait]
pub impl StateLoad for State {
    fn load() -> anyhow::Result<Self> where Self : Sized {
        Ok(
            Self { config: Config::load()?, todo: TodoState::load()? }
        )
    }

    fn save(&self) -> anyhow::Result<()> {
        self.config.save()?;
        self.todo.save()?;
        Ok(())
    }
}