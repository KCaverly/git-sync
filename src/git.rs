use std::process;

pub struct Git {}

impl Git {
    pub fn add(repo_directory: &str, path: &str) {
        if !path.contains(".git") {
            let cmd = process::Command::new("git")
                .args(vec!["add", &path])
                .stdout(process::Stdio::null())
                .current_dir(repo_directory)
                .status()
                .unwrap();

            if cmd.success() {
                println!("{}", format!("ADDED TO GIT: {path}"));
            }
        }
    }

    pub fn commit(repo_directory: &str, message: &str) {
        let cmd = process::Command::new("git")
            .args(vec!["commit", "-m", message])
            .stdout(process::Stdio::null())
            .current_dir(repo_directory)
            .status()
            .unwrap();

        println!("GIT COMMITTED: {}", cmd.success());
    }

    pub fn push(repo_directory: &str) {
        let cmd = process::Command::new("git")
            .arg("push")
            .stdout(process::Stdio::null())
            .stderr(process::Stdio::null())
            .current_dir(repo_directory)
            .status()
            .unwrap();

        println!("GIT PUSHED: {}", cmd.success());
    }

    pub fn pull(repo_directory: &str) {
        let cmd = process::Command::new("git")
            .arg("pull")
            .stdout(process::Stdio::null())
            .stderr(process::Stdio::null())
            .current_dir(repo_directory)
            .status()
            .unwrap();

        println!("GIT PULLED: {}", cmd.success());
    }
}
