use crate::repo::CheckOutInfo;
use log::debug;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Repository {
    pub name: String,
    pub organization: String,
    pub cloned_locally: bool,
    pub checkout_info: CheckOutInfo,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Configuration {
    pub configuration_path: String,
    pub configuration_file_name: String,
    pub configuration_full_path: String,
    pub repository: Vec<Repository>,
}
pub(crate) trait Saves {
    async fn save(&self) -> Result<(), anyhow::Error>;
}

pub(crate) trait Loads {
    async fn load(&mut self) -> Result<(), anyhow::Error>;
}

impl Saves for Configuration {
    async fn save(&self) -> Result<(), anyhow::Error> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(self.configuration_full_path.clone(), content).await?;
        Ok(())
    }
}

impl Loads for Configuration {
    async fn load(&mut self) -> Result<(), anyhow::Error> {
        if let Ok(exists) = fs::try_exists(&self.configuration_full_path).await {
            if !exists {
                debug!("Configuration file does not exist");
                // Create
                self.save().await?;
            } else {
                let content = fs::read_to_string(self.configuration_full_path.clone()).await?;
                // set the in-memory configuration
                *self = serde_json::from_str::<Configuration>(&content)?;
            }
        }
        Ok(())
    }
}

impl Default for Configuration {
    fn default() -> Self {
        let default_path = ".".to_string();
        let default_name = "config.palette".to_string();
        let mut path = PathBuf::new();
        path.push(default_path.clone());
        path.push(default_name.clone());
        Configuration {
            configuration_path: default_path,
            configuration_file_name: default_name,
            configuration_full_path: path.as_path().to_str().unwrap().to_string(),
            repository: vec![],
        }
    }
}

impl Configuration {
    pub fn add_repository(&mut self, repository: Repository) {
        self.repository.push(repository);
    }
    pub fn get_repository_mut(&mut self, name: String) -> &mut Repository {
        self.repository.iter_mut().find(|r| r.name == name).unwrap()
    }
    pub fn get_repository(&self) -> &Vec<Repository> {
        &self.repository
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::tempdir;
    use tokio::fs;

    fn create_test_configuration(temp_dir: &str) -> Configuration {
        let config_file = "test_config.palette";
        let mut path = PathBuf::new();
        path.push(temp_dir);
        path.push(config_file);

        Configuration {
            configuration_path: temp_dir.to_string(),
            configuration_file_name: config_file.to_string(),
            configuration_full_path: path.to_str().unwrap().to_string(),
            repository: vec![],
        }
    }

    fn create_test_repository() -> Repository {
        Repository {
            name: "test-repo".to_string(),
            organization: "test-org".to_string(),
            cloned_locally: true,
            checkout_info: CheckOutInfo {
                branch_name: "main".to_string(),
                commit_sha: "abc123def456".to_string(),
            },
        }
    }

    #[tokio::test]
    async fn test_configuration_default() {
        let config = Configuration::default();
        assert_eq!(config.configuration_path, ".");
        assert_eq!(config.configuration_file_name, "config.palette");
        assert_eq!(config.configuration_full_path, "./config.palette");
        assert!(config.repository.is_empty());
    }

    #[tokio::test]
    async fn test_add_repository() {
        let mut config = Configuration::default();
        let repo = create_test_repository();

        config.add_repository(repo.clone());

        assert_eq!(config.repository.len(), 1);
        assert_eq!(config.repository[0].name, "test-repo");
        assert_eq!(config.repository[0].organization, "test-org");
        assert!(config.repository[0].cloned_locally);
    }

    #[tokio::test]
    async fn test_get_repository() {
        let mut config = Configuration::default();
        let repo = create_test_repository();
        config.add_repository(repo);

        let repos = config.get_repository();
        assert_eq!(repos.len(), 1);
        assert_eq!(repos[0].name, "test-repo");
    }

    #[tokio::test]
    async fn test_get_repository_mut() {
        let mut config = Configuration::default();
        let repo = create_test_repository();
        config.add_repository(repo);

        let repo_mut = config.get_repository_mut("test-repo".to_string());
        repo_mut.cloned_locally = false;

        assert!(!config.repository[0].cloned_locally);
    }

    #[tokio::test]
    async fn test_save_and_load_configuration() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let temp_path = temp_dir.path().to_str().unwrap();

        // Create and save configuration
        let mut config = create_test_configuration(temp_path);
        let repo = create_test_repository();
        config.add_repository(repo);

        config.save().await.expect("Failed to save configuration");

        // Verify file was created
        assert!(
            fs::try_exists(&config.configuration_full_path)
                .await
                .unwrap()
        );

        // Load configuration
        let mut loaded_config = create_test_configuration(temp_path);
        loaded_config
            .load()
            .await
            .expect("Failed to load configuration");

        assert_eq!(loaded_config.repository.len(), 1);
        assert_eq!(loaded_config.repository[0].name, "test-repo");
        assert_eq!(loaded_config.repository[0].organization, "test-org");
        assert!(loaded_config.repository[0].cloned_locally);
    }

    #[tokio::test]
    async fn test_load_nonexistent_configuration_creates_file() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let temp_path = temp_dir.path().to_str().unwrap();

        let mut config = create_test_configuration(temp_path);

        // Ensure file doesn't exist initially
        assert!(
            !fs::try_exists(&config.configuration_full_path)
                .await
                .unwrap()
        );

        config.load().await.expect("Failed to load configuration");

        // File should be created
        assert!(
            fs::try_exists(&config.configuration_full_path)
                .await
                .unwrap()
        );
        assert!(config.repository.is_empty());
    }

    #[tokio::test]
    async fn test_configuration_serialization() {
        let mut config = Configuration::default();
        let repo = create_test_repository();
        config.add_repository(repo);

        let json = serde_json::to_string_pretty(&config).expect("Failed to serialize");

        assert!(json.contains("test-repo"));
        assert!(json.contains("test-org"));
        assert!(json.contains("main"));
        assert!(json.contains("abc123def456"));
    }

    #[tokio::test]
    async fn test_repository_default() {
        let repo = Repository::default();
        assert!(repo.name.is_empty());
        assert!(repo.organization.is_empty());
        assert!(!repo.cloned_locally);
        assert!(repo.checkout_info.branch_name.is_empty());
        assert!(repo.checkout_info.commit_sha.is_empty());
    }

    #[tokio::test]
    async fn test_multiple_repositories() {
        let mut config = Configuration::default();

        let repo1 = Repository {
            name: "repo1".to_string(),
            organization: "org1".to_string(),
            cloned_locally: true,
            checkout_info: CheckOutInfo::default(),
        };

        let repo2 = Repository {
            name: "repo2".to_string(),
            organization: "org2".to_string(),
            cloned_locally: false,
            checkout_info: CheckOutInfo::default(),
        };

        config.add_repository(repo1);
        config.add_repository(repo2);

        assert_eq!(config.repository.len(), 2);
        assert_eq!(config.repository[0].name, "repo1");
        assert_eq!(config.repository[1].name, "repo2");

        let found_repo = config.get_repository_mut("repo1".to_string());
        assert_eq!(found_repo.organization, "org1");
    }
}
