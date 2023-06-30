use actix_web::{ HttpServer, App };

use crate::endpoints::todo_endpoints::{ get_todos, create_todo, update_todo };

pub async fn start_server() -> std::io::Result<()>{
  println!("Starting Server");
  HttpServer::new(move || {
      App::new()
          .service(get_todos)
          .service(create_todo)
          .service(update_todo)
  })
  .bind(("127.0.0.1", 8081))?
  .run()
  .await
}