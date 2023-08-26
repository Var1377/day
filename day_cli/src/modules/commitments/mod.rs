mod cli;

pub use cli::CommitmentCli;

use crate::config::Configurable;
use day_core::modules::commitments::{Commitment, CommitmentType};
use enum_iterator::all;

impl Configurable for Commitment {
    fn run_configurator(&mut self) -> anyhow::Result<()> {
        *self = inquire::Select::<CommitmentType>::new("What sort of event is this?", all().collect())
            .prompt()?.into();
        
        match self {
            Commitment::Event(event) => event.run_configurator(),
            Commitment::Ical(ical) => {
                *ical = inquire::Text::new("What is the url of the ical calendar?").prompt()?;
                Ok(())
            }
        }
    }
}
