mod cli;

use std::path::PathBuf;

pub use cli::CommitmentCli;
use once_cell::sync::Lazy;

use crate::{config::Configurable, fs::DATA_DIR};
use day_core::modules::commitments::{Commitment, CommitmentState, CommitmentType};
use enum_iterator::all;


impl Configurable for Commitment {
    fn run_configurator(&mut self) -> anyhow::Result<()> {
        *self =
            inquire::Select::<CommitmentType>::new("What sort of event is this?", all().collect())
                .prompt()?
                .into();

        match self {
            Commitment::Event(event) => event.run_configurator(),
            Commitment::Ical(ical) => {
                *ical = inquire::Text::new("What is the url of the ical calendar?").prompt()?;
                Ok(())
            }
        }
    }
}

static COMMITMENT_STATE_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let mut path = DATA_DIR.clone();
    path.push("commitments.json");
    path
});

#[extension_trait]
pub impl CommitmentStateLoad for CommitmentState {
    fn load() -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let contents = crate::fs::file_contents(&COMMITMENT_STATE_PATH)?;
        if let Some(contents) = contents {
            Ok(serde_json::from_str(&contents)?)
        } else {
            Ok(Self::default())
        }
    }

    fn save(&mut self) -> anyhow::Result<()> {
        // Sort todos by id descending, this is the same as ordering by creation date
        self.normalize();
        std::fs::write(&*COMMITMENT_STATE_PATH, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }
}
