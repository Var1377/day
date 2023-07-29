use super::SleepSchedule;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Uberman {}

impl SleepSchedule for Uberman {}