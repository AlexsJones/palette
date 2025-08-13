mod repo;
mod config;

use clap::{Parser, Subcommand};
use anyhow::Error;
use crate::config::{Configuration, Loads, Repository, Saves};
use crate::repo::{Manager, Pulls};

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
enum Command {
    Push {},
    Pull {},
    List {},
    Add {
        #[clap(short, long)]
        organization: String,
        #[clap(short, long)]
        name: String,
    },
    Remove {
        #[clap(short, long)]
        name: String,
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    env_logger::init();

    let repo_manager = Manager::default();
    let mut configuration_manager = Configuration::default();
    // Load configuration if it is already present
    configuration_manager.load().await.expect("Could not load configuration, something went wrong!");
    let args = Args::parse();

    match args.command {
        Command::Push { .. } => {
            // List each repos commit locally vs the remote
            for repo in configuration_manager.get_repository() {

            }
            // confirm action
            // push
        }
        Command::Pull { .. } => {
            // get each repo and update
            for repo in configuration_manager.get_repository() {
                repo_manager.update(repo)?
            }
        }
        Command::List { .. } => {
            for repo in configuration_manager.get_repository() {
                println!("{} branch:{} commit:{} checked out: {}", repo.name, repo.checkout_info.branch_name, repo.checkout_info.commit_sha, if repo.cloned_locally { "yes"} else { "no" });

            }
        }
        Command::Add { organization, name } => {

            // Add the repository to the configuration index
            let mut repository = Repository::default();
            repository.name = name.clone();
            repository.organization = organization.clone();
            configuration_manager.add_repository(repository.clone());
            configuration_manager.save().await.expect("Could not save configuration");
            // Pull the repository and update the index
            let checkout_info = repo_manager.clone(organization, name.clone())?;
            
            let saved_repo = configuration_manager.get_repository_mut(name);
            saved_repo.checkout_info = checkout_info.clone();
            saved_repo.cloned_locally = true;
            configuration_manager.save().await.expect("Could not save configuration");
            
        }
        Command::Remove {  .. } => {}
    }
    Ok(())
}
