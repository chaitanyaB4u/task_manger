use actix_web::{web, App,  HttpServer};
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::RwLock;
mod model;
use model::User;
use crate::handler::{create_task, create_user, delete_task, get_task, get_tasks, update_task};
mod handler;
mod db;
mod tests;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = web::Data::new(RwLock::new(HashMap::<Uuid, User>::new()));
    println!("Server is running at: 127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .route("/users", web::post().to(create_user))
            .route("/users/{user_id}/tasks", web::post().to(create_task))
            .route("/users/{user_id}/tasks", web::get().to(get_tasks))
            .route("/users/{user_id}/tasks/{task_id}", web::get().to(get_task))
            .route("/users/{user_id}/tasks/{task_id}", web::put().to(update_task))
            .route("/users/{user_id}/tasks/{task_id}", web::delete().to(delete_task))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}



