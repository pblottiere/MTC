use serde::Serialize;

use actix_web::{get, web, HttpResponse};

use crate::projects::Projects;

#[get("/api/projects")]
async fn projects_list(projects: actix_web::web::Data<Projects>) -> HttpResponse {
    #[derive(Clone, Serialize)]
    struct JsonProject {
        name: String,
    }
    let mut ps: Vec<JsonProject> = vec![];

    for p in projects.projects.lock().unwrap().iter() {
        ps.push(JsonProject {
            name: (*p.name).to_string(),
        })
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

    let layers = p.layers();
    HttpResponse::Ok().body("coucou".to_string())
}
