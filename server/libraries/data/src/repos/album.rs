use crate::models::*;
use crate::repos::Repository;
use mysql as my;
use mysql::params;

pub struct AlbumRepository {}

impl AlbumRepository {
    pub fn new() -> Self {
        AlbumRepository {}
    }
}

impl Repository for AlbumRepository {
    type Item = Album;

    fn select_query(&self) -> String {
        String::from("SELECT id, name, year, image, artist_id, external_id FROM albums")
    }

    fn insert_query(&self) -> String {
        String::from(
            r"
        INSERT INTO albums (name, year, image, artist_id, external_id)
        VALUES (:name, :year, :image, :artist_id, :external_id)",
        )
    }

    fn order_by(&self) -> String {
        String::from("ORDER BY year DESC, name ASC")
    }

    fn create<'a>(
        &self,
        item: &'a mut Self::Item,
        pool: &my::Pool,
    ) -> Result<&'a mut Self::Item, String> {
        let mut stmt = pool.prepare(self.insert_query()).unwrap();
        let result = stmt.execute(params! {
            "name" => &item.name,
            "year" => item.year,
            "image" => &item.image,
            "artist_id" => item.artist_id,
            "external_id" => &item.external_id,
        });

        match result {
            Err(err) => Err(err.to_string()),
            Ok(_) => Ok(item),
        }
    }
}
