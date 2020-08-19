use r2d2_diesel::ConnectionManager;
use diesel::pg::PgConnection;
use r2d2::{ Pool, PooledConnection };

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn init_pool(db_url: String) -> PgPool {
  let connection_manager = ConnectionManager::<PgConnection>::new(db_url);
  Pool::new(connection_manager).expect("failed to create connection pool")
}

pub struct Conn(pub PooledConnection<ConnectionManager<PgConnection>>);