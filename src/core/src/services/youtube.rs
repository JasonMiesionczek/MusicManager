use data::models::*;
use std::process::Command;

pub struct YoutubeService {}

impl YoutubeService {
    pub fn new() -> YoutubeService {
        YoutubeService {}
    }

    pub fn get_album_image_url(&self, album_id: &str) -> String {
        let scraper_path = dotenv::var("SCRAPER_PATH").expect("scraper path not specified");
        let output = Command::new(scraper_path)
            .env("DISPLAY", ":99")
            .arg("image")
            .arg(album_id)
            .output()
            .expect("failed to get song data");

        String::from_utf8_lossy(&output.stdout)
            .trim()
            .replace("failed to create drawable", "")
            .to_string()
    }

    pub fn get_album_data(&self, artist_name: &str) -> Vec<AlbumMeta> {
        let scraper_path = dotenv::var("SCRAPER_PATH").expect("scraper path not specified");
        let output = Command::new(scraper_path)
            .env("DISPLAY", ":99")
            .arg("album")
            .arg(artist_name)
            .output()
            .expect("failed to get album data");

        let json = String::from_utf8_lossy(&output.stdout)
            .trim()
            .replace("failed to create drawable", "");
        let albums: Vec<AlbumMeta> = serde_json::from_str(json.as_str()).unwrap();
        albums
    }

    pub fn get_track_list(&self, album_id: &str) -> Vec<SongMeta> {
        let scraper_path = dotenv::var("SCRAPER_PATH").expect("scraper path not specified");
        let output = Command::new(scraper_path)
            .env("DISPLAY", ":99")
            .arg("song")
            .arg(album_id)
            .output()
            .expect("failed to get song data");

        let json = String::from_utf8_lossy(&output.stdout)
            .trim()
            .replace("failed to create drawable", "");
        let songs: Vec<SongMeta> = serde_json::from_str(json.as_str()).unwrap();
        songs
    }

    pub fn download_song(&self, song_id: &str, filename: &str) -> bool {
        let music_dir =
            dotenv::var("MUSIC_DOWNLOAD_DIR").expect("download directory not specified");
        let output_file = format!("{}/{}", music_dir, filename);
        let output = Command::new("youtube-dl")
            .arg("--extract-audio")
            .arg("--audio-format")
            .arg("mp3")
            .arg("--output")
            .arg(output_file)
            .arg(format!("https://music.youtube.com/watch?v={}", song_id))
            .status()
            .expect("failed to execute");
        output.success()
    }

    pub fn download_image(&self, album_id: &str, image_url: &str) -> bool {
        let image_dir =
            dotenv::var("IMAGE_DOWNLOAD_DIR").expect("image download directory not specified");
        let output_file = format!("{}/{}.jpg", image_dir, album_id);
        let output = Command::new("curl")
            .arg(image_url)
            .arg("--output")
            .arg(output_file)
            .status()
            .expect("failed to download image");
        output.success()
    }
}
