use std::error::Error;
use std::path::Path;

use git2::{self, Repository, IndexAddOption};
use git2::build::RepoBuilder;

use config::{Config, GitSource, GitCredentials};

fn get_callbacks(config: &GitCredentials) -> git2::RemoteCallbacks {
    let mut callbacks = git2::RemoteCallbacks::new();
    callbacks.credentials(move |_user, _username_from_url, _allowed_types| {
        git2::Cred::ssh_key(&config.username, None, config.ssh_key_path.as_ref(), None)
    });
    callbacks
}

pub fn get_latest(config: &Config) -> Result<(), Box<Error>> {
    get_repo(&config.src, &config.credentials, config.tmp_dir.join("src"))?;
    get_repo(&config.dst, &config.credentials, config.tmp_dir.join("dst"))?;
    Ok(())
}

pub fn commit_changes(config: &Config) -> Result<(), Box<Error>> {
    let repo = Repository::open(config.tmp_dir.join("dst"))?;

    // Check for changes
    let stats = repo.diff_index_to_workdir(None, None)?.stats()?;
    if stats.files_changed() == 0 {
        println!("{} already up to date", config.dst.branch);
        return Ok(());
    }
    println!("{} files changed (+{}, -{})",
        stats.files_changed(),
        stats.insertions(),
        stats.deletions()
    );

    // Add changes to index
    let mut index = repo.index()?;
    index.update_all(Some("."), None)?;
    index.add_all(Some("."), IndexAddOption::DEFAULT | IndexAddOption::CHECK_PATHSPEC, None)?;

    // Write index
    let oid = index.write_tree()?;
    let tree = repo.find_tree(oid)?;

    println!("Committing changes");
    let parent = repo.head()?.resolve()?.peel(git2::ObjectType::Commit)?.into_commit().unwrap();
    let sig = git2::Signature::now(&config.author_name, &config.author_email)?;
    let message = format!("Update {}", config.dst.branch);
    let commit = repo.commit(Some("HEAD"), &sig, &sig, &message, &tree, &[&parent])?;

    // Reset repo index to latest commit
    repo.reset(repo.find_commit(commit)?.as_object(), git2::ResetType::Hard, None)?;

    Ok(())
}

pub fn push_update(config: &Config) -> Result<(), Box<Error>> {
    let repo = Repository::open(config.tmp_dir.join("dst"))?;

    let mut remote = repo.find_remote("origin")?;
    remote.connect_auth(git2::Direction::Push, Some(get_callbacks(&config.credentials)), None)?;

    let mut options = git2::PushOptions::new();
    options.remote_callbacks(get_callbacks(&config.credentials));
    let ref_spec = format!("refs/heads/{0}:refs/heads/{0}", config.dst.branch);
    remote.push(&[&ref_spec], Some(&mut options))?;

    Ok(())
}

fn get_repo<P: AsRef<Path>>(source: &GitSource, creds: &GitCredentials, into: P)
    -> Result<Repository, git2::Error>
{
    let mut fetch_opts = git2::FetchOptions::new();
    fetch_opts.remote_callbacks(get_callbacks(creds));

    let path = into.as_ref();
    let repo = match path.exists() {
        // Repo already cloned, open it and fetch changes
        true => {
            let repo = Repository::open(path)?;
            repo.find_remote("origin")?.fetch(&[&source.branch], Some(&mut fetch_opts), None)?;
            repo
        }

        // Repo needs to be cloned
        false => {
            RepoBuilder::new()
                .fetch_options(fetch_opts)
                .branch(&source.branch)
                .clone(&source.url, path)?
        }
    };

    // Checkout latest
    {
        let local = repo.find_branch(&source.branch, git2::BranchType::Local)?;
        let upstream = local.upstream()?.get().peel(git2::ObjectType::Commit)?;
        repo.reset(&upstream, git2::ResetType::Hard, None)?;
    }

    Ok(repo)
}
