use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

use crate::{models::weather::Weather, repository::Repository};

const PATH: &str = "/weather";

pub fn service<R: Repository>(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope(PATH)
            .route("/", web::get().to(get::<R>))
            .route("/", web::post().to(post::<R>)),
    );
}

async fn post<R: Repository>(weather: web::Json<Weather>, repo: web::Data<R>) -> HttpResponse {
    match repo.create_register_weather(&weather).await {
        Ok(weather) => HttpResponse::Ok().json(weather),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error {}", e)),
    }
}

async fn get<R: Repository>(repo: web::Data<R>) -> HttpResponse {
    print!("GET!");
    match repo.get().await {
        Ok(weather) => HttpResponse::Ok().json(weather),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error {}", e)),
    }
}
