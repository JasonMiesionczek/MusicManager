use data::models::*;
use std::process::Command;

pub enum DownloadError {
    NotAvailable,
    Forbidden,
    Unspecified(String),
}

pub struct YoutubeService {}

impl YoutubeService {
    pub fn new() -> YoutubeService {
        YoutubeService {}
    }

    pub fn get_artist_image_url(&self, artist_name: &str) -> Result<String, &'static str> {
        let scraper_path = dotenv::var("SCRAPER_PATH").expect("scraper path not specified");
        let output = match Command::new(scraper_path)
            .env("DISPLAY", ":99")
            .arg("artist")
            .arg(artist_name)
            .output()
        {
            Err(_) => return Err("Error executing scraper"),
            Ok(output) => output,
        };

        //.expect("failed to get song data");

        Ok(String::from_utf8_lossy(&output.stdout)
            .trim()
            .replace("failed to create drawable", "")
            .to_string())
    }

    pub fn get_album_image_url(&self, album_id: &str) -> Result<String, &'static str> {
        let scraper_path = dotenv::var("SCRAPER_PATH").expect("scraper path not specified");
        let output = match Command::new(scraper_path)
            .env("DISPLAY", ":99")
            .arg("image")
            .arg(album_id)
            .output()
        {
            Err(_) => return Err("Error executing scraper"),
            Ok(output) => output,
        };

        Ok(String::from_utf8_lossy(&output.stdout)
            .trim()
            .replace("failed to create drawable", "")
            .to_string())
    }

    pub fn get_album_data(&self, artist_name: &str) -> Result<Vec<AlbumMeta>, &'static str> {
        let scraper_path = dotenv::var("SCRAPER_PATH").expect("scraper path not specified");
        let output = match Command::new(scraper_path)
            .env("DISPLAY", ":99")
            .arg("album")
            .arg(artist_name)
            .output()
        {
            Err(_) => return Err("Error executing scraper"),
            Ok(output) => output,
        };

        let json = String::from_utf8_lossy(&output.stdout)
            .trim()
            .replace("failed to create drawable", "");

        if json.contains("ABORT") {
            return Err("Aborted");
        }

        let albums: Vec<AlbumMeta> = match serde_json::from_str(json.as_str()) {
            Err(_) => return Err("Error serializing album data"),
            Ok(albums) => albums,
        };
        Ok(albums)
    }

    pub fn get_track_list(&self, album_id: &str) -> Result<Vec<SongMeta>, &'static str> {
        let scraper_path = dotenv::var("SCRAPER_PATH").expect("scraper path not specified");
        let output = match Command::new(scraper_path)
            .env("DISPLAY", ":99")
            .arg("song")
            .arg(album_id)
            .output()
        {
            Err(_) => return Err("Error executing scraper"),
            Ok(output) => output,
        };

        let json = String::from_utf8_lossy(&output.stdout)
            .trim()
            .replace("failed to create drawable", "");

        if json.contains("ABORT") {
            return Err("Aborted");
        }

        let songs: Vec<SongMeta> = match serde_json::from_str(json.as_str()) {
            Err(_) => return Err("Error serializing song data"),
            Ok(songs) => songs,
        };
        Ok(songs)
    }

    pub fn download_song(&self, song_id: &str, filename: &str) -> Result<bool, DownloadError> {
        let music_dir =
            dotenv::var("MUSIC_DOWNLOAD_DIR").expect("download directory not specified");
        let output_file = format!("{}/{}", music_dir, filename);
        let output = match Command::new("youtube-dl")
            .arg("--extract-audio")
            .arg("--audio-format")
            .arg("mp3")
            .arg("--output")
            .arg(output_file)
            .arg(format!("https://music.youtube.com/watch?v={}", song_id))
            .output()
        {
            Err(_) => {
                return Err(DownloadError::Unspecified(String::from(
                    "Error executing curl",
                )));
            }
            Ok(output) => output,
        };

        //output.success()
        let output_str = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let combined = output_str + stderr;
        //println!("{}", combined);
        let has_error = combined.contains("ERROR");

        if has_error {
            if combined.contains("This video is not available") {
                return Err(DownloadError::NotAvailable);
            } else if combined.contains("403 Forbidden") {
                return Err(DownloadError::Forbidden);
            } else {
                return Err(DownloadError::Unspecified(String::from(
                    "Error executing curl",
                )));
            }
        }

        Ok(true)
    }

    pub fn download_image(&self, image_id: &str, image_url: &str) -> Result<bool, &'static str> {
        let image_dir =
            dotenv::var("IMAGE_DOWNLOAD_DIR").expect("image download directory not specified");
        let output_file = format!("{}/{}.jpg", image_dir, image_id);
        //println!("IMAGE URL: {}", image_url);
        match Command::new("curl")
            .arg(image_url)
            .arg("--output")
            .arg(output_file)
            .output()
        {
            Err(_) => return Err("Error executing curl"),
            Ok(output) => output,
        };
        Ok(true)
    }

    pub fn generate_waveform(&self, filename: &str) -> Result<bool, &'static str> {
        let image_dir =
            dotenv::var("IMAGE_DOWNLOAD_DIR").expect("image download directory not specified");
        let music_dir =
            dotenv::var("MUSIC_DOWNLOAD_DIR").expect("download directory not specified");
        let input_file = format!("{}/{}", music_dir, filename);
        let output_file = format!("{}/{}.png", image_dir, filename);
        match Command::new("ffmpeg")
            .arg("-y")
            .arg("-i")
            .arg(input_file)
            .arg("-filter_complex")
            .arg("showwavespic=s=1024x200:colors=white")
            .arg("-frames:v")
            .arg("1")
            .arg(output_file)
            .output()
        {
            Err(_) => return Err("Error generating waveform"),
            Ok(output) => output,
        };
        Ok(true)
    }
}
