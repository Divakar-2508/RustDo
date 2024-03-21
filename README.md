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

