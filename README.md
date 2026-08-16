# logscan

> Blazing fast multi-threaded CLI server log parser and pattern analyzer written in **Rust**.

[![Rust](https://img.shields.io/badge/Rust-2021-DEA584?style=flat-square&logo=rust)](https://rust-lang.org)
[![Rayon](https://img.shields.io/badge/Concurrency-Rayon-red?style=flat-square)](https://github.com/rayon-rs/rayon)
[![Mmap](https://img.shields.io/badge/I%2FO-memmap2-blue?style=flat-square)](https://github.com/RazrFalcon/memmap2-rs)
[![Docker](https://img.shields.io/badge/Docker-Ready-2496ED?style=flat-square&logo=docker)](https://docker.com)
[![License](https://img.shields.io/badge/License-MIT-blue?style=flat-square)](LICENSE)

`#rust` `#cli` `#log-analyzer` `#regex` `#memory-mapped` `#parallel-processing` `#observability` `#devops`

---

## Features

- **Zero-Copy Memory Mapping:** Reads large multi-gigabyte log files using `memmap2` without loading the entire file into RAM.
- **Parallel Chunk Processing:** Scales across all CPU cores with `rayon` work-stealing parallelism.
- **Status Code Distribution:** Visual percentage breakdown of 2xx, 3xx, 4xx, and 5xx responses with color coding.
- **Top IP & Endpoint Ranking:** Aggregates unique client IPs and requested URLs with configurable `--top` limits.
- **Regex Filtering:** Filter lines by custom regex patterns or HTTP status codes (e.g. `--status 5xx` or `--status 404`).

## Performance

Processes typical web server access logs at **~1.5–3.0 million lines/second** on a standard multi-core machine.

## Installation & Build

### With Cargo

```bash
# Build release binary
cargo build --release

# Install locally
cargo install --path .
```

### With Docker

```bash
docker build -t logscan .
docker run --rm -v $(pwd)/logs:/logs logscan --file /logs/access.log
```

## Usage Examples

```bash
# Basic log file analysis
logscan --file /var/log/nginx/access.log

# Show top 25 IPs and endpoints
logscan --file access.log --top 25

# Analyze only 5xx errors
logscan --file access.log --status 5xx

# Filter by custom regex (e.g., bot crawlers)
logscan --file access.log --pattern "Googlebot|bingbot"
```

## Sample Output

```text
==================================================
  ⚡ LogScan Analysis Report
==================================================
Processed: 1,450,200 lines (284.10 MB) in 0.482s (3,008,713 lines/sec)
Matched:   1,450,200 lines (100.0%)

── HTTP Status Codes ──
  [200]  1,320,400 requests ( 91.1%)
  [304]     75,200 requests (  5.2%)
  [404]     42,100 requests (  2.9%)
  [500]     12,500 requests (  0.9%)

── Top 5 IP Addresses ──
  192.168.1.105            84,120 requests
  10.0.0.15                62,400 requests
  172.16.0.42              51,100 requests

── Top 5 Endpoints / Paths ──
  /api/v1/health          120,400 requests
  /static/app.js           98,200 requests
  /login                   45,000 requests
```
