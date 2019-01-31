use crate::models::*;
use crate::repos::Repository;
use mysql as my;
use mysql::params;
use uuid;

pub struct SongRepository {}

impl Repository for SongRepository {
    type Item = Song;

    fn select_query(&self) -> String {
        String::from("SELECT id, name, track, duration, album_id, external_id, filename FROM songs")
    }

    fn insert_query(&self) -> String {
        String::from(
            r"
        INSERT INTO songs (name, track, duration, album_id, external_id, filename)
        VALUES (:name, :track_num, :duration, :album_id, :external_id, :filename)",
        )
    }

    fn create<'a>(
        &self,
        item: &'a mut Self::Item,
        pool: my::Pool,
    ) -> Result<&'a mut Self::Item, &'static str> {
        let mut stmt = pool.prepare(self.insert_query()).unwrap();
        let exid = uuid::Uuid::new_v4().to_string();
        let filename = format!("{}.mp3", exid);
        stmt.execute(params! {
            "name" => &item.name,
            "track_num" => &item.track_num,
            "duration" => &item.duration,
            "album_id" => &item.album_id,
            "external_id" => &item.external_id,
            "filename" => &filename,
        })
        .unwrap();
        item.filename = filename;
        Ok(item)
    }
}
