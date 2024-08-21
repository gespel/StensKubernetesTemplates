
use serde::{Deserialize, Serialize};
use crate::core::models::{BaseMetadata, Container, Env, Labels, MatchLabels, Selector, Spec, Template, TemplateMetadata, TemplateSpec};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Deployment {
    #[serde(rename = "apiVersion")]
    api_version: String,
    kind: String,
    metadata: BaseMetadata,
    spec: Spec
}

impl Deployment {
    pub fn new(name: String, image: String) -> Deployment {
        Deployment{
            api_version: "apps/v1".to_string(),
            kind: "Deployment".to_string(),
            metadata: BaseMetadata {
                name: name.clone(),
                labels: Labels {
                    name: name.clone()
                }
            },
            spec: Spec {
                replicas: 1,
                selector: Selector {
                    match_labels: MatchLabels {
                        name: name.clone()
                    }
                },
                template: Template {
                    metadata: TemplateMetadata {
                        labels: Labels {
                            name: name.clone()
                        }
                    },
                    spec: TemplateSpec {
                        containers: vec![
                            Container {
                                name: name.clone(),
                                image,
                                env: vec![
                                    Env {
                                        name: "TEST_VAR".to_string(),
                                        value: "asdasd".to_string(),
                                    }
                                ],
                            }
                        ],
                    }
                },
            },
        }
    }
}