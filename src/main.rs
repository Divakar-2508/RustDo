#[macro_use] extern crate rocket;
use std::net::Ipv4Addr;

use rocket::serde::{Deserialize, Serialize};
use rocket::Config;
use rocket::{serde::json::Json, State};

use sqlx::{Pool, Row, Sqlite};

mod db;

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate="rocket::serde")]
struct Todo {
    id: i32,
    name: String,
    done: bool,
}

#[get("/")]
fn get_help() -> String {
    String::from("
        Routes:
            [get]    get_todo/<id> (use without id to get all data)
            [post]   add_todo (with JSON Body {id, name, done}) 
            [delete] delete_todo/<id>
            [get]    `/` - show this message
    ")
}

#[post("/add_todo", format="json", data="<todo>")]
async fn add_todo(todo: Json<Todo>, pool: &State<Pool<Sqlite>>) -> String {
    if let Err(err) = db::add_row(pool, todo.into_inner()).await {
        err.to_string()
    } else {
        String::from("Added Successfully")
    }
} 

#[get("/get_todo")]
async fn get_todos(pool: &State<Pool<Sqlite>>) -> Json<Vec<Todo>> {
    let result = db::get_all_rows(pool).await.unwrap();

    Json(result.into_iter().map(|row| {
        Todo {
            id: row.get("task_id"),
            name: row.get("task"),
            done: row.get("completed")
        }
    }).collect::<Vec<Todo>>())
}

#[get("/get_todo/<id>")]
async fn get_todo(pool: &State<Pool<Sqlite>>, id: i32) -> Result<Json<Todo>, String> {
    let result = db::get_one_row(pool, id).await;

    if let Err(err) = result {
        return Err(format!("No Todo with the Specified Id\nError: {}", err));
    } else {
        let row = result.unwrap();
        return Ok(
            Json(
                Todo { id: row.get("task_id"), name: row.get("task"), done: row.get("completed") }
            )
        );
    }
}

#[delete("/delete_todo/<id>")]
async fn delete_todo(pool: &State<Pool<Sqlite>>, id: i32) -> String {
    match db::delete_row(pool, id).await {
        Err(err) => format!("Error: {}", err),
        Ok(res) => res
    }
}

#[launch]
async fn launch() -> _ {
    let pool = db::establish_connection().await.expect("Can't establish connection");

    let cors = rocket_cors::CorsOptions::default().to_cors().unwrap();

    rocket::build()
        .manage(pool)
        .configure(Config {
            port: 8000,
            address: Ipv4Addr::new(0, 0, 0, 0).into(),
            ..Config::default()
        })
        .attach(cors)
        .mount("/", routes![get_help, add_todo, get_todos, delete_todo, get_todo])
}