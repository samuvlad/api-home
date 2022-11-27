mod repository;
mod weather;

mod models;
use actix_web::{web, App, HttpServer};

use crate::repository::BdRepository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server started");
    dotenv::dotenv().ok();
    let repo = BdRepository::from_env()
        .await
        .expect("Repository initialization error");
    let repo = web::Data::new(repo);

    HttpServer::new(move || {
        App::new()
            .app_data(repo.clone())
            .configure(weather::weather::service::<BdRepository>)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
