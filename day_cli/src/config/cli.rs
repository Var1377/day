use clap::{Args, Subcommand};

use crate::{
    cli::{Cli, Runnable},
    fs::{file_contents, CONFIG_PATH},
    modules::sleep::cli::SleepArgs,
};

use day_core::{config::Config, state::State};

use super::Configurable;

#[derive(Args, Debug)]
pub struct ConfigCli {
    #[clap(subcommand)]
    subcmd: Option<SubCommand>,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    #[clap(visible_aliases = &["s"])]
    /// configure sleep settings
    Sleep(SleepArgs),
    /// open the config file in your editor
    #[clap(visible_aliases = &["e"])]
    Edit,
    /// print the path to the config file
    #[clap(visible_aliases = &["p"])]
    Path
}

impl Runnable for ConfigCli {
    type Args = Cli;
    fn run(&self, _cli: &Cli, state: &mut State) -> anyhow::Result<()> {
        match &self.subcmd {
            Some(subcmd) => match subcmd {
                SubCommand::Sleep(sleep_args) => sleep_args.run(&(), state)?,
                SubCommand::Edit => {
                    let new_config = inquire::Editor::new(&format!(
                        "Starting editor at {}",
                        CONFIG_PATH.display()
                    ))
                    .with_predefined_text(&serde_json::to_string_pretty(&state.config)?)
                    .with_validator(|contents: &str| {
                        match serde_json::from_str::<Config>(&contents) {
                            Ok(_) => Ok(inquire::validator::Validation::Valid),
                            Err(e) => Ok(inquire::validator::Validation::Invalid(e.into())),
                        }
                    })
                    .with_file_extension("json")
                    .prompt()?;

                    state.config = serde_json::from_str(&new_config)?;
                },
                SubCommand::Path => {
                    println!("Configuration File: {}", CONFIG_PATH.display());
                }
            },
            None => {
                state.config.run_optional_configurator()?;
            }
        };

        state.config.save()?;
        Ok(())
    }
}

#[extension_trait]
pub impl ConfigLoad for Config {
    fn load() -> anyhow::Result<Self> where Self: Sized {
        match file_contents(&CONFIG_PATH)? {
            Some(contents) => {
                let config: Config = serde_json::from_str(&contents)?;
                Ok(config)
            }
            None => {
                if inquire::Confirm::new("No config file found. Create one?")
                    .with_default(true)
                    .prompt()?
                {
                    let mut config = Config::default();
                    config.run_configurator()?;
                    config.save()?;
                    println!(
                        "Config file created at {}, running configurator",
                        CONFIG_PATH.display()
                    );
                    match inquire::Confirm::new("Would you like to continue execution?")
                        .with_default(true)
                        .prompt()?
                    {
                        true => Ok(config),
                        false => Err(anyhow::anyhow!("Execution cancelled")),
                    }
                } else {
                    Err(anyhow::anyhow!("No configuration file found"))
                }
            }
        }
    }

    fn save(&self) -> anyhow::Result<()> {
        let contents = serde_json::to_string_pretty(&self)?;
        std::fs::write(&*CONFIG_PATH, contents)?;
        Ok(())
    }
}
