use std::fs;
use std::sync::{RwLock, Arc};
use std::thread;
use std::time::Duration;
use std::collections::HashSet;

use chrono::NaiveDateTime;
use git2::{Repository, Remote};

use crate::files;

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

    // generate builds path if it doesnt exist
    fs::create_dir_all(files::builds_path()).unwrap();

    let mut remote = repo.find_remote("origin").unwrap();

    loop {
        attempt_update(&repo, &mut remote, commits_rw.clone());
        thread::sleep(Duration::from_secs(60));
    }
}

fn attempt_update(repo: &Repository, remote: &mut Remote, commits_rw: Arc<RwLock<Commits>>) {
    let mut netplay: Option<Commit> = None;
    let mut builds = vec!();

    // find available build zips
    let mut build_zips = HashSet::new();
    for entry in fs::read_dir(files::builds_path()).unwrap() {
        build_zips.insert(entry.unwrap().file_name().into_string().unwrap());
    }

    // checkout lastest origin/master
    if let Err(_) = remote.fetch(&["master"], None, None) {
        return; // Abort on network issues
    }

    let latest_commit_oid = repo.refname_to_id("refs/remotes/origin/master").unwrap();
    let mut commit = repo.find_commit(latest_commit_oid).unwrap();
    loop {
        let time = commit.time();
        let date = NaiveDateTime::from_timestamp(time.seconds() + time.offset_minutes() as i64 * 60, 0);
        let date = date.format("%Y-%m-%d %H:%M UTC").to_string();
        let hash = format!("{:.15}", commit.id());

        let build_commit = Commit {
            message: commit.message().unwrap_or("NON-UTF8 MESSAGE").to_string(),
            windows: build_zips.contains(format!("pfsandbox-{}-windows.zip", hash).as_str()),
            linux:   build_zips.contains(format!("pfsandbox-{}-linux.tar.gz", hash).as_str()),
            hash,
            date
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
    pub hash:    String,
    pub message: String,
    pub windows: bool,
    pub linux:   bool,
    pub date:    String,
}
