/*

#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

use std::sync::RwLock;
use rocket::State;
use rocket::request::Form;
use rocket::response::Redirect;


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

*/

mod shortener;

use std::path::Path;
use shortener::SurlCollection;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
use serde::Deserialize;

#[derive(Deserialize)]
struct UrlForm {
    url: String,
}

#[post("/shorten/")]
async fn shorten(repo: web::Data<Mutex<SurlCollection>>, url_form: web::Form<UrlForm>) -> impl Responder {
    let mut repo = repo.lock().unwrap();
    let url = url_form.url.clone();
    let id = repo.store(&url).await;
    HttpResponse::Ok().body(id)
}

#[get("/{id}")]
async fn echo(repo: web::Data<Mutex<SurlCollection>>, web::Path(id): web::Path<String>) -> impl Responder {
    match repo.lock().unwrap().lookup(&id).await {
        Some(url) => HttpResponse::PermanentRedirect().header("Location",format!("{}", url)).body(""),
        _ => HttpResponse::NotFound().body("Requested ID was not found.")
    }
}

#[get("/test")]
async fn test() -> impl Responder {
    shortener::test().await;
    HttpResponse::Ok().body("Done")
}

/*#[get("/")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}*/


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let repo = web::Data::new(Mutex::new(SurlCollection::new().await));

    HttpServer::new(move || {
        App::new()
            .app_data(repo.clone())
            .service(shorten)
            .service(test)
            .service(echo)
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
