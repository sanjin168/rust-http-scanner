use colored::*;
use std::io::Write;

pub fn init_logger() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .format(|buf, record| {
            let time = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let (color, level) = match record.level() {
                log::Level::Error => (colored::Color::Red, "ERR"),
                log::Level::Warn => (colored::Color::Yellow, "WAN"),
                log::Level::Info => (colored::Color::Green, "INF"),
                log::Level::Debug => (colored::Color::Blue, "DEB"),
                log::Level::Trace => (colored::Color::Magenta, "TRA"),
            };
            let log_line = format!("{} {} -- {}", time, level, record.args());
            
            writeln!(
                buf,
                "{}",
                log_line.color(color).bold()
            )
        })
        .init();
}