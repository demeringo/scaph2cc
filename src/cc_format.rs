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

pub fn save_cc_file(carbon_crush_result: CarbonCrushResult, filename: &str) {
    // Save the JSON structure into the other file.
    std::fs::write(
        filename,
        serde_json::to_string_pretty(&carbon_crush_result).unwrap(),
    )
    .unwrap();
}

pub fn build_cc_result(
    value: f64,
    appid: &str,
    cicommitrefname: &str,
    cipipelineurl: &str,
) -> CarbonCrushResult {
    CarbonCrushResult {
        value: value.to_string(),
        appid: appid.to_string(),
        cicommitrefname: cicommitrefname.to_string(),
        cicommitsha: "".to_string(),
        cijob: "".to_string(),
        cijoburl: "".to_string(),
        cipipelineurl: cipipelineurl.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading_carbon_crush_results() {
        read_cc_file("./tests/measure-summary.json");
    }
    #[test]
    fn test_print_cc_file() {
        print_cc_file(read_cc_file("./tests/measure-summary.json"));
    }

    #[test]
    fn test_save_cc_file() {
        let filename = "test-generated-result";
        let carbon_crush_result = build_cc_result(123.0, "myapp1", "main", "http://whatever/job/123");
        save_cc_file(carbon_crush_result, filename)
        
    }
    #[test]
    fn test_build_cc_result(){
        let carbon_crush_result = build_cc_result(123.0, "myapp1", "main", "http://whatever/job/123");
    }
}
