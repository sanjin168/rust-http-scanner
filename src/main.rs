mod command;
mod request;
mod response;
mod running;
mod utils;
mod log_config;
mod output;

use clap::Parser;
use command::Args;
use running::run;
use log_config::init_logger;
use utils::show;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    show();
    let args = Args::parse();
    init_logger();

    run(args).await?;
    
    Ok(())
}
