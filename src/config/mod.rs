use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use tokio::fs;
use log::debug;
use crate::repo::CheckOutInfo;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Repository {
    pub name: String,
    pub organization: String,
    pub cloned_locally: bool,
    pub checkout_info: CheckOutInfo,
}
#[derive(Serialize, Deserialize)]
pub struct Configuration {
    configuration_path: String,
    configuration_file_name: String,
    configuration_full_path: String,
    repository: Vec<Repository>,
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

        if let Ok(exists) = fs::try_exists(&self.configuration_full_path).await{
            if !exists {
               debug!("Configuration file does not exist");
               // Create
               self.save().await?;
            }else {
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