use data::migrations::get_migrations;
use dotenv::var;
use mysql as my;

const HISTORY_TABLE_NAME: &str = "__migration_history";

pub fn run_migrations() {
    prepare_database();
    let migrations = get_migrations();
    let mut current_migration = get_current_migration() as usize;

    for (num, sql) in migrations.iter().enumerate() {
        let num = num + 1;
        if num > current_migration {
            print!("[ ] {}: {}", num, sql);
            execute_migration(sql.to_string());
            update_migrations(num as u32);
            current_migration = num.clone();
            println!("\r[✓] {}: {}", num, sql);
        } else {
            println!("[✓] {}: {}", num, sql);
        }
    }
}

fn run_query(query: &str) -> my::Result<my::QueryResult> {
    let pool = get_pool();
    pool.prep_exec(query, ())
}

fn get_pool() -> my::Pool {
    let url = var("DATABASE_URL").expect("could not find DATABASE_URL");
    let pool = my::Pool::new(url).unwrap();
    pool
}

fn execute_migration(sql: String) {
    run_query(&sql).unwrap();
}

fn get_current_migration() -> u32 {
    let query = format!(
        "select coalesce(max(current_migration),0) as current_migration from {}",
        HISTORY_TABLE_NAME
    );
    let mut result = run_query(&query).unwrap();
    let row = result.nth(0).unwrap();
    let max = my::from_row::<u32>(row.unwrap());
    max
}

fn update_migrations(num: u32) {
    let pool = get_pool();
    pool.prep_exec(
        format!(
            "UPDATE {} SET current_migration = {}",
            HISTORY_TABLE_NAME, num
        ),
        (),
    )
    .unwrap();
}

fn prepare_database() {
    let pool = get_pool();

    let prep_queries = vec![
        format!(
            "CREATE TABLE IF NOT EXISTS {} (current_migration INT NOT NULL)",
            HISTORY_TABLE_NAME
        ),
        format!(
            "INSERT INTO {0} (current_migration) SELECT 0 FROM DUAL WHERE NOT EXISTS (SELECT * FROM {0})",
            HISTORY_TABLE_NAME
        )
    ];

    for query in &prep_queries {
        pool.prep_exec(query, ()).unwrap();
    }
}
