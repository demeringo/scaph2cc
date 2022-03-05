use serde::Deserialize;
use std::fs::File;
use std::path::Path;
mod cc_format;
use crate::cc_format::CarbonCrushResult;

mod scaph_reader;
use crate::scaph_reader::read_scaph_file;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct User {
    username: String,
    first_name: String,
    last_name: String,
    email: String,
}

fn show_users(filename: &str) {
    let json_file_path = Path::new(filename);
    let file = File::open(json_file_path).expect("file not found");
    let users: Vec<User> = serde_json::from_reader(file).expect("error while reading");
    for user in users {
        println!(
            "Hello {} aka {} {} {}",
            user.username, user.first_name, user.last_name, user.email
        )
    }
}

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
    show_users("./tests/sample-users.json");

    let ccres = read_cc_file("./tests/measure-summary.json");
    print_cc_file(ccres);

    read_scaph_file();
}
