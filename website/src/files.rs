use std::path::PathBuf;
use dirs;

pub fn get_path() -> PathBuf {
    match dirs::home_dir() {
        Some (mut home) => {
            home.push("PF_Sandbox_Website");
            home
        }
        None => {
            panic!("could not get path of home");
        }
    }
}

pub fn git_path() -> PathBuf {
    let mut path = get_path();
    path.push("git_repo");
    path
}

pub fn builds_path() -> PathBuf {
    let mut path = get_path();
    path.push("builds");
    path
}
