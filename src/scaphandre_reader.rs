use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::fs::File;
use std::path::PathBuf;

/// A vector of all Scaphandre measures
///
/// This is the equivalent of Scaphandre json exporter output.
pub type ScaphandreMeasures = Vec<Measure>;

/// A Scaphandre data point (measure).
///
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
    /// Power usage of the host (in microWatts)
    pub consumption: f32,
    /// Timestamp of the measure
    pub timestamp: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Consumer {
    pub exe: PathBuf,
    pub pid: i64,
    /// Power usage of the process (in microWatts)
    pub consumption: f32,
    /// Timestamp of the measure
    pub timestamp: f64,
}

/// Extracts Scaphandre measures from a Scaphandre json file
///
pub fn read_scaph_file(scaphandre_json_file: &PathBuf) -> ScaphandreMeasures {
    //dbg!("Reading scaphandre file {:?}", scaphandre_json_file);
    let file = File::open(scaphandre_json_file).expect("file not found");
    let results: ScaphandreMeasures = serde_json::from_reader(file).expect("error while reading");
    results
}

/// Returns the average (arithmetic mean) value of all elements of a vector
fn average(data: Vec<f32>) -> Option<f32> {
    let sum = data.iter().sum::<f32>() as f32;
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum / count as f32),
        _ => None,
    }
}
/// Calculate the average consumption of a given process.
///
/// * Process are filtered on name
///
pub fn average_consumption(scaphandre_json_file: &PathBuf, process_name: &str) -> f32 {
    println!(
        "Calculating average consumption of process[{}] from file[{:?}]",
        process_name, scaphandre_json_file
    );
    let scaph_results: ScaphandreMeasures = read_scaph_file(scaphandre_json_file);
    let mut consumptions: Vec<f32> = Vec::new();

    for meas in scaph_results {
        for proc in meas.consumers {
            if proc.exe.ends_with(process_name) {
                consumptions.push(proc.consumption)
            }
        }
    }
    match average(consumptions) {
        Some(res) => res,
        None => panic!("Cannot calculate average consumption"),
    }
}

/// Extract the total duration of a process
///
/// * Duration is obtained by reading the scaphandre json output, filtering on process name,
/// and calculating the difference between the last and first timestamps of the process.
/// * returns 0 if process cannot be found
///
pub fn process_duration_seconds(scaphandre_json_file: &PathBuf, process_name: &str) -> f32 {
    println!(
        "Extracting duration consumption of process: {} from file[{:?}]",
        process_name, scaphandre_json_file
    );
    let mut first_timestamp: f64 = 0.0;
    let mut last_timestamp: f64 = 0.0;

    let scaph_results: ScaphandreMeasures = read_scaph_file(scaphandre_json_file);

    for measure in scaph_results {
        for process in measure.consumers {
            if process.exe == PathBuf::from(process_name) {
                if first_timestamp == 0.0 {
                    first_timestamp = process.timestamp;
                }
                last_timestamp = process.timestamp;
            }
        }
    }
    // FIXME: handle the case where process can never be found
    // FIXME: what happens the process appears only once ?
    (last_timestamp - first_timestamp) as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading_scaphandre_full_report() {
        let scaphandre_json_file = PathBuf::from("./tests/scaphandre-full-report.json");
        let measures: ScaphandreMeasures = read_scaph_file(&scaphandre_json_file);
        assert_eq!(measures.len(), 22);
    }
    #[test]
    fn test_reading_a_proc_name() {
        let scaphandre_json_file = PathBuf::from("./tests/scaphandre-full-report.json");
        let measures: ScaphandreMeasures = read_scaph_file(&scaphandre_json_file);
        assert_eq!(measures[20].consumers[1].exe, PathBuf::from("gnome-shell"));
    }
    #[test]
    fn test_reading_a_ts() {
        let scaphandre_json_file = PathBuf::from("./tests/scaphandre-full-report.json");
        let measures: ScaphandreMeasures = read_scaph_file(&scaphandre_json_file);
        assert_eq!(
            measures[20].consumers[1].timestamp,
            1646408484.4658008 as f64
        );
    }

    #[test]
    fn test_mean() {
        let data = vec![1.0, 2.0, 3.0];
        let result = average(data);
        match result {
            Some(res) => assert_eq!(res, 2.0),
            None => panic!(),
        }

        let data = vec![1.0, 2.0, 3.0, 4.0];
        let result = average(data);
        match result {
            Some(res) => assert_eq!(res, 2.5),
            None => panic!(),
        }

        let empty: Vec<f32> = Vec::new();
        let result = average(empty);
        match result {
            Some(res) => assert_eq!(res, 2.0),
            None => assert!(true),
        }
    }

    #[test]
    fn test_average_consumption_full() {
        let scaphandre_json_file = PathBuf::from("./tests/scaphandre-full-report.json");
        let process_name = "stress-ng";

        let res: f32 = average_consumption(&scaphandre_json_file, process_name);
        assert_eq!(res, 7269277.5 as f32);
    }

    #[test]
    fn test_average_consumption_simple() {
        let scaphandre_json_file = PathBuf::from("./tests/scaphandre-simple-report.json");
        let process_name = "stress-ng";

        let res: f32 = average_consumption(&scaphandre_json_file, process_name);
        assert_eq!(res, 7867854.0 as f32);
    }

    #[test]
    fn test_process_duration_simple() {
        let scaphandre_json_file = PathBuf::from("./tests/scaphandre-simple-report.json");
        let process_name = "stress-ng";

        let duration_seconds_f64 = process_duration_seconds(&scaphandre_json_file, process_name);

        assert_eq!(duration_seconds_f64, 2.0367724895477295 as f32);
    }
}
