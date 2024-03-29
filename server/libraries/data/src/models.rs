use mysql::{from_row, Row};
use mysql_common::*;
use serde_derive::*;
use std::fmt::Display;

#[derive(Deserialize, Debug, Serialize, Clone, PartialEq)]
pub struct AlbumMeta {
    pub id: String,
    pub name: String,
    pub artist: String,
    pub image: String,
    pub year: String,
}

#[derive(Deserialize, Debug, Serialize, Clone, PartialEq)]
pub struct SongMeta {
    pub id: String,
    pub name: String,
    pub num: u32,
    pub album_id: String,
}

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    Albums { data: Vec<AlbumMeta> },
    Songs { data: Vec<SongMeta> },
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum TaskStatus {
    Pending,
    Queued,
    InProgress,
    Complete,
    Failed,
}

impl Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            TaskStatus::Pending => write!(f, "pending"),
            TaskStatus::Queued => write!(f, "queued"),
            TaskStatus::InProgress => write!(f, "in-progress"),
            TaskStatus::Complete => write!(f, "complete"),
            TaskStatus::Failed => write!(f, "failed"),
        }
    }
}

#[derive(Deserialize, Debug, Serialize, Clone, PartialEq)]
pub enum TaskType {
    GetArtistImage(String),
    GetAlbumData(String),
    GetAlbumImage(AlbumMeta),
    GetSongData(AlbumMeta),
    DownloadSong(SongMeta),
    GenerateWaveform(SongMeta),
}

impl Display for TaskType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        //write!(f, "({}, {})", self.x, self.y)
        match &self {
            TaskType::GetArtistImage(_) => write!(f, "get_artist_image"),
            TaskType::GetAlbumData(_) => write!(f, "get_album_data"),
            TaskType::GetAlbumImage(_) => write!(f, "get_album_image"),
            TaskType::GetSongData(_) => write!(f, "get_song_data"),
            TaskType::DownloadSong(_) => write!(f, "download_song"),
            TaskType::GenerateWaveform(_) => write!(f, "generate_waveform"),
        }
    }
}

#[derive(Deserialize, Debug, Serialize, Clone, PartialEq)]
pub struct Task {
    pub id: u32,
    pub status: TaskStatus,
    pub task_type: TaskType,
    pub external_id: String,
    pub retry_count: u32,
}

impl Task {
    pub fn new(task_type: TaskType) -> Self {
        Task {
            id: 0,
            status: TaskStatus::Pending,
            task_type: task_type,
            external_id: String::from(""),
            retry_count: 0,
        }
    }
}

unsafe impl Send for Task {}
unsafe impl Sync for Task {}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let data = match self.task_type {
            TaskType::GetAlbumData(ref artist_name) => format!("{}", artist_name),
            TaskType::GetAlbumImage(ref album_meta) => {
                format!("{}: {}", album_meta.artist, album_meta.name)
            }
            TaskType::GetArtistImage(ref artist_name) => format!("{}", artist_name),
            TaskType::GetSongData(ref album_meta) => {
                format!("{}: {}", album_meta.artist, album_meta.name)
            }
            TaskType::DownloadSong(ref song_meta) => format!("{}", song_meta.name),
            TaskType::GenerateWaveform(ref song_meta) => format!("{}", song_meta.name),
        };
        write!(f, "[#{}]: {} ({})", self.id, self.task_type, data)
    }
}

