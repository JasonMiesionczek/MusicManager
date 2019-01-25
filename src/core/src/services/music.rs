use crate::entities::*;
use std::process::Command;

pub struct MusicService {}

impl MusicService {
    pub fn new() -> MusicService {
        MusicService {}
    }

    pub fn get_album_data(&self, artist_name: &str) -> Vec<AlbumMeta> {
        let output = Command::new("xvfb-run")
            .arg("--auto-servernum")
            .arg("--server-num=1")
            .arg("/home/jason/Projects/music-manager/target/release/scraper")
            .arg("album")
            .arg(artist_name)
            .output()
            .expect("failed to get album data");

        let json = String::from_utf8_lossy(&output.stdout)
            .trim()
            .replace("failed to create drawable", "");
        //println!("{}", json);
        let albums: Vec<AlbumMeta> = serde_json::from_str(json.as_str()).unwrap();
        albums
    }

    pub fn get_track_list(&self, album_id: &str) -> Vec<SongMeta> {
        let output = Command::new("xvfb-run")
            .arg("--auto-servernum")
            .arg("--server-num=1")
            .arg("/home/jason/Projects/music-manager/target/release/scraper")
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
        let output = Command::new("youtube-dl")
            .arg("--extract-audio")
            .arg("--audio-format")
            .arg("mp3")
            .arg("--output")
            .arg(filename)
            .arg(format!("https://music.youtube.com/watch?v={}", song_id))
            .status()
            .expect("failed to execute");
        output.success()
    }
}
