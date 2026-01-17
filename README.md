# tmstat-rs

**tmstat-rs** is a small, fast Rust-based CLI tool to display (selective opinionated) system stats
inside your `tmux` status line — with [**caching**](#how-it-works-and-stuff) for efficiency, and
optional gradients & formatting.

![Tmux statusline with tmstat-rs showing network traffic, memory and CPU usages](./assets/images/example.png)

## Platform Support

**Linux only** - This tool reads CPU statistics from `/proc/stat` and only works on Linux systems.

**Tested and supported architectures:**
- x86_64 (64-bit Intel/AMD)

**Untested architectures:**
- aarch64 (64-bit ARM)
- armv7 (32-bit ARM)
- i686 (32-bit Intel/AMD)

**Note:** `tmstat-rs` should still work on untested architectures, since there's no architecture
specific code.

## Features

- **CPU usage**: Track percentage of time spent non-idle
- **Memory usage**: Monitor percentage of memory in use
- **Network traffic**: Display bytes received and transmitted with SI units
- **Load averages**: Show 1, 5, and 15-minute load averages
- **Color gradients**: Automatic green-to-red color coding based on usage levels
- **Efficient caching**: Minimizes overhead when used in tmux status bars
- **Flexible formatting**: Tmux-like format strings for complete customization

## Installation

Build from source:

```bash
git clone https://github.com/yourusername/tmstat-rs
cd tmstat-rs
cargo build --release
```

## Usage

### Basic Example

```bash
tmstat-rs -F "CPU: {c}% Mem: {m}%"
```

### In tmux Configuration

Add to your `~/.tmux.conf`:

```tmux
set -g status-interval 5
set -g status-right "#(tmstat-rs -F 'CPU:{c}% #[fg=#{gc}]■#[default] RAM:{m}% #[fg=#{gm}]■#[default]' #{client_pid} 5)"
```

This displays CPU and memory usage with color-coded indicators that update every 5 seconds.

### Advanced Example

```console
tmstat-rs -F "↓{d}{du} ↑{u}{uu} | Load:{l1} {l5} {l15}" -e 4:6.1 -i eth0 -p #{client_pid} 10
```

This shows network traffic on `eth0`, load averages, with values formatted to 4-6 characters wide and 1 decimal place precision, using physical cores for load calculations.

## Format Strings

The `-F` option accepts a format string with replacement patterns:

### CPU Usage
- `{c}` - CPU percentage (non-idle time)
- `{gc}` - Color gradient (green to red based on CPU usage)

### Memory Usage
- `{m}` - Memory percentage
- `{gm}` - Color gradient (green to red based on memory usage)

### Network Traffic
- `{d}` - Bytes downloaded (received)
- `{u}` - Bytes uploaded (transmitted)
- `{du}` - Download units (K, M, G, T)
- `{uu}` - Upload units (K, M, G, T)

### Load Averages
- `{l1}`, `{gl1}` - 1-minute load average and gradient
- `{l5}`, `{gl5}` - 5-minute load average and gradient
- `{l15}`, `{gl15}` - 15-minute load average and gradient

Load average gradients are calculated relative to the number of CPU cores.

## Options

### Required

- `-F <FORMAT>` - Format string for output (see Format Strings above)

### Optional Arguments

- `[CLIENT_PID]` - Tmux client PID for cache identification (required when using cached patterns)
- `[STATUS_INTERVAL]` - Status refresh interval in seconds (default: 15)

### Flags

- `-e, --extras <MIN>:<MAX>.<PRECISION>` - Format number display
  - `MIN` - Minimum width (right-padded with spaces)
  - `MAX` - Maximum width (shows overflow as `9999+`)
  - `PRECISION` - Decimal places (f64 precision, default: 0)
  - Example: `-e 3:5.1` for 3-5 characters wide with 1 decimal place

- `-i, --interfaces <NAME1[/NAME2]...>` - Monitor specific network interfaces
  - Default: All interfaces except loopback
  - Use `..` to include loopback
  - Examples: `-ieth0`, `-ieth0/wlan0`, `-ieth0 -iwlan0`

- `-p, --physical` - Use physical CPU cores instead of logical cores for load average gradient calculations

- `-c, --cachedir <DIR>` - Cache directory location (default: `/tmp/tmstat/`)

- `-h, --help` - Show help (use `--help` for detailed information)

- `-V, --version` - Display version information

## Fun (?) reads

### How it works and stuff

Firstly, `tmstat-rs` runs *only* when it's invoked with `#()`. It is as "frequent" as the
`status-interval` your client is using. Users with lower `status-interval`s (say 1 second) will see
more sensible stats than users with a higher `status-interval` (say 5 second).

The first time it is invoked it finds no past or "old" stats, so it only caches the current stats
in a temp file with the name `XYZ` for a passed client_pid of `XYZ`) in the cachedir[^1], and sets
(appends) two hooks[^2] at `client-detached` and `session-closed` for the removal of that cache file.

For every subsequent run it updates the cache, and prints new usage stats to stdout.

Additionally, you could also redirect the stderr of `tmstat-rs` inside the same `#()` with
`2> /tmp/tmcpu.log` if something's not working.

[^1]: cachedir is `/tmp/tmcpu/` by default.

[^2]: The same hook is set for both `client-detached` and `session-closed`. Not setting hooks can
be achieved with `--no-hook`.

### ANOTHER system stats monitor?

While looking for a system stats monitor for my `tmux` statusline, I looked through some options, but
was kind of annoyed by the fact that all (couldn't have been more than one or two) of them `sleep`
every time `tmux` evaluates `#()`'s. Taking a *noticeable* time to end execution, throwing away
current values, I wasn't happy. So, I wrote one in Rust that cached current stats in `/tmp` for the
next delta calculation for CPU usage. I looked once more, and found
[tmux-plugins/tmux-cpu](https://github.com/tmux-plugins/tmux-cpu). Although `tmstat-rs`'s code was
not inspired by [tmux-plugins/tmux-cpu](https://github.com/tmux-plugins/tmux-cpu), they're quite
similar, as they both use caching. Then, I thought "let's make it use-able by people" and here we
are. I would say the (deemed) *selling point* of `tmstat-rs` would be caching.

## Limitations

- You cannot use the same replacement pattern in multiple `#()` blocks for the same attached tmux client
- Cache files are not automatically cleaned up; consider setting up a cron job to periodically clear the cache directory

## Performance

tmstat-rs is designed to be lightweight and fast, making it suitable for frequent updates in tmux status bars with negligible overhead.

## License

This project is licensed under either of

* Apache License, Version 2.0, ([Apache-2.0](./LICENSES/Apache-2.0) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([MIT](./LICENSES/MIT) or http://opensource.org/licenses/MIT)

at your option.
