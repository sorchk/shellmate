use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "shellmate",
    version,
    about = "AI-powered shell command assistant"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate a shell command from natural language
    Generate {
        /// The natural language prompt
        prompt: String,
        /// Shell type (bash/zsh/sh/fish)
        #[arg(long, default_value = "bash")]
        shell: String,
    },
    /// Check if a command passes security rules
    Check {
        /// The command to check
        command: String,
    },
    /// Show current configuration
    Config,
    /// Install shell integration
    Install {
        /// Target shell (bash/zsh/sh/auto)
        #[arg(long, default_value = "auto")]
        shell: String,
    },
}
