# YouTube MP3 Downloader

A Rust application that downloads and converts YouTube videos to MP3 format using yt-dlp and ffmpeg.

## Description

This tool allows you to batch download YouTube videos and convert them to MP3 format. It reads URLs from a text file and processes them sequentially, saving the output as MP3 files in a designated directory.

## Prerequisites

- [yt-dlp](https://github.com/yt-dlp/yt-dlp) executable
- [ffmpeg](https://ffmpeg.org/) executable

Both executables should be placed in a `libs` directory in the project root.

## Project Structure

```
youtube-mp3-downloader/
├── libs/
│   ├── yt-dlp
│   └── ffmpeg
├── output/          # Downloaded MP3 files
├── src/
│   ├── lib.rs
│   └── main.rs
└── links.txt        # Your YouTube links
```

## Installation

1. Create the required directories:
```bash
mkdir output
```

2. Make sure the executables have proper permissions if needed:
```bash
chmod +x libs/yt-dlp
chmod +x libs/ffmpeg
```

## Usage

1. Create a text file containing YouTube URLs (one per line):
```
https://www.youtube.com/watch?v=example1
https://www.youtube.com/watch?v=example2
```

2. Run the application with the path to your links file:
```bash
cargo run path/to/your/links.txt
```

The downloaded MP3 files will be saved in the `output` directory.

## Features

- Batch processing of YouTube links
- Automatic conversion to MP3 format
- Error handling for missing dependencies
- Progress feedback during downloads
- Configurable output directory

## Error Handling

The application handles several types of errors:
- Missing yt-dlp or ffmpeg executables
- Invalid or missing input file
- Download failures
- File system operations

## Technical Details

- Written in Rust
- Uses standard library for file operations and process management
- Downloads audio in format ID 249 (opus)
- Converts downloaded audio to MP3 format

## Contributing

Feel free to submit issues and enhancement requests!

## License

This project is available under the MIT License. See the LICENSE file for more details.

## Notes

- Make sure you have proper rights to download the content
- The download speed depends on your internet connection
- Some videos might not be available for download due to restrictions and codes(so make sure to check availability)
