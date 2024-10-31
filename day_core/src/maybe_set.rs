use serde::{Deserialize, Deserializer, Serialize};

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Serialize, Default)]
#[serde(transparent)]
pub struct MaybeSet<T> (
    #[serde(deserialize_with = "deserialize_some", bound(deserialize = "T: Deserialize<'de>", serialize = "T : Serialize"), skip_serializing_if = "Option::is_none")]
    pub Option<Option<T>>,
);

// Any value that is present is considered Some value, including null.
fn deserialize_some<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where T: Deserialize<'de>,
          D: Deserializer<'de>
{
    Deserialize::deserialize(deserializer).map(Some)
}

impl<T> MaybeSet<T> {
    pub fn is_set(&self) -> bool {
        self.0.is_some()
    }

    pub fn is_not_set(&self) -> bool {
        self.0.is_none()
    }

    pub fn flatten(self) -> Option<T> {
        self.0.flatten()
    }
}