use casual;
use std::process;

pub struct Git {}

impl Git {
    pub fn add(repo_directory: &str, path: &str) {
        println!("file!");
        if !path.contains(".git") {
            let cmd = process::Command::new("git")
                .args(vec!["add", &path])
                .stdout(process::Stdio::null())
                .current_dir(repo_directory)
                .status()
                .unwrap();

            if cmd.sucess() {
                println!("{}", format!("ADDED TO GIT: {path}"));
            }
        }
    }

    pub fn commit(repo_directory: &str, message: &str) {
        println!("message!");
        let cmd = process::Command::new("git")
            .args(vec!["commit", "-m", message])
            .stdout(process::Stdio::null())
            .current_dir(repo_directory)
            .status()
            .unwrap();
    }

    pub fn ask_for_commit_message() -> String {
        let commit_message: String = casual::prompt("Please enter commit message: ").get();
        return commit_message;
    }

    pub fn push(repo_directory: &str) {
        println!("Pushing!");
        let cmd = process::Command::new("git")
            .arg("push")
            .stdout(process::Stdio::null())
            .stderr(process::Stdio::null())
            .current_dir(repo_directory)
            .status()
            .unwrap();
    }
}
