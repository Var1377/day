#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct Constraints {

}

#[serde_inline_default]
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct FlexibleEvent {
    #[serde(default, flatten)]
    constraints: Option<Constraints>,
}

impl std::fmt::Display for FlexibleEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.constraints {
            Some(c) => write!(f, "{c}"),
            None => write!(f, "No constraints")
        }
    }
}

impl std::fmt::Display for Constraints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TODO: Change this at some point")
    }
}