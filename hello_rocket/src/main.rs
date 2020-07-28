#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::NamedFile;
use std::path::*;

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join("hello.html")).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}