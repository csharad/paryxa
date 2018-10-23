#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate uuid;
#[macro_use]
extern crate failure;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate actix_web;
extern crate bcrypt;
extern crate dotenv;
extern crate tokio_threadpool;
extern crate futures;

use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use errors::SResult;
use models::user::{User, UserForm};
use serde::{de::DeserializeOwned, Serialize};
use std::env;

mod db_types;
mod errors;
mod handlers;
mod models;
#[allow(unused_imports)]
mod schema;

type PgPool = Pool<ConnectionManager<PgConnection>>;
type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

fn pg_pool() -> PgPool {
    dotenv::dotenv().unwrap();

    let postgres_url = env::var("DATABASE_URL").expect("Could not find `DATABASE_URL` in the env.");
    let manager = ConnectionManager::<PgConnection>::new(postgres_url);
    Pool::new(manager).expect("Postgres connection pool could not be created")
}

pub use handlers::rest_resources;

pub struct AppState {
    pg_pool: PgPool,
}

impl AppState {
    pub fn new() -> AppState {
        AppState { pg_pool: pg_pool() }
    }

    fn pooled_pg(&self) -> SResult<PooledPg> {
        Ok(self.pg_pool.get()?)
    }
}
