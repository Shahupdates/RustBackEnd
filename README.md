# RustBackEnd
![bf1ba39ab2b825164931a7ed2917544e](https://github.com/Shahupdates/RustBackEnd/assets/120000782/2a66c7cb-7d01-4ab1-9716-08a654cd54a2)

# ActixWebTaskService

ActixWebTaskService is a task management system built with Actix Web and AWS's DynamoDB. It allows for the creation, retrieval, and state transition of tasks.

## Features

- **Task Creation**: Submit a new task with a user ID, task type, and source file.
- **Task Retrieval**: Get the details of an existing task using its global ID.
- **Task State Transition**: Transition the state of an existing task to a new state.

## API Endpoints

- `GET /task/{task_global_id}`: Retrieves a task using its global ID.
- `POST /task`: Submits a new task with a user ID, task type, and source file.

## Task States

A task can be in one of several states, and each state has rules about which states it can transition to.

## Repository

The `DDBRepository` is responsible for interacting with DynamoDB to persist and retrieve tasks.

## Error Handling

The system has a well-defined set of errors, each with its own HTTP status code. For example, if a task is not found, a 404 status code is returned.

## Running the Application

To run the application, use the following command:

```bash
cargo run
```

The application will start on http://127.0.0.1:80.
