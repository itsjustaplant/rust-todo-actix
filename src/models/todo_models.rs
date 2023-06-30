use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ToDoModel {
    pub id: String,
    pub task: String,
    pub status: String,
}
