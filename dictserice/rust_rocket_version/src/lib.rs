#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

extern crate redis;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate time;

pub mod types;
pub mod services;
pub mod web;
