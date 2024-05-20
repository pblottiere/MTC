use std::thread;
use std::sync::Mutex;
use std::time::Duration;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, web::Data};

use crate::config::Config;
use crate::projects::{Projects, Project};

mod config;
mod projects;


#[get("/map/{project}")]
async fn map_project(path: web::Path<String>, projects: actix_web::web::Data<Projects>) -> impl Responder {
    match projects.get(path.to_string()) {
        Ok(p) => p,
        Err(_) => return HttpResponse::BadRequest().body("gloups"),
    };

    let msg = format!("Hello, {}!", path.to_string());
    HttpResponse::Ok().body(msg)
}

fn read_projects(_psql_service: String, projects: actix_web::web::Data<Projects>) -> () {
    loop {
        thread::sleep(Duration::from_secs(1));
        projects.projects.lock().unwrap().push(Project{name: "coucou".to_string()});
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // read config from environment variables
    let config = Config::new();

    // config is dynamic so shared between threads
    let projects = Data::new(Projects {
        projects: Mutex::new(vec![]),
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
