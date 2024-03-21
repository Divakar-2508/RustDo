use crate::Todo;
use sqlx::{
    sqlite::{Sqlite, SqliteConnectOptions, SqliteRow}, Error, Pool
};

const INSERT_QUERY: &str = "INSERT INTO todos (task_id, task, completed) values ($1, $2, $3)";
const SELECT_QUERY: &str = "SELECT * FROM todos";
const DELETE_QUERY: &str = "DELETE FROM todos";
const WHERE_CLAUSE: &str = "WHERE task_id = $1";
const UPDATE_QUERY: &str = "UPDATE todos SET";
const CREATE_QUERY: &str = "
    CREATE TABLE IF NOT EXISTS todos(
        task_id INTEGER PRIMARY KEY NOT NULL,
        task TEXT NOT NULL,
        completed BOOLEAN NOT NULL
);";

pub async fn add_row(pool: &Pool<Sqlite>, todo: Todo) -> Result<(), Error> {
    sqlx::query(INSERT_QUERY)
        .bind(todo.id)
        .bind(todo.name)
        .bind(todo.done)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_all_rows(pool: &Pool<Sqlite>) -> Result<Vec<SqliteRow>, sqlx::Error> {
    sqlx::query(SELECT_QUERY).fetch_all(pool).await
}

pub async fn get_one_row(pool: &Pool<Sqlite>, task_id: i32) -> Result<SqliteRow, sqlx::Error> {
    let query = format!("{} {}", SELECT_QUERY, WHERE_CLAUSE);

    sqlx::query(&query).bind(task_id).fetch_one(pool).await
}

pub async fn delete_row(pool: &Pool<Sqlite>, id: i32) -> Result<String, sqlx::Error> {
    let query = format!("{} {}", DELETE_QUERY, WHERE_CLAUSE);
    let result = sqlx::query(&query).bind(id).execute(pool).await?;

    if result.rows_affected() == 0 {
        return Ok(format!("No Todo with id {}", id));
    } else {
        return Ok("Delete Successful".to_string());
    }
}

pub async fn update_row(pool: &Pool<Sqlite>, id: i32, done: Option<bool>, name: Option<String>) -> Result<String, sqlx::Error> {
    let result = if done.is_some() && name.is_some() {
        let query = format!("{} completed = $2, task = $3 {}", UPDATE_QUERY, WHERE_CLAUSE);

        sqlx::query(&query)
            .bind(id)
            .bind(done.unwrap())
            .bind(name.unwrap())
            .execute(pool).await?
    } else if done.is_some() {
        let query = format!("{} completed=$2 {}", UPDATE_QUERY, WHERE_CLAUSE);

        sqlx::query(&query)
            .bind(id)
            .bind(done.unwrap())
            .execute(pool).await?
    } else {
        let query = format!("{} task=$2 {}", UPDATE_QUERY, WHERE_CLAUSE);

        sqlx::query(&query)
            .bind(id)
            .bind(name.unwrap())
            .execute(pool).await?
    };

    if result.rows_affected() == 0 {
        return Ok(format!("No Todo with id {}", id));
    }
    
    Ok("Update Successful".to_string())
}

pub async fn establish_connection() -> Result<Pool<Sqlite>, sqlx::Error> {
    let database_url = "todo.db";

    let pool_options = SqliteConnectOptions::new()
        .filename(database_url)
        .create_if_missing(true);

    let pool = sqlx::sqlite::SqlitePool::connect_with(pool_options).await?;
    sqlx::query(CREATE_QUERY).execute(&pool).await?;

    Ok(pool)
}
