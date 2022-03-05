use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub value: String,
    pub appid: String,
    pub cijob: String,
    pub cicommitsha: String,
    pub cijoburl: String,
    pub cicommitrefname: String,
    pub cipipelineurl: String,
}
