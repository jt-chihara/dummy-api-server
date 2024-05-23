use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

async fn get_user() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "id": 1,
        "name": "sample"
    }))
}

async fn create_user(user: web::Json<User>) -> impl Responder {
    HttpResponse::Ok().json(user.into_inner())
}

async fn update_user(user: web::Json<User>) -> impl Responder {
    HttpResponse::Ok().json(user.into_inner())
}

async fn delete_user(user_id: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().json(json!({
        "message": format!("User with id {} deleted", user_id)
    }))
}

#[derive(Deserialize, Serialize)]
struct User {
    id: i32,
    name: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
            .route("/user", web::get().to(get_user))
            .route("/user", web::post().to(create_user))
            .route("/user", web::put().to(update_user))
            .route("/user/{id}", web::delete().to(delete_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
