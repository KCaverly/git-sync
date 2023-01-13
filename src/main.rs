mod git;
mod watcher;

use clap::Parser;
use ctrlc;
use git::Git;
use std::{env, process};
use watcher::RepoWatcher;

#[derive(Parser, Default, Debug)]
struct Arguments {
    #[clap(short, long)]
    /// The commit message to use when commiting
    message: Option<String>,

    #[clap(short, long)]
    /// The repo directory to monitor
    location: Option<String>,

    #[clap(short, long)]
    /// The duration to ping duration when watching
    duration: Option<u64>,

    #[clap(short, long)]
    /// Commit & Push Every <refresh> minutes
    refresh: Option<u64>,
}

fn main() {
    let args = Arguments::parse();

    let dir: String;
    if args.location.is_some() {
        dir = args.location.unwrap();
    } else {
        dir = env::current_dir().unwrap().as_path().display().to_string();
    }

    let duration: u64;
    if args.duration.is_some() {
        duration = args.duration.unwrap();
    } else {
        // Default to pinging every 30 seconds
        duration = 30;
    }

    let refresh: u64;
    if args.refresh.is_some() {
        refresh = args.refresh.unwrap();
    } else {
        // Default to hourly
        refresh = 60 * 60;
    }

    let message: String;
    if args.message.is_some() {
        message = args.message.unwrap();
    } else {
        message = "(git-sync) auto commit".to_string();
    }

    let dir_clone = dir.clone();
    let message_clone = message.clone();

    ctrlc::set_handler(move || {
        Git::commit(&dir_clone, &message_clone);
        Git::push(&dir_clone);
        process::exit(0);
    });

    println!("WATCHING @ {} EVERY {} SECONDS", &dir, duration);

    Git::pull(&dir);

    RepoWatcher::watch(&dir, duration, refresh, &message)
}
