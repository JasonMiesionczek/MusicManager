#![allow(proc_macro_derive_resolution_fallback)]
use super::schema::*;
use diesel::{Insertable, Queryable};

#[derive(Queryable, Debug, Insertable)]
#[table_name = "tasks"]
pub struct Task {
    pub id: i32,
    pub name: Option<String>,
    pub status: i32,
    pub task_type: i32,
}

#[derive(Queryable, Debug, Insertable, Clone)]
#[table_name = "artists"]
pub struct Artist {
    pub id: i32,
    pub name: Option<String>,
    pub external_id: Option<String>,
}

#[derive(Queryable, Debug, Insertable)]
#[table_name = "albums"]
pub struct Album {
    pub id: i32,
    pub name: Option<String>,
    pub year: Option<String>,
    pub image: Option<String>,
    pub artist_id: i32,
    pub external_id: Option<String>,
}

#[derive(Queryable, Debug, Insertable)]
#[table_name = "songs"]
pub struct Song {
    pub id: i32,
    pub name: Option<String>,
    pub track_num: Option<i32>,
    pub duration: Option<i32>,
    pub album_id: Option<i32>,
    pub external_id: Option<String>,
}
