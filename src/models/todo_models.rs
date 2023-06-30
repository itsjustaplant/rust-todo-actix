use serde::Deserialize;

#[derive(Debug)]
#[derive(Deserialize)]
pub struct ToDoModel {
  pub id: String,
  pub task: String,
  pub status: String
}
