#![feature(proc_macro_hygiene, decl_macro)]

use ::data::{
    models::*,
    repos::{Repository, TaskRepository},
};
use core::services::{LibraryAlbum, LibraryArtist, LibraryService};
use dotenv;
use mysql;
use rocket::State;
use rocket::*;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use rocket_cors::Cors;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Library {
    artists: Vec<LibraryArtist>,
    albums: Vec<LibraryAlbum>,
}

#[get("/library/<level>", format = "json", rank = 5)]
fn get(level: String, repo: State<LibraryService>) -> Option<Json<Library>> {
    let pool = get_pool();
    let artists = repo.get_all_music(&pool);
    match &level[..] {
        "artists" => Some(Json(Library {
            artists: artists,
            albums: vec![],
        })),
        "albums" => {
            let mut albums = Vec::new();
            for artist in artists {
                for album in artist.albums {
                    albums.push(album);
                }
            }
            Some(Json(Library {
                artists: vec![],
                albums: albums,
            }))
        }
        _ => Some(Json(Library {
            artists: vec![],
            albums: vec![],
        })),
    }
}

#[get("/library/albums/<artist_id>", format = "json", rank = 1)]
fn albums(artist_id: String, service: State<LibraryService>) -> Option<Json<Vec<LibraryAlbum>>> {
    let pool = get_pool();
    let albums = service.get_artist_albums(artist_id.as_str(), &pool);
    Some(Json(albums))
}

#[get("/tasks", format = "json")]
fn tasks(tasks: State<TaskRepository>) -> Option<Json<Vec<Task>>> {
    let pool = get_pool();
    let tasks = tasks.get_all(&pool);
    Some(Json(tasks))
}

fn get_pool() -> mysql::Pool {
    let url = dotenv::var("DATABASE_URL").expect("could not find DATABASE_URL");
    let pool = mysql::Pool::new(url).unwrap();
    pool
}

fn cors_options_all() -> Cors {
    // You can also deserialize this
    Default::default()
}

fn rckt() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", StaticFiles::from("www/build"))
        .mount("/api", routes![get, tasks, albums])
        .attach(cors_options_all())
        .manage(LibraryService::new())
        .manage(TaskRepository::new())
}

fn main() {
    dotenv::dotenv().ok();
    rckt().launch();
}
