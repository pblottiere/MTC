use ureq;
use actix_web::{get, web, HttpRequest, HttpResponse};

use crate::projects::Projects;

#[get("/ogc/{project}")]
async fn wms(
    path: web::Path<String>,
    req: HttpRequest,
    projects: actix_web::web::Data<Projects>,
) -> HttpResponse {
    let p = match projects.project(path.to_string()) {
        Ok(p) => p,
        Err(_) => return HttpResponse::BadRequest().body("Project doesn't not exist"),
    };

    let query_str = req.query_string();
    let wms_url = format!("{}?{}", p.uri, query_str);
    match ureq::get(wms_url.as_str()).call() {
        Ok(response) => {
            let len: usize = response.header("Content-Length")
                .unwrap().parse().expect("INVALID!!");
            let mut bytes: Vec<u8> = Vec::with_capacity(len);
            let _ = response.into_reader()
                .read_to_end(&mut bytes);

            return HttpResponse::Ok()
                .content_type("image/png")
                .body(bytes); //response.into_string().unwrap());
        }
        Err(ureq::Error::Status(_code, response)) => {
            return HttpResponse::Ok().body(response.into_string().unwrap());
        }
        Err(ureq::Error::Transport(transport)) => {
            return HttpResponse::BadRequest().body(transport.message().unwrap().to_string());
        }
    }
}
