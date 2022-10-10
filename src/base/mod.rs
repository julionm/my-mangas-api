mod models;

use actix_web::{
    get,
    post,
    Responder,
    HttpResponse,
    web::{
        ServiceConfig
    }
};

use models::{ Manga, MangaList };

#[get("/my-mangas")]
async fn get_mangas() -> impl Responder {
    let mut mangas: Vec<Manga> = Vec::new();
    mangas.push(Manga { name: "One Piece", src: "https://mangalivre.com.br" });
    mangas.push(Manga { name: "Hanako-kun", src: "https://mangalivre.com.br" });

    let manga_list = MangaList { content: mangas };

    manga_list
}

#[post("/my-mangas")]
async fn post_mangas() -> impl Responder {
    HttpResponse::Ok()
}

pub fn base_service(cfg: &mut ServiceConfig) {
    cfg.service(get_mangas);
    cfg.service(post_mangas);
}