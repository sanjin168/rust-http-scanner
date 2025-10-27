// use std::fs;

use crate::response::Response;
use crate::utils::generate_timestamp_filename;

pub fn _output_to_txt(_res: &Vec<Response>) -> Result<String, Box<dyn std::error::Error>> {
    let filename = generate_timestamp_filename("txt");
    Ok(filename)
}

pub fn _output_to_csv(_res: &Vec<Response>) -> Result<String, Box<dyn std::error::Error>> {

    let filename = generate_timestamp_filename("csv");
    Ok(filename)
}

pub fn _output_to_json(_res: &Vec<Response>) -> Result<String, Box<dyn std::error::Error>> {

    let filename = generate_timestamp_filename("json");
    Ok(filename)
}