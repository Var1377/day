use crate::{config::ConfigCli, state::StateLoad};
use clap::{Parser, Subcommand};
use day_core::{time::{HourMinute, TimeOfDay}, state::State};

pub trait Runnable {
    type Args;
    fn run(&self, args: &Self::Args, state: &mut State) -> anyhow::Result<()>;
}

#[derive(Parser, Debug)]
#[clap(
    name = "day.rs",
    version = "0.1.0",
    author = "Varun Latthe (Var1337)",
    about,
    after_help = "Day.rs is a command line tool to help you maximise efficiency around an already packed day. To see what it can do, run `day config`"
)]
pub struct Cli {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

impl Cli {
    pub fn run(&self) -> anyhow::Result<()> {
        let mut state = State::load()?;

        match &self.subcmd {
            SubCommand::Config(config_args) => config_args.run(&self, &mut state),
        }
    }
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    #[clap(visible_aliases = &["cfg", "c"])]
    /// Show or change configuration values
    Config(ConfigCli),
}

#[extension_trait::extension_trait]
pub impl HourMinuteCliExt for HourMinute {
    fn prompt_default_now(msg: &str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Self::prompt_with_default(msg, Self::default())
    }

    fn prompt(msg: &str) -> anyhow::Result<Self> where Self: Sized {
        let time = inquire::Text::new(&format!("{msg} (hh:mm)"))
            .with_validator(|s: &str| match s.parse::<HourMinute>() {
                Ok(_) => Ok(inquire::validator::Validation::Valid),
                Err(e) => Ok(inquire::validator::Validation::Invalid(e.into())),
            })
            .prompt()?;

        // unwrap is safe because of the validator
        Ok(time.parse().unwrap())
    }

    fn prompt_with_default(msg: &str, default: Self) -> anyhow::Result<Self> where Self: Sized {
        let time = inquire::Text::new(&format!("{msg} (hh:mm)"))
            .with_default(&default.to_string())
            .with_validator(|s: &str| match s.parse::<HourMinute>() {
                Ok(_) => Ok(inquire::validator::Validation::Valid),
                Err(e) => Ok(inquire::validator::Validation::Invalid(e.into())),
            })
            .prompt()?;

        // unwrap is safe because of the validator
        Ok(time.parse().unwrap())
    }
}

#[extension_trait::extension_trait]
pub impl TimeOfDayCliExt for TimeOfDay {
    fn prompt(msg: &str) -> anyhow::Result<Self> where Self: Sized {
        let time = inquire::Text::new(&format!("{msg} (hh:mm)"))
            .with_validator(|s: &str| match s.parse::<TimeOfDay>() {
                Ok(_) => Ok(inquire::validator::Validation::Valid),
                Err(e) => Ok(inquire::validator::Validation::Invalid(e.into())),
            })
            .prompt()?;

        // unwrap is safe because of the validator
        Ok(time.parse().unwrap())
    }

    fn prompt_with_default(msg: &str, default: Self) -> anyhow::Result<Self> where Self: Sized {
        let time = inquire::Text::new(&format!("{msg} (hh:mm)"))
            .with_default(&default.to_string())
            .with_validator(|s: &str| match s.parse::<TimeOfDay>() {
                Ok(_) => Ok(inquire::validator::Validation::Valid),
                Err(e) => Ok(inquire::validator::Validation::Invalid(e.into())),
            })
            .prompt()?;

        // unwrap is safe because of the validator
        Ok(time.parse().unwrap())
    }
}
