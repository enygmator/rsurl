#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

use std::sync::RwLock;
use rocket::State;
use rocket::request::Form;
use rocket::response::Redirect;

mod repository;
mod shortener;
use repository::Repository;

#[derive(FromForm)]
struct Url {
    url: String,
}

#[get("/<id>")]
fn lookup(repo: State<RwLock<Repository>>, id: String) -> Result<Redirect, &'static str> {
    match repo.read().unwrap().lookup(&id) {
        Some(url) => Ok(Redirect::permanent(format!("{}", url))),
        _ => Err("Requested ID was not found.")
    }
}

#[post("/", data = "<url_form>")]
fn shorten(repo: State<RwLock<Repository>>, url_form: Form<Url>) -> Result<String, String> {
    let ref url = url_form.url;
    let mut repo = repo.write().unwrap();
    let id = repo.store(&url);
    Ok(id.to_string())
}

fn main() {
    rocket::ignite().manage(RwLock::new(Repository::new()))
        .mount("/", routes![lookup, shorten])
        .launch();
}
