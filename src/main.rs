use std::fs::File;
use std::path::Path;
mod cc_format;
use crate::cc_format::CarbonCrushResult;

mod scaph_reader;
mod user_reader;

fn read_cc_file(filename: &str) -> CarbonCrushResult {
    let json_file_path = Path::new(filename);
    let file = File::open(json_file_path).expect("file not found");
    let ccres: CarbonCrushResult = serde_json::from_reader(file).expect("error while reading");
    return ccres;
}

fn print_cc_file(ccres: CarbonCrushResult) {
    println!(
        "appid:{}  pipelineurl:{} value:{}",
        ccres.appid, ccres.cipipelineurl, ccres.value
    )
}

fn main() {
    user_reader::show_users("./tests/sample-users.json");

    let ccres = read_cc_file("./tests/measure-summary.json");
    print_cc_file(ccres);

    scaph_reader::read_scaph_file();
}


#[cfg(test)]
mod tests {
    use super::scaph_reader;

    #[test]
    fn test_reading_scaphandre_full_report() {
        scaph_reader::read_scaph_file();
    }

    #[test]
    fn test_reading_carbon_crush_results() {
        super::read_cc_file("./tests/measure-summary.json");
    }
}