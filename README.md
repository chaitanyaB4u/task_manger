Task Management Web Application in Rust with Actix-web
------------------------------------------------------

This document describes a simple task management web application built using Rust and the Actix-web framework.

### Prerequisites

*   Rust (>=1.74)
    

### Setup and Running

1.  Ensure you have Rust installed (version >= 1.74).
    
2.  Clone or download the project repository.
    
3.  Navigate to the project directory in your terminal.
    
4.  Build and run the application:
    
```sh
   cargo build  
   cargo run   
```
The application will be accessible at http://127.0.0.1:8080.

### API Endpoints

The application offers various API endpoints for user and task management:

*   **Create User (POST /users):**
    
    *   Request Body: JSON object with a "name" field (e.g., {"name": "John Doe"})
        
*   **Create Task (POST /users/{user\_id}/tasks):**
    
    *   Path Parameter: {user\_id} - ID of the user for whom the task is created.
        
    *   Request Body: JSON object with the following fields:
        
        *   "title": Title of the task.
            
        *   "description": Description of the task (optional).
            
        *   "due\_date": Due date of the task in YYYY-MM-DD format (optional).
            
        *   "status": Status of the task (e.g., "ToDo", "InProgress", "Done").
            
*   **List Tasks (GET /users/{user\_id}/tasks):**
    
    *   Path Parameter: {user\_id} - ID of the user for whom to list tasks.
        
*   **Get Task (GET /users/{user\_id}/tasks/{task\_id}):**
    
    *   Path Parameters:
        
        *   {user\_id} - ID of the user who owns the task.
            
        *   {task\_id} - ID of the specific task to retrieve.
            
*   **Update Task (PUT /users/{user\_id}/tasks/{task\_id}):**
    
    *   Path Parameters: Same as Get Task.
        
    *   Request Body: JSON object with fields to update (e.g., title, description, due\_date, status).
        
*   **Delete Task (DELETE /users/{user\_id}/tasks/{task\_id}):**
    
    *   Path Parameters: Same as Get Task.
        

### Example Usage

**Creating a User:**
```sh
curl -X POST -H "Content-Type: application/json" -d '{"name": "John Doe"}' http://127.0.0.1:8080/users   
```
**Creating a Task for a User:**

1.  Replace \<the user ID> with the actual ID obtained from the previous request.
    
2.  Run the following command:
    
```sh
curl -X POST -H "Content-Type: application/json" -d '{ "title": "New Task", "description": "Task Description", "due_date": "2023-12-31", "status": "ToDo" }' http://127.0.0.1:8080/users/<the user ID>/tasks
```
**Listing Tasks for a User:**

1. Replace \<the user ID> with the actual user ID.
    
2.  Run the following command:
    
```sh
  curl http://127.0.0.1:8080/users/<the user ID>/tasks
  ```

**(Replace and accordingly in the following examples)**

*   **Getting a Specific Task:**
    
```sh
curl http://127.0.0.1:8080/users/<the user ID>/tasks/<the task ID>
```

*   **Updating a Task:**
```sh
curl -X PUT -H "Content-Type: application/json" -d '{ "title": "Updated Task", "description": "Updated Description", "due_date": "2024-01-31", "status": "In Progress" }' http://127.0.0.1:8080/users/<the user ID>/tasks/<the task ID>
```

*   **Deleting a Task:**
    
```sh
curl -X DELETE http://127.0.0.1:8080/users/<the user ID>/tasks/<the task ID>
```

### Testing

To run the test suite:
```sh
cargo test
```

### License

This project is licensed under the MakerStudio License. Refer to the LICENSE file for details.

### Acknowledgements

*   Actix-web: The powerful web framework for Rust used in this project.
    
*   Rust: The programming language
