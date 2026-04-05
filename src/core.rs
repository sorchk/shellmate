use std::time::Instant;

use crate::command::CommandGenerator;
use crate::config::Config;
use crate::context::ShellContext;
use crate::error::AppError;
use crate::llm;
use crate::security::{CheckResult, SecurityChecker};
use crate::ui::{Stats, TerminalUi};

#[derive(Debug, Clone)]
pub enum State {
    Idle,
    Thinking,
    Processing,
    Success,
    Blocked(String),
    Error(String),
}

pub enum ProcessResult {
    Command(String),
    Blocked(String),
    Error(String),
}

pub struct CoreEngine {
    generator: CommandGenerator,
    checker: SecurityChecker,
    ui: TerminalUi,
}

impl CoreEngine {
    pub fn new(config: &Config) -> Result<Self, AppError> {
        let provider = llm::create_provider(&config.llm)?;
        let generator = CommandGenerator::new(provider, config.llm.max_tokens);
        let checker = SecurityChecker::new(&config.security)?;
        let ui = TerminalUi::new(&config.ui);
        Ok(Self {
            generator,
            checker,
            ui,
        })
    }

    pub fn process(&self, input: &str, shell: &str) -> ProcessResult {
        self.ui.show_thinking();
        let start = Instant::now();

        let context = ShellContext::build(shell);

        let generated = match self.generator.generate(input, &context) {
            Ok(cmd) => cmd,
            Err(e) => {
                self.ui.show_error(&e.to_string());
                return ProcessResult::Error(e.to_string());
            }
        };

        let elapsed = start.elapsed().as_secs_f64();
        let stats = Stats {
            elapsed_secs: elapsed,
            total_tokens: generated.usage.total_tokens,
            completion_tokens: generated.usage.completion_tokens,
            first_token_ms: elapsed * 1000.0,
        };

        match self.checker.check_command(&generated.command) {
            CheckResult::Pass => {
                self.ui.show_success(&stats);
                ProcessResult::Command(generated.command)
            }
            CheckResult::Blocked(cmd) => {
                self.ui.show_blocked(&stats, &cmd);
                ProcessResult::Blocked(cmd)
            }
        }
    }
}
