use mysql as my;

pub mod artist;

pub trait Repository {
    type Item;

    fn create<'a>(
        &self,
        item: &'a mut Self::Item,
        pool: my::Pool,
    ) -> Result<&'a mut Self::Item, &'static str>;
    fn find_by_id(&self, id: u32, pool: my::Pool) -> Option<Self::Item>;
    fn get_all(&self, pool: my::Pool) -> Vec<Self::Item>;

    fn query_and_map<F>(&self, pool: my::Pool, query: String, func: F) -> Vec<Self::Item>
    where
        F: Fn(my::Row) -> Self::Item,
    {
        let result: Vec<Self::Item> = pool
            .prep_exec(query, ())
            .map(|result| result.map(|x| x.unwrap()).map(func).collect())
            .unwrap();
        result
    }
}
