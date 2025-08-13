mod repo;
mod config;
use regex::Regex;
use std::io;
use clap::{Parser, Subcommand};
use anyhow::Error;
use crate::config::{Configuration, Loads, Repository, Saves};
use crate::repo::{Manager, Pulls, Pushes};

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
            // List each repo commits locally vs the remote
            let mut candidate_updates = vec![];
            for repo in configuration_manager.get_repository() {

                if let Ok((is_different, commit)) = repo_manager.compare(repo) {
                        println!("{}: {} ",repo.name,  commit );
                    if is_different && candidate_updates.len() > 0 {
                        candidate_updates.push(repo);
                    }
                }
            }
            println!("Please confirm that you wish to push repositories [y/N]");
            // confirm action
            let mut input: String = String::new(); // Create a string variable
            io::stdin() // Get the standard input stream
                .read_line(&mut input) // The read_line function reads data until it reaches a '\n' character
                .expect("Unable to read Stdin"); // In case the read operation fails, it panics with the given message
            let re = Regex::new("(?i)^n").unwrap(); // (?i) = case-insensitive, ^n = starts with 'n'
            if re.is_match(&input) {
                return Ok(());
            }
            
            for repo in candidate_updates {
                println!("Pushing {}", repo.name);
                let repo_manager = repo_manager.clone();
                let name = repo.name.clone();
                tokio::spawn(async move {
                     repo_manager.push(name).await.expect("Repo Manager");
                });
            }
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

            println!("Adding repository...");
            // Add the repository to the configuration index
            let mut repository = Repository::default();
            repository.name = name.clone();
            repository.organization = organization.clone();
            configuration_manager.add_repository(repository.clone());
            configuration_manager.save().await.expect("Could not save configuration");
            // Pull the repository and update the index
            let checkout_info = repo_manager.clone_repo(organization, name.clone())?;
            
            let saved_repo = configuration_manager.get_repository_mut(name);
            saved_repo.checkout_info = checkout_info.clone();
            saved_repo.cloned_locally = true;
            configuration_manager.save().await.expect("Could not save configuration");
            
        }
        Command::Remove {  name } => {
            println!("Removing repository...");
        }
    }
    Ok(())
}
