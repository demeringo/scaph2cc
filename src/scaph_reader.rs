use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::fs::File;
use std::path::Path;

pub type ScaphResults = Vec<Measure>;

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

pub fn read_scaph_file(filename: &str) -> ScaphResults {
    println!("reading scaph file");
    let json_file_path = Path::new(filename);
    let file = File::open(json_file_path).expect("file not found");
    let results: ScaphResults = serde_json::from_reader(file).expect("error while reading");
    return results;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_reading_scaphandre_full_report() {
        let res: ScaphResults = read_scaph_file("./tests/scaphandre-full-report.json");
        assert_eq!(res.len(), 22);
    }
    #[test]
    fn test_reading_a_proc_name(){
      let res: ScaphResults = read_scaph_file("./tests/scaphandre-full-report.json");
      assert_eq!(res[20].consumers[1].exe, "gnome-shell");
    }
    #[test]
    fn test_reading_a_ts(){
      let res: ScaphResults = read_scaph_file("./tests/scaphandre-full-report.json");
      assert_eq!(res[20].consumers[1].timestamp, 1646408484.4658008 as f64);
    }
}
