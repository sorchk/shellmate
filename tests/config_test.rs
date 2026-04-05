use shellmate::config::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_default_config() {
    let config = Config::default();

    assert_eq!(config.trigger.prefixes, vec!["@ai", "#ai", "/ai"]);
    assert_eq!(config.trigger.shortcut, "Ctrl+G");

    assert_eq!(config.llm.provider, "openai");
    assert_eq!(config.llm.model, "gpt-4-turbo");
    assert_eq!(config.llm.timeout, 30);
    assert!(config.llm.api_key.is_none());
    assert!(config.llm.base_url.is_none());
    assert!(config.llm.api_type.is_none());
    assert!(config.llm.max_tokens.is_none());

    assert_eq!(config.security.mode, "strict");
    assert_eq!(
        config.security.block_patterns,
        vec![
            "rm",
            "mkfs",
            "mkfs.ext4",
            "dd",
            "wipefs",
            "fdisk",
            "parted",
            "sfdisk",
            "shred",
            "-delete",
            "> /dev/",
            "cfdisk",
            "gdisk",
            "sgdisk",
            "blkdiscard",
            "halt",
            "killall",
            "iptables -F",
            "--no-preserve-root",
            "-exec",
            "apt remove",
            "apt purge",
            "| sh",
            "| bash",
            "chmod -R 777 /",
            "shutdown",
            "reboot",
            "poweroff",
            "init 0",
            "init 1",
            "init 6",
            ":(){:|:&};:",
        ]
    );

    assert_eq!(config.ui.position, "top");
    assert_eq!(config.ui.success_duration, 2600);
}

#[test]
fn test_config_serialize_deserialize() {
    let config = Config::default();
    let yaml = serde_yaml::to_string(&config).expect("serialize failed");
    let loaded: Config = serde_yaml::from_str(&yaml).expect("deserialize failed");
    assert_eq!(config, loaded);
}

#[test]
fn test_config_roundtrip_via_file() {
    let tmp_dir = TempDir::new().expect("tempdir failed");
    let config_path = tmp_dir.path().join("config.yaml");

    let config = Config::default();
    let yaml = serde_yaml::to_string(&config).expect("serialize failed");
    fs::write(&config_path, &yaml).expect("write failed");

    let content = fs::read_to_string(&config_path).expect("read failed");
    let loaded: Config = serde_yaml::from_str(&content).expect("deserialize failed");

    assert_eq!(config, loaded);
}

#[test]
fn test_config_load_or_default_missing_file() {
    let config = Config::load_or_default();
    assert_eq!(config.trigger.prefixes, vec!["@ai", "#ai", "/ai"]);
    assert_eq!(config.llm.provider, "openai");
    assert_eq!(config.security.mode, "strict");
}

#[test]
fn test_config_save_creates_directory() {
    let tmp_dir = TempDir::new().expect("tempdir failed");
    let nested = tmp_dir.path().join(".shellmate_test_nested");
    assert!(!nested.exists());

    let config_path = nested.join("config.yaml");
    let config = Config::default();
    let yaml = serde_yaml::to_string(&config).expect("serialize failed");
    fs::create_dir_all(&nested).expect("create_dir failed");
    fs::write(&config_path, &yaml).expect("write failed");

    assert!(nested.exists());
    assert!(config_path.exists());

    let loaded: Config =
        serde_yaml::from_str(&fs::read_to_string(&config_path).expect("read failed"))
            .expect("deserialize failed");
    assert_eq!(config, loaded);
}
