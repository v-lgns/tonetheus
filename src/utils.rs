use clap::Parser;
use regex::Regex;

// argument parser format
#[derive(Parser, Debug, Clone)]
#[command(version)]
pub struct Args {
    #[arg(long)]
    pub name: String,

    #[arg(long, default_value_t = String::from("127.0.0.1"))]
    pub host: String,

    #[arg(long, default_value_t = 8000)]
    pub port: u32,
}

// strip ansi color codes from input string
pub fn decolorize(input: &str) -> String {
    let ansi_escape_pattern = r"\u{1b}\[[0-?]*[ -/]*[@-~]";
    let re = Regex::new(ansi_escape_pattern).unwrap();
    re.replace_all(input, "").to_string()
}
