use crate::error::AppError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TriggerConfig {
    #[serde(default = "TriggerConfig::default_prefixes")]
    pub prefixes: Vec<String>,
    #[serde(default = "TriggerConfig::default_shortcut")]
    pub shortcut: String,
}

impl TriggerConfig {
    fn default_prefixes() -> Vec<String> {
        vec!["@ai".into(), "#ai".into(), "/ai".into()]
    }

    fn default_shortcut() -> String {
        "Ctrl+G".into()
    }
}

impl Default for TriggerConfig {
    fn default() -> Self {
        Self {
            prefixes: Self::default_prefixes(),
            shortcut: Self::default_shortcut(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct LlmConfig {
    #[serde(default = "LlmConfig::default_provider")]
    pub provider: String,
    #[serde(default = "LlmConfig::default_model")]
    pub model: String,
    #[serde(default = "LlmConfig::default_timeout")]
    pub timeout: u64,
    #[serde(default)]
    pub api_key: Option<String>,
    #[serde(default)]
    pub base_url: Option<String>,
    #[serde(default)]
    pub api_type: Option<String>,
    #[serde(default)]
    pub max_tokens: Option<u32>,
}

impl LlmConfig {
    pub fn is_configured(&self) -> bool {
        self.api_key.is_some()
            || self.provider != "openai"
            || self.model != "gpt-4-turbo"
            || self.base_url.is_some()
            || self.api_type.is_some()
    }

    fn default_provider() -> String {
        "openai".into()
    }

    fn default_model() -> String {
        "gpt-4-turbo".into()
    }

    fn default_timeout() -> u64 {
        30
    }
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            provider: Self::default_provider(),
            model: Self::default_model(),
            timeout: Self::default_timeout(),
            api_key: None,
            base_url: None,
            api_type: None,
            max_tokens: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SecurityConfig {
    #[serde(default = "SecurityConfig::default_mode")]
    pub mode: String,
    #[serde(default = "SecurityConfig::default_block_patterns")]
    pub block_patterns: Vec<String>,
}

impl SecurityConfig {
    fn default_mode() -> String {
        "strict".into()
    }

    fn default_block_patterns() -> Vec<String> {
        vec![
            "rm".into(),
            "mkfs".into(),
            "mkfs.ext4".into(),
            "dd".into(),
            "wipefs".into(),
            "fdisk".into(),
            "parted".into(),
            "sfdisk".into(),
            "shred".into(),
            "-delete".into(),
            "> /dev/".into(),
            "cfdisk".into(),
            "gdisk".into(),
            "sgdisk".into(),
            "blkdiscard".into(),
            "halt".into(),
            "killall".into(),
            "iptables -F".into(),
            "--no-preserve-root".into(),
            "-exec".into(),
            "apt remove".into(),
            "apt purge".into(),
            "| sh".into(),
            "| bash".into(),
            "chmod -R 777 /".into(),
            "shutdown".into(),
            "reboot".into(),
            "poweroff".into(),
            "init 0".into(),
            "init 1".into(),
            "init 6".into(),
            ":(){:|:&};:".into(),
        ]
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            mode: Self::default_mode(),
            block_patterns: Self::default_block_patterns(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UiConfig {
    #[serde(default = "UiConfig::default_position")]
    pub position: String,
    #[serde(default = "UiConfig::default_success_duration")]
    pub success_duration: u64,
}

impl UiConfig {
    fn default_position() -> String {
        "top".into()
    }

    fn default_success_duration() -> u64 {
        2600
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            position: Self::default_position(),
            success_duration: Self::default_success_duration(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct Config {
    #[serde(default)]
    pub trigger: TriggerConfig,
    #[serde(default)]
    pub llm: LlmConfig,
    #[serde(default)]
    pub security: SecurityConfig,
    #[serde(default)]
    pub ui: UiConfig,
}

impl Config {
    pub fn config_dir() -> Result<PathBuf, AppError> {
        dirs::home_dir()
            .map(|p| p.join(".shellmate"))
            .ok_or_else(|| AppError::ConfigError("Cannot determine home directory".into()))
    }

    pub fn config_path() -> Result<PathBuf, AppError> {
        Config::config_dir().map(|d| d.join("config.yaml"))
    }

    pub fn load() -> Result<Self, AppError> {
        let path = Config::config_path()?;
        let content = fs::read_to_string(&path)
            .map_err(|e| AppError::ConfigError(format!("Failed to read config: {}", e)))?;
        serde_yaml::from_str(&content)
            .map_err(|e| AppError::ConfigError(format!("Failed to parse config: {}", e)))
    }

    pub fn save(&self) -> Result<(), AppError> {
        let dir = Config::config_dir()?;
        fs::create_dir_all(&dir)
            .map_err(|e| AppError::ConfigError(format!("Failed to create config dir: {}", e)))?;
        let path = Config::config_path()?;
        let content = serde_yaml::to_string(self)
            .map_err(|e| AppError::ConfigError(format!("Failed to serialize config: {}", e)))?;
        fs::write(&path, content)
            .map_err(|e| AppError::ConfigError(format!("Failed to write config: {}", e)))
    }

    pub fn load_or_default() -> Self {
        Config::load().unwrap_or_default()
    }
}
