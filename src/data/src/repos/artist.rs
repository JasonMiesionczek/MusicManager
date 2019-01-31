use crate::models::*;
use crate::repos::Repository;
use mysql as my;
use mysql::params;
use uuid;

pub struct ArtistRepository {}

impl Repository for ArtistRepository {
    type Item = Artist;

    fn select_query(&self) -> String {
        String::from("SELECT id, name, external_id FROM artists")
    }

    fn insert_query(&self) -> String {
        String::from("INSERT INTO artists (name, external_id) VALUES (:name, :external_id)")
    }

    fn create<'a>(
        &self,
        item: &'a mut Self::Item,
        pool: &my::Pool,
    ) -> Result<&'a mut Self::Item, &'static str> {
        let mut stmt = pool.prepare(self.insert_query()).unwrap();
        let exid = uuid::Uuid::new_v4().to_string();
        stmt.execute(params! {
            "name" => &item.name,
            "external_id" => exid.clone(),
        })
        .unwrap();
        item.external_id = exid;
        if item.id == 0 {
            item.id = self.get_last_id(pool);
        }
        Ok(item)
    }
}
