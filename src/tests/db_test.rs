use uuid::Uuid;

use crate::{db::{create_task_for_user, create_user_in_db, delete_task_from_db, get_task_for_user, get_tasks_for_user, update_task_in_db}, model::{CreateTask, CreateUser, Db, TaskStatus}};



use std::{collections::HashMap, sync:: RwLock};

// Helper function to create a new in-memory database
fn create_db() -> Db {
    RwLock::new(HashMap::new())
}

#[test]
fn test_create_user() {
    let db = create_db();
    let user_data = CreateUser {
        name: String::from("John Doe"),
    };
    let user_id = create_user_in_db(&db, &user_data);
    
    let db_read = db.read().unwrap();
    assert!(db_read.contains_key(&user_id));
    assert_eq!(db_read[&user_id].name, "John Doe");
}

#[test]
fn test_create_task_for_user() {
    let db = create_db();
    let user_data = CreateUser {
        name: String::from("John Doe"),
    };
    let user_id = create_user_in_db(&db, &user_data);

    let task_data = CreateTask {
        title: String::from("New Task"),
        description: String::from("Task Description"),
        due_date: String::from("2023-12-31"),
        status: TaskStatus::ToDo,
    };
    let task_id = create_task_for_user(&db, user_id, &task_data).unwrap();

    let db_read = db.read().unwrap();
    let user = db_read.get(&user_id).unwrap();
    let task = user.tasks.get(&task_id).unwrap();
    assert_eq!(task.title, "New Task");
    assert_eq!(task.description, "Task Description");
}

#[test]
fn test_get_tasks_for_user() {
    let db = create_db();
    let user_data = CreateUser {
        name: String::from("John Doe"),
    };
    let user_id = create_user_in_db(&db, &user_data);

    let task_data = CreateTask {
        title: String::from("New Task"),
        description: String::from("Task Description"),
        due_date: String::from("2023-12-31"),
        status:TaskStatus::ToDo,
    };
    let task_id = create_task_for_user(&db, user_id, &task_data).unwrap();

    let tasks = get_tasks_for_user(&db, user_id).unwrap();
    assert!(tasks.contains_key(&task_id));
}

#[test]
fn test_get_task_for_user() {
    let db = create_db();
    let user_data = CreateUser {
        name: String::from("John Doe"),
    };
    let user_id = create_user_in_db(&db, &user_data);

    let task_data = CreateTask {
        title: String::from("New Task"),
        description: String::from("Task Description"),
        due_date: String::from("2023-12-31"),
        status:TaskStatus::ToDo,
    };
    let task_id = create_task_for_user(&db, user_id, &task_data).unwrap();

    let task = get_task_for_user(&db, user_id, task_id).unwrap();
    assert_eq!(task.title, "New Task");
    assert_eq!(task.description, "Task Description");
}

#[test]
fn test_update_task_in_db() {
    let db = create_db();
    let user_data = CreateUser {
        name: String::from("John Doe"),
    };
    let user_id = create_user_in_db(&db, &user_data);

    let task_data = CreateTask {
        title: String::from("New Task"),
        description: String::from("Task Description"),
        due_date: String::from("2023-12-31"),
        status:TaskStatus::ToDo,
    };
    let task_id = create_task_for_user(&db, user_id, &task_data).unwrap();

    let updated_task_data = CreateTask {
        title: String::from("Updated Task"),
        description: String::from("Updated Description"),
        due_date: String::from("2024-01-31"),
        status:TaskStatus::InProgress,
    };

    let updated_task = update_task_in_db(&db, user_id, task_id, &updated_task_data).unwrap();

    assert_eq!(updated_task.title, "Updated Task");
    assert_eq!(updated_task.description, "Updated Description");
    assert_eq!(updated_task.due_date, "2024-01-31");
    assert_eq!(updated_task.status,TaskStatus::InProgress);
}

#[test]
fn test_delete_task_from_db() {
    let db = create_db();
    let user_data = CreateUser {
        name: String::from("John Doe"),
    };
    let user_id = create_user_in_db(&db, &user_data);

    let task_data = CreateTask {
        title: String::from("New Task"),
        description: String::from("Task Description"),
        due_date: String::from("2023-12-31"),
        status: TaskStatus::ToDo,
    };
    let task_id = create_task_for_user(&db, user_id, &task_data).unwrap();

    delete_task_from_db(&db, user_id, task_id).unwrap();

    let db_read = db.read().unwrap();
    let user = db_read.get(&user_id).unwrap();
    assert!(!user.tasks.contains_key(&task_id));
}

#[test]
fn test_error_handling_user_not_found() {
    let db = create_db();
    let invalid_user_id = Uuid::new_v4();
    
    let task_data = CreateTask {
        title: String::from("New Task"),
        description: String::from("Task Description"),
        due_date: String::from("2023-12-31"),
        status: TaskStatus::ToDo,
    };

    assert!(create_task_for_user(&db, invalid_user_id, &task_data).is_err());
    assert!(get_tasks_for_user(&db, invalid_user_id).is_err());
    assert!(get_task_for_user(&db, invalid_user_id, Uuid::new_v4()).is_err());
    assert!(update_task_in_db(&db, invalid_user_id, Uuid::new_v4(), &task_data).is_err());
    assert!(delete_task_from_db(&db, invalid_user_id, Uuid::new_v4()).is_err());
}

#[test]
fn test_error_handling_task_not_found() {
    let db = create_db();
    let user_data = CreateUser {
        name: String::from("John Doe"),
    };
    let user_id = create_user_in_db(&db, &user_data);
    let invalid_task_id = Uuid::new_v4();
    
    let task_data = CreateTask {
        title: String::from("Updated Task"),
        description: String::from("Updated Description"),
        due_date: String::from("2024-01-31"),
        status: TaskStatus::InProgress,
    };

    assert!(get_task_for_user(&db, user_id, invalid_task_id).is_err());
    assert!(update_task_in_db(&db, user_id, invalid_task_id, &task_data).is_err());
    assert!(delete_task_from_db(&db, user_id, invalid_task_id).is_err());
}