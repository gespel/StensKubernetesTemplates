
use serde::{Deserialize, Serialize};
use crate::models::{BaseMetadata, Labels, Port, ServiceSelector, ServiceSpec};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Service {
    #[serde(rename = "apiVersion")]
    api_version: String,
    kind: String,
    metadata: BaseMetadata,
    spec: ServiceSpec
}

impl Service {
    pub fn new(name: String, target_backend: String) -> Service {
        Service {
            api_version: "v1".to_string(),
            kind: "Service".to_string(),
            metadata: BaseMetadata {
                name: name.clone(),
                labels: Labels {
                    name: name.clone()
                }
            },
            spec: ServiceSpec {
                selector: ServiceSelector {
                    name : target_backend
                },
                ports: vec![
                    Port {
                        protocol: "TCP".to_string(),
                        port: 80,
                        target_port: 8080,
                    }
                ],
                type_name: "ClusterIP".to_string(),
            },
        }
    }
}