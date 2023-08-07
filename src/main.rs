use serde::{Deserialize, Serialize};
use actix_web::{http, get, App, HttpServer, Responder, HttpResponse};
use actix_cors::Cors;

#[derive(Debug, Serialize, Deserialize)]
struct Door {
    status: bool,
}

#[get("/api/status/door")]
async fn status_door() -> HttpResponse {
    HttpResponse::Ok().json(Door { status: true })
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://0.0.0.0:3000")
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().ends_with(b".0.0.0.0:3000")
            })
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .service(status_door)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}