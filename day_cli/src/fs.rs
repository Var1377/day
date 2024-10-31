use dirs::{config_dir, data_dir};
use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;
use std::env;
use std::path::Path;
use std::{collections::HashMap, path::PathBuf};

static ENV_VARS: Lazy<HashMap<String, String>> = Lazy::new(|| env::vars().collect());

fn from_env(name: &str) -> Option<String> {
    ENV_VARS.get(name).cloned()
}

fn dir_env_or_default(name: &str, default: &Path) -> std::io::Result<PathBuf> {
    let path = if let Some(dir) = from_env(name) {
        dir.into()
    } else {
        let mut path = default.to_owned();
        path.push("day");
        path
    };
    std::fs::create_dir_all(&path)?;
    Ok(path)
}

pub static CONFIG_DIR: Lazy<PathBuf> = Lazy::new(|| {
    dir_env_or_default("DAY_CONFIG_DIR", &config_dir().unwrap())
        .expect("Failed to create config dir")
});

pub static DATA_DIR: Lazy<PathBuf> = Lazy::new(|| {
    dir_env_or_default("DAY_DATA_DIR", &data_dir().unwrap())
        .expect("Failed to create data dir")
});

pub static CONFIG_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = CONFIG_DIR.clone();
    path.push("config.json");
    path
});

pub fn file_contents(path: &Path) -> std::io::Result<Option<String>> {
    if path.try_exists()? {
        Ok(Some(std::fs::read_to_string(path)?))
    } else {
        Ok(None)
    }
}

pub trait JsonEditable {
    fn run_editor(&mut self, message: &str) -> anyhow::Result<()>;
}

impl<T> JsonEditable for T
where
    T: DeserializeOwned + serde::Serialize,
{
    fn run_editor(&mut self, message: &str) -> anyhow::Result<()> {
        let new_config = inquire::Editor::new(message)
        .with_predefined_text(&serde_json::to_string_pretty(&self)?)
        .with_validator(|contents: &str| {
            match serde_json::from_str::<T>(contents) {
                Ok(_) => Ok(inquire::validator::Validation::Valid),
                Err(e) => Ok(inquire::validator::Validation::Invalid(e.into())),
            }
        })
        .with_file_extension("json")
        .prompt()?;

        *self = serde_json::from_str(&new_config)?;
        Ok(())
    }
}

use day_core::{
    config::Config,
    state::State,
};

#[extension_trait]
pub impl LoadAndSave for State {
    fn load() -> anyhow::Result<Self> where Self : Sized {
        let config = Config::load()?;
        let slices = config
            .slices
            .iter()
            .cloned()
            .filter(|slice| slice.enabled())
            .map(|slice| slice.to_root_slice())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(State {
            config,
            slices,
        })
    }

    fn save(&self) -> anyhow::Result<()> {
        self.config.save()?;
        Ok(())
    }
}

impl LoadAndSave for Config {
    fn load() -> anyhow::Result<Self> where Self : Sized {
        if let Some(contents) = file_contents(&CONFIG_PATH)? {
            serde_json::from_str(&contents).map_err(Into::into)
        } else {
            let config = Self::default();
            Ok(config)
        }
    }

    fn save(&self) -> anyhow::Result<()> {
        std::fs::write(&*CONFIG_PATH, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }
}