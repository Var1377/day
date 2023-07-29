use super::SleepSchedule;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Everyman {}

impl SleepSchedule for Everyman {}