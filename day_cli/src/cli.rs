use std::fmt::Display;

use crate::{
    config::{ConfigCli, Configurable},
    modules::commitments::CommitmentCli,
    modules::todo::TodoArgs,
    state::{StateArgs, StateLoad},
};
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use clap::{Parser, Subcommand};
use day_core::{
    state::State,
    time::TimeOfDay,
    weekly::{Day, Weekly},
};
use enum_iterator::all;

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

// impl Configurable for DateTime<Local> {
//     fn run_configurator(&mut self) -> anyhow::Result<()> {
//         let date = inquire::DateSelect::new("Select a date").prompt()?;
//         let time = inquire::CustomType::<TimeOfDay>::new("Select a time").prompt()?;

//         *self = date.and_time(time.into()).and_local_timezone(Local).earliest().expect("Invalid datetime");
//         Ok(())
//     }
// }

#[extension_trait]
pub impl DatetimeConfig for NaiveDateTime {
    fn run_configurator(&mut self, msg: &str) -> Result<(), inquire::InquireError> {
        let date = inquire::DateSelect::new(&format!("{msg} - Date")).with_default(self.date()).prompt()?;
        let time = inquire::CustomType::<TimeOfDay>::new(&format!("{msg} - Time")).with_default(self.time().into()).prompt()?;
        *self = chrono::NaiveDateTime::new(date, time.into());
        Ok(())
    }
}

impl DatetimeConfig for DateTime<Local> {
    fn run_configurator(&mut self,msg: &str,) -> Result<(),inquire::InquireError> {
        let mut naive: NaiveDateTime = self.naive_local();
        naive.run_configurator(msg)?;
        *self = naive.and_local_timezone(Local).earliest().expect("Invalid timezone conversion");
        Ok(())
    }
}