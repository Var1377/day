use serde::{Serialize, Deserialize};
use crate::modules::{sleep::SleepConfig, todos::TodoConfig};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Config {
    #[serde(default)]
    pub sleep: SleepConfig,
    #[serde(default)]
    pub todo: TodoConfig,
}

