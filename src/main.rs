extern crate env_logger;
extern crate paryxa_server;
extern crate warp;

use paryxa_server::{graphiql, graphql};
use std::env;
use warp::Filter;

const LOG: &str = "paryxa-server";

fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", LOG);
    }
    let log = warp::log(LOG);

    env_logger::init();
    warp::serve(graphiql().or(graphql()).with(log)).run(([127, 0, 0, 1], 4000));
}
