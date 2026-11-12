use std::ops::Deref;
use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct Database {
    pool : PgPool
}

impl Deref for Database {
    type Target = PgPool;
    fn deref(&self) -> &Self::Target {
        &self.pool
    }
}