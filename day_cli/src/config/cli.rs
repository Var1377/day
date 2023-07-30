use clap::{Args, Subcommand};

use crate::{
    cli::{Cli, Runnable},
    fs::{file_contents, CONFIG_PATH, JsonEditable},
    modules::sleep::SleepArgs,
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
    /// Configure sleep settings
    Sleep(SleepArgs),
    /// Open the config file in your editor
    #[clap(visible_aliases = &["e"])]
    Edit,
    /// Print the path to the config file
    #[clap(visible_aliases = &["p"])]
    Path,
}

impl Runnable for ConfigCli {
    type Args = Cli;
    fn run(&self, _cli: &Cli, state: &mut State) -> anyhow::Result<()> {
        let save = match &self.subcmd {
            Some(subcmd) => match subcmd {
                SubCommand::Sleep(sleep_args) => {
                    sleep_args.run(&(), state)?;
                    true
                }
                SubCommand::Edit => {
                    state.config.run_editor(&format!(
                        "Starting editor at {}",
                        CONFIG_PATH.display()
                    ))?;
                    true
                }
                SubCommand::Path => {
                    println!("{}", CONFIG_PATH.display());
                    false
                }
            },
            None => {
                state.config.run_optional_configurator()?;
                true
            }
        };
        if save {
            state.config.save()?;
            println!("Configuration saved to {}", CONFIG_PATH.display());
        }
        Ok(())
    }
}

#[extension_trait]
pub impl ConfigLoad for Config {
    fn load() -> anyhow::Result<Self>
    where
        Self: Sized,
    {
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
                    println!(
                        "Config file created at {} with defaults",
                        CONFIG_PATH.display()
                    );
                    match inquire::Confirm::new("Would you like to configure your settings?")
                        .with_default(true)
                        .prompt()?
                    {
                        true => config.run_configurator()?,
                        false => (),
                    };
                    config.save()?;
                    match inquire::Confirm::new("Would you like to continue with your command?")
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