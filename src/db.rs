use diesel::mysql::MysqlConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;
use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome};
use rocket::{ Request, State };
use std::ops::Deref;

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn init_pool(db_url: String) -> Pool {
    let manager = ConnectionManager::<MysqlConnection>::new(db_url);
    r2d2::Pool::new(manager).expect("db pool failure")
}

pub struct Conn(pub r2d2::PooledConnection<ConnectionManager<MysqlConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for Conn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Conn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Conn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for Conn {
    type Target = MysqlConnection;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}