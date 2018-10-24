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
extern crate futures;
extern crate tokio_threadpool;
#[macro_use]
extern crate juniper;
extern crate serde_json;

use actix_web::actix::*;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use errors::SResult;
use std::env;

mod db_types;
mod errors;
mod gql_schema;
mod graphql;
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

pub use gql_schema::create_schema;
pub use graphql::{graphiql, graphql, GraphQLExecutor};

pub struct AppState {
    pg_pool: PgPool,
    gql_executor: Addr<GraphQLExecutor>,
}

impl AppState {
    pub fn new(gql_executor: Addr<GraphQLExecutor>) -> AppState {
        AppState {
            pg_pool: pg_pool(),
            gql_executor,
        }
    }

    fn pooled_pg(&self) -> SResult<PooledPg> {
        Ok(self.pg_pool.get()?)
    }
}
