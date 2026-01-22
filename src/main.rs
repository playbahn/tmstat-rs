use clap::{CommandFactory, Parser};

use crate::config::Config;

mod cpu;
mod load;
mod mem;
mod net;

mod config;
mod help;
mod util;

fn main() {
    let mut config = Config::parse();

    #[cfg(debug_assertions)]
    dbg!(&config);

    let mut fmtstr = String::new();
    std::mem::swap(&mut fmtstr, &mut config.format);

    if config.extras.is_none() {
        config.extras = Some((0, !0, 0))
    }

    let config = config;

    let cpu = fmtstr.contains("{c}") || fmtstr.contains("{gc}");

    let net = (fmtstr.contains("{d}") || fmtstr.contains("{du}"))
        || (fmtstr.contains("{u}") || fmtstr.contains("{uu}"));

    let load = (fmtstr.contains("{l1}") || fmtstr.contains("{gl1}"))
        || (fmtstr.contains("{l5}") || fmtstr.contains("{gl5}"))
        || (fmtstr.contains("{l15}") || fmtstr.contains("{gl15}"));

    let use_cache = cpu || net || load;

    if use_cache && config.client_pid.is_none() {
        Config::command()
            .error(
                clap::error::ErrorKind::MissingRequiredArgument,
                "[CLIENT_PID] can't be empty when using cpu, network, or loadavg",
            )
            .exit();
    }

    // NOTE: WITHOUT caching

    if fmtstr.contains("{m}") || fmtstr.contains("{gm}") {
        crate::mem::format(&mut fmtstr, &config);
    }

    if !use_cache {
        println!("{fmtstr}");

        #[cfg(debug_assertions)]
        util::tmux_debug_display(&fmtstr, &config);

        return;
    }

    // NOTE: WITH caching

    if let Err(e) = std::fs::create_dir(&config.cachedir)
        && e.kind() != std::io::ErrorKind::AlreadyExists
    {
        panic!("Could not create {}: {e}", &config.cachedir.display())
    }

    if cpu {
        crate::cpu::format(&mut fmtstr, &config);
    }
    if net {
        crate::net::format(&mut fmtstr, &config);
    }
    if load {
        crate::load::format(&mut fmtstr, &config);
    }

    println!("{fmtstr}");

    #[cfg(debug_assertions)]
    util::tmux_debug_display(&fmtstr, &config);
}
