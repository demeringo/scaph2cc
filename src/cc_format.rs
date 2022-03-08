use serde::Deserialize;
use serde::Serialize;
use std::fs::File;
use std::path::Path;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CarbonCrushResult {
    pub consumption: String,
    pub app_id: String,
    pub duration: String,
    pub branch: String,
    pub commit_sha: String,
    pub energy: String,
    pub ci_pipeline_url: String,
}

pub fn read_cc_file(filename: &str) -> CarbonCrushResult {
    let json_file_path = Path::new(filename);
    let file = File::open(json_file_path).expect("file not found");
    let result: CarbonCrushResult = serde_json::from_reader(file).expect("error while reading");
    return result;
}

pub fn print_cc_file(c: CarbonCrushResult) {
    println!("Full results {:?}", c);
    println!(
        "Extract: appid:{}  pipeline url:{} consumption:{}, energy {}, duration",
        c.app_id, c.ci_pipeline_url, c.consumption, c.duration
    )
}

pub fn save_cc_file(carbon_crush_result: CarbonCrushResult, filename: &str) {
    println!("Saving results: {:?} to {}", carbon_crush_result, filename);
    // Save the JSON structure into the other file.
    std::fs::write(
        filename,
        serde_json::to_string_pretty(&carbon_crush_result).unwrap(),
    )
    .unwrap();
}

pub fn build_cc_result(
    consumption: f64,
    app_id: &str,
    branch: &str,
    commit_sha: &str,
    ci_pipeline_url: &str,
    energy: f64,
    duration: f64,
) -> CarbonCrushResult {
    CarbonCrushResult {
        consumption: consumption.to_string(),
        app_id: app_id.to_string(),
        energy: energy.to_string(),
        branch: branch.to_string(),
        ci_pipeline_url: ci_pipeline_url.to_string(),
        commit_sha: commit_sha.to_string(),
        duration: duration.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading_carbon_crush_results() {
        read_cc_file("./tests/carbon-crush-sample.json");
    }
    #[test]
    fn test_print_cc_file() {
        print_cc_file(read_cc_file("./tests/carbon-crush-sample.json"));
    }

    #[test]
    fn test_save_cc_file() {
        let filename = "test-generated-result.json";
        let carbon_crush_result = build_cc_result(
            123.0,
            "myapp1",
            "main",
            "d50e3b5ed5c27a848008abd5beb3d9e6c37c3f33",
            "http://whatever/job/123",
            1230.0,
            10.0,
        );
        save_cc_file(carbon_crush_result, filename)
    }
    #[test]
    fn test_build_cc_result() {
        let _carbon_crush_result = build_cc_result(
            123.0,
            "myapp1",
            "main",
            "d50e3b5ed5c27a848008abd5beb3d9e6c37c3f33",
            "http://whatever/job/123",
            1230.0,
            10.0,
        );
    }
}
