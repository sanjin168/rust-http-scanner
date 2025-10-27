use clap::{ Parser, ColorChoice };

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(color = ColorChoice::Auto)]
pub struct Args {
    #[arg(long, help = "Target url input.")]
    pub target: Option<String>,

    #[arg(short, long, help = "Input urls file path.")]
    pub file: Option<String>,

    #[arg(long, help = "Request time out.", default_value_t = 5)]
    pub timeout: u64,

    #[arg(long, help = "Follow redirect.", default_value_t = true)]
    pub follow_redirect: bool,

    #[arg(long, help = "Max redirect.", default_value_t = 5)]
    pub max_redirect: usize,

    #[arg(short, long, help = "Thread number.", default_value_t = 64)]
    pub threads: usize,

    #[arg(long, help = "Proxy address.")]
    pub proxy: Option<String>,

    #[arg(short, long, help = "Output file format. txt/csv/json ", default_value_t = String::from("txt"))]
    pub output: String
}