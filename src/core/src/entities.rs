use serde_derive::*;

#[derive(Deserialize, Debug, Serialize)]
pub struct AlbumMeta {
    pub id: String,
    pub name: String,
    pub artist: String,
    pub image: String,
    pub year: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct SongMeta {
    pub id: String,
    pub name: String,
    pub num: u32,
}

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    Albums { data: Vec<AlbumMeta> },
    Songs { data: Vec<SongMeta> },
}
