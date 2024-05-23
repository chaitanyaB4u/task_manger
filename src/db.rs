use std::collections::HashMap;

use uuid::Uuid;

use crate::model::{CreateTask, CreateUser, Db, ErrorResponse, Task, User};

pub fn create_user_in_db(db: &Db, user_data: &CreateUser) -> Uuid {
    let mut db = db.write().unwrap();
    let user_id = Uuid::new_v4();
    let new_user = User {
        id: user_id,
        name: user_data.name.clone(),
        tasks: HashMap::new(),
    };
    db.insert(user_id, new_user);
    user_id
}
pub fn create_task_for_user(db: &Db, user_id: Uuid, task_data: &CreateTask) -> Result<Uuid, ErrorResponse> {
    let mut db = db.write().unwrap();
    
    if let Some(user) = db.get_mut(&user_id) {
        let task_id = Uuid::new_v4();
        let new_task = Task {
            id: task_id,
            title: task_data.title.clone(),
            description: task_data.description.clone(),
            due_date: task_data.due_date.clone(),
            status: task_data.status.clone(),
        };
        user.tasks.insert(task_id, new_task);
        return Ok(task_id);
    } 
        Err(ErrorResponse {
            error: format!("User with id {} not found", user_id),
        })
}
pub fn get_tasks_for_user(db: &Db, user_id: Uuid) -> Result<HashMap<Uuid, Task>, ErrorResponse> {
    let db = db.read().unwrap();
    
    if let Some(user) = db.get(&user_id) {
        return Ok(user.tasks.clone());
    } 
        Err(ErrorResponse {
            error: format!("User with id {} not found", user_id),
        })
    
}
pub fn get_task_for_user(db: &Db, user_id: Uuid, task_id: Uuid) -> Result<Task, ErrorResponse> {
    let db = db.read().unwrap();
    
    if let Some(user) = db.get(&user_id) {
        if let Some(task) = user.tasks.get(&task_id) {
            Ok(task.clone())
        } else {
            Err(ErrorResponse {
                error: format!("Task with id {} not found", task_id),
            })
        }
    } else {
        Err(ErrorResponse {
            error: format!("User with id {} not found", user_id),
        })
    }
}

pub fn update_task_in_db(
    db: &Db,
    user_id: Uuid,
    task_id: Uuid,
    task_data: &CreateTask,
) -> Result<Task, ErrorResponse> {
    let mut db = db.write().unwrap();
    
    if let Some(user) = db.get_mut(&user_id) {
        if let Some(existing_task) = user.tasks.get_mut(&task_id) {
            existing_task.title = task_data.title.clone();
            existing_task.description = task_data.description.clone();
            existing_task.due_date = task_data.due_date.clone();
            existing_task.status = task_data.status.clone();
            Ok(existing_task.clone())
        } else {
            Err(ErrorResponse {
                error: format!("Task with id {} not found", task_id),
            })
        }
    } else {
        Err(ErrorResponse {
            error: format!("User with id {} not found", user_id),
        })
    }
}

pub fn delete_task_from_db(
    db: &Db,
    user_id: Uuid,
    task_id: Uuid,
) -> Result<(), ErrorResponse> {
    let mut db = db.write().unwrap();
    
    if let Some(user) = db.get_mut(&user_id) {
        if user.tasks.remove(&task_id).is_some() {
            Ok(())
        } else {
            Err(ErrorResponse {
                error: format!("Task with id {} not found", task_id),
            })
        }
    } else {
        Err(ErrorResponse {
            error: format!("User with id {} not found", user_id),
        })
    }
}

