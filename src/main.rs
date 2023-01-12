use casual;
use ctrlc;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::process;
use std::sync::mpsc::channel;
use std::time::Duration;

fn git_add(path: String) {
    if !path.contains(".git") {
        let add = process::Command::new("git")
            .args(vec!["add", &path])
            .stdout(process::Stdio::null())
            .current_dir("/home/kcaverly/kb")
            .status()
            .unwrap();

        if add.success() {
            println!("{}", format!("ADDED TO GIT: {path}"));
        }
    }
}

fn git_commit_and_push() {
    let commit_message: String = casual::prompt("Please enter commit message: ").get();

    let git_commit = process::Command::new("git")
        .args(vec!["commit", "-m", &commit_message])
        .stdout(process::Stdio::null())
        .current_dir("/home/kcaverly/kb")
        .status()
        .unwrap();

    if git_commit.success() {
        println!("Successfully committed, pushing to origin...");

        let git_push = process::Command::new("git")
            .arg("push")
            .stdout(process::Stdio::null())
            .stderr(process::Stdio::null())
            .current_dir("/home/kcaverly/kb")
            .status()
            .unwrap();

        if git_push.success() {
            println!("Pushed to origin.");
        }
    }
}

fn register_event(event: DebouncedEvent) {
    match &event {
        DebouncedEvent::Rescan => (),
        DebouncedEvent::Write(path) => git_add(path.as_path().display().to_string()),
        DebouncedEvent::Chmod(path) => git_add(path.as_path().display().to_string()),
        DebouncedEvent::Create(path) => git_add(path.as_path().display().to_string()),
        DebouncedEvent::NoticeWrite(path) => git_add(path.as_path().display().to_string()),
        DebouncedEvent::NoticeRemove(path) => git_add(path.as_path().display().to_string()),
        DebouncedEvent::Remove(path) => git_add(path.as_path().display().to_string()),
        DebouncedEvent::Rename(old_path, new_path) => {
            git_add(new_path.as_path().display().to_string());
        }
        _ => (),
    }
}

fn main() {
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        git_commit_and_push();
        process::exit(0);
    });

    // Create a channel to receive the events.
    let (sender, receiver) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(sender, Duration::from_secs(5)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher
        .watch("/home/kcaverly/kb", RecursiveMode::Recursive)
        .unwrap();

    loop {
        match receiver.recv() {
            Ok(event) => register_event(event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
