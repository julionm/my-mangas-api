use actix_web::{
    Responder,
    HttpRequest,
    HttpResponse,
    body::{
        BoxBody
    },
    http::header::{ ContentType }
};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct MangaList {
    pub content: Vec<Manga>
}

impl Responder for MangaList {
    type Body = BoxBody;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
    
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Manga {
    pub name: String,
    pub src: String,
}

impl Responder for Manga {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
    
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

// TODO this will be used to save the prints
struct Image {
    content: [u8; 256]
}
