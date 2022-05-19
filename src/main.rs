mod auth_middleware;
mod routes;
mod types;

use actix_web::{App, HttpServer};
use auth_middleware::Authorization;
use routes::*;
use std::io::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(delete_messages)
            .service(get_messages)
            .service(get_messages_with_id)
            .service(post_messages)
            .service(put_messages)
            .wrap(Authorization)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
