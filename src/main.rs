#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

extern crate harsh;

use rocket::request::Form;
use rocket::response::Redirect;
use rocket::State;
use std::sync::RwLock;

mod repository;
mod shortener;
use repository::Repository;

#[derive(FromForm, Debug)]
struct Url {
    url: String,
}

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      POST /
          Ex: curl --data \"url=https://www.endler.dev\" http://localhost:8000
          It shold respond with a shortened url like http://localhost:8000/gY

      GET /<id>
          Redirects to shortned link. Try from browser or using the example below.
          Ex: curl -i http://localhost:8000/gY
    "
}
#[get("/<id>")]
fn lookup(repo: State<RwLock<Repository>>, id: String) -> Result<Redirect, &'static str> {
    match repo.read().unwrap().lookup(&id) {
        Some(url) => Ok(Redirect::permanent(format!("{}", url))),
        _ => Err("Requested ID was not found."),
    }
}

#[post("/", data = "<url_form>")]
fn shorten(repo: State<RwLock<Repository>>, url_form: Form<Url>) -> Result<String, String> {
    let ref url = format!("{}", url_form.into_inner().url);
    if !url.starts_with("https") && !url.starts_with("http") {
        return Err(format!("Not a valid URL {:?}", url));
    }
    let mut repo = repo.write().unwrap();
    let id = repo.store(&url);
    Ok(format!("http://localhost:8000/{}", id))
}
fn main() {
    rocket::ignite()
        .manage(RwLock::new(Repository::new()))
        .mount("/", routes![index, lookup, shorten])
        .launch();
}
