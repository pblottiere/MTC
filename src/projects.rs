use std::error::Error;
use std::sync::Mutex;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Project {
    pub name: String,
    pub layers: Vec<(String, String)>,
    // datetime, creator
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
        let p = Project {
            name: "my_project".to_string(),
            layers: vec![("my_layer".to_string(), "http://qgiserver/".to_string())],
        };
        self.projects.lock().unwrap().push(p);
    }
}
