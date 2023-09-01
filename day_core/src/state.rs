use crate::{config::Config, modules::{todos::TodoState, commitments::CommitmentState}};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct State {
    pub config: Config,
    pub todo: TodoState,
    pub commitments: CommitmentState,
}