pub const PID: &str = color_print::cstr!("Pass in <bold>#{client_pid}</>");

pub const PID_LONG: &str = color_print::cstr!(
    "Pass in <bold>#{client_pid}</>. Used to uniquely identify caches when multiple tmux clients \
are calling <bold>tmstat-rs</>. Required if format string has usage strings (replacement patterns) \
that require caching."
);

pub const STATUS_INTERVAL: &str =
    color_print::cstr!("<bold>status-interval</> of the client calling <bold>tmstat-rs</>");

pub const STATUS_INTERVAL_LONG: &str = color_print::cstr!(
    "<bold>status-interval</> (in seconds) of the client calling <bold>tmstat-rs</>. This is ONLY \
used to check if caches are recent enough. To specify the <bold>status-interval</> you are using, \
please do not just put <bold>$(tmux\u{00A0}show\u{00A0}-gqv\u{00A0}status-interval)</> here. \
Hardcoding is always faster than running a command. Unused if format string does not have usage \
strings (replacement patterns) that require caching."
);

pub const FORMAT_LONG: &str = color_print::cstr!(
    "Tmux-like format string to use for printing stats. Patterns of the form <bold>{x}</> are \
replaced by (opinionated) appropriate values or stats and of the form <bold>{gx}</> are replaced \
by hexadecimal colors from green to red going from low to high values of <bold>{x}</>. Suited for \
tmux style string colors. The replacement patterns are:
    CPU:     <bold>{c}</>   <bold>{gc}</>   Percentage of time spent non-idle in the last \
<bold>-g</>/<bold>-s</>/<bold>-w</>/<bold>-p</> <bold>status-interval</> seconds
    Memory:  <bold>{m}</>   <bold>{gm}</>   Percentage of memory in use
    Network: <bold>{d}</>   <bold>{u}</>    Bytes received and transmitted in the last \
<bold>-g</>/<bold>-s</>/<bold>-w</>/<bold>-p</> <bold>status-interval</> seconds
             <bold>{du}</>  <bold>{uu}</>   SI units (<bold>K</>, <bold>M</>, <bold>G</>, <bold>T</>)
    LoadAvg: <bold>{l1}</>  <bold>{gl1}</>  Load average over the last 1 minute
             <bold>{l5}</>  <bold>{gl5}</>  Load average over the last 5 minutes
             <bold>{l15}</> <bold>{gl15}</> Load average over the last 15 minutes
             Gradients for loads are calculated in respect to no. of cores (see \
<bold>--physical</>)
One shortcoming of <bold>tmstat-rs</> is that you cannot use the same replacement patterns in \
mutiple <bold>#()</>'s for the same attached client."
);

pub const EXTRAS_LONG: &str = color_print::cstr!(
    "Mininum width, maximum width, and precision to apply to usage strings. Stats are padded to \
right with spaces for minimum width. Precision is an f64 precision, 0 by default. Precision will \
be overridden to satisfy maximum width. If the integer portion exceeds maximum width, it displays \
e.g. 9999+ for a maximum width of 4."
);

pub const INTERFACE_LONG: &str = color_print::cstr!(
    "Show network traffic for specific interface(s). By default, all interfaces' combined traffic \
is shown, except <bold>lo</> (loopback). A value of <bold>..</> (two U+002E's) will include \
loopback. <bold>-iabc/def</> or <bold>-iabc</> <bold>-idef</> will show traffic for both \
<bold>abc</> and <bold>def</>."
);

pub const PHYSICAL_LONG: &str = "Calculation of load average gradients uses no. of logical cores \
by default. This will change it to the no. of physical cores.";

pub const CACHEDIR_LONG: &str = "Directory to cache stats in. Different cache files are used for \
different stats, each taking up 16 bytes of space. <bold>tmstat-rs</> does not clean these up from \
time to time. You may want to set up a cron job for this.";
