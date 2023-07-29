use serde::{Serialize, Deserialize};
use crate::modules::sleep::Sleep;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub sleep: Sleep
}

