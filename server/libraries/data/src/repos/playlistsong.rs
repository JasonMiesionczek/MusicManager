use crate::models::PlaylistSongItem;
use crate::repos::Repository;
use mysql::params;

pub struct PlaylistSongRepository {}

impl PlaylistSongRepository {
    pub fn new() -> Self {
        PlaylistSongRepository {}
    }
}

impl Repository for PlaylistSongRepository {
    type Item = PlaylistSongItem;

    fn select_query(&self) -> String {
        String::from("SELECT id, playlist_id, song_id FROM playlists_songs")
    }

    fn insert_query(&self) -> String {
        String::from(
            "INSERT INTO playlists_songs (playlist_id, song_id) VALUES (:playlist_id, :song_id)",
        )
    }

    fn order_by(&self) -> String {
        String::from("ORDER BY song_id")
    }

    fn create<'a>(
        &self,
        item: &'a mut Self::Item,
        pool: &mysql::Pool,
    ) -> Result<&'a mut Self::Item, String> {
        let mut stmt = pool.prepare(self.insert_query()).unwrap();
        let result =
            stmt.execute(params! { "playlist_id" => &item.playlist_id, "song_id" => &item.song_id});
        match result {
            Err(err) => Err(err.to_string()),
            Ok(_) => Ok(item),
        }
    }
}
