extern crate actix_web;
extern crate env_logger;
extern crate paryxa_server;

use actix_web::{
    actix::*,
    middleware::{
        identity::{CookieIdentityPolicy, IdentityService},
        Logger,
    },
    server, App, HttpRequest, HttpResponse,
};
use paryxa_server::{create_schema, graphiql, graphql, AppState, GraphQLExecutor};
use std::env;

fn logger() -> Logger {
    if cfg!(debug_assertions) {
        Logger::new(r#""%r" - %D ms"#)
    } else {
        Logger::default()
    }
}

fn not_found(_: &HttpRequest<AppState>) -> HttpResponse {
    HttpResponse::NotFound()
        .content_type("text/plain; charset=utf8")
        .body("Not Found")
}

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();
    let sys = System::new("paryxa");

    let schema = create_schema();
    let addr = SyncArbiter::start(3, move || GraphQLExecutor::new(schema.clone()));

    server::new(move || {
        App::with_state(AppState::new(addr.clone()))
            .middleware(logger())
            .middleware(IdentityService::new(
                // Set a different private key
                CookieIdentityPolicy::new(&[0; 32]).name("paryxahub"),
            )).resource("/graphql", |r| r.post().with_async(graphql))
            .resource("/", |r| {
                if cfg!(debug_assertions) {
                    r.get().f(graphiql);
                }
            }).default_resource(|r| r.f(not_found))
    }).bind("127.0.0.1:4000")
    .unwrap()
    .start();

    println!("Started http server at http://localhost:4000");
    let _ = sys.run();
}
