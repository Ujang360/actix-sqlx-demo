#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use anyhow::Result as AnyResult;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{query, MySqlPool};
use std::env;
use tapa_trait_serde::IJsonSerializable;

const DB_URI: &str = "DATABASE_URL";

#[derive(Deserialize)]
struct RequestPath {
    id: String,
}

#[derive(Deserialize, IJsonSerializable, Serialize)]
struct UserInfo {
    email: String,
    fullname: Option<String>,
    id: String,
    password: String,
    username: String,
}

#[get("/users/{id}")] // <- define path parameters
async fn get_user_info(
    request_path: web::Path<RequestPath>,
    db_pool: web::Data<MySqlPool>,
) -> impl Responder {
    match query!("SELECT * FROM users WHERE id=?", request_path.id)
        .fetch_one(db_pool.as_ref())
        .await
    {
        Ok(user_info) => HttpResponse::Ok().body(
            UserInfo {
                email: user_info.email,
                fullname: user_info.fullname,
                id: user_info.id,
                password: user_info.password,
                username: user_info.username,
            }
            .to_json_string_pretty(),
        ),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

#[get("/users")] // <- define path parameters
async fn get_all_users(db_pool: web::Data<MySqlPool>) -> impl Responder {
    match query!("SELECT `id` FROM users")
        .fetch_all(db_pool.as_ref())
        .await
    {
        Ok(indices) => HttpResponse::Ok().body(format!("{:#?}", indices)),
        Err(error) => HttpResponse::InternalServerError().body(error.to_string()),
    }
}

#[actix_web::main]
async fn main() -> AnyResult<()> {
    dotenv().ok();
    env_logger::init();

    let db_uri = env::var(DB_URI).unwrap();
    println!("Running with {}={}", DB_URI, db_uri);
    let mysql_pool = MySqlPool::connect(&db_uri).await?;

    HttpServer::new(move || {
        App::new()
            .data(mysql_pool.clone())
            .wrap(Logger::default())
            .service(get_user_info)
            .service(get_all_users)
    })
    .bind("0.0.0:7681")?
    .run()
    .await?;

    Ok(())
}
