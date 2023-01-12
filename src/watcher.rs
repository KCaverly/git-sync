use crate::git::Git;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};

struct FolderWatcher {}

impl FolderWatcher {
    pub fn register_event(repo_directory: &str, event: DebouncedEvent) {
        match &event {
            DebouncedEvent::Rescan => (),
            DebouncedEvent::Write(path) => {
                Git::add(repo_directory, &path.as_path().display().to_string())
            }
            DebouncedEvent::Chmod(path) => (),
            DebouncedEvent::Create(path) => (),
            DebouncedEvent::NoticeWrite(path) => (),
            DebouncedEvent::NoticeRemove(path) => (),
            DebouncedEvent::Remove(path) => (),
            DebouncedEvent::Rename(old_path, new_path) => (),
            _ => (),
        }
    }

    pub fn watch(path: &str, duration: u64) {
        // Create a channel to recieve the events.
        let (sender, receiver) = channel();

        // Create a watcher object, delivering debounced events
        let mut watcher = watcher(sender, Duration::from_secs(duration)).unwrap();

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher.watch(directory, RecursiveMode::Recursive).unwrap();

        loop {
            match receiver.recv() {
                Ok(event) => Self::register_event(event),
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    }
}
