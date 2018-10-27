#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate uuid;
#[macro_use]
extern crate failure;
extern crate bcrypt;
extern crate dotenv;
extern crate futures;
#[macro_use]
extern crate juniper;
extern crate juniper_warp;
#[macro_use]
extern crate log;
extern crate base64;
extern crate warp;

use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use gql_schema::create_schema;
use std::env;
use warp::{filters::BoxedFilter, Filter, Rejection};

mod basic;
mod db_types;
mod errors;
mod gql_schema;
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

pub struct Context {
    pub conn: PooledPg,
}

impl juniper::Context for Context {}

pub fn graphiql(
) -> impl Filter<Extract = (warp::http::Response<Vec<u8>>,), Error = Rejection> + Clone {
    warp::get2()
        .and(warp::index())
        .and(juniper_warp::graphiql_handler("/graphql"))
}

fn graphql_context() -> BoxedFilter<(Context,)> {
    let pg_pool = pg_pool();
    warp::any()
        .and_then(move || match pg_pool.get() {
            Ok(pooled) => Ok(Context { conn: pooled }),
            Err(_) => Err(warp::reject::server_error()),
        }).boxed()
}

pub fn graphql(
) -> impl Filter<Extract = (warp::http::Response<Vec<u8>>,), Error = Rejection> + Clone {
    let graphql_filter = juniper_warp::make_graphql_filter(create_schema(), graphql_context());
    warp::path("graphql").and(graphql_filter)
}
