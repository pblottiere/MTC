use roxmltree;
use serde::Serialize;
use std::error::Error;
use std::sync::Mutex;
use ureq;

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
        let mut lyrs: Vec<Layer> = Vec::new();
        let getcapa_url = format!("{}?SERVICE=WMS&REQUEST=GetCapabilities", self.uri);
        match ureq::get(getcapa_url.as_str()).call() {
            Ok(response) => {
                let xmldoc = response.into_string().unwrap();
                let doc = roxmltree::Document::parse(xmldoc.as_str()).unwrap();
                for node in doc.descendants() {
                    if node.is_element() {
                        match node.tag_name().name() {
                            "Layer" => {
                                for n in node.children() {
                                    if n.is_element() {
                                        match n.tag_name().name() {
                                            "Layer" => {
                                                let mut layer_name = String::from("");
                                                for nn in n.children() {
                                                    if nn.is_element() {
                                                        match nn.tag_name().name() {
                                                            "Title" => {
                                                                layer_name =
                                                                    nn.text().unwrap().to_string();
                                                            }
                                                            _ => (),
                                                        }
                                                    }
                                                }
                                                let layer = Layer { name: layer_name };
                                                lyrs.push(layer);
                                            }
                                            _ => (),
                                        }
                                    }
                                }
                                break;
                            }
                            _ => (),
                        }
                    }
                }
            }
            Err(ureq::Error::Status(_code, _response)) => {
                return Err::<Vec<Layer>, String>(_response.status_text().to_string());
            }
            Err(ureq::Error::Transport(_transport)) => {
                return Err::<Vec<Layer>, String>(_transport.message().unwrap().to_string());
            }
        }
        return Ok(lyrs);
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
