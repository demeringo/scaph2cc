use serde::Deserialize;
use serde::Serialize;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CarbonCrushResult {
    pub value: String,
    pub appid: String,
    pub cijob: String,
    pub cicommitsha: String,
    pub cijoburl: String,
    pub cicommitrefname: String,
    pub cipipelineurl: String,
}
