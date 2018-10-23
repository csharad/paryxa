extern crate actix_web;
extern crate env_logger;
extern crate paryxa_server;

use actix_web::{
    middleware::{
        identity::{CookieIdentityPolicy, IdentityService},
        Logger,
    },
    server, App, HttpRequest, HttpResponse,
};
use paryxa_server::{rest_resources, AppState};
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

    server::new(|| {
        App::with_state(AppState::new())
            .middleware(logger())
            .middleware(IdentityService::new(
                // Set a different private key
                CookieIdentityPolicy::new(&[0; 32]).name("paryxahub"),
            )).scope("/", rest_resources)
            .default_resource(|r| r.f(not_found))
    }).bind("127.0.0.1:4000")
    .unwrap()
    .run();
}
