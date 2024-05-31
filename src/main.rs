use std::thread;
use serde::Serialize;
use std::sync::Mutex;
use std::time::Duration;

use actix_web::{get, web, web::Data, App, HttpResponse, HttpServer};

use crate::config::Config;
use crate::projects::Projects;

mod config;
mod projects;

#[get("/api/projects")]
async fn projects_list(
    projects: actix_web::web::Data<Projects>,
) -> HttpResponse {
    #[derive(Clone, Serialize)]
    struct JsonProject {
        name: String
    }
    let mut ps : Vec<JsonProject> = vec![];

    for p in projects.projects.lock().unwrap().iter() {
        ps.push(JsonProject{name: (*p.name).to_string()})
    }
    HttpResponse::Ok().json(ps)
}

#[get("/api/projects/{project}")]
async fn project(
    path: web::Path<String>,
    projects: actix_web::web::Data<Projects>,
) -> HttpResponse {
    let p = match projects.project(path.to_string()) {
        Ok(p) => p,
        Err(_) => return HttpResponse::BadRequest().body("Project doesn't not exist"),
    };

    HttpResponse::Ok().json(p)
}

#[get("/api/projects/{project}/layers")]
async fn project_layers(
    path: web::Path<String>,
    projects: actix_web::web::Data<Projects>,
) -> HttpResponse {
    let p = match projects.project(path.to_string()) {
        Ok(p) => p,
        Err(_) => return HttpResponse::BadRequest().body("Project doesn't not exist"),
    };

    let msg = format!("Hello, {}!", p.name.to_string());
    HttpResponse::Ok().body(msg)
}

fn update_projects(_psql_service: String, projects: actix_web::web::Data<Projects>) -> () {
    loop {
        thread::sleep(Duration::from_secs(1));
        projects.update();
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

    thread::spawn(move || update_projects(config.psql_service, projects_ptr));

    HttpServer::new(move || {
        App::new()
            .app_data(projects.clone())
            .service(projects_list)
            .service(project)
            .service(project_layers)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
