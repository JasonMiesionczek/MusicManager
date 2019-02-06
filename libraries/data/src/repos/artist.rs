use crate::models::*;
use crate::repos::Repository;
use mysql as my;
use mysql::params;
use uuid::Uuid;

pub struct ArtistRepository {}

impl ArtistRepository {
    pub fn new() -> Self {
        ArtistRepository {}
    }
}

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
    ) -> Result<&'a mut Self::Item, String> {
        let mut stmt = pool.prepare(self.insert_query()).unwrap();
        let exid = Uuid::new_v4().to_string();
        let result = stmt.execute(params! {
            "name" => &item.name,
            "external_id" => exid.clone(),
        });

        match result {
            Err(err) => Err(err.to_string()),
            Ok(_) => {
                item.external_id = exid;
                if item.id == 0 {
                    item.id = self.get_last_id(pool);
                }
                Ok(item)
            }
        }
    }
}
