use ureq;

use crate::projects::Layer;

pub struct WMS {
    pub uri: String,
}

impl WMS {
    pub fn layers(&self) -> Result<Vec<Layer>, String> {
        let mut lyrs: Vec<Layer> = Vec::new();
        match self.getcapabilities() {
            Ok(xmldoc) => {
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
            Err(err_msg) => {
                return Err::<Vec<Layer>, String>(err_msg);
            }
        }
        return Ok(lyrs);
    }

    pub fn getcapabilities(&self) -> Result<String, String> {
        let getcapa_url = format!("{}?SERVICE=WMS&REQUEST=GetCapabilities", self.uri);
        match ureq::get(getcapa_url.as_str()).call() {
            Ok(response) => {
                return Ok(response.into_string().unwrap());
            }
            Err(ureq::Error::Status(_code, response)) => {
                return Err::<String, String>(response.status_text().to_string());
            }
            Err(ureq::Error::Transport(transport)) => {
                return Err::<String, String>(transport.message().unwrap().to_string());
            }
        }
    }
}
