use std::env;
use std::path::Path;
use git2::build::RepoBuilder;
use git2::{Cred, FetchOptions, RemoteCallbacks};
use serde::{Deserialize, Serialize};
use crate::config::Repository;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CheckOutInfo {
    pub branch_name: String,
    pub commit_sha: String,
}
pub(crate) trait Pulls {
    fn clone(&self, organization: String, name: String) -> Result<CheckOutInfo, anyhow::Error>;
    fn update(&self, repo: &crate::config::Repository) -> Result<(), anyhow::Error>;
}
pub(crate) trait Pushes {
    fn push(&self, name: String) -> Result<(), anyhow::Error>;
}

#[derive(Default)]
pub(crate) struct Manager{

}
impl Pulls for Manager {
    fn clone(&self, org_name: String, name: String) -> Result<CheckOutInfo, anyhow::Error> {
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

    fn push(&self, name: String) -> Result<(), anyhow::Error> {
        todo!()
    }
}