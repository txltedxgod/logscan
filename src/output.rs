use crate::analyzer::LogStatistics;
use colored::*;

pub fn print_report(stats: &LogStatistics, top_n: usize, duration_secs: f64, file_size_mb: f64) {
    println!("\n{}", "==================================================".cyan().bold());
    println!("{}", "  ⚡ LogScan Analysis Report".cyan().bold());
    println!("{}", "==================================================".cyan().bold());

    println!(
        "Processed: {} lines ({:.2} MB) in {:.3}s ({:.0} lines/sec)",
        stats.total_lines.to_string().yellow().bold(),
        file_size_mb,
        duration_secs,
        stats.total_lines as f64 / duration_secs.max(0.001)
    );
    println!(
        "Matched:   {} lines ({:.1}%)\n",
        stats.matched_lines.to_string().green().bold(),
        (stats.matched_lines as f64 / stats.total_lines.max(1) as f64) * 100.0
    );

    // Status Code Breakdown
    if !stats.status_counts.is_empty() {
        println!("{}", "── HTTP Status Codes ──".bright_white().bold());
        let mut sorted_statuses: Vec<_> = stats.status_counts.iter().collect();
        sorted_statuses.sort_by(|a, b| b.1.cmp(a.1));

        for (&code, &count) in sorted_statuses {
            let pct = (count as f64 / stats.matched_lines.max(1) as f64) * 100.0;
            let code_str = match code {
                200..=299 => code.to_string().green(),
                300..=399 => code.to_string().blue(),
                400..=499 => code.to_string().yellow(),
                500..=599 => code.to_string().red().bold(),
                _ => code.to_string().white(),
            };
            println!("  [{:>3}] {:>8} requests ({:>5.1}%)", code_str, count, pct);
        }
        println!();
    }

    // Top IP Addresses
    if !stats.ip_counts.is_empty() {
        println!("{}", format!("── Top {} IP Addresses ──", top_n).bright_white().bold());
        let mut sorted_ips: Vec<_> = stats.ip_counts.iter().collect();
        sorted_ips.sort_by(|a, b| b.1.cmp(a.1));

        for (ip, &count) in sorted_ips.into_iter().take(top_n) {
            println!("  {:<20} {:>8} requests", ip.cyan(), count);
        }
        println!();
    }

    // Top Request Paths
    if !stats.path_counts.is_empty() {
        println!("{}", format!("── Top {} Endpoints / Paths ──", top_n).bright_white().bold());
        let mut sorted_paths: Vec<_> = stats.path_counts.iter().collect();
        sorted_paths.sort_by(|a, b| b.1.cmp(a.1));

        for (path, &count) in sorted_paths.into_iter().take(top_n) {
            println!("  {:<40} {:>8} requests", path, count);
        }
        println!();
    }

    // Error Samples
    if !stats.error_samples.is_empty() {
        println!("{}", "── Sample Error Lines (4xx / 5xx) ──".bright_red().bold());
        for sample in stats.error_samples.iter().take(3) {
            println!("  {}", sample.dimmed());
        }
        println!();
    }
}
