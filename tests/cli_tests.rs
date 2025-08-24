use std::fs;
use std::process::Command;
use tempfile::tempdir;

#[test]
fn test_cli_help() {
    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("palette") || stdout.contains("Palette"));
    assert!(
        stdout.contains("Palette simplifies the process of working with numerous repositories")
    );
}

#[test]
fn test_cli_version() {
    let output = Command::new("cargo")
        .args(["run", "--", "--version"])
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("palette"));
}

#[test]
fn test_config_file_creation() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let config_path = temp_dir.path().join("test_config.palette");

    // Create a simple config file
    let config_content = r#"{
        "configuration_path": ".",
        "configuration_file_name": "test_config.palette",
        "configuration_full_path": "./test_config.palette",
        "repository": []
    }"#;

    fs::write(&config_path, config_content).expect("Failed to write config");

    assert!(config_path.exists());
    let contents = fs::read_to_string(&config_path).expect("Failed to read config");
    assert!(contents.contains("repository"));
    assert!(contents.contains("configuration_path"));
}

#[test]
fn test_repository_json_structure() {
    let repo_json = r#"{
        "name": "test-repo",
        "organization": "test-org",
        "cloned_locally": true,
        "checkout_info": {
            "branch_name": "main",
            "commit_sha": "abc123def456"
        }
    }"#;

    // Test that the JSON can be parsed
    let parsed: serde_json::Value =
        serde_json::from_str(repo_json).expect("Failed to parse repository JSON");

    assert_eq!(parsed["name"], "test-repo");
    assert_eq!(parsed["organization"], "test-org");
    assert_eq!(parsed["cloned_locally"], true);
    assert_eq!(parsed["checkout_info"]["branch_name"], "main");
    assert_eq!(parsed["checkout_info"]["commit_sha"], "abc123def456");
}
