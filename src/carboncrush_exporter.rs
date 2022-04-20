use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

/// The structure that describe Carbon Crush input
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

/// Save a carbon crush results as a file
pub fn save_carboncrush_file(
    carbon_crush_result: CarbonCrushResult,
    carboncrush_json_file: PathBuf,
) {
    println!(
        "Saving results: {:?} to {:?}",
        carbon_crush_result, carboncrush_json_file
    );
    // Save the JSON structure into the other file.
    std::fs::write(
        carboncrush_json_file,
        serde_json::to_string_pretty(&carbon_crush_result).unwrap(),
    )
    .unwrap();
}

/// Build a carbon crush data structure with the passed values.
///
pub fn build_carboncrush_result(
    consumption: f32,
    app_id: &str,
    branch: &str,
    commit_sha: &str,
    ci_pipeline_url: &str,
    energy: f32,
    duration: f32,
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
    fn test_build_cc_result() {
        let _carboncrush_result = build_carboncrush_result(
            123.0,
            "myapp1",
            "main",
            "d50e3b5ed5c27a848008abd5beb3d9e6c37c3f33",
            "http://whatever/job/123",
            1230.0,
            10.0,
        );
    }
    #[test]
    fn test_save_cc_file() {
        let carboncrush_json_file = PathBuf::from("test-generated-result.json");
        let carboncrush_result = build_carboncrush_result(
            123.0,
            "myapp1",
            "main",
            "d50e3b5ed5c27a848008abd5beb3d9e6c37c3f33",
            "http://whatever/job/123",
            1230.0,
            10.0,
        );
        save_carboncrush_file(carboncrush_result, carboncrush_json_file)
    }
}
