use std::thread;
use std::sync::Mutex;
use chrono::prelude::*;
use std::time::Duration;
use serde_derive::Deserialize;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, web::Data};

use crate::config::Config;

mod config;

// #[derive(Copy, Clone)]
pub struct SharedData {
    pending_indexes: Mutex<Vec<String>>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Debug, Deserialize)]
pub struct WMSParams {
    #[serde(alias = "REQUEST")]
    request: String,
}

#[get("/wms")]
async fn map(req: web::Query<WMSParams>) -> impl Responder {
    dbg!("request ={}", &req.request);
    HttpResponse::Ok().body("Map!")
}

#[get("/map/{project}")]
async fn map_project(path: web::Path<String>, shared_data: actix_web::web::Data<SharedData>) -> impl Responder {
    let friend = path.into_inner();
    let now = shared_data.pending_indexes.lock().unwrap().pop();
    dbg!("[{}] Welcome {}", now, friend);
    HttpResponse::Ok().body("Map porjtect!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

fn read_projects(psql_service: String, shared_data: actix_web::web::Data<SharedData>) -> () {
    loop {
        thread::sleep(Duration::from_secs(1));
        println!("Updating time.... {}", psql_service);
        let now: DateTime<Local> = Local::now();
        // let mut data = data.lock().unwrap();
        // data.insert(String::from("time_now"), now.to_rfc2822());
        shared_data.pending_indexes.lock().unwrap().push(now.to_rfc2822());
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // read config from environment variables
    let config = Config::new();

    // config is dynamic so shared between threads
    let projects = Data::new(SharedData {
        pending_indexes: Mutex::new(vec![]),
    });
    let projects_ptr = projects.clone(); // only copies the pointer

    thread::spawn(move || {
        read_projects(config.psql_service, projects_ptr)
    });

    HttpServer::new(move || {
        App::new()
            .app_data(projects.clone())
            .service(map_project)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
