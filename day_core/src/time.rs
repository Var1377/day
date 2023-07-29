use chrono::{DateTime, Local, Timelike, Utc};
use serde::{Deserialize, Serialize};
use sscanf::scanf;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeOfDay(
    /// hour
    pub u32,
    /// minute
    pub u32,
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
/// The same thing as TimeOfDay, but without normalization
pub struct HourMinute(
    /// hour
    pub u32,
    /// minute
    pub u32,
);

macro_rules! impls {
    ($($type:ty),*) => {
        $(
            impl Serialize for $type {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    return serializer.serialize_str(&format!("{hr:02}:{min:02}", hr = self.0, min = self.1));
                }
            }

            impl<'de> Deserialize<'de> for $type {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    String::deserialize(deserializer)?.parse().map_err(serde::de::Error::custom)
                }
            }

            impl Display for $type {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    f.write_fmt(format_args!("{:02}:{:02}", self.0, self.1))
                }
            }

            // impl $type {
            //     pub fn prompt(msg: &str) -> anyhow::Result<Self> {
            //         let time = inquire::Text::new(&format!("{msg} (hh:mm)"))
            //             .with_validator(|s: &str| match s.parse::<TimeOfDay>() {
            //                 Ok(_) => Ok(inquire::validator::Validation::Valid),
            //                 Err(e) => Ok(inquire::validator::Validation::Invalid(e.into())),
            //             })
            //             .prompt()?;
            
            //         // unwrap is safe because of the validator
            //         Ok(time.parse().unwrap())
            //     }
            
            //     pub fn prompt_with_default(msg: &str, default: Self) -> anyhow::Result<Self> {
            //         let time = inquire::Text::new(&format!("{msg} (hh:mm)"))
            //             .with_default(&default.to_string())
            //             .with_validator(|s: &str| match s.parse::<TimeOfDay>() {
            //                 Ok(_) => Ok(inquire::validator::Validation::Valid),
            //                 Err(e) => Ok(inquire::validator::Validation::Invalid(e.into())),
            //             })
            //             .prompt()?;
            
            //         // unwrap is safe because of the validator
            //         Ok(time.parse().unwrap())
            //     }
            // }
        )*
    };
}

impls!(TimeOfDay, HourMinute);

impl From<(u32, u32)> for HourMinute {
    fn from(t: (u32, u32)) -> Self {
        Self(t.0, t.1)
    }
}

impl FromStr for TimeOfDay {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match scanf!(s, "{}:{}", u32, u32) {
            Ok((hr, min)) => Self::new(hr, min),
            Err(_) => Err("TimeOfDay couldn't be deserialized, use hh:mm"),
        }
    }
}

impl FromStr for HourMinute {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match scanf!(s, "{}:{}", u32, u32) {
            Ok((hr, min)) => Ok(Self(hr, min)),
            Err(_) => Err("HourMinute couldn't be deserialized, use hh:mm"),
        }
    }
}

impl From<DateTime<Local>> for TimeOfDay {
    fn from(t: DateTime<Local>) -> Self {
        let time = t.time();
        Self(time.hour(), time.minute())
    }
}

impl From<DateTime<Utc>> for TimeOfDay {
    fn from(t: DateTime<Utc>) -> Self {
        t.with_timezone(&Local).into()
    }
}



impl std::ops::Add for TimeOfDay {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.1 + rhs.1 >= 60 {
            Self((self.0 + rhs.0 + 1) % 24, self.1 + rhs.1 - 60)
        } else {
            Self((self.0 + rhs.0) % 24, self.1 + rhs.1)
        }
    }
}

impl std::ops::Sub for TimeOfDay {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut hour = if self.0 < rhs.0 {
            self.0 + 24 - rhs.0
        } else {
            self.0 - rhs.0
        };

        let min = if self.1 < rhs.1 {
            hour -= 1;
            self.1 + 60 - rhs.1
        } else {
            self.1 - rhs.1
        };

        Self(hour % 24, min % 60)
    }
}

impl TimeOfDay {
    pub fn new(hr: u32, min: u32) -> Result<Self, &'static str> {
        if hr > 24 || min > 60 {
            Err("invalid time")
        } else {
            Ok(Self(hr, min))
        }
    }

    /// Rounds up to the nearest 5 minutes
    pub fn round_up(&self) -> Self {
        let mut min = self.1;
        let mut hr = self.0;

        if min % 5 == 0 {
            return *self;
        }

        min += 5 - (min % 5);

        if min == 60 {
            min = 0;
            hr += 1;
        }

        Self(hr, min).correct()
    }

    /// Rounds down to the nearest 5 minutes
    pub fn round_down(&self) -> Self {
        let mut min = self.1;
        let hr = self.0;

        min -= min % 5;

        Self(hr, min).correct()
    }

    /// Rounds to the nearest 5 minutes
    pub fn round(&self) -> Self {
        if self.1 % 5 > 2 {
            self.round_up()
        } else {
            self.round_down()
        }
    }

    pub fn correct(&self) -> Self {
        Self(self.0 % 24, self.1 % 60)
    }

}

impl Default for TimeOfDay {
    fn default() -> Self {
        Local::now().into()
    }
}

