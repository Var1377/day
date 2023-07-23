use std::path::Display;

use self::{
    biphasic::Biphasic, dymaxion::Dymaxion, everyman::Everyman, monophasic::Monophasic,
    uberman::Uberman,
};

mod biphasic;
mod cli;
mod dymaxion;
mod everyman;
mod monophasic;
mod uberman;

pub struct Sleep {
    monophasic: Monophasic,
    biphasic: Biphasic,
    everyman: Everyman,
    uberman: Uberman,
    dymaxion: Dymaxion,

    pub sleep_schedule: SleepScheduleType,
}

pub trait SleepSchedule {}

custom_derive!{
    #[derive(Debug, IterVariants(SleepScheduleTypeVariants), EnumDisplay)]
    pub enum SleepScheduleType {
        Monophasic,
        Biphasic,
        Everyman,
        Uberman,
        Dymaxion,
    }
}