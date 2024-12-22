use serde::de::DeserializeOwned;
use std::fs;

pub fn read_dummy<T: DeserializeOwned>(relative_path: &str) -> T {
    let base_path = "src/dummy_data/";
    let full_path = format!("{}{}", base_path, relative_path);

    let json_data = fs::read_to_string(&full_path).expect("Unable to read JSON file");
    serde_json::from_str::<T>(&json_data).expect("Unable to parse JSON")
}
