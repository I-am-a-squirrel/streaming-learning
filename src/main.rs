use actix_web::{get, web, web::resource, App, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(resource("/video_cards_list").get()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
