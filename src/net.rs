use std::io::BufRead;

use crate::util;
use crate::util::Cache;
use crate::util::CacheMod::Net;

use crate::config::Config;

const MSG: &str = "Did /proc/net/dev change its format?";

pub fn format(fmtstr: &mut String, config: &Config) {
    let interfaces: Vec<&str> = config.interfaces.iter().map(|s| s.as_str()).collect();
    let all = interfaces.contains(&"..");
    let get_bytes = |col: Option<&str>| col.and_then(|col| col.parse::<u64>().ok()).expect(MSG);

    let (rxn, txn) = std::io::BufReader::new(
        std::fs::File::open("/proc/net/dev").expect("Could not open /proc/net/dev"),
    )
    .lines()
    .skip(2)
    .inspect(|res| {
        if let Err(e) = res {
            eprintln!("Could not read line: {e}")
        }
    })
    .filter_map(Result::ok)
    .filter_map(|line| {
        let mut cols = line.split_whitespace();
        let name = cols.next().expect(MSG);

        if interfaces.is_empty() {
            name.trim_end_matches(':') != "lo"
        } else {
            interfaces.contains(&name) || all
        }
        .then(|| {
            let rxn = get_bytes(cols.next());
            let txn = get_bytes(cols.nth(7));

            (rxn, txn)
        })
    })
    .reduce(|acc, i| (acc.0 + i.0, acc.1 + i.1))
    .expect("Found no interfaces");

    let max_age = std::time::Duration::from_secs(config.status_interval + 1);
    let mut rx_tx_bytes =
        Cache::init(config, Net, max_age).unwrap_or_else(|e| panic!("Couldn't access cache: {e}"));
    let (rxp, txp) = rx_tx_bytes.get_recent().unwrap_or((rxn, txn));

    let (rxd, rxu) = format_net_bytes(rxn.wrapping_sub(rxp), config);
    let (txd, txu) = format_net_bytes(txn.wrapping_sub(txp), config);

    util::replace_matches(fmtstr, "{d}", &rxd);
    util::replace_matches(fmtstr, "{du}", rxu);
    util::replace_matches(fmtstr, "{u}", &txd);
    util::replace_matches(fmtstr, "{uu}", txu);

    rx_tx_bytes
        .update(rxn, txn)
        .unwrap_or_else(|e| panic!("Couldn't update cache: {e}"));
}

fn format_net_bytes(bytes: u64, config: &Config) -> (String, &'static str) {
    let mut bytes = bytes as f64;
    let mut units = [" ", "K", "M", "G", "T"].into_iter();
    while units.next().is_some() {
        bytes /= 1024.0;
        if bytes < 1024.0 {
            break;
        }
    }

    let bytes = util::format_metric(bytes, config);

    (bytes, units.next().unwrap_or("T"))
}
