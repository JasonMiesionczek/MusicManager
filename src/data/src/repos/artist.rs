use crate::models::*;
use crate::repos::Repository;
use mysql as my;
use mysql::params;
use uuid;

pub struct ArtistRepository {}

impl Repository for ArtistRepository {
    type Item = Artist;

    fn create<'a>(
        &self,
        item: &'a mut Self::Item,
        pool: my::Pool,
    ) -> Result<&'a mut Self::Item, &'static str> {
        let mut stmt = pool
            .prepare("INSERT INTO artists (name, external_id) VALUES (:name, :external_id)")
            .unwrap();
        let exid = uuid::Uuid::new_v4().to_string();
        stmt.execute(params! {
            "name" => &item.name,
            "external_id" => exid.clone(),
        })
        .unwrap();
        item.external_id = exid;
        Ok(item)
    }

    fn find_by_id(&self, id: u32, pool: my::Pool) -> Option<Self::Item> {
        let query = format!(
            "SELECT id, name, external_id FROM artists WHERE id = {}",
            id
        );
        let results = self.query_and_map(pool, query, |row| {
            let (id, name, external_id) = my::from_row(row);
            Artist {
                id,
                name,
                external_id,
            }
        });
        if let Some(value) = results.get(0) {
            Some(value.to_owned())
        } else {
            None
        }
    }

    fn get_all(&self, pool: my::Pool) -> Vec<Self::Item> {
        let query = "SELECT id, name, external_id FROM artists";
        let results = self.query_and_map(pool, String::from(query), |row| {
            let (id, name, external_id) = my::from_row(row);
            Artist {
                id,
                name,
                external_id,
            }
        });
        results
    }
}
