use crate::util;
use crate::util::Cache;
use crate::util::CacheMod::Load;

use crate::config::Config;

const MSG: &str = "Did /proc/loadavg change its format?";

pub fn format(fmtstr: &mut String, config: &Config) {
    let loadavg = std::fs::read_to_string("/proc/loadavg").expect("Could not open /proc/stat");

    let loads: Vec<f64> = loadavg
        .split_whitespace()
        .take(3)
        .map(|load| load.parse().expect(MSG))
        .collect();

    // cache is usable if it it was updated after booting up. Not perfect, but will do just fine
    let mut ts: libc::timespec = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let ts0 = ts;
    let ret = unsafe { libc::clock_gettime(libc::CLOCK_BOOTTIME, &mut ts) };
    let ts = if ret == 0 { ts } else { ts0 };
    let uptime = std::time::Duration::from_secs(ts.tv_sec as u64);

    let mut write_cores = None;

    let mut cached_cores =
        Cache::init(config, Load, uptime).unwrap_or_else(|e| panic!("Couldn't access cache: {e}"));
    let (logical, physical) = cached_cores.get_recent().unwrap_or_else(|| {
        let logical = num_cpus::get() as u64;
        let physical = num_cpus::get_physical() as u64;
        write_cores = Some((logical, physical));
        (logical, physical)
    });

    let cores = if config.physical { physical } else { logical } as f64;

    let pats = [("{l1}", "{gl1}"), ("{l5}", "{gl5}"), ("{l15}", "{gl15}")];

    loads.into_iter().enumerate().for_each(|(idx, load)| {
        if fmtstr.contains(pats[idx].1) {
            util::replace_matches(fmtstr, pats[idx].1, &util::gradient(load / cores));
        }
        util::replace_matches(fmtstr, pats[idx].0, &util::format_metric(load, config));
    });

    if let Some((logical, physical)) = write_cores {
        cached_cores
            .update(logical, physical)
            .unwrap_or_else(|e| panic!("Couldn't update cache: {e}"));
    }
}
