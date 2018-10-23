use actix_web::{http::Method, HttpRequest, Scope};
use errors::{Error, SResult};
use futures::Future;
use {AppState, PooledPg};

mod user_handler;

pub fn rest_resources(s: Scope<AppState>) -> Scope<AppState> {
    s.nested("/users", |s| {
        s.resource("", |r| {
            r.post().with_async(user_handler::create_user);
            r.get().with_async(user_handler::get_all_user);
        }).nested("/{uuid}", |s| {
            s.resource("", |r| {
                r.method(Method::PATCH)
                    .with_async(user_handler::update_user);
                r.get().with_async(user_handler::get_user);
                r.delete().with_async(user_handler::delete_user);
            }).resource("/user-type", |r| {
                r.method(Method::PATCH)
                    .with_async(user_handler::update_user_type);
            })
        })
    }).resource("/login", |r| {
        r.post().with_async(user_handler::login);
    }).resource("/logout", |r| {
        r.post().f(user_handler::logout);
    })
}

/// Convenience fn to execute database queries with the CPU pool.
fn exec_db<T: 'static + Send>(
    req: &HttpRequest<AppState>,
    operation: impl FnOnce(PooledPg) -> SResult<T> + Send + 'static,
) -> impl Future<Item = T, Error = Error> {
    let conn = req.state().pooled_pg();
    req.cpu_pool().spawn_fn(move || Ok(operation(conn?)?))
}
