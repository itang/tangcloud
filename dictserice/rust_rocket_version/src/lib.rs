#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde;
extern crate serde_json;
//#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate redis;
extern crate time;


pub mod types;
pub mod services;
pub mod web;
