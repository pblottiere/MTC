use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use actix_web::{web::Data, App, HttpServer};

use crate::api::admin;
use crate::config::Config;
use crate::projects::Projects;

mod api;
mod config;
mod projects;
mod wms;

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
            .service(admin::projects_list)
            .service(admin::project)
            .service(admin::project_layers)
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
