use std::fmt::Display;
use crate::time::HourMinute;


#[serde_inline_default]
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Monophasic {
    #[serde_inline_default(5)]
    pub cycles: u8,
}

impl Default for Monophasic {
    fn default() -> Self {
        Self {
            cycles: 5,
        }
    }
}

impl Display for Monophasic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "🌚 Monophasic: {time}", time = HourMinute::from(self.cycles as u32))
    }
}