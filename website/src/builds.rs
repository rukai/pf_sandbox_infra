use std::sync::{RwLock, Arc};
use std::thread;
use std::time::Duration;

use git2::{Repository, Remote};

use files;

pub fn build_reader() -> Arc<RwLock<Commits>> {
    let commits = Arc::new(RwLock::new(Commits { netplay: None, builds: vec!() }));
    let commits2 = commits.clone();
    thread::spawn(move || {
        run(commits2);
    });
    commits
}

fn run(commits_rw: Arc<RwLock<Commits>>) {
    let url = "https://github.com/rukai/pf_sandbox";
    let repo = match Repository::open(files::git_path()) {
        Ok(repo) => repo,
        Err(_)   => Repository::clone(url, files::git_path()).unwrap(),
    };

    let mut remote = repo.find_remote("origin").unwrap();

    loop {
        attempt_update(&repo, &mut remote, commits_rw.clone());
        thread::sleep(Duration::from_secs(60));
    }
}

fn attempt_update(repo: &Repository, remote: &mut Remote, commits_rw: Arc<RwLock<Commits>>) {
    let mut netplay: Option<Commit> = None;
    let mut builds = vec!();

    // checkout lastest origin/master
    if let Err(_) = remote.fetch(&["master"], None, None) {
        return; // Abort on network issues
    }

    let latest_commit_oid = repo.refname_to_id("refs/remotes/origin/master").unwrap();
    let mut commit = repo.find_commit(latest_commit_oid).unwrap();
    loop {
        let build_commit = Commit {
            revision: format!("alpha-201-DEADCAFE"),
            hash:     format!("{}", commit.id()),
            message:  commit.message().unwrap_or("NON-UTF8 MESSAGE").to_string(),
            windows:  false,
            linux:    true,
            date_unix: commit.time().seconds()
        };
        if netplay.is_none() /* && commit_has_netplay_tag */ {
            netplay = Some(build_commit.clone());
        }

        builds.push(build_commit);

        if let Some(parent) = commit.parents().next() {
            commit = parent;
        }
        else {
            break;
        }
    }

    let mut commits_write = commits_rw.write().unwrap();
    *commits_write = Commits { netplay, builds };
}

#[derive(Serialize, Clone)]
pub struct Commits {
    pub netplay: Option<Commit>,
    pub builds:  Vec<Commit>
}

#[derive(Serialize, Clone)]
pub struct Commit {
    pub revision:  String,
    pub hash:      String,
    pub message:   String,
    pub windows:   bool,
    pub linux:     bool,
    pub date_unix: i64,
}
