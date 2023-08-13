use super::task::{Task, CompletedTask};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TodoState {
    #[serde(default)]
    pub todos: Vec<Task>,
    #[serde(default)]
    pub completed: Vec<CompletedTask>,
}

impl TodoState {
    pub fn normalize(&mut self) {
        self.todos.sort();
        self.completed.sort();
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TodoConfig;

