#[derive(Debug)]
pub struct Task {
    pub id: i32,
    pub name: Option<String>,
    pub status: i32,
    pub task_type: i32,
}

#[derive(Debug, Clone)]
pub struct Artist {
    pub id: i32,
    pub name: String,
    pub external_id: String,
}

#[derive(Debug)]
pub struct Album {
    pub id: i32,
    pub name: Option<String>,
    pub year: Option<String>,
    pub image: Option<String>,
    pub artist_id: i32,
    pub external_id: Option<String>,
}

#[derive(Debug)]
pub struct Song {
    pub id: i32,
    pub name: Option<String>,
    pub track_num: Option<i32>,
    pub duration: Option<i32>,
    pub album_id: Option<i32>,
    pub external_id: Option<String>,
}
