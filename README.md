## RustDo

RustDo is a simple yet powerful Todo API built using the Rust programming language and the Rocket web framework. This API allows users to manage their todo lists effortlessly, including adding, viewing, updating, and deleting tasks.

## Usage

- To add a new todo, make a POST request to `/add_todo` with the following JSON payload:
  ```json
  {
    "id": 1,
    "name": "Buy groceries",
    "done": false
  }
  ```
- To get all todos, make a GET request to `/get_todo`.

- To get a specific todo, make a GET request to `/get_todo/{id}` where `{id}` is the ID of the todo.

- To delete a todo, make a DELETE request to `/delete_todo/{id}` where `{id}` is the ID of the todo.

