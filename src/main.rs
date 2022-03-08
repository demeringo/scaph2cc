mod cc_format;
use crate::cc_format::*;

mod scaph_reader;

fn main() {
    let app_id = "myapp1";
    let branch = "main";
    let pipeline_url = "http://test/url";

    let scaph_filename = "scaphandre-full-report.json";
    let cc_filename = "carbon-crush-data.json";
    let process_name = "stress-ng";

    let average_consumption = scaph_reader::average_consumption(scaph_filename, process_name);
    let duration = scaph_reader::process_duration_seconds(scaph_filename, process_name);
    let total_energy = duration * average_consumption;

    let carbon_crush_results = build_cc_result(average_consumption, app_id, branch, pipeline_url);
    save_cc_file(carbon_crush_results, cc_filename);
    
}
