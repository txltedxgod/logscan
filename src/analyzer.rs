use crate::parser::{parse_line, ParsedLogLine};
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct LogStatistics {
    pub total_lines: usize,
    pub matched_lines: usize,
    pub status_counts: HashMap<u16, usize>,
    pub ip_counts: HashMap<String, usize>,
    pub path_counts: HashMap<String, usize>,
    pub error_samples: Vec<String>,
}

impl LogStatistics {
    pub fn merge(&mut self, other: LogStatistics) {
        self.total_lines += other.total_lines;
        self.matched_lines += other.matched_lines;

        for (k, v) in other.status_counts {
            *self.status_counts.entry(k).or_insert(0) += v;
        }
        for (k, v) in other.ip_counts {
            *self.ip_counts.entry(k).or_insert(0) += v;
        }
        for (k, v) in other.path_counts {
            *self.path_counts.entry(k).or_insert(0) += v;
        }
        if self.error_samples.len() < 10 {
            let needed = 10 - self.error_samples.len();
            self.error_samples.extend(other.error_samples.into_iter().take(needed));
        }
    }
}

pub fn analyze_chunks(
    chunks: Vec<&[u8]>,
    pattern: Option<&Regex>,
    status_filter: Option<&str>,
) -> LogStatistics {
    chunks
        .into_par_iter()
        .map(|chunk| {
            let mut stats = LogStatistics::default();
            let text = String::from_utf8_lossy(chunk);

            for line in text.lines() {
                if line.trim().is_empty() {
                    continue;
                }
                stats.total_lines += 1;

                if let Some(ref re) = pattern {
                    if !re.is_match(line) {
                        continue;
                    }
                }

                let parsed = parse_line(line);

                if let Some(sf) = status_filter {
                    let matches_status = match parsed.status {
                        Some(code) => {
                            if sf.ends_with("xx") {
                                let prefix = sf.chars().next().unwrap_or('0');
                                code.to_string().starts_with(prefix)
                            } else if let Ok(target) = sf.parse::<u16>() {
                                code == target
                            } else {
                                true
                            }
                        }
                        None => false,
                    };

                    if !matches_status {
                        continue;
                    }
                }

                stats.matched_lines += 1;

                if let Some(code) = parsed.status {
                    *stats.status_counts.entry(code).or_insert(0) += 1;
                    if code >= 400 && stats.error_samples.len() < 5 {
                        stats.error_samples.push(line.to_string());
                    }
                }

                if let Some(ip) = parsed.ip {
                    *stats.ip_counts.entry(ip).or_insert(0) += 1;
                }

                if let Some(path) = parsed.path {
                    *stats.path_counts.entry(path).or_insert(0) += 1;
                }
            }

            stats
        })
        .reduce(LogStatistics::default, |mut a, b| {
            a.merge(b);
            a
        })
}
