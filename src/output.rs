use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::response::Response;
use crate::utils::generate_timestamp_filename;

const OUTPUT_DIR: &str = "output";

pub fn output_to_txt(res: &Vec<Response>) -> Result<String, Box<dyn std::error::Error>> {
    fs::create_dir_all(OUTPUT_DIR)?;
    let basename = generate_timestamp_filename("txt");
    let filepath = Path::new(OUTPUT_DIR).join(basename);
    let file = File::create(&filepath)?;
    let mut writer = BufWriter::new(file);

    for r in res {
        writeln!(writer, "{}  {}  {}  {}", r.status_code, r.content_length, r.url, r.title)?;
    }

    writer.flush()?;

    Ok(filepath.to_string_lossy().into_owned())
}

pub fn output_to_csv(res: &Vec<Response>) -> Result<String, Box<dyn std::error::Error>> {
    fs::create_dir_all(OUTPUT_DIR)?;
    let basename = generate_timestamp_filename("csv");
    let filepath = Path::new(OUTPUT_DIR).join(basename);
    let mut wtr = csv::Writer::from_path(&filepath)?;

    for r in res {
        wtr.serialize(r)?;
    }

    wtr.flush()?;

    Ok(filepath.to_string_lossy().into_owned())
}

pub fn output_to_json(res: &Vec<Response>) -> Result<String, Box<dyn std::error::Error>> {
    fs::create_dir_all(OUTPUT_DIR)?;
    let basename = generate_timestamp_filename("json");
    let filepath = Path::new(OUTPUT_DIR).join(basename);
    let file = File::create(&filepath)?;
    let mut writer = BufWriter::new(file);

    serde_json::to_writer_pretty(writer, res)?;

    Ok(filepath.to_string_lossy().into_owned())
}