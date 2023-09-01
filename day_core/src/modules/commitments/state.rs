use super::Commitment;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommitmentState {
    pub commitments: Vec<Commitment>,
}

impl CommitmentState {
    pub fn normalize(&mut self) {
        
    }
}