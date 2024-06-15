use serde::Serialize;
use std::error::Error;
use std::sync::Mutex;

use crate::wms::WMS;

#[derive(Clone, Serialize)]
pub struct Layer {
    pub name: String,
}

#[derive(Clone, Serialize)]
pub struct Project {
    pub name: String,
    pub uri: String,
    pub author: String,
    // datetime
}

impl Project {
    pub fn layers(&self) -> Result<Vec<Layer>, String> {
        let wms = WMS {
            uri: self.uri.clone(),
        };
        match wms.layers() {
            Ok(layers) => {
                return Ok(layers);
            }
            Err(err_msg) => {
                return Err::<Vec<Layer>, String>(err_msg);
            }
        }
    }
}

pub struct Projects {
    pub projects: Mutex<Vec<Project>>,
}

impl Projects {
    pub fn project(&self, name: String) -> Result<Project, Box<dyn Error>> {
        for project in self.projects.lock().unwrap().iter() {
            match &project.name.as_str() {
                a if name == a.to_string() => return Ok(project.clone()),
                _ => return Err("".into()),
            };
        }
        return Err("".into());
    }

    pub fn update(&self) {
        // TODO: read in postgresql
        let p = Project {
            name: "my_project".to_string(),
            uri: "http://localhost/qgisserver".to_string(),
            author: "pblottiere".to_string(),
        };
        self.projects.lock().unwrap().push(p);
    }
}
