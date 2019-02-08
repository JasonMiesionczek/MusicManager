#![feature(proc_macro_hygiene, decl_macro)]

use ::data::{
    models::*,
    repos::{AlbumRepository, ArtistRepository, Repository, TaskRepository},
};
use core::services::{LibraryAlbum, LibraryArtist, LibraryService, LibrarySong};
use dotenv;
use mysql;
use rocket::State;
use rocket::*;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use rocket_cors::Cors;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ArtistResult {
    artists: Vec<Artist>,
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
fn get(service: State<LibraryService>) -> Option<Json<ArtistResult>> {
    let pool = ::data::get_pool();
    let artists = service.get_artists(&pool);
    Some(Json(ArtistResult { artists: artists }))
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

fn cors_options_all() -> Cors {
    // You can also deserialize this
    Default::default()
}

fn rckt() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", StaticFiles::from("www/build"))
        .mount("/api", routes![get, tasks, albums, songs])
        .attach(cors_options_all())
        .manage(LibraryService::new())
        .manage(TaskRepository::new())
}

fn main() {
    dotenv::dotenv().ok();
    rckt().launch();
}
