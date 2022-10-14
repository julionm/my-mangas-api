use actix_web::{ 
    post,
    Responder,
    HttpResponse,
    web::{
        Json,
        ServiceConfig
    }
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Auth {
    username: String,
    password: String
}

#[post("/login")]
async fn login(auth: Json<Auth>) -> impl Responder {

    if auth.username == String::from("julionm") {
        println!("Everything is great! You're logged!")
    }

    HttpResponse::Ok()
}

pub fn auth_service(cfg: &mut ServiceConfig) {
    cfg.service(login);
}
