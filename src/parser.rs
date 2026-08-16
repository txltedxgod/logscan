use regex::Regex;
use std::sync::OnceLock;

#[derive(Debug, Clone)]
pub struct ParsedLogLine {
    pub ip: Option<String>,
    pub status: Option<u16>,
    pub method: Option<String>,
    pub path: Option<String>,
    pub raw: String,
}

static LOG_REGEX: OnceLock<Regex> = OnceLock::new();

fn get_log_regex() -> &'static Regex {
    LOG_REGEX.get_or_init(|| {
        // Standard Common Log Format / Nginx format matcher
        Regex::new(r#"^(\S+)\s+\S+\s+\S+\s+\[([^\]]+)\]\s+"([A-Z]+)\s+([^"\s]+)\s+HTTP/[0-9.]+"\s+(\d{3})\s+(\d+|-)"#).unwrap()
    })
}

pub fn parse_line(line: &str) -> ParsedLogLine {
    let re = get_log_regex();
    if let Some(caps) = re.captures(line) {
        let ip = caps.get(1).map(|m| m.as_str().to_string());
        let method = caps.get(3).map(|m| m.as_str().to_string());
        let path = caps.get(4).map(|m| m.as_str().to_string());
        let status = caps.get(5).and_then(|m| m.as_str().parse::<u16>().ok());

        ParsedLogLine {
            ip,
            status,
            method,
            path,
            raw: line.to_string(),
        }
    } else {
        // Fallback: simple whitespace-split or raw line
        ParsedLogLine {
            ip: None,
            status: None,
            method: None,
            path: None,
            raw: line.to_string(),
        }
    }
}
