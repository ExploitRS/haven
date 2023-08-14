use std::fs;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use actix_web::{http, web, get, post, App, HttpServer, HttpResponse, Error };
use actix_cors::Cors;
use toml;
use controller::{ HavenCntrlr, State};

const FILENAME: &str = "door.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Door {
    state: bool,
}

impl PartialEq for Door {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl TryInto<State> for Door {
    type Error = &'static str;

    fn try_into(self) -> Result<State, Self::Error> {
        match &self.state {
            true => Ok(State::Locked),
            false => Ok(State::Unlocked),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Cfg {
    door: Door,
}

struct Server {
    controller: HavenCntrlr,
}

impl Server {
    fn new() -> Self {
        let controller = HavenCntrlr::new();
        Server {
            controller,
        }
    }
}

// impl DerefMut for Server {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         Arc::get_mut(&mut self.controller).unwrap()
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
async fn update_door_status(body: web::Bytes, data: web::Data<Mutex<Server>>) -> Result<HttpResponse, Error> {
    
    let door = serde_json::from_slice::<Door>(&body)?;
    let server = data.clone();
    let mut server = server.lock().unwrap();

    match Door::try_into(door).unwrap() {
        State::Locked => server.controller.lock().unwrap(),
        State::Unlocked => server.controller.unlock().unwrap(),
    }

    let door = Door { state: server.controller.is_locked()};

    Ok(HttpResponse::Ok().json(door.clone()))
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
        
        let data = web::Data::new(Mutex::new(Server::new()));

        App::new()
            .app_data(web::Data::clone(&data))
            .wrap(cors)
            .service(status_door)
            .service(update_door_status)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}