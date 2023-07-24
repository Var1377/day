mod cli;
mod configurator;
pub use cli::ConfigCli;

use serde::{Serialize, Deserialize};
use crate::modules::sleep::Sleep;


pub trait Configurable {
    fn run_configurator(&mut self) -> anyhow::Result<()>;

    fn run_optional_configurator(&mut self) -> anyhow::Result<()> {
        self.run_configurator()
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub sleep: Sleep
}

