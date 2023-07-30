use serde::{Serialize, Deserialize};
use enum_iterator::Sequence;
use std::{fmt::Display, str::FromStr};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Weekly<T> {
    #[serde(default)]
    pub monday: T,
    #[serde(default)]
    pub tuesday: T,
    #[serde(default)]
    pub wednesday: T,
    #[serde(default)]
    pub thursday: T,
    #[serde(default)]
    pub friday: T,
    #[serde(default)]
    pub saturday: T,
    #[serde(default)]
    pub sunday: T,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Sequence, Default)]
pub enum Day {
    #[default]
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Day::Monday => "Monday",
            Day::Tuesday => "Tuesday",
            Day::Wednesday => "Wednesday",
            Day::Thursday => "Thursday",
            Day::Friday => "Friday",
            Day::Saturday => "Saturday",
            Day::Sunday => "Sunday",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Day {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Monday" => Ok(Day::Monday),
            "Tuesday" => Ok(Day::Tuesday),
            "Wednesday" => Ok(Day::Wednesday),
            "Thursday" => Ok(Day::Thursday),
            "Friday" => Ok(Day::Friday),
            "Saturday" => Ok(Day::Saturday),
            "Sunday" => Ok(Day::Sunday),
            _ => Err(()),
        }
    }
}

impl<T> Weekly<T> {
    pub fn get(&self, day: Day) -> &T {
        match day {
            Day::Monday => &self.monday,
            Day::Tuesday => &self.tuesday,
            Day::Wednesday => &self.wednesday,
            Day::Thursday => &self.thursday,
            Day::Friday => &self.friday,
            Day::Saturday => &self.saturday,
            Day::Sunday => &self.sunday,
        }
    }

    pub fn get_mut(&mut self, day: Day) -> &mut T {
        match day {
            Day::Monday => &mut self.monday,
            Day::Tuesday => &mut self.tuesday,
            Day::Wednesday => &mut self.wednesday,
            Day::Thursday => &mut self.thursday,
            Day::Friday => &mut self.friday,
            Day::Saturday => &mut self.saturday,
            Day::Sunday => &mut self.sunday,
        }
    }
}