#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct Constraints {

}

#[serde_inline_default]
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct FlexibleEvent {
    #[serde(default, flatten)]
    pub constraints: Constraints,
}

impl std::fmt::Display for FlexibleEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.constraints)
    }
}

impl std::fmt::Display for Constraints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TODO: Change this at some point")
    }
}