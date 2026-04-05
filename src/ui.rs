pub struct TerminalUi {
    pub position: String,
    pub success_duration_ms: u64,
}

impl TerminalUi {
    pub fn new(config: &crate::config::UiConfig) -> Self {
        Self {
            position: config.position.clone(),
            success_duration_ms: config.success_duration,
        }
    }

    pub fn show_thinking(&self) {
        eprintln!("\x1b[90m⠋ 思考中...\x1b[0m");
    }

    pub fn show_success(&self, stats: &Stats) {
        let tps = if stats.elapsed_secs > 0.0 {
            stats.completion_tokens as f64 / stats.elapsed_secs
        } else {
            0.0
        };
        eprintln!(
            "✦ 思考完成，按回车执行，按ESC取消 (耗时:{:.1}s, Tokens:{}, 首字延时:{:.0}ms, 每秒:{:.1} tokens)",
            stats.elapsed_secs, stats.total_tokens, stats.first_token_ms, tps
        );
    }

    pub fn show_blocked(&self, stats: &Stats, command: &str) {
        let tps = if stats.elapsed_secs > 0.0 {
            stats.completion_tokens as f64 / stats.elapsed_secs
        } else {
            0.0
        };
        eprintln!(
            "\x1b[33m⚠ 已拦截风险命令\x1b[0m (耗时:{:.1}s, Tokens:{}, 首字延时:{:.0}ms, 每秒:{:.1} tokens)\n  \x1b[33m{}\x1b[0m",
            stats.elapsed_secs, stats.total_tokens, stats.first_token_ms, tps, command
        );
    }

    pub fn show_error(&self, message: &str) {
        eprintln!("\x1b[2K\r\x1b[31m✗ Error: {}\x1b[0m", message);
    }

    pub fn clear_notification(&self) {
        eprint!("\x1b[2K\r");
    }
}

pub struct Stats {
    pub elapsed_secs: f64,
    pub total_tokens: u32,
    pub completion_tokens: u32,
    pub first_token_ms: f64,
}
