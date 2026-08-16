use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "logscan",
    author = "txltedxgod",
    version = "0.1.0",
    about = "Blazing fast multi-threaded CLI log parser & pattern analyzer"
)]
pub struct Cli {
    /// Path to the log file to analyze
    #[arg(short, long)]
    pub file: PathBuf,

    /// Regex pattern to search and filter by
    #[arg(short, long)]
    pub pattern: Option<String>,

    /// Top N results to display for IP addresses and paths
    #[arg(short, long, default_value_t = 10)]
    pub top: usize,

    /// Filter by HTTP status code (e.g. 500, 404, 2xx, 5xx)
    #[arg(short = 's', long)]
    pub status: Option<String>,

    /// Number of worker threads (defaults to logical CPU cores)
    #[arg(short = 't', long)]
    pub threads: Option<usize>,
}
