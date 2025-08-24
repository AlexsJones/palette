use palette::config::{Configuration, Repository};
use palette::repo::CheckOutInfo;
use tempfile::tempdir;

#[tokio::test]
async fn test_configuration_integration() {
    let temp_dir = tempdir().expect("Failed to create temp dir");
    let config_path = temp_dir.path().join("config.palette");

    let mut config = Configuration {
        configuration_path: temp_dir.path().to_str().unwrap().to_string(),
        configuration_file_name: "config.palette".to_string(),
        configuration_full_path: config_path.to_str().unwrap().to_string(),
        repository: vec![],
    };

    // Test adding repositories
    let repo1 = Repository {
        name: "repo1".to_string(),
        organization: "org1".to_string(),
        cloned_locally: true,
        checkout_info: CheckOutInfo {
            branch_name: "main".to_string(),
            commit_sha: "abc123".to_string(),
        },
    };

    let repo2 = Repository {
        name: "repo2".to_string(),
        organization: "org2".to_string(),
        cloned_locally: false,
        checkout_info: CheckOutInfo {
            branch_name: "develop".to_string(),
            commit_sha: "def456".to_string(),
        },
    };

    config.add_repository(repo1);
    config.add_repository(repo2);

    assert_eq!(config.get_repository().len(), 2);
    assert_eq!(config.get_repository()[0].name, "repo1");
    assert_eq!(config.get_repository()[1].name, "repo2");
}

#[tokio::test]
async fn test_checkout_info_operations() {
    let mut checkout = CheckOutInfo::default();
    assert!(checkout.branch_name.is_empty());
    assert!(checkout.commit_sha.is_empty());

    checkout.branch_name = "feature-branch".to_string();
    checkout.commit_sha = "abcd1234".to_string();

    // Test cloning
    let cloned = checkout.clone();
    assert_eq!(cloned.branch_name, "feature-branch");
    assert_eq!(cloned.commit_sha, "abcd1234");

    // Test serialization/deserialization
    let serialized = serde_json::to_string(&checkout).expect("Serialization failed");
    let deserialized: CheckOutInfo =
        serde_json::from_str(&serialized).expect("Deserialization failed");

    assert_eq!(deserialized.branch_name, "feature-branch");
    assert_eq!(deserialized.commit_sha, "abcd1234");
}

#[tokio::test]
async fn test_repository_operations() {
    let mut repo = Repository::default();
    assert!(repo.name.is_empty());
    assert!(repo.organization.is_empty());
    assert!(!repo.cloned_locally);

    // Test updating fields
    repo.name = "test-repo".to_string();
    repo.organization = "test-org".to_string();
    repo.cloned_locally = true;
    repo.checkout_info.branch_name = "main".to_string();
    repo.checkout_info.commit_sha = "xyz789".to_string();

    // Test cloning
    let cloned_repo = repo.clone();
    assert_eq!(cloned_repo.name, "test-repo");
    assert_eq!(cloned_repo.organization, "test-org");
    assert!(cloned_repo.cloned_locally);
    assert_eq!(cloned_repo.checkout_info.branch_name, "main");
    assert_eq!(cloned_repo.checkout_info.commit_sha, "xyz789");

    // Test serialization
    let json = serde_json::to_string_pretty(&repo).expect("Serialization failed");
    assert!(json.contains("test-repo"));
    assert!(json.contains("test-org"));
    assert!(json.contains("main"));
    assert!(json.contains("xyz789"));

    let deserialized: Repository = serde_json::from_str(&json).expect("Deserialization failed");
    assert_eq!(deserialized.name, repo.name);
    assert_eq!(deserialized.organization, repo.organization);
    assert_eq!(deserialized.cloned_locally, repo.cloned_locally);
}
