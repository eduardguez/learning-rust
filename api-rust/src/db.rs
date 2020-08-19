use rocket::request::{ self, FromRequest };
use rocket::{ Outcome, Request, State };
use r2d2::{ Pool, PooledConnection };
use r2d2_diesel::ConnectionManager;
use diesel::pg::PgConnection;
use rocket::http::Status;
use std::ops::Deref;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn init_pool(db_url: String) -> PgPool {
  let connection_manager = ConnectionManager::<PgConnection>::new(db_url);
  Pool::new(connection_manager).expect("failed to create connection pool")
}

pub struct Conn(pub PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for Conn {
  type Error = ();

  fn from_request(request: &'a Request<'r>) -> request::Outcome<Conn, ()> {
    let pool = request.guard::<State<PgPool>>()?;
    match pool.get() {
      Ok(conn) => Outcome::Success(Conn(conn)),
      Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
    }
  }
}

impl Deref for Conn {
  type Target = PgConnection;

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}