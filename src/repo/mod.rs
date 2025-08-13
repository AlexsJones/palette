use std::env;
use std::path::Path;
use anyhow::anyhow;
use git2::build::RepoBuilder;
use git2::{Cred, FetchOptions, PushOptions, RemoteCallbacks, Status, StatusOptions};
use serde::{Deserialize, Serialize};
use crate::config::Repository;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckOutInfo {
    pub branch_name: String,
    pub commit_sha: String,
}
pub(crate) trait Pulls {
    fn clone_repo(&self, organization: String, name: String) -> Result<CheckOutInfo, anyhow::Error>;
    fn update(&self, repo: &crate::config::Repository) -> Result<(), anyhow::Error>;
}
pub(crate) trait Pushes {
    async fn push(&self, name: String) -> Result<(), anyhow::Error>;
    fn compare(
        &self,
        repo: &crate::config::Repository,
    ) -> Result<(bool, String), anyhow::Error>;
}

#[derive(Default, Clone)]
pub(crate) struct Manager{

}
impl Pulls for Manager {
    fn clone_repo(&self, org_name: String, name: String) -> Result<CheckOutInfo, anyhow::Error> {
        let repo_url = format!(
            "git@github.com:{}/{}.git",
            org_name,
            name
        );
        let mut builder = RepoBuilder::new();
        let mut fetch_options = FetchOptions::new();
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, _username_from_url, _allowed_types| {
            Cred::ssh_key(
                "git",
                Some(std::path::Path::new(&format!(
                    "{}/.ssh/id_rsa.pub",
                    env::var("HOME").unwrap()
                ))),
                std::path::Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
                None,
            )
        });
        fetch_options.remote_callbacks(callbacks);
        builder.fetch_options(fetch_options);

        let checked_out = builder
            .clone(repo_url.as_str(), Path::new(name.as_str()));

        // Save the git commit hash and branch to the config
        let checked_out = checked_out?;

        let branch_name = checked_out.head()?;
        let commit = checked_out.head()?.peel_to_commit()?.id().to_string();

        println!(
            "Check out complete, branch is {} at commit {} ",
            branch_name.name().clone().unwrap(),
            commit.clone()
        );
        Ok(CheckOutInfo{
            branch_name: branch_name.name().unwrap().to_string(),
            commit_sha: commit.clone(),
        })
    }
    fn update(&self, repo: &crate::config::Repository) -> Result<(), anyhow::Error> {

        let r = repo.clone();
        let repo = git2::Repository::open(r.name.clone())?;

        // 2. Prepare callbacks for SSH credentials
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, _username_from_url, _allowed_types| {
            Cred::ssh_key(
                "git",
                Some(Path::new(&format!("{}/.ssh/id_rsa.pub", env::var("HOME").unwrap()))),
                Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
                None,
            )
        });

        // 3. Set up fetch options
        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);

        // 4. Fetch from origin
        let mut remote = repo.find_remote("origin")?;
        remote.fetch(&["main"], Some(&mut fetch_options), None)?; // branch could be param

        // 5. Get the updated branch tip
        let fetch_head = repo.find_reference("FETCH_HEAD")?;
        let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;

        // 6. Merge into current branch
        let analysis = repo.merge_analysis(&[&fetch_commit])?;
        if analysis.0.is_fast_forward() {
            let refname = format!("refs/heads/main"); // branch param here too
            let mut reference = repo.find_reference(&refname)?;
            reference.set_target(fetch_commit.id(), "Fast-Forward")?;
            repo.set_head(&refname)?;
            repo.checkout_head(Some(
                git2::build::CheckoutBuilder::default().force(),
            ))?;
            println!("Fast-forwarded to {}", fetch_commit.id());
        } else if analysis.0.is_normal() {
            repo.merge(&[&fetch_commit], None, None)?;
            println!("Merged changes into current branch");
        } else {
            println!("{} already up-to-date", r.name);
        }
        Ok(())
    }
}
impl Pushes for Manager {


    fn compare(
        &self,
        repo: &crate::config::Repository,
    ) -> Result<(bool, String), anyhow::Error> {
        let r = repo.clone();
        let repo = git2::Repository::open(r.name.clone())?;

        // 0. Check for unstaged changes
        let mut status_opts = StatusOptions::new();
        status_opts.include_untracked(true).recurse_untracked_dirs(true);
        let statuses = repo.statuses(Some(&mut status_opts))?;

        if !statuses.is_empty() {
            println!("⚠️  Warning: There are uncommitted changes in '{}'", r.name);
            for entry in statuses.iter() {
                let s = entry.status();
                let path = entry.path().unwrap_or("<unknown>");
                if s.contains(Status::WT_MODIFIED) {
                    println!("  - Modified: {}", path);
                }
                if s.contains(Status::WT_NEW) {
                    println!("  - Untracked: {}", path);
                }
                if s.contains(Status::INDEX_MODIFIED) {
                    println!("  - Staged change: {}", path);
                }
            }
        }

        // 1. Get local branch tip commit SHA
        let head = repo.head()?;
        let local_commit = head.peel_to_commit()?.id();

        // 2. Prepare SSH callbacks
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, _username_from_url, _allowed_types| {
            Cred::ssh_key(
                "git",
                Some(Path::new(&format!("{}/.ssh/id_rsa.pub", env::var("HOME").unwrap()))),
                Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
                None,
            )
        });

        // 3. Fetch remote without merging
        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);

        let mut remote = repo.find_remote("origin")?;
        remote.fetch(&["main"], Some(&mut fetch_options), None)?;

        // 4. Get the remote tip commit SHA from FETCH_HEAD
        let fetch_head = repo.find_reference("FETCH_HEAD")?;
        let remote_commit = fetch_head.peel_to_commit()?.id();

        // 5. Compare commits
        if local_commit == remote_commit {
            Ok((false, format!("Local and remote are both at {}", local_commit)))
        } else {
            Ok((
                true,
                format!(
                    "Local is at {}, remote is at {}",
                    local_commit, remote_commit
                ),
            ))
        }
    }
    async fn push(&self, name: String) -> Result<(), anyhow::Error> {
        // 1. Open the repo
        let repo = git2::Repository::open(Path::new(&name))?;

        // 2. Check for uncommitted changes
        let mut status_opts = StatusOptions::new();
        status_opts.include_untracked(true);
        let statuses = repo.statuses(Some(&mut status_opts))?;

        if !statuses.is_empty() {
            return Err(anyhow!("Cannot push: repository has uncommitted changes"));
        }

        // 3. Get the current branch name
        let head_ref = repo.head()?;
        let branch_name = head_ref
            .shorthand()
            .ok_or_else(|| anyhow!("Unable to determine current branch"))?
            .to_string();

        // 4. Set up credentials
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, _username_from_url, _allowed_types| {
            Cred::ssh_key(
                "git",
                Some(Path::new(&format!("{}/.ssh/id_rsa.pub", env::var("HOME").unwrap()))),
                Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
                None,
            )
        });

        let mut push_opts = PushOptions::new();
        push_opts.remote_callbacks(callbacks);

        // 5. Push
        let mut remote = repo.find_remote("origin")?;
        let refspec = format!("refs/heads/{}:refs/heads/{}", branch_name, branch_name);
        remote.push(&[&refspec], Some(&mut push_opts))?;

        println!("Pushed branch '{}' to origin", branch_name);
        Ok(())
    }
}