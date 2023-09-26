use super::Commitment;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommitmentState {
    pub commitments: Vec<Commitment>,
    pub icals: Vec<String>,
}

impl CommitmentState {
    pub fn normalize(&mut self) {
        // self.commitments.sort_by_key(|e| e.inner.min_start())
    }
}