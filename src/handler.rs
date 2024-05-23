use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;
use crate::{db::{create_task_for_user, create_user_in_db, delete_task_from_db, get_task_for_user, get_tasks_for_user, update_task_in_db}, model::{CreateTask, CreateUser, Db}};


//Create user
pub async fn create_user(db: web::Data<Db>, user: web::Json<CreateUser>) -> impl Responder {
    let user_id = create_user_in_db(&db, &user);
    HttpResponse::Ok().json(user_id)
}

//Create task for user
pub async fn create_task(
    db: web::Data<Db>,
    path: web::Path<Uuid>,
    task: web::Json<CreateTask>,
) -> impl Responder {
    let user_id = path.into_inner();
    match create_task_for_user(&db, user_id, &task) {
        Ok(task_id) => HttpResponse::Ok().json(task_id),
        Err(error_response) => HttpResponse::NotFound().json(error_response),
    }
}

// Get all tasks for given user
pub async fn get_tasks(db: web::Data<Db>, path: web::Path<Uuid>) -> impl Responder {
    let user_id = path.into_inner();
    match get_tasks_for_user(&db, user_id) {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(error_response) => HttpResponse::NotFound().json(error_response),
    }
}

//Get task for given user and task
pub async fn get_task(
    db: web::Data<Db>,
    path: web::Path<(Uuid, Uuid)>,
) -> impl Responder {
    let (user_id, task_id) = path.into_inner();
    match get_task_for_user(&db, user_id, task_id) {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(error_response) => HttpResponse::NotFound().json(error_response),
    }
}

//Update the task
pub async fn update_task(
    db: web::Data<Db>,
    path: web::Path<(Uuid, Uuid)>,
    task: web::Json<CreateTask>,
) -> impl Responder {
    let (user_id, task_id) = path.into_inner();
    match update_task_in_db(&db, user_id, task_id, &task) {
        Ok(updated_task) => HttpResponse::Ok().json(updated_task),
        Err(error_response) => HttpResponse::NotFound().json(error_response),
    }
}

//Delete the task
pub async fn delete_task(
    db: web::Data<Db>,
    path: web::Path<(Uuid, Uuid)>,
) -> impl Responder {
    let (user_id, task_id) = path.into_inner();
    match delete_task_from_db(&db, user_id, task_id) {
        Ok(()) => HttpResponse::Ok().json("Success"),
        Err(error_response) => HttpResponse::NotFound().json(error_response),
    }
}
#[cfg(test)]
mod tests {

    use actix_web::{test, web, App};
    use uuid::Uuid;
    use std::collections::HashMap;
    use crate::handler::create_user;
    use crate::model::{self, CreateUser, User};
    use model::Db;

    #[actix_rt::test]
    async fn test_create_user() {
        // Create a mock database
        let db = web::Data::new(Db::new(HashMap::<Uuid, User>::new()));

        // Create a test application with the create_user route
        let app = test::init_service(
            App::new()
                .app_data(db.clone())
                .route("/users", web::post().to(create_user))
        ).await;

        // Create a test user payload
        let new_user = CreateUser {
            name: "Test User".to_string(),
        };

        // Create a test request
        let req = test::TestRequest::post()
            .uri("/users")
            .set_json(&new_user)
            .to_request();

        // Execute the request and get the response
        let resp = test::call_service(&app, req).await;

        // Check if the response status is OK
        assert!(resp.status().is_success());

        // Deserialize the response body to get the user ID
        let result: Uuid = test::read_body_json(resp).await;

        // Check if the user ID is valid
        assert!(db.read().unwrap().contains_key(&result));
    }

}
