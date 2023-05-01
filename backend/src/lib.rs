pub mod item;
pub mod schema;
pub mod view;

use dotenvy::dotenv;
use std::env;
use diesel::prelude::*;
use deadpool_diesel::sqlite::*;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn establish_deadpool_connection() -> deadpool_diesel::Pool<deadpool_diesel::Manager<diesel::SqliteConnection>> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").unwrap();

    // set up connection pool
    let manager = Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    Pool::builder(manager)
        .build()
        .unwrap()
}
