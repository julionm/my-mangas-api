use actix_web::{
    get,
    post,
    Responder,
    HttpResponse,
    web::{
        ServiceConfig
    }
};

#[get("/my-mangas")]
async fn get_mangas() -> impl Responder {
    HttpResponse::Ok()
}

#[post("/my-mangas")]
async fn post_mangas() -> impl Responder {
    HttpResponse::Ok()
}

pub fn base_service(cfg: &mut ServiceConfig) {
    cfg.service(get_mangas);
    cfg.service(post_mangas);
}