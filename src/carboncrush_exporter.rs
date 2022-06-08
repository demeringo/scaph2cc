use std::fmt;
use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

use quick_junit::*;

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

impl fmt::Display for CarbonCrushResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "consumption: {}", self.consumption)
    }
}

/// Save a carbon crush results as a file
pub fn save_carboncrush_file(
    carbon_crush_result: &CarbonCrushResult,
    carboncrush_json_file: PathBuf,
) {
    println!(
        "Saving results: {:?} to {:?}",
        carbon_crush_result, carboncrush_json_file
    );
    // Save the JSON structure into the other file.
    std::fs::write(
        carboncrush_json_file,
        serde_json::to_string_pretty(carbon_crush_result).unwrap(),
    )
    .unwrap();
}

pub fn save_as_junit_report(carbon_crush_result: &CarbonCrushResult, report_file: PathBuf) {
    let report = build_junit_report(carbon_crush_result);
    // Save the junit report into file.
    std::fs::write(report_file, report.to_string().unwrap()).unwrap();
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

pub fn build_junit_report(carbon_crush_result: &CarbonCrushResult) -> Report {
    let mut report = Report::new("Power measure");
    let test_suite_name = carbon_crush_result.app_id.as_str();
    let mut test_suite = TestSuite::new(test_suite_name);
    let mut success_case = TestCase::new(
        carbon_crush_result.commit_sha.as_str(),
        TestCaseStatus::success(),
    );

    success_case.set_system_out(format!("{:?}", carbon_crush_result));

    test_suite.add_test_cases([success_case]);
    report.add_test_suite(test_suite);
    report
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
        save_carboncrush_file(&carboncrush_result, carboncrush_json_file)
    }
    #[test]
    fn test_build_junit_report() {
        let carboncrush_result = build_carboncrush_result(
            123.0,
            "myapp1",
            "main",
            "d50e3b5ed5c27a848008abd5beb3d9e6c37c3f33",
            "http://whatever/job/123",
            1230.0,
            10.0,
        );
        let report: Report = build_junit_report(&carboncrush_result);
        println!("{}", report.to_string().unwrap());
        const EXPECTED_XML: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<testsuites name="Power measure" tests="1" failures="0" errors="0">
    <testsuite name="myapp1" tests="1" disabled="0" errors="0" failures="0">
        <testcase name="d50e3b5ed5c27a848008abd5beb3d9e6c37c3f33">
            <system-out>CarbonCrushResult { consumption: &quot;123&quot;, app_id: &quot;myapp1&quot;, duration: &quot;10&quot;, branch: &quot;main&quot;, commit_sha: &quot;d50e3b5ed5c27a848008abd5beb3d9e6c37c3f33&quot;, energy: &quot;1230&quot;, ci_pipeline_url: &quot;http://whatever/job/123&quot; }</system-out>
        </testcase>
    </testsuite>
</testsuites>
"#;
        assert_eq!(report.to_string().unwrap(), EXPECTED_XML);
    }
    #[test]
    fn test_save_junit_report() {
        let carboncrush_result = build_carboncrush_result(
            123.0,
            "myapp1",
            "main",
            "d50e3b5ed5c27a848008abd5beb3d9e6c37c3f33",
            "http://whatever/job/123",
            1230.0,
            10.0,
        );
        let mut junit_report: PathBuf = PathBuf::new();
        junit_report.push("tests/carbon-crush-report.xml");
        save_as_junit_report(&carboncrush_result, junit_report);
    }
}
