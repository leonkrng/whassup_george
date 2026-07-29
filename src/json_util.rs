use serde::{Serialize, de::DeserializeOwned};
use std::path::Path;
use std::{
    fs::File,
    io::{BufReader, Write},
};

pub fn get_from_json<T: DeserializeOwned>(path: &Path) -> Result<T, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    return Ok(serde_json::from_reader(reader)?);
}

pub fn save_to_json_file<T: Serialize>(
    json_object: T,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let json_string: String = serde_json::to_string_pretty(&json_object).unwrap();
    let mut file = File::create(path).unwrap();

    return Ok(file.write_all(&json_string.as_bytes())?);
}
