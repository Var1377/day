use day_core::{
    config::Config,
    state::State,
};

use crate::config::ConfigLoad;

#[extension_trait]
pub impl StateLoad for State {
    fn load() -> anyhow::Result<Self> where Self : Sized {
        Ok(
            Self { config: Config::load()? }
        )
    }
}