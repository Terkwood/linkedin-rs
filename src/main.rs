// SPDX-License-Identifier: MIT
extern crate log;
extern crate tokio;
extern crate warp;

use log::info;
use std::env;
use warp::Filter;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("{}", VERSION);

    // GET /auth => 200 OK with body "NO AUTH YET"
    let auth = warp::path!("auth").map(|| format!("NO AUTH YET"));

    warp::serve(auth).run(([0, 0, 0, 0], 3030)).await;
}
