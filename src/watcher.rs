use crate::git::Git;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::{Duration, SystemTime};

pub struct RepoWatcher {}

impl RepoWatcher {
    pub fn register_event(repo_directory: &str, event: DebouncedEvent) {
        match &event {
            DebouncedEvent::Rescan => (),
            DebouncedEvent::Write(path) => {
                Git::add(repo_directory, &path.as_path().display().to_string())
            }
            DebouncedEvent::Chmod(path) => {
                Git::add(repo_directory, &path.as_path().display().to_string())
            }
            DebouncedEvent::Create(path) => {
                Git::add(repo_directory, &path.as_path().display().to_string())
            }
            DebouncedEvent::NoticeWrite(path) => {
                Git::add(repo_directory, &path.as_path().display().to_string())
            }
            DebouncedEvent::NoticeRemove(path) => {
                Git::add(repo_directory, &path.as_path().display().to_string())
            }
            DebouncedEvent::Remove(path) => {
                Git::add(repo_directory, &path.as_path().display().to_string())
            }
            DebouncedEvent::Rename(_old_path, new_path) => {
                Git::add(repo_directory, &new_path.as_path().display().to_string())
            }
            _ => (),
        }
    }

    pub fn watch(directory: &str, duration: u64, refresh: u64, message: &str) {
        let mut start_time = SystemTime::now();

        // Create a channel to recieve the events.
        let (sender, receiver) = channel();

        // Create a watcher object, delivering debounced events
        let mut watcher = watcher(sender, Duration::from_secs(duration)).unwrap();

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher.watch(directory, RecursiveMode::Recursive).unwrap();

        loop {
            match start_time.elapsed() {
                Ok(elapsed) => {
                    if elapsed.as_secs() > refresh {
                        println!("{} Seconds Elapsed", elapsed.as_secs());
                        Git::commit(directory, &message);
                        Git::push(directory);

                        start_time = SystemTime::now();
                    }
                }
                Err(e) => {
                    println!("Error: {e:?}");
                }
            }

            match receiver.recv() {
                Ok(event) => Self::register_event(directory, event),
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    }
}
