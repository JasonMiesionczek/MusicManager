use crate::models::Playlist;
use crate::repos::Repository;
use mysql::params;

pub struct PlaylistRepository {}

impl PlaylistRepository {
    pub fn new() -> Self {
        PlaylistRepository {}
    }
}

impl Repository for PlaylistRepository {
    type Item = Playlist;

    fn select_query(&self) -> String {
        String::from("SELECT id, name FROM playlists")
    }

    fn insert_query(&self) -> String {
        String::from("INSERT INTO playlists (name) VALUES (:name)")
    }

    fn order_by(&self) -> String {
        String::from("ORDER BY name")
    }

    fn create<'a>(
        &self,
        item: &'a mut Self::Item,
        pool: &mysql::Pool,
    ) -> Result<&'a mut Self::Item, String> {
        let mut stmt = pool.prepare(self.insert_query()).unwrap();
        let result = stmt.execute(params! { "name" => &item.name});
        match result {
            Err(err) => Err(err.to_string()),
            Ok(_) => Ok(item),
        }
    }
}
