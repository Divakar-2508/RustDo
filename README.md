## Usage

- To add a new todo, make a POST request to `/todos` with the following JSON payload:
  ```json
  {
    "id": 1,
    "name": "Buy groceries",
    "done": false
  }
  ```

- To get all todos, make a GET request to `/todos`.

- To get a specific todo, make a GET request to `/todos/{id}` where `{id}` is the ID of the todo.

- To delete a todo, make a DELETE request to `/todos/{id}` where `{id}` is the ID of the todo.
