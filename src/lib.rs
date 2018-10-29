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
extern crate ttl_cache;
extern crate warp;
#[macro_use]
extern crate lazy_static;

use basic::BasicUser;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use errors::{Error, SResult};
use gql_schema::create_schema;
use models::user::{verify_user, User};
use std::{env, sync::Mutex, time::Duration};
use ttl_cache::TtlCache;
use warp::{
    filters::BoxedFilter,
    http::{Response, StatusCode},
    Filter, Rejection,
};

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
    pub user: Option<User>,
}

impl Context {
    fn auth_user(&self) -> SResult<&User> {
        self.user.as_ref().ok_or(Error::Unauthorized)
    }

    fn admin_only(&self) -> SResult<&User> {
        self.auth_user().and_then(|user| {
            if user.is_admin() {
                Ok(user)
            } else {
                Err(Error::Unauthorized)
            }
        })
    }

    fn me_only(&self, id: i32) -> SResult<&User> {
        self.auth_user().and_then(|user| {
            if user.id == id {
                Ok(user)
            } else {
                Err(Error::Unauthorized)
            }
        })
    }
}

impl juniper::Context for Context {}

pub fn graphiql() -> impl Filter<Extract = (Response<Vec<u8>>,), Error = Rejection> + Clone {
    warp::get2()
        .and(warp::index())
        .and(juniper_warp::graphiql_handler("/graphql"))
}

fn graphql_context() -> BoxedFilter<(Context,)> {
    pg_conn()
        .and(basic::basic_optional())
        .and_then(user_lookup)
        .map(|(pooled, user)| Context { conn: pooled, user })
        .boxed()
}

pub fn graphql() -> impl Filter<Extract = (Response<Vec<u8>>,), Error = Rejection> + Clone {
    let graphql_filter = juniper_warp::make_graphql_filter(create_schema(), graphql_context());
    warp::path("graphql")
        .and(graphql_filter)
        .recover(handle_error)
        .unify()
}

fn pg_conn() -> impl Filter<Extract = (PooledPg,), Error = Rejection> + Clone {
    let pg_pool = pg_pool();
    warp::any().and_then(move || match pg_pool.get() {
        Ok(pooled) => Ok(pooled),
        Err(_) => Err(warp::reject::server_error()),
    })
}

lazy_static! {
    static ref AUTH_CACHE: Mutex<TtlCache<BasicUser, i32>> = Mutex::new(TtlCache::new(10000));
}

fn user_lookup(
    conn: PooledPg,
    user: Option<BasicUser>,
) -> Result<(PooledPg, Option<User>), Rejection> {
    if let Some(user) = user {
        // Check if the user is already in cache.
        {
            let cache = AUTH_CACHE.lock().unwrap();
            if let Some(&user_id) = cache.get(&user) {
                let user = User::find(user_id, &conn).map_err(|_| warp::reject::forbidden())?;
                return Ok((conn, Some(user)));
            }
        }

        // Else verify the user.
        let found_user = User::find_by_email(&user.username, &conn)
            .and_then(|found| verify_user(found, &user.password))
            .map_err(|_| warp::reject::forbidden())?;

        // And remember it for the rest of the day as verification is a very
        // slow process.
        {
            let mut cache = AUTH_CACHE.lock().unwrap();
            cache.insert(user, found_user.id, Duration::from_secs(86400));
        }

        Ok((conn, Some(found_user)))
    } else {
        Ok((conn, None))
    }
}

const UNAUTHORIZED: &str = r#"{"data":null,"errors":[{"message":"This is an unauthorized request.","extensions":{"kind":"UNAUTHORIZED"}}]}"#;
const SERVER_ERROR: &str = r#"{"data":null,"errors":[{"message":"Something bad happened..","extensions":{"kind":"INTERNAL_SERVER_ERROR"}}]}"#;

fn graphql_like_response(body: &'static str) -> Response<Vec<u8>> {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(body.to_string().into_bytes())
        .unwrap()
}

fn handle_error(err: Rejection) -> Result<Response<Vec<u8>>, Rejection> {
    match err.status() {
        StatusCode::FORBIDDEN => Ok(graphql_like_response(UNAUTHORIZED)),
        StatusCode::INTERNAL_SERVER_ERROR => Ok(graphql_like_response(SERVER_ERROR)),
        _ => Err(err),
    }
}
