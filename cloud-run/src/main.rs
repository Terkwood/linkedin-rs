#![feature(proc_macro_hygiene, decl_macro)]

extern crate chrono;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod cors;
pub mod models;
pub mod routes;
fn main() {
    todo!("oauth")
}
