use actix_web::{ 
    post,
    Responder,
    HttpResponse,
    web::{
        ServiceConfig
    }
};

#[post("/login")]
async fn login() -> impl Responder {
    HttpResponse::Ok()
}

pub fn auth_service(cfg: &mut ServiceConfig) {
    cfg.service(login);
}
