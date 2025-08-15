mod repo;
mod config;
use regex::Regex;
use std::io;
use clap::{Parser, Subcommand};
use anyhow::Error;
use colorize::AnsiColor;
use tokio::fs;
use crate::config::{Configuration, Loads, Repository, Saves};
use crate::repo::{Branches, Manager, Pulls, Pushes};
use std::process::Command as stdCommand;
use std::io::{Write};
#[derive(Parser)]
#[command(name = "palette")]
#[command(about = "A powerful command-line tool for managing multiple GitHub repositories")]
#[command(long_about = "Palette simplifies the process of working with numerous repositories by providing a unified interface for common Git operations across all your projects.")]
#[command(version)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
enum Command {
    #[command(about = "Push changes for repositories that are ahead of remote")]
    #[command(long_about = "Identifies repositories that have commits ahead of their remote and pushes them after user confirmation. Shows a list of repositories to be pushed and prompts for confirmation before proceeding.")]
    Push {},
    
    #[command(about = "Pull latest changes for repositories")]
    #[command(long_about = "Pull latest changes for all tracked repositories or a specific repository. Automatically clones missing repositories during bulk pull operations and updates configuration with latest checkout information.")]
    Pull {
        #[clap(short, long, help = "Name of a specific repository to pull")]
        name: Option<String>
    },
    
    #[command(about = "List all tracked repositories with their status")]
    #[command(long_about = "Display a comprehensive overview of all tracked repositories showing current branch, commit hash (first 8 characters), whether the local branch is ahead of remote, and checkout status with color-coded output.")]
    List {},
    
    #[command(about = "Switch all repositories to a specific branch")]
    #[command(long_about = "Switch all tracked repositories to the specified branch. Optionally create new branches when --create is used. Updates configuration with new checkout information.")]
    Switch {
        #[clap(short, long, help = "Name of the branch to switch to")]
        branch_name: String,
        #[clap(short, long, help = "Create the branch if it doesn't exist")]
        create: Option<bool>
    },
    
    #[command(about = "Add a new repository to track and clone it")]
    #[command(long_about = "Add a new repository to the configuration file and clone it locally. The repository will be tracked and included in future bulk operations.")]
    Add {
        #[clap(short, long, help = "GitHub organization or username that owns the repository")]
        organization: String,
        #[clap(short, long, help = "Name of the repository to add")]
        name: String,
    },
    #[command(about="Execute an arbitrary command in all repositories")]
    #[command(long_about = "Execute the specified command in each tracked repository's directory. Displays output from all repositories sequentially. Useful for running checks, builds, or any command across your entire repository collection.")]
    Exec {
        #[clap(short, long, help = "Command to execute in all tracked repositories")]
        run_command: String,
    },
    #[command(about = "Remove a repository from tracking")]
    #[command(long_about = "Remove a repository from the configuration file. Note: This command is currently under development and only prints a message.")]
    Remove {
        #[clap(short, long, help = "Name of the repository to remove from tracking")]
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
                    if is_different {
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
        },
        Command::Switch { branch_name, create} => {
            for repo in configuration_manager.get_repository() {

                let checkoutInfo = repo_manager.change_branch(branch_name.clone(), repo, if create.is_some() { create.unwrap()} else { false })?;
                println!("{} switched branch to {}", repo.name, checkoutInfo.branch_name)
            }
        },
        Command::Pull { name } => {

            if let Some(name) = name {
                // This is a hack to pass through the right data structure, but all we need is the name
                let mut repository = Repository::default();
                repository.name = name;
                repo_manager.update(&repository)?;
                return Ok(())
            }
            // get each repo and update
            for repo in configuration_manager.get_repository() {
                // If the repository doesn't exist, clone instead
                // this is a nice to have to keep palette in sync
                let repo = repo.clone();
                if let Ok(exists) =  fs::try_exists(repo.name.clone()).await {
                    if !exists {
                        println!("Repository {} was missing, fetching...", repo.name);
                        add_repo(repo.organization, repo.name, configuration_manager.clone(),
                                 repo_manager.clone(), false).await?;
                        continue;
                    }
                }
                repo_manager.update(&repo)?
            }
        }
        Command::List { .. } => {
            for repo in configuration_manager.get_repository() {

                if let Ok((is_different, commit)) = repo_manager.compare(repo) {
                    if is_different  {
                        println!("{} branch:{}, commit:{:.8}, ahead of remote: {}, checked out: {}", repo.name, repo.checkout_info.branch_name, repo.checkout_info.commit_sha.as_str(), "yes".yellow(), if repo.cloned_locally { "yes".green()} else { "no".red() });
                    }else {
                        println!("{} branch:{}, commit:{:.8}, ahead of remote: {}, checked out: {}", repo.name, repo.checkout_info.branch_name, repo.checkout_info.commit_sha.as_str(), "no".green(),  if repo.cloned_locally { "yes".green()} else { "no".red() });
                    }
                }

            }
        }
        Command::Add { organization, name } => {
            add_repo(organization, name, configuration_manager, repo_manager, true).await?;
        },
        Command::Exec { run_command } => {
            for repo in configuration_manager.get_repository() {
                // use repo name as our path
                let path = repo.clone().name;
                let output = stdCommand::new("/bin/sh")
                    .arg("-c").arg(run_command.clone())
                    .current_dir(path)
                    .output()?;
                io::stdout().write_all(&output.stdout)?;
            }
        },
        Command::Remove {  name } => {
            println!("Removing repository...");
        }
    }
    Ok(())
}

async fn add_repo(organization: String, name: String, mut configuration_manager: Configuration,
repo_manager: Manager, add_to_config: bool) -> Result<(), anyhow::Error>{
    let mut repository = Repository::default();
    if add_to_config {

        repository.name = name.clone();
        repository.organization = organization.clone();
        configuration_manager.add_repository(repository.clone());
        configuration_manager.save().await.expect("Could not save configuration");
    }
    // Pull the repository and update the index
    let checkout_info = repo_manager.clone_repo(organization, name.clone())?;
    let saved_repo = configuration_manager.get_repository_mut(name);
    saved_repo.checkout_info = checkout_info.clone();
    saved_repo.cloned_locally = true;
    configuration_manager.save().await.expect("Could not save configuration");

    Ok(())
}
