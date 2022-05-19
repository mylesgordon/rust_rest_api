mod types;

use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use std::io::Result;
use types::SimpleMessage;

#[delete("/messages")]
async fn delete_messages(_path: web::Path<(u32,)>) -> impl Responder {
    HttpResponse::Ok()
}

#[get("/messages")]
async fn get_messages() -> impl Responder {
    let res = SimpleMessage {
        message: "GET all messages response from server".to_string(),
    };
    HttpResponse::Ok().json(web::Json(res))
}

#[get("/messages/{id}")]
async fn get_messages_with_id(path: web::Path<(u32,)>) -> impl Responder {
    println!("Message {} requested", path.into_inner().0);

    let res = SimpleMessage {
        message: "GET a single message from server".to_string(),
    };
    HttpResponse::Ok().json(web::Json(res))
}

#[post("/messages")]
async fn post_messages(body: web::Json<SimpleMessage>) -> impl Responder {
    println!("Recieved message: {}", body.message);
    HttpResponse::Created()
}

#[put("/messages/{id}")]
async fn put_messages(path: web::Path<(u32,)>, body: web::Json<SimpleMessage>) -> impl Responder {
    println!(
        "Message {} updated, with body {}",
        path.into_inner().0,
        body.message
    );
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(delete_messages)
            .service(get_messages)
            .service(get_messages_with_id)
            .service(post_messages)
            .service(put_messages)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
