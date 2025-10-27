use std::fs;
use chrono::Local;
use scraper::{ Html, Selector };

pub fn show() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let authors = env!("CARGO_PKG_AUTHORS");
    let license = env!("CARGO_PKG_LICENSE");

    println!(r#"
════════════════════════════════════════════════════════════════════════════
        Name: {}
        Version: {}
        Author: {}
        Github: https://github.com/sanjin168/rust-http-scanner.git
        License: {}  
════════════════════════════════════════════════════════════════════════════
    "#, name, version, authors, license);
}

// Read urls file
pub fn read_file(path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let lines = content
        .lines()
        .filter(|line| (line.starts_with("http://") || line.starts_with("https://")))
        .map(|line| line.to_string())
        .collect();
    Ok(lines)
}

// Parse http request title
pub fn parse_title(html_content: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let html = Html::parse_document(html_content);
    let selector = match Selector::parse("title") {
        Ok(selector) => selector,
        Err(e) => {
            return Err(format!("Failed to parse title selector: {}", e).into());
        }
    };
    Ok(
        html
            .select(&selector)
            .next()
            .map(|element| element.text().collect::<String>().replace("\n", "").replace("  ", ""))
    )
}

pub fn generate_timestamp_filename(extension: &str) -> String {
    let now = Local::now();
    let timestamp = now.format("%Y-%m-%d_%H-%M-%S").to_string();
    format!("results_{}.{}", timestamp, extension)
}