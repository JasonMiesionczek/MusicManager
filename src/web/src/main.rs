#![feature(proc_macro_hygiene, decl_macro)]

use ::data::models::*;
use ::data::repos::{artist::ArtistRepository, Repository};
use core::services::library::*;
use dotenv;
use mysql;
use rocket::State;
use rocket::*;
use rocket_contrib::json::Json;
use rocket_contrib::serve::{Options, StaticFiles};
use serde_derive::{Deserialize, Serialize};
use std::sync::Mutex;

type ArtistRepositoryMutex = Mutex<ArtistRepository>;

#[derive(Serialize, Deserialize)]
struct Library {
    artists: Vec<LibraryArtist>,
}

#[get("/library", format = "json", rank = 5)]
fn get(repo: State<LibraryService>) -> Option<Json<Library>> {
    let pool = get_pool();
    let artists = repo.get_all_music(&pool);
    Some(Json(Library { artists: artists }))
}

fn get_pool() -> mysql::Pool {
    let url = dotenv::var("DATABASE_URL").expect("could not find DATABASE_URL");
    let pool = mysql::Pool::new(url).unwrap();
    pool
}

fn rckt() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", StaticFiles::from("www/build"))
        .mount("/api", routes![get])
        .manage(LibraryService::new())
}

fn main() {
    dotenv::dotenv().ok();
    rckt().launch();
}
