use core::services::task::TaskService;
use data::models::*;
use data::schema::artists;
use data::schema::tasks;
use data::*;
use diesel::prelude::*;
use std::process::Command;
use uuid::Uuid;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[derive(Deserialize, Debug, Serialize)]
pub struct AlbumMeta {
    id: String,
    name: String,
    artist: String,
    image: String,
    year: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct SongMeta {
    id: String,
    name: String,
    num: u32,
}

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    Albums { data: Vec<AlbumMeta> },
    Songs { data: Vec<SongMeta> },
}

fn get_album_data(artist_name: &str) -> Vec<AlbumMeta> {
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

fn get_track_list(album_id: &str) -> Vec<SongMeta> {
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

fn download_song(song_id: &str, filename: &str) -> bool {
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

fn create_artist(artist_name: &str, db: &MysqlConnection) -> Artist {
    use data::schema::artists::dsl::*;
    let existing_artists = artists
        .filter(name.eq(name))
        .load::<Artist>(db)
        .expect("could not get existing artists");
    if existing_artists.len() > 0 {
        return existing_artists[0].clone();
    }

    let artist = Artist {
        id: 0,
        name: Some(String::from(artist_name)),
        external_id: Some(Uuid::new_v4().to_string()),
    };

    let _result = diesel::insert_into(artists).values(&artist).execute(db);

    artist
}

fn main() {
    openssl_probe::init_ssl_cert_env_vars();
    println!("Hello, world!");

    let connection = establish_connection();

    let ts = TaskService::new(&connection);

    if let Ok(t) = ts.create_task("some new task", 0) {
        println!("{:?}", t);
    }

    let results = tasks::dsl::tasks.load::<Task>(&connection).expect("error");
    for task in results {
        println!("{:?}", task);
    }

    let albums = get_album_data("tesseract");

    for album in albums {
        println!("{}", album.name);
        let artist = create_artist(&album.artist, &connection);
        let songs = get_track_list(album.id.as_str());
        for song in songs {
            println!("\t{}: {}", song.num, song.name);
            let filename = format!("{}-{}-{}.mp3", album.artist, album.name, song.name);
            println!("\t\tDownloading {}", filename);
            download_song(song.id.as_str(), filename.as_str());
        }
    }
    /*
    let result = Command::new("youtube-dl")
        .arg("--extract-audio")
        .arg("--audio-format")
        .arg("mp3")
        .arg("--output")
        .arg("testfile.mp3")
        .arg(format!(
            "https://music.youtube.com/watch?v={}",
            "Yua0Q-j0yWQ"
        ))
        .status()
        .expect("failed to execute");
    if result.success() {
        println!("downloaded");
    } else {
        println!("failed");
    }
    */
}
