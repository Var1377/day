use dirs::{config_dir, data_dir};
use once_cell::sync::Lazy;
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