pub mod migrations;
pub mod models;
pub mod repos;

use dotenv;

pub fn get_pool() -> mysql::Pool {
    let url = dotenv::var("DATABASE_URL").expect("could not find DATABASE_URL");
    let pool = mysql::Pool::new(url).unwrap();
    pool
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
