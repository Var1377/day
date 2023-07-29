use super::SleepSchedule;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Dymaxion {}

impl SleepSchedule for Dymaxion {}