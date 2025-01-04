use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;

pub fn run(config: AppConfig) -> Result<(), Box<dyn Error>> {
    let mut fh = File::open(config.yt_links)?;

    let mut content = String::new();

    fh.read_to_string(&mut content)?;

    for url in content.lines() {
        download_music(url)?
    }
    println!("Finished downloading songs!");
    Ok(())
}

pub fn download_music(url: &str) -> Result<(), Box<dyn Error>> {
    let libraries_dir = PathBuf::from("libs");
    let output_dir = PathBuf::from("output");
    let yt_dlp_path = libraries_dir.join("yt-dlp");
    let ffmpeg_path = libraries_dir.join("ffmpeg");

    if !yt_dlp_path.exists() {
        eprintln!("yt-dlp executable not found at {:?}", yt_dlp_path);
        return Err("yt-dlp executable missing".into());
    }
    if !ffmpeg_path.exists() {
        eprintln!("ffmpeg executable not found at {:?}", ffmpeg_path);
        return Err("ffmpeg executable missing".into());
    }

    std::fs::create_dir_all(&output_dir)?;

    let format_id = "249";

    let output_template = output_dir.join("%(title)s.%(ext)s");
    let output_template = output_template.to_str().unwrap();

    let output = Command::new(yt_dlp_path)
        .arg("-f")
        .arg(format_id)
        .arg("-x")
        .arg("--audio-format")
        .arg("mp3")
        .arg("-o")
        .arg(output_template)
        .arg(url)
        .output()?;

    if output.status.success() {
        println!("Finished downloading song!");
    } else {
        eprintln!(
            "Error during download: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        return Err("Download failed".into());
    }

    Ok(())
}

pub struct AppConfig {
    yt_links: String,
}

impl AppConfig {
    pub fn new(mut args: std::env::Args) -> Result<AppConfig, &'static str> {
        args.next();
        let yt_links = match args.next() {
            Some(arg) => arg,
            None => return Err("please provide path to file with yt links"),
        };
        Ok(AppConfig { yt_links })
    }
}
