use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, web::Data};
use serde_derive::Deserialize;
use std::thread;
use std::time::Duration;
use std::sync::Mutex;
use chrono::prelude::*;

use crate::config::postgresql::ConfigPostgreSQL;

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

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

fn read_config(shared_data: actix_web::web::Data<SharedData>) -> () {
    loop {
        thread::sleep(Duration::from_secs(1));
        println!("Updating time.... ");
        let now: DateTime<Local> = Local::now();
        // let mut data = data.lock().unwrap();
        // data.insert(String::from("time_now"), now.to_rfc2822());
        shared_data.pending_indexes.lock().unwrap().push(now.to_rfc2822());
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let shared_data = Data::new(SharedData {
        pending_indexes: Mutex::new(vec![]),
    });
    let data = shared_data.clone(); // only copies the pointer

    thread::spawn(move || {
        read_config(data)
    });

    HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
            .service(map_project)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
