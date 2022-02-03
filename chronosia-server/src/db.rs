use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use r2d2::Pool;
use std::env;

// The Postgres-specific connection pool managing all database connections.
pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

// Retrieves database url from .env file and return a pool with the new diesel connection
// R2D2 Connection pool => https://github.com/sfackler/r2d2#:~:text=A%20connection%20pool%20maintains%20a,check%20the%20health%20of%20connections.
pub fn get_pool() -> PostgresPool {
    // Environment
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("Error connecting to Database");
    // Conection
    let connection = ConnectionManager::<PgConnection>::new(url);
    // Pool
    r2d2::Pool::builder()
        .build(connection)
        .expect("could not build connection pool")
}
