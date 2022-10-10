use std::io::Result;
use actix_web::{
    HttpServer,
    App,
    web:: {
        scope,
    }
};

mod base;
mod auth;

use auth::auth_service;
use base::base_service;

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                scope("/api").configure(base_service)
            )
            .service(
                scope("/auth").configure(auth_service)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
