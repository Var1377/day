use crate::event::FixedTiming;

pub struct Schedule {
    events: Vec<FixedTiming>,
}

impl From<Vec<FixedTiming>> for Schedule {
    fn from(mut events: Vec<FixedTiming>) -> Self {
        events.sort();
        Self {
            events,
        }
    }
}

impl IntoIterator for Schedule {
    type Item = FixedTiming;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.events.into_iter()
    }
}