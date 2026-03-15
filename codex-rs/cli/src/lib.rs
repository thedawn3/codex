pub mod debug_sandbox;
mod exit_status;
pub mod login;

use clap::Parser;
use codex_utils_cli::CliConfigOverrides;

#[derive(Debug, Parser)]
pub struct SeatbeltCommand {
    /// Convenience alias for low-friction sandboxed automatic execution (network-disabled sandbox that can write to cwd and TMPDIR)
    /// 快捷自动模式：禁网沙箱，但允许写入当前目录和 TMPDIR。
    #[arg(long = "full-auto", default_value_t = false)]
    pub full_auto: bool,

    /// While the command runs, capture macOS sandbox denials via `log stream` and print them after exit
    /// 运行期间抓取 macOS 沙箱拒绝日志，并在退出后打印。
    #[arg(long = "log-denials", default_value_t = false)]
    pub log_denials: bool,

    #[clap(skip)]
    pub config_overrides: CliConfigOverrides,

    /// Full command args to run under seatbelt.
    /// 要在 Seatbelt 沙箱中运行的完整命令及参数。
    #[arg(trailing_var_arg = true)]
    pub command: Vec<String>,
}

#[derive(Debug, Parser)]
pub struct LandlockCommand {
    /// Convenience alias for low-friction sandboxed automatic execution (network-disabled sandbox that can write to cwd and TMPDIR)
    /// 快捷自动模式：禁网沙箱，但允许写入当前目录和 TMPDIR。
    #[arg(long = "full-auto", default_value_t = false)]
    pub full_auto: bool,

    #[clap(skip)]
    pub config_overrides: CliConfigOverrides,

    /// Full command args to run under landlock.
    /// 要在 Landlock 沙箱中运行的完整命令及参数。
    #[arg(trailing_var_arg = true)]
    pub command: Vec<String>,
}

#[derive(Debug, Parser)]
pub struct WindowsCommand {
    /// Convenience alias for low-friction sandboxed automatic execution (network-disabled sandbox that can write to cwd and TMPDIR)
    /// 快捷自动模式：禁网沙箱，但允许写入当前目录和 TMPDIR。
    #[arg(long = "full-auto", default_value_t = false)]
    pub full_auto: bool,

    #[clap(skip)]
    pub config_overrides: CliConfigOverrides,

    /// Full command args to run under Windows restricted token sandbox.
    /// 要在 Windows 受限令牌沙箱中运行的完整命令及参数。
    #[arg(trailing_var_arg = true)]
    pub command: Vec<String>,
}
