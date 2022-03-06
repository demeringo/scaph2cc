use serde::Deserialize;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    username: String,
    first_name: String,
    last_name: String,
    email: String,
}

pub fn show_users(filename: &str) {
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