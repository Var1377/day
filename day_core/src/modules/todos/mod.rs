use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TodoState {
    pub todos: Vec<Todo>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TodoConfig;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub name: String,
    pub description: String,
    pub deadline: Option<NaiveDate>,
    pub completed: bool,
}

impl Default for Todo {
    fn default() -> Self {
        Self {
            name: "New Todo".to_string(),
            description: String::new(),
            deadline: None,
            completed: false,
        }
    }
}