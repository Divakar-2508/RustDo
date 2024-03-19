use sqlx::{postgres::{PgPoolOptions, PgRow}, Error, Pool, Postgres};

use crate::Todo;

const INSERT_QUERY: &str = "INSERT INTO todos (task_id, task, completed) values ($1, $2, $3)";
const SELECT_QUERY: &str = "SELECT * FROM todos";
const DELETE_QUERY: &str = "DELETE FROM todos";
const WHERE_CLAUSE: &str = "WHERE task_id = $1";

pub async fn add_row(pool: &Pool<Postgres>, todo: Todo) -> Result<(), Error> {
    sqlx::query(INSERT_QUERY)
        .bind(todo.id)
        .bind(todo.name)
        .bind(todo.done)
        .execute(pool).await?;
    Ok(())
}

pub async fn get_all_rows(pool: &Pool<Postgres>) -> Result<Vec<PgRow>, sqlx::Error> {
    sqlx::query(SELECT_QUERY)
        .fetch_all(pool)
        .await
}

pub async fn get_one_row(pool: &Pool<Postgres>, task_id: i32) -> Result<PgRow, sqlx::Error> {
    let query = format!("{} {}", SELECT_QUERY, WHERE_CLAUSE);

    sqlx::query(&query)
        .bind(task_id)
        .fetch_one(pool)
        .await
}

pub async fn delete_row(pool: &Pool<Postgres>, id: i32) -> Result<String, sqlx::Error> {
    let query = format!("{} {}", DELETE_QUERY, WHERE_CLAUSE);
    let result = sqlx::query(&query)
        .bind(id)
        .execute(pool)
        .await?;     

    if result.rows_affected() == 0 {
        return Ok(format!("No Todo with id {}", id));
    } else {
        return Ok("Delete Successful".to_string());
    }
}

pub async fn establish_connection() -> Result<Pool<Postgres>, sqlx::Error> {
    let database_url = "postgresql://postgres:sHARINGAN@1@localhost/todos";

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await?;

    Ok(pool)
}