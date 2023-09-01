use std::fmt::Display;

use crate::{
    config::{ConfigCli, Configurable},
    modules::commitments::CommitmentCli,
    modules::todo::TodoArgs,
    state::{StateArgs, StateLoad},
};
use chrono::{DateTime, Local, NaiveDateTime};
use clap::{Parser, Subcommand};
use day_core::{
    state::State,
    time::{HourMinute, TimeOfDay},
    weekly::{Day, Weekly},
};
use enum_iterator::all;
use inquire::validator::{CustomTypeValidator, Validation};

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
    after_help = "Day.rs is a command line tool to help you maximise efficiency around your schedule. To see what it can do, run \"day config\""
)]
pub struct Cli {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

impl Cli {
    pub fn run(&self) -> anyhow::Result<()> {
        let mut state = State::load()?;

        match &self.subcmd {
            SubCommand::Config(config_args) => config_args.run(self, &mut state)?,
            SubCommand::Todo(todo_args) => todo_args.run(self, &mut state)?,
            SubCommand::Data(data_args) => data_args.run(self, &mut state)?,
            SubCommand::Commitments(commitment_args) => commitment_args.run(self, &mut state)?,
        };

        state.save()?;
        Ok(())
    }
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    #[clap(visible_aliases = &["cfg"])]
    /// Show or change configuration values
    Config(ConfigCli),

    #[clap(visible_aliases = &["c"])]
    /// Manage your commitments
    Commitments(CommitmentCli),

    #[clap(visible_aliases = &["d"])]
    /// Show and edit all data stored by day
    Data(StateArgs),

    #[clap(visible_aliases = &["t"])]
    /// Manage your todo list
    Todo(TodoArgs),
}

#[derive(Debug)]
struct WeeklyDisplay<T: Display>(Day, T);

impl<T> Display for WeeklyDisplay<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}: {}", self.0, self.1))
    }
}

impl<T> Configurable for Weekly<T>
where
    T: Configurable + Clone + Display,
{
    fn run_configurator(&mut self) -> anyhow::Result<()> {
        let selection = inquire::MultiSelect::new(
            "Which days would you like to configure?",
            all::<Day>()
                .map(|day| WeeklyDisplay(day, self.get(day)))
                .collect(),
        )
        .prompt()?
        .into_iter()
        .map(|day| day.0)
        .collect::<Vec<_>>();

        if let Some(day) = selection.first() {
            let day = *day;
            match day {
                Day::Monday => self.monday.run_configurator()?,
                Day::Tuesday => self.tuesday.run_configurator()?,
                Day::Wednesday => self.wednesday.run_configurator()?,
                Day::Thursday => self.thursday.run_configurator()?,
                Day::Friday => self.friday.run_configurator()?,
                Day::Saturday => self.saturday.run_configurator()?,
                Day::Sunday => self.sunday.run_configurator()?,
            }
            // assign the value of the first day to all the other days
            for days in selection.windows(2) {
                let before = days[0];
                let after = days[1];

                match after {
                    Day::Monday => self.monday = self.get(before).clone(),
                    Day::Tuesday => self.tuesday = self.get(before).clone(),
                    Day::Wednesday => self.wednesday = self.get(before).clone(),
                    Day::Thursday => self.thursday = self.get(before).clone(),
                    Day::Friday => self.friday = self.get(before).clone(),
                    Day::Saturday => self.saturday = self.get(before).clone(),
                    Day::Sunday => self.sunday = self.get(before).clone(),
                }
            }
        };

        Ok(())
    }
}

pub trait CustomConfigurable {
    type Options: Default;
    fn run_configurator(&mut self, msg: &str) -> Result<(), inquire::InquireError> {
        self.run_configurator_with_options(msg, Default::default())
    }

    fn run_configurator_with_options(
        &mut self,
        msg: &str,
        options: Self::Options,
    ) -> Result<(), inquire::InquireError>;
}

#[derive(Debug, Default)]
pub struct DateTimeConfigOptions {
    pub default: Option<NaiveDateTime>,
    pub min: Option<NaiveDateTime>,
    pub max: Option<NaiveDateTime>,
}

#[derive(Debug, Clone)]
pub struct TimeOfDayMinValidator {
    pub min: TimeOfDay,
}

impl CustomTypeValidator<TimeOfDay> for TimeOfDayMinValidator {
    fn validate(&self, input: &TimeOfDay) -> Result<Validation, inquire::CustomUserError> {
        if input < &self.min {
            return Ok(Validation::Invalid("This time is below the minimum value".into()))
        }

        Ok(Validation::Valid)
    }
}

#[derive(Debug, Clone)]
pub struct TimeOfDayMaxValidator {
    pub max: TimeOfDay,
}

impl CustomTypeValidator<TimeOfDay> for TimeOfDayMaxValidator {
    fn validate(&self, input: &TimeOfDay) -> Result<Validation, inquire::CustomUserError> {
        if input > &self.max {
            return Ok(Validation::Invalid("This time is above the maximum value".into()))
        }

        Ok(Validation::Valid)
    }
}

impl CustomConfigurable for NaiveDateTime {
    type Options = DateTimeConfigOptions;
    fn run_configurator_with_options(
        &mut self,
        msg: &str,
        options: Self::Options,
    ) -> Result<(), inquire::InquireError> {
        let default = options.default.unwrap_or(*self);
        let datepicker_message = format!("{msg} - Date");

        let mut datepicker =
            inquire::DateSelect::new(&datepicker_message).with_default(default.date());

        if let Some(min) = options.min {
            datepicker = datepicker.with_min_date(min.date());
        }

        if let Some(max) = options.max {
            datepicker = datepicker.with_min_date(max.date());
        }

        let date = datepicker.prompt() ?;


        let timepicker_message = format!("{msg} - Time");
        let mut timepicker = inquire::CustomType::<TimeOfDay>::new(&timepicker_message)
            .with_default(default.time().into());

        if let Some(min) = options.min {
            if date == min.date() {
                let validator = TimeOfDayMinValidator {
                    min: min.time().into()
                };

                timepicker = timepicker.with_validator(validator);
            }
        }

        if let Some(max) = options.max {
            if date == max.date() {
                let validator = TimeOfDayMaxValidator {
                    max: max.time().into()
                };

                timepicker = timepicker.with_validator(validator);
            }
        }

        let time = timepicker.prompt()?;
        *self = chrono::NaiveDateTime::new(date, time.into());
        Ok(())
    }
}

impl CustomConfigurable for DateTime<Local> {
    type Options = DateTimeConfigOptions;
    fn run_configurator_with_options(
        &mut self,
        msg: &str,
        options: Self::Options,
    ) -> Result<(), inquire::InquireError> {
        let mut naive: NaiveDateTime = self.naive_local();
        naive.run_configurator_with_options(msg, options)?;
        *self = naive
            .and_local_timezone(Local)
            .earliest()
            .expect("Timezone conversion error");
        Ok(())
    }
}

macro_rules! time_config {
    ($type:ty) => {
        impl CustomConfigurable for $type {
            type Options = ();

            fn run_configurator_with_options(
                &mut self,
                msg: &str,
                _options: Self::Options,
            ) -> Result<(), inquire::InquireError> {
                *self = inquire::CustomType::<Self>::new(msg)
                    .with_default(*self)
                    .prompt()?;
                Ok(())
            }
        }
    };
}

time_config!(HourMinute);
time_config!(TimeOfDay);
