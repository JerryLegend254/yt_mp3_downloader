extern crate yt_mp3_downloader;
use std::{env, process};

use yt_mp3_downloader::AppConfig;

fn main() {
    let config = AppConfig::new(env::args()).unwrap_or_else(|err| {
        eprintln!("something went wrong: {}", err);
        process::exit(1);
    });

    if let Err(e) = yt_mp3_downloader::run(config) {
        eprintln!("error running the application: {}", e);
        process::exit(1)
    }
}
