use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

//use std::time::Duration;

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
    pub consumption: f32,
    pub timestamp: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Consumer {
    pub exe: PathBuf,
    pub pid: i64,
    pub consumption: f32,
    pub timestamp: f64,
}

pub fn read_scaph_file(filename: &str) -> ScaphResults {
    //println!("reading scaph file {}", filename);
    let json_file_path = Path::new(filename);
    let file = File::open(json_file_path).expect("file not found");
    let results: ScaphResults = serde_json::from_reader(file).expect("error while reading");
    return results;
}

pub fn mean(data: Vec<f32>) -> Option<f32> {
    let sum = data.iter().sum::<f32>() as f32;
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    }
}

pub fn average_consumption(scaphandre_file_name: &str, process_name: &str) -> f32 {
    println!(
        "Calculating average consumption of process[{}] from file[{}]",
        process_name, scaphandre_file_name
    );
    let scaph_results: ScaphResults = read_scaph_file(scaphandre_file_name);
    let mut consumptions: Vec<f32> = Vec::new();

    for meas in scaph_results {
        for proc in meas.consumers {
            if proc.exe.ends_with(process_name) {
                consumptions.push(proc.consumption)
            }
        }
    }
    match mean(consumptions) {
        Some(res) => res,
        None => panic!("Cannot calculate mean consumption"),
    }
}

pub fn process_duration_seconds(scaphandre_file_name: &str, process_name: &str) -> f32 {
    println!(
        "Extracting duration consumption of process: {} from file[{}]",
        process_name, scaphandre_file_name
    );
    let mut first_timestamp: f64 = 0.0;
    let mut last_timestamp: f64 = 0.0;

    let scaph_results: ScaphResults = read_scaph_file(scaphandre_file_name);

    for meas in scaph_results {
        for proc in meas.consumers {
            if proc.exe == PathBuf::from(process_name) {
                if first_timestamp == 0.0 {
                    first_timestamp = proc.timestamp;
                }
                last_timestamp = proc.timestamp;
            }
        }
    }

    (last_timestamp - first_timestamp) as f32
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
    fn test_reading_a_proc_name() {
        let res: ScaphResults = read_scaph_file("./tests/scaphandre-full-report.json");
        assert_eq!(res[20].consumers[1].exe, PathBuf::from("gnome-shell"));
    }
    #[test]
    fn test_reading_a_ts() {
        let res: ScaphResults = read_scaph_file("./tests/scaphandre-full-report.json");
        assert_eq!(res[20].consumers[1].timestamp, 1646408484.4658008 as f64);
    }

    #[test]
    fn test_mean() {
        let data = vec![1.0, 2.0, 3.0];
        let result = mean(data);
        match result {
            Some(res) => assert_eq!(res, 2.0),
            None => panic!(),
        }

        let data = vec![1.0, 2.0, 3.0, 4.0];
        let result = mean(data);
        match result {
            Some(res) => assert_eq!(res, 2.5),
            None => panic!(),
        }

        let empty: Vec<f32> = Vec::new();
        let result = mean(empty);
        match result {
            Some(res) => assert_eq!(res, 2.0),
            None => assert!(true),
        }
    }

    #[test]
    fn test_average_consumption() {
        let filename = "./tests/scaphandre-full-report.json";
        let process = "stress-ng";

        let res: f32 = average_consumption(filename, process);
        assert_eq!(res, 7269277.5 as f32);
    }

    #[test]
    fn test_average_consumption_small() {
        let filename = "./tests/scaphandre-simple-report.json";
        let process = "stress-ng";

        let res: f32 = average_consumption(filename, process);
        assert_eq!(res, 7867854.0 as f32);
    }

    #[test]
    fn test_process_duration_small() {
        let filename = "./tests/scaphandre-simple-report.json";
        let process = "stress-ng";

        let duration_seconds_f64 = process_duration_seconds(filename, process);

        assert_eq!(duration_seconds_f64, 2.0367724895477295 as f32);
    }
}
