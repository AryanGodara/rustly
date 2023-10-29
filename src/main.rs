#[macro_use]
extern crate rocket;

extern crate harsh;

use std::sync::RwLock;
use rocket::State;
use rocket::form::Form;
use rocket::response::Redirect;
use rocket::request::FromRequest;
use rocket::request::{self, Request};
use rocket::outcome::Outcome;

mod repository;
mod shortener;
use repository::Repository;

#[derive(FromForm, Debug)]
struct Url {
    url: String,
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for State<RwLock<Repository>> {
    type Error = ();

    fn from_request(request: &'a Request<'_>) -> request::Outcome<Self, Self::Error> {
        let state = request.guard::<State<RwLock<Repository>>>()?;
        Outcome::Success(state)
    }
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
fn lookup(repo: State<RwLock<Repository>>,id: &str) -> Result<Redirect, &'static str> {
    match repo.read().unwrap().lookup(id) {
        Some(url) => Ok(Redirect::permanent(url)),
        _ => Err("Requested ID was not found.\n")
    }
}

#[post("/", data = "<url_form>")]
fn shorten(repo: State<RwLock<Repository>>,url_form: Form<Url>) -> Result<String, String> {
    let ref url = url_form.url;
    let mut repo = repo.write().unwrap();
    let id = repo.store(url);
    Ok(id.to_string())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(RwLock::new(Repository::new()))
        .mount("/", routes![index, lookup, shorten])
}