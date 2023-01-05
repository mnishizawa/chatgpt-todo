extern crate actix;
extern crate actix_web;
extern crate env_logger;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_postgres;

use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::Arc;
//use serde_json::json;
use tokio_postgres::{Client, NoTls};

#[derive(Serialize, Deserialize)]
struct Todo {
    id: i32,
    title: String,
    completed: bool,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let (client, connection) =
        tokio_postgres::connect("postgresql://postgres:example@localhost/todos", NoTls)
            .await
            .unwrap();

    tokio::spawn(async move {
        connection.await.unwrap();
    });

    let client = Arc::new(client);

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(client.clone()))
            .route("/todos", web::get().to(get_todos))
            .route("/todos", web::post().to(create_todo))
            .route("/todos/{id}", web::delete().to(delete_todo))
            .route("/todos/{id}", web::patch().to(update_todo))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

async fn get_todos(client: web::Data<Arc<Client>>) -> impl Responder {
    let mut todos = Vec::new();
    let statement = "SELECT id, title, completed FROM todos";
    let rows = client.query(statement, &[]).await.unwrap();
    for row in rows {
        let todo = Todo {
            id: row.get(0),
            title: row.get(1),
            completed: row.get(2),
        };
        todos.push(todo);
    }
    HttpResponse::Ok().json(todos)
}

async fn create_todo(client: web::Data<Arc<Client>>, todo: web::Json<Todo>) -> impl Responder {
    let statement = "INSERT INTO todos (title, completed) VALUES ($1, $2) RETURNING id";
    let rows = client
        .query(statement, &[&todo.title, &todo.completed])
        .await
        .unwrap();
    let id: i32 = rows[0].get(0);
    let new_todo = Todo {
        id,
        ..todo.into_inner()
    };
    HttpResponse::Ok().json(new_todo)
}

async fn update_todo(
    client: web::Data<Arc<Client>>,
    path: web::Path<i32>,
    todo: web::Json<Todo>,
) -> impl Responder {
    let statement = "UPDATE todos SET title = $1, completed = $2 WHERE id = $3";
    client
        .query(
            statement,
            &[&todo.title, &todo.completed, &path.into_inner()],
        )
        .await
        .unwrap();
    HttpResponse::Ok().json(todo.into_inner())
}

async fn delete_todo(client: web::Data<Arc<Client>>, path: web::Path<i32>) -> impl Responder {
    let statement = "DELETE FROM todos WHERE id = $1";
    client
        .query(statement, &[&path.into_inner()])
        .await
        .unwrap();
    HttpResponse::Ok().finish()
}
