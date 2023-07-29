use super::SleepSchedule;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Biphasic {}

impl SleepSchedule for Biphasic {}
