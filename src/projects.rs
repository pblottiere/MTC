use std::sync::Mutex;
use std::error::Error;

#[derive(Clone)]
pub struct Project {
    pub name: String,
}

pub struct Projects {
    pub projects: Mutex<Vec<Project>>,
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
