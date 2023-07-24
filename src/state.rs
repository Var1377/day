use crate::config::Config;

#[derive(Debug)]
pub struct State {
    pub config: Config,
}

impl State {
    pub fn load() -> anyhow::Result<Self> {
        Ok(
            Self { config: Config::load()? }
        )
    }
}