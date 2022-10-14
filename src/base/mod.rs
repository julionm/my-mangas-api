mod models;

use actix_web::{
    get,
    post,
    Responder,
    HttpResponse,
    web::{
        Json,
        ServiceConfig
    }
};

use models::{ Manga, MangaList };

#[get("/my-mangas")]
async fn get_mangas() -> impl Responder {
    let mut mangas: Vec<Manga> = Vec::new();
    mangas.push(Manga { name: "One Piece".into(), src: "https://mangalivre.com.br".into() });
    mangas.push(Manga { name: "Hanako-kun".into(), src: "https://mangalivre.com.br".into() });

    let manga_list = MangaList { content: mangas };

    manga_list
}

#[post("/my-mangas")]
async fn post_mangas(mangas: Json<MangaList>) -> impl Responder {

    for manga in &mangas.content {
        println!("The {} manga was sent to the server!", manga.name);
    }

    HttpResponse::Ok()
}

pub fn base_service(cfg: &mut ServiceConfig) {
    cfg.service(get_mangas);
    cfg.service(post_mangas);
}

#[cfg(test)]
mod base_api_tests {

    use super::{ MangaList, Manga };

    #[test]
    fn get_mangas() {

    }

    #[test]
    fn post_mangas() {
        let a = b"array de bytes?";

        
    }

}