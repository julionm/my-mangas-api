use std::io::Result;
use actix_web::{ HttpServer, HttpResponse, App, get, Responder };

#[get("/")]
async fn handle_base() -> impl Responder {
    println!("logando o caminho base");

    HttpResponse::Ok()
}

#[get("/hello")]
async fn handle_hello() -> impl Responder {
    println!("logando o hello!");

    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(handle_base)
            .service(handle_hello)
    })
    .bind(("http://127.0.0.1", 8080))?
    .run()
    .await
}
