use crate::{config::Config, modules::todos::TodoState};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct State {
    pub config: Config,
    pub todo: TodoState,
}