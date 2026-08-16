mod analyzer;
mod cli;
mod output;
mod parser;
mod reader;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use output::print_report;
use reader::MmapLogReader;
use regex::Regex;
use std::time::Instant;

fn main() -> Result<()> {
    let args = Cli::parse();

    if let Some(threads) = args.threads {
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build_global()
            .ok();
    }

    let pattern_regex = match &args.pattern {
        Some(pat) => Some(Regex::new(pat)?),
        None => None,
    };

    let start_time = Instant::now();
    let reader = MmapLogReader::open(&args.file)?;
    let size_bytes = reader.size_bytes();

    let num_threads = rayon::current_num_threads();
    let chunks = reader.create_line_chunks(num_threads * 4);

    let stats = analyzer::analyze_chunks(chunks, pattern_regex.as_ref(), args.status.as_deref());
    let elapsed = start_time.elapsed().as_secs_f64();

    let size_mb = size_bytes as f64 / (1024.0 * 1024.0);
    print_report(&stats, args.top, elapsed, size_mb);

    Ok(())
}