impl From<Row> for Task {
    fn from(mut item: Row) -> Self {
        let id: u32 = item.take("id").unwrap();
        let status: String = item.take("status").unwrap();
        let status_enum = match status.as_str() {
            "pending" => TaskStatus::Pending,
            "queued" => TaskStatus::Queued,
            "in-progress" => TaskStatus::InProgress,
            "complete" => TaskStatus::Complete,
            "failed" => TaskStatus::Failed,
            _ => panic!("unknown status"),
        };
        let task_data: String = item.take("task_data").unwrap();
        let task_type: String = item.take("task_type").unwrap();
        let task_enum = match task_type.as_str() {
            "get_artist_image" => TaskType::GetArtistImage(task_data),
            "get_album_data" => TaskType::GetAlbumData(task_data),
            "get_song_data" => {
                let album: AlbumMeta = serde_json::from_str(task_data.as_str()).unwrap();
                TaskType::GetSongData(album)
            }
            "get_album_image" => {
                let album: AlbumMeta = serde_json::from_str(task_data.as_str()).unwrap();
                TaskType::GetAlbumImage(album)
            }
            "download_song" => {
                let song: SongMeta = serde_json::from_str(task_data.as_str()).unwrap();
                TaskType::DownloadSong(song)
            }
            "generate_waveform" => {
                let song: SongMeta = serde_json::from_str(task_data.as_str()).unwrap();
                TaskType::GenerateWaveform(song)
            }
            _ => panic!("unknown task type"),
        };
        let external_id: String = item.take("external_id").unwrap();
        let retry_count: u32 = match item.take("retry_count") {
            Some(val) => val,
            None => 0,
        };
        Task {
            id,
            status: status_enum,
            task_type: task_enum,
            external_id,
            retry_count,
        }
    }
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Artist {
    pub id: u32,
    pub name: String,
    pub external_id: String,
}

impl Artist {
    pub fn new(name: String) -> Self {
        Artist {
            id: 0,
            name,
            external_id: String::from(""),
        }
    }
}

impl From<Row> for Artist {
    fn from(item: Row) -> Self {
        let (id, name, external_id) = from_row(item);
        Artist {
            id,
            name,
            external_id,
        }
    }
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Album {
    pub id: u32,
    pub name: String,
    pub year: u32,
    pub image: String,
    pub artist_id: u32,
    pub external_id: String,
}

impl Album {
    pub fn new(
        name: String,
        year: u32,
        image: String,
        artist_id: u32,
        external_id: String,
    ) -> Self {
        Album {
            id: 0,
            name,
            year,
            image,
            artist_id,
            external_id,
        }
    }
}

impl From<Row> for Album {
    fn from(item: Row) -> Self {
        let (id, name, year, image, artist_id, external_id) = from_row(item);
        Album {
            id,
            name,
            year,
            image,
            artist_id,
            external_id,
        }
    }
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Song {
    pub id: u32,
    pub name: String,
    pub track_num: u32,
    pub duration: u32,
    pub album_id: u32,
    pub external_id: String,
    pub filename: String,
}

impl Song {
    pub fn new(name: String, track_num: u32, album_id: u32, external_id: String) -> Self {
        Song {
            id: 0,
            name,
            track_num,
            duration: 0,
            album_id,
            external_id,
            filename: String::from(""),
        }
    }
}

impl From<Row> for Song {
    fn from(item: Row) -> Self {
        let (id, name, track, duration, album_id, external_id, filename) = from_row(item);
        Song {
            id,
            name,
            track_num: track,
            duration,
            album_id,
            external_id,
            filename,
        }
    }
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Playlist {
    pub id: u32,
    pub name: String,
}

impl Playlist {
    pub fn new(name: String) -> Self {
        Playlist { id: 0, name }
    }
}

impl From<Row> for Playlist {
    fn from(item: Row) -> Self {
        let (id, name) = from_row(item);
        Playlist { id, name }
    }
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct PlaylistSongItem {
    pub id: u32,
    pub playlist_id: u32,
    pub song_id: u32,
}

impl PlaylistSongItem {
    pub fn new(playlist_id: u32, song_id: u32) -> Self {
        PlaylistSongItem {
            id: 0,
            playlist_id,
            song_id,
        }
    }
}

impl From<Row> for PlaylistSongItem {
    fn from(item: Row) -> Self {
        let (id, playlist_id, song_id) = from_row(item);
        PlaylistSongItem {
            id,
            playlist_id,
            song_id,
        }
    }
}
