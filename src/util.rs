use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;
use std::os::unix::fs::FileExt;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use palette::{Hsl, IntoColor, Srgb};

use crate::config::Config;

pub enum CacheMod {
    Cpu,
    Load,
    Net,
}

pub struct Cache {
    file: File,
    path: PathBuf,
    mtime: SystemTime,
    max_age: Duration,
    len: u64,
}

impl Cache {
    pub fn init(config: &Config, module: CacheMod, max_age: Duration) -> std::io::Result<Self> {
        let file_name = config.client_pid.as_ref().expect("main() ensures PID is present");
        let mut path = config.cachedir.join(file_name);
        path.set_extension(match module {
            CacheMod::Cpu => "cpu",
            CacheMod::Load => "load",
            CacheMod::Net => "net",
        });

        let file = File::options()
            .read(true)
            .write(true)
            .append(false)
            .truncate(false)
            .create(true)
            .open(&path)
            .inspect_err(|e| eprintln!("Could not open/create {}: {e}", path.display()))?;

        let meta = file
            .metadata()
            .inspect_err(|e| eprintln!("Could not get metadata ({}): {e}", path.display()))?;
        let mtime = meta.modified().inspect_err(|e| {
            eprintln!(
                "Could not get the last modification time ({}): {e}",
                path.display()
            )
        })?;

        Ok(Self {
            path,
            file,
            mtime,
            max_age,
            len: meta.len(),
        })
    }

    pub fn get(&mut self) -> Option<(u64, u64)> {
        if self.is_empty() {
            return None;
        }

        let mut buf = [0u8; 16];
        self.file
            .read_exact(&mut buf)
            .inspect_err(|e| eprintln!("Could not read from {}: {e}", self.path.display()))
            .ok()?;

        let u64_a = buf[..8].try_into().unwrap();
        let u64_b = buf[8..].try_into().unwrap();
        dbg!(u64_a, u64_b);
        Some((u64::from_be_bytes(u64_a), u64::from_be_bytes(u64_b)))
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get_recent(&mut self) -> Option<(u64, u64)> {
        if !self.is_hot() {
            dbg!("not hot");
            return None;
        }

        self.get()
    }

    #[inline]
    fn is_hot(&self) -> bool {
        self.mtime > SystemTime::now() - self.max_age
    }

    pub fn update(&mut self, u64_a: u64, u64_b: u64) -> std::io::Result<()> {
        let mut bytes = [0u8; 16];
        bytes[..8].copy_from_slice(&u64_a.to_be_bytes());
        bytes[8..].copy_from_slice(&u64_b.to_be_bytes());

        let write_all_at = self.file.write_all_at(&bytes, 0).inspect_err(|e| {
            eprintln!(
                "Couldn't write to {}: {e}. Truncating.",
                self.path.display()
            )
        });
        if write_all_at.is_ok() {
            return write_all_at;
        }
        self.file
            .set_len(0)
            .inspect_err(|e| eprintln!("Couldn't truncate {}: {e}", self.path.display()))
    }
}

pub fn replace_matches(fmtstr: &mut String, from: &str, to: &str) {
    let mut last_match = 0;
    while let Some(curpos) = fmtstr[last_match..].find(from) {
        last_match += curpos;
        fmtstr.replace_range(last_match..last_match + from.len(), to);
    }
}

pub fn extras_parser(s: &str) -> Result<(usize, usize, usize), ParseIntError> {
    let (widths, precision) = s.split_once('.').unwrap_or((s, ""));
    let (minw, maxw) = widths.split_once(':').unwrap_or((widths, ""));

    let parse = |s: &str, val: usize| match s.parse::<usize>() {
        Err(e) if *e.kind() == std::num::IntErrorKind::Empty => Ok(val),
        res => res,
    };

    let minw = parse(minw, 0)?;
    let maxw = parse(maxw, !0)?;
    let precision = parse(precision, 0)?;

    Ok((minw, maxw, precision))
}

pub fn gradient(multiplier: f64) -> String {
    let hue = 120.0 - (120.0 * multiplier);
    let hsl: Hsl<_, f64> = Hsl::new(hue, 1.0, 0.5);
    let rgb: Srgb<f64> = hsl.into_color();
    let (r, g, b) = rgb.into_format::<u8>().into_components();
    format!("#{r:02X}{g:02X}{b:02X}")
}

pub fn format_metric(value: f64, config: &Config) -> String {
    let (minw, maxw, precision) = config.extras.unwrap_or((0, !0, 0));
    dbg!(value, minw, maxw, precision);
    let mut usage = format!("{:minw$.precision$}", value);

    if usage.len() > maxw {
        if usage.find('.').is_none_or(|dot| dot > maxw) {
            usage = "9".repeat(maxw) + "+";
        } else {
            usage.truncate(maxw);
            if usage.ends_with('.') {
                let usage: &mut Vec<u8> = unsafe { usage.as_mut_vec() };
                usage.rotate_right(1);
                usage[0] = b' '; // pad to the right
            }
        }
    }

    dbg!(&usage);

    usage
}

#[cfg(debug_assertions)]
pub fn tmux_debug_display(fmtstr: &str, config: &Config) {
    if let Some(delay) = config.display {
        let _ = dbg!(
            std::process::Command::new("tmux")
                .args(["display", "-d", &format!("{delay}"), fmtstr])
                .spawn()
        );
    }
}
