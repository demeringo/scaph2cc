use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

pub type Results = Vec<Measure>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Measure {
    pub host: Host,
    pub consumers: Vec<Consumer>,
    pub sockets: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Host {
    pub consumption: f64,
    pub timestamp: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Consumer {
    pub exe: String,
    pub pid: i64,
    pub consumption: f64,
    pub timestamp: f64,
}
