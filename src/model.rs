use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Serialize, Deserialize)]
pub struct CreateUser {
    pub name: String,
}
#[derive(Serialize, Deserialize, Clone,Debug,PartialEq)]
pub enum TaskStatus {
    ToDo,
    InProgress,
    Done,
}
#[derive(Serialize, Deserialize)]
pub struct CreateTask {
    pub title: String,
    pub description: String,
    pub due_date: String,
    pub status: TaskStatus,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct ErrorResponse {
    pub error: String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub tasks: HashMap<Uuid, Task>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub due_date: String,
    pub status: TaskStatus,
}

pub type Db = RwLock<HashMap<Uuid, User>>;
