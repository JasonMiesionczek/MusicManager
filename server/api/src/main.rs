#![feature(proc_macro_hygiene, decl_macro)]

use ::data::{
    models::*,
    repos::{AlbumRepository, ArtistRepository, PlaylistRepository, Repository, TaskRepository},
};
use core::services::LibraryService;
use dotenv;
use rocket::State;
use rocket::*;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use rocket_cors::Cors;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ArtistsResult {
    artists: Vec<ArtistResult>,
}

#[derive(Serialize, Deserialize)]
struct ArtistResult {
    artist: Artist,
    album_count: u32,
}

#[derive(Serialize, Deserialize)]
struct AlbumResult {
    albums: Vec<Album>,
    artist: Artist,
}

#[derive(Serialize, Deserialize)]
struct SongResult {
    artist: Artist,
    album: Album,
    songs: Vec<Song>,
}

#[get("/library/artists", format = "json", rank = 5)]
fn artists(service: State<LibraryService>) -> Option<Json<ArtistsResult>> {
    let pool = ::data::get_pool();
    let artists = service.get_artists(&pool);
    let artists = artists
        .into_iter()
        .map(|a| ArtistResult {
            artist: a.clone(),
            album_count: service
                .get_artist_albums(a.clone().id.to_string().as_str(), &pool)
                .len() as u32,
        })
        .collect();
    Some(Json(ArtistsResult { artists: artists }))
}

#[get("/library/albums/<artist_id>", format = "json", rank = 1)]
fn albums(artist_id: String, service: State<LibraryService>) -> Option<Json<AlbumResult>> {
    let pool = ::data::get_pool();
    let albums = service.get_artist_albums(artist_id.as_str(), &pool);
    let artist_repo = ArtistRepository::new();
    if let Some(artist) = artist_repo.find_by_id(artist_id.parse::<u32>().unwrap(), &pool) {
        Some(Json(AlbumResult {
            albums: albums,
            artist: artist,
        }))
    } else {
        None
    }
}

#[get("/library/songs/<album_id>", format = "json")]
fn songs(album_id: String, service: State<LibraryService>) -> Option<Json<SongResult>> {
    let pool = ::data::get_pool();
    let album_repo = AlbumRepository::new();
    let artist_repo = ArtistRepository::new();
    if let Some(album) = album_repo.find_by_id(album_id.parse::<u32>().unwrap(), &pool) {
        if let Some(artist) = artist_repo.find_by_id(album.artist_id, &pool) {
            let songs = service.get_album_songs(album_id.as_str(), &pool);
            Some(Json(SongResult {
                artist: artist,
                album: album,
                songs: songs,
            }))
        } else {
            None
        }
    } else {
        None
    }
}

#[get("/tasks", format = "json")]
fn tasks(tasks: State<TaskRepository>) -> Option<Json<Vec<Task>>> {
    let pool = ::data::get_pool();
    let tasks = tasks.get_all(&pool);
    Some(Json(tasks))
}

#[get("/playlists", format = "json")]
fn playlists(playlist_repo: State<PlaylistRepository>) -> Option<Json<Vec<Playlist>>> {
    let pool = ::data::get_pool();
    let playlists = playlist_repo.get_all(&pool);
    Some(Json(playlists))
}

#[derive(Deserialize)]
struct PlaylistForm {
    name: String,
}

#[post("/playlist", data = "<playlist>", format = "json")]
fn create_playlist(
    playlist: Json<PlaylistForm>,
    repo: State<PlaylistRepository>,
) -> Option<Json<bool>> {
    let mut pl = Playlist::new(playlist.name.clone());
    let pool = ::data::get_pool();
    repo.create(&mut pl, &pool).unwrap();
    Some(Json(true))
}

#[derive(Deserialize)]
struct PlaylistSong {
    playlist_id: u32,
    song_id: u32,
}

#[post("/playlist_song", data = "<playlist_song>", format = "json")]
fn add_song_to_playlist(playlist_song: Json<PlaylistSong>) -> Option<Json<bool>> {
    let pool = ::data::get_pool();
    pool.prep_exec(
        format!(
            "INSERT INTO playlists_songs (playlist_id, song_id) VALUES ({}, {})",
            playlist_song.playlist_id, playlist_song.song_id
        ),
        (),
    )
    .unwrap();
    Some(Json(true))
}

#[get("/playlist/<playlist_id>", format = "json")]
fn get_playlist_songs(playlist_id: u32, service: State<LibraryService>) -> Option<Json<Vec<Song>>> {
    let songs = service.get_playlist_songs(playlist_id, &::data::get_pool());
    Some(Json(songs))
}

fn cors_options_all() -> Cors {
    // You can also deserialize this
    Default::default()
}

fn rckt() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", StaticFiles::from("www/build"))
        .mount(
            "/api",
            routes![
                artists,
                tasks,
                albums,
                songs,
                playlists,
                create_playlist,
                add_song_to_playlist,
                get_playlist_songs,
            ],
        )
        .attach(cors_options_all())
        .manage(LibraryService::new())
        .manage(TaskRepository::new())
        .manage(PlaylistRepository::new())
}

fn main() {
    dotenv::dotenv().ok();
    rckt().launch();
}
