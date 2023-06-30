use actix_web::{ get, post, put, web::{ Json }, Error };
use mongodb:: { bson::{doc, oid::ObjectId } };

use crate::models::todo_models::{ ToDoModel };
use crate::db;

#[get("/todo")]
pub async fn get_todos() -> Result<String, Error> {
    Ok(format!("ToDo")) // should be the docs
}

#[post("/todo")]
pub async fn create_todo(todo: Json<ToDoModel>) -> Result<String, Box<dyn std::error::Error>> {
    let doc = doc! { "task": todo.task.clone(), "status": "ACTIVE" };
    let collection = db::get_collection().await?;
    collection.insert_one(doc, None).await?;

    Ok(format!("ToDo")) // should be the inserted mongodb::result::InsertedOne
}

#[put("/todo")]
pub async fn update_todo(todo: Json<ToDoModel>) -> Result<String, Box<dyn std::error::Error>> {
    let filter = doc! { "_id": ObjectId::parse_str(todo.id.clone().as_str())? };
    let update = doc! { "$set": { "task": todo.task.clone(), "status": todo.status.clone() } };
    let collection = db::get_collection().await?;
    collection.update_one(filter, update, None).await?;

    Ok(format!("ToDo")) // should be the updated doc
}
