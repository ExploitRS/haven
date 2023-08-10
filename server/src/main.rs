use std::fs;
use serde::{Deserialize, Serialize};
use actix_web::{http, web, get, post, App, HttpServer, HttpResponse, Error};
use actix_cors::Cors;
use toml;

const FILENAME: &str = "door.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Door {
    status: bool,
}

impl PartialEq for Door {
    fn eq(&self, other: &Self) -> bool {
        self.status == other.status
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Cfg {
    door: Door,
}

// impl From<&Door> for Cfg {
//     fn from(door: &Door) -> Self {
//         Cfg {
//             door,
//         }
//     }
// }

#[get("/api/status/door")]
async fn status_door() -> HttpResponse {
    let contents = fs::read_to_string(FILENAME)
        .expect("Something went wrong reading the file");

    let cfg = toml::from_str::<Cfg>(&contents).unwrap();
    let door = cfg.door;

    HttpResponse::Ok().json(door)
    // HttpResponse::Ok().json(Door { status: true })
}

#[post("api/status/door")]
async fn update_door_status(body: web::Bytes) -> Result<HttpResponse, Error> {
    println!("{:?}", body);
    let new_door = serde_json::from_slice::<Door>(&body)?;

    let contents = fs::read_to_string(FILENAME)
        .expect("Something went wrong reading the file");
    let cfg = toml::from_str::<Cfg>(&contents).unwrap();
    let current_door = cfg.door;
    let mut new_cfg: Cfg = Cfg{ door: current_door.clone() };

    if current_door != new_door {
        new_cfg = Cfg { door: new_door };
        let new_contents = toml::to_string(&new_cfg).unwrap();
        fs::write(FILENAME, new_contents).expect("Unable to write file");
    }

    Ok(HttpResponse::Ok().json(&new_cfg.door))
    // let contents = fs::read_to_string(FILENAME)
    //     .expect("Something went wrong reading the file");

    // let cfg = toml::from_str::<Cfg>(&contents).unwrap();
    // let door = cfg.door;

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
            .service(update_door_status)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}