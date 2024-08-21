use std::fs::Metadata;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Env {
    name: String,
    value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Container {
    name: String,
    image: String,
    env: Vec<Env>
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TemplateSpec {
    containers: Vec<Container>

}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TemplateMetadata {
    labels: Labels
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Template {
    metadata: TemplateMetadata,
    spec: TemplateSpec

}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct MatchLabels {
    name: String,

}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Selector {
    matchLabels: MatchLabels

}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Port {
    protocol: String,
    port: i32,
    targetPort: i32
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct ServiceSelector {
    app: String
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct ServiceSpec {
    selector: ServiceSelector,
    ports: Vec<Port>,
    #[serde(rename = "type")]
    typeName: String
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Spec {
    replicas: i32,
    selector: Selector,
    template: Template
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Labels {
    name: String,

}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct BaseMetadata {
    name: String,
    labels: Labels
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Service {
    apiVersion: String,
    kind: String,
    metadata: BaseMetadata,
    spec: ServiceSpec
}

impl Service {
    fn new(name: String) -> Service {
        Service {
            apiVersion: "v1".to_string(),
            kind: "Service".to_string(),
            metadata: BaseMetadata {
                name: name.clone(),
                labels: Labels {
                    name: name.clone()
                }
            },
            spec: ServiceSpec {
                selector: ServiceSelector { app: "".to_string() },
                ports: vec![
                    Port {
                        protocol: "TCP".to_string(),
                        port: 80,
                        targetPort: 8080,
                    }
                ],
                typeName: "ClusterIP".to_string(),
            },
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Deployment {
    apiVersion: String,
    kind: String,
    metadata: BaseMetadata,
    spec: Spec
}

impl Deployment {
    fn new(name: String, image: String) -> Deployment {
        Deployment{
            apiVersion: "apps/v1".to_string(),
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
                    matchLabels: MatchLabels {
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
                                name: "test".to_string(),
                                image: "test:1.0.0".to_string(),
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

fn main() {
    let d = Deployment::new("test".to_string(), "test:1.0.0".to_string());
    let s = Service::new("testsvc".to_string());

    let yaml = serde_yaml::to_string(&d).unwrap();
    let yaml2 = serde_yaml::to_string(&s).unwrap();
    println!("{}", yaml);
    println!("{}", yaml2);
}
