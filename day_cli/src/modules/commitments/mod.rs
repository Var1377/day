mod cli;

use std::path::PathBuf;

pub use cli::CommitmentCli;
use once_cell::sync::Lazy;

use crate::{fs::DATA_DIR, table::TableFmt};
use day_core::{
    event::{EventDetails, EventRepetition, FixedTiming, InflexibleEvent},
    modules::commitments::{Commitment, CommitmentState, CustomEventInner},
};

impl TableFmt for Commitment {
    fn headers() -> Vec<&'static str> {
        let mut headers = vec![];

        headers.extend(EventDetails::headers());
        headers.extend(CustomEventInner::headers());
        headers.push("Repeats");
        headers.extend(EventRepetition::headers());

        headers
    }

    fn row(self) -> comfy_table::Row {
        let mut row = comfy_table::Row::new();

        for cell in self.details.row().cell_iter().cloned() {
            row.add_cell(cell);
        }

        for cell in self.inner.row().cell_iter().cloned() {
            row.add_cell(cell);
        }

        row.add_cell(self.repetition.is_some().into());
        self.repetition
            .map(|r| r.row())
            .unwrap_or_default()
            .cell_iter()
            .for_each(|c| {
                row.add_cell(c.clone());
            });

        row
    }
}

impl TableFmt for CustomEventInner {
    fn headers() -> Vec<&'static str> {
        InflexibleEvent::headers()
    }

    fn row(self) -> comfy_table::Row {
        match self {
            CustomEventInner::Inflexible(t) => t.row(),
            CustomEventInner::Flexible(_) => unimplemented!(),
        }
    }
}

impl TableFmt for InflexibleEvent {
    fn headers() -> Vec<&'static str> {
        FixedTiming::headers()
    }

    fn row(self) -> comfy_table::Row {
        self.timing.row()
    }
}

impl TableFmt for FixedTiming {
    fn headers() -> Vec<&'static str> {
        ["Start", "End"].into()
    }

    fn row(self) -> comfy_table::Row {
        [
            self.start.format("%Y-%m-%d %H:%M"),
            self.end.format("%Y-%m-%d %H:%M"),
        ]
        .into()
    }
}

impl TableFmt for EventRepetition {
    fn headers() -> Vec<&'static str> {
        ["Repitition End", "Interval", "Pattern"].into()
    }

    fn row(self) -> comfy_table::Row {
        [
            self.repeat_end
                .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
                .unwrap_or_default(),
            self.interval.to_string(),
            self.pattern.to_string(),
        ]
        .into()
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
