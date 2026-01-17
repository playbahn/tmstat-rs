use std::path::PathBuf;

use clap::Parser;

use crate::help;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Config {
    #[arg(help = help::PID, long_help = help::PID_LONG)]
    pub client_pid: Option<String>,

    #[arg(default_value_t = 15, help = help::STATUS_INTERVAL, long_help = help::STATUS_INTERVAL_LONG)]
    pub status_interval: u64,

    /// Tmux-like format string to use for printing stats
    #[arg(short = 'F', long_help = help::FORMAT_LONG)]
    pub format: String,

    /// Mininum-maximum widths and precision to apply to usage strings
    #[arg(short, long, long_help = help::EXTRAS_LONG)]
    #[arg(value_name = "MIN>:<MAX>.<PRECISION", value_parser = crate::util::extras_parser)]
    pub extras: Option<(usize, usize, usize)>,

    /// Show network speeds for specific interface(s)
    #[arg(short, long, value_delimiter = '/', value_name = "NAME1[/NAME2]...", long_help = help::INTERFACE_LONG)]
    pub interfaces: Vec<String>,

    /// Use physical cores for calculating load average gradients
    #[arg(short, long, long_help = help::PHYSICAL_LONG)]
    pub physical: bool,

    /// Directory to cache stats in
    #[arg(short, long, long_help = help::CACHEDIR_LONG)]
    #[arg(value_name = "DIR", default_value = "/tmp/tmstat/")]
    pub cachedir: PathBuf,

    /// Display output in status line with `tmux display-message`
    #[cfg(debug_assertions)]
    #[arg(short, value_name = "DELAY", alias = "delay")]
    pub display: Option<u16>,
}
