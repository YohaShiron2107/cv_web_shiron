use actix_files::Files;
use actix_web::{get, App, HttpServer, Responder};
use actix_web_lab::respond::Html;

#[get("/")]
async fn index() -> impl Responder {
    let html_content = include_str!("../templates/index.html");
    Html(html_content.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server starting at http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/static", "./static").show_files_listing())
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}