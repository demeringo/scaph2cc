use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::path::Path;

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

pub fn read_cc_file(filename: &str) -> CarbonCrushResult {
    let json_file_path = Path::new(filename);
    let file = File::open(json_file_path).expect("file not found");
    let ccres: CarbonCrushResult = serde_json::from_reader(file).expect("error while reading");
    return ccres;
}

pub fn print_cc_file(ccres: CarbonCrushResult) {
    println!(
        "appid:{}  pipelineurl:{} value:{}",
        ccres.appid, ccres.cipipelineurl, ccres.value
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading_carbon_crush_results() {
        read_cc_file("./tests/measure-summary.json");
    }
}