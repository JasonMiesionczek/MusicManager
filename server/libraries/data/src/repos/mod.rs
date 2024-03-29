use std::fmt::Display;
use mysql as my;
use std::collections::HashMap;

mod album;
mod artist;
mod playlist;
mod playlistsong;
mod song;
mod task;

pub use album::AlbumRepository;
pub use artist::ArtistRepository;
pub use playlist::PlaylistRepository;
pub use playlistsong::PlaylistSongRepository;
pub use song::SongRepository;
pub use task::TaskRepository;

pub enum UpdateValue {
    Str(String),
    Int(u32),
}

impl Display for UpdateValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            UpdateValue::Str(val) => write!(f, "'{}'", val),
            UpdateValue::Int(val) => write!(f, "{}", val),
        }
    }
}

pub trait Repository {
    type Item: From<my::Row> + Clone;

    fn select_query(&self) -> String;
    fn insert_query(&self) -> String;
    fn order_by(&self) -> String;

    fn create<'a>(
        &self,
        item: &'a mut Self::Item,
        pool: &my::Pool,
    ) -> Result<&'a mut Self::Item, String>;

    fn update(&self, table: &str, values: HashMap<&str, UpdateValue>, id: u32, pool: &my::Pool) {
        let params = values
            .iter()
            .map(|(k, v)| format!("{} = {}", k, v))
            .collect::<Vec<String>>()
            .join(", ");
        let query = format!("UPDATE {} SET {} WHERE id = {}", table, params, id);
        pool.prep_exec(query, ()).unwrap();
    }

    fn get_last_id(&self, pool: &my::Pool) -> u32 {
        let query = "SELECT LAST_INSERT_ID()";
        let mut result = pool.prep_exec(query, ()).unwrap();
        let row = result.nth(0).unwrap();
        let id = my::from_row::<u32>(row.unwrap());
        id
    }

    fn find_by_id(&self, id: u32, pool: &my::Pool) -> Option<Self::Item> {
        let query = format!("{} WHERE id = {}", self.select_query(), id);
        let results = self.query_and_map(pool, query, |row| Self::Item::from(row));
        results.get(0).cloned()
    }

    fn find_one_by(&self, predicate: HashMap<&str, &str>, pool: &my::Pool) -> Option<Self::Item> {
        self.find_by(predicate, &pool).get(0).cloned()
    }

    fn find_by(&self, predicate: HashMap<&str, &str>, pool: &my::Pool) -> Vec<Self::Item> {
        let params = predicate
            .iter()
            .map(|(k, v)| format!("{} = '{}'", k, v))
            .collect::<Vec<String>>()
            .join(" AND ");
        let query = format!(
            "{} WHERE {} {}",
            self.select_query(),
            params,
            self.order_by()
        );
        let results = self.query_and_map(pool, query, |row| Self::Item::from(row));
        results
    }

    fn find_by_external_id(&self, external_id: String, pool: &my::Pool) -> Option<Self::Item> {
        let mut predicate = HashMap::new();
        predicate.insert("external_id", external_id.as_str());
        let results = self.find_by(predicate, pool);
        results.get(0).cloned()
    }

    fn get_all(&self, pool: &my::Pool) -> Vec<Self::Item> {
        self.query_and_map(
            &pool,
            format!("{} {}", self.select_query(), self.order_by()),
            |row| Self::Item::from(row),
        )
    }

    fn query_and_map<F>(&self, pool: &my::Pool, query: String, func: F) -> Vec<Self::Item>
    where
        F: Fn(my::Row) -> Self::Item,
    {
        //println!("{}", query);
        let result: Vec<Self::Item> = pool
            .prep_exec(query, ())
            .map(|result| result.map(|x| x.unwrap()).map(func).collect())
            .unwrap();
        result
    }
}
