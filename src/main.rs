use std::thread;
use std::sync::Mutex;
use std::error::Error;
use std::time::Duration;

use serde_derive::Deserialize;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, web::Data};

use crate::config::Config;

mod config;

#[derive(Clone)]
pub struct Project {
    pub name: String,
}

pub struct Projects {
    projects: Mutex<Vec<Project>>,
}

impl Projects {
    pub fn get(&self, name: String) -> Result<Project, Box<dyn Error>> {
        for project in self.projects.lock().unwrap().iter() {
            match &project.name.as_str() {
                a if name == a.to_string() => return Ok(project.clone()),
                _ => return Err("".into()),
            };
        }
        return Err("".into());
    }
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
async fn map_project(path: web::Path<String>, projects: actix_web::web::Data<Projects>) -> impl Responder {
    let project: Project = match projects.get(path.to_string()) {
        Ok(p) => p,
        Err(e) => return HttpResponse::BadRequest().body("gloups"),
    };

    let msg = format!("Hello, {}!", path.to_string());
    HttpResponse::Ok().body(msg)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

fn read_projects(psql_service: String, projects: actix_web::web::Data<Projects>) -> () {
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
