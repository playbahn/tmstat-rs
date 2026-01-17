use crate::config::Config;
use crate::util;

use std::io::BufRead;

pub fn format(fmtstr: &mut String, config: &Config) {
    let mut total = String::new();
    let mut available = String::new();
    let mut meminfo = std::io::BufReader::new(
        std::fs::File::open("/proc/meminfo").expect("Your system does not have a /proc/meminfo"),
    );

    meminfo
        .read_line(&mut total)
        .unwrap_or_else(|e| panic!("Error reading field MemTotal: {e}"));
    meminfo
        .skip_until(b'\n')
        .unwrap_or_else(|e| panic!("Error skipping field MemFree: {e}"));
    meminfo
        .read_line(&mut available)
        .unwrap_or_else(|e| panic!("Error reading field MemAvailable: {e}"));
    drop(meminfo);

    let parse_line = |line: &str| {
        line.split_whitespace()
            .nth(1)
            .and_then(|col| col.parse::<u64>().ok())
            .expect("Fails if /proc/meminfo changes its format")
    };
    let (total, available) = (parse_line(&total), parse_line(&available));
    let in_use = (total - available) as f64 / total as f64;

    if fmtstr.contains("{gm}") {
        util::replace_matches(fmtstr, "{gm}", &util::gradient(in_use));
    }

    util::replace_matches(fmtstr, "{m}", &util::format_metric(in_use * 100.0, config));
}
