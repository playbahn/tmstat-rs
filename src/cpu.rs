use std::io::BufRead;

use crate::util;
use crate::util::Cache;
use crate::util::CacheMod::Cpu;

use crate::config::Config;

const MSG: &str = "Did /proc/stat change its format?";

pub fn format(fmtstr: &mut String, config: &Config) {
    let mut line1 = String::new();
    let (mut nonidlen, mut totaln) = (0, 0);

    std::io::BufReader::new(std::fs::File::open("/proc/stat").expect("Could not open /proc/stat"))
        .read_line(&mut line1)
        .unwrap_or_else(|e| panic!("Error reading from /proc/stat: {e:#?}"));

    let mut stats = line1
        .split_whitespace()
        .skip(1)
        .map(|time| time.parse::<u64>().expect(MSG));

    (0..3).for_each(|_| nonidlen += stats.next().expect(MSG)); // user + nice + system
    (0..2).for_each(|_| totaln += stats.next().expect(MSG)); // idle + iowait
    (0..3).for_each(|_| nonidlen += stats.next().expect(MSG)); // irq + softirq + steal
    // guest and guest nice are already accounted for in user and nice respectively
    // https://github.com/htop-dev/htop/blob/01a3c9e04668ecebba31972fd351ba818e19f9e2/linux/LinuxMachine.c#L444
    totaln += nonidlen;

    let max_age = std::time::Duration::from_secs(config.status_interval + 1);
    let mut jiffies =
        Cache::init(config, Cpu, max_age).unwrap_or_else(|e| panic!("Couldn't access cache: {e}"));
    let (nonidlep, totalp) = jiffies.get_recent().unwrap_or((nonidlen, 1));
    dbg!(nonidlep, totalp);

    let usage = if nonidlen == nonidlep {
        0.0
    } else {
        nonidlen.wrapping_sub(nonidlep) as f64 / totaln.wrapping_sub(totalp) as f64
    };

    if fmtstr.contains("{gc}") {
        util::replace_matches(fmtstr, "{gc}", &util::gradient(usage));
    }

    util::replace_matches(fmtstr, "{c}", &util::format_metric(usage * 100.0, config));

    jiffies
        .update(nonidlen, totaln)
        .unwrap_or_else(|e| panic!("Couldn't update cache: {e}"));
}
