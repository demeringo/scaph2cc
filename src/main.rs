use serde::Deserialize;
use std::fs::File;
use std::path::Path;
mod cc_format;
use crate::cc_format::CarbonCrushResult;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct User {
    username: String,
    first_name: String,
    last_name: String,
    email: String,
}

fn main() {
    let json_file_path = Path::new("./tests/measure-summary.json");
    let file = File::open(json_file_path).expect("file not found");
    let ccres: CarbonCrushResult = serde_json::from_reader(file).expect("error while reading");

    println!(
        "Hello {} aka {} {}",
        ccres.appid, ccres.cipipelineurl, ccres.value)
    /*let users: Vec<User> = serde_json::from_reader(file).expect("error while reading");
    for user in users {
        println!(
            "Hello {} aka {} {}",
            user.username, user.first_name, user.last_name
        )
    }*/
}
