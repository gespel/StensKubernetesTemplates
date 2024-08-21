use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Env {
    pub(crate) name: String,
    pub(crate) value: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Container {
    pub(crate) name: String,
    pub(crate) image: String,
    pub(crate) env: Vec<Env>
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TemplateSpec {
    pub(crate) containers: Vec<Container>

}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TemplateMetadata {
    pub(crate) labels: Labels
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Template {
    pub(crate) metadata: TemplateMetadata,
    pub(crate) spec: TemplateSpec

}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct MatchLabels {
    pub(crate) name: String,

}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Selector {
    pub(crate) matchLabels: MatchLabels

}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Port {
    pub(crate) protocol: String,
    pub(crate) port: i32,
    pub(crate) targetPort: i32
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ServiceSelector {
    pub(crate) name: String
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ServiceSpec {
    pub(crate) selector: ServiceSelector,
    pub(crate) ports: Vec<Port>,
    #[serde(rename = "type")]
    pub(crate) typeName: String
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Spec {
    pub(crate) replicas: i32,
    pub(crate) selector: Selector,
    pub(crate) template: Template
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Labels {
    pub(crate) name: String,

}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BaseMetadata {
    pub(crate) name: String,
    pub(crate) labels: Labels
}