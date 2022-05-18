mod models;
mod routes;
mod services;
mod utils;

#[macro_use]
extern crate rocket;
extern crate chrono;
extern crate dotenv;
extern crate serde;

use dotenv::dotenv;
use rocket::{Catcher, Route};
use std::vec;

#[get("/")]
fn index() -> &'static str {
    return "Hello, world!";
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let mut routes: Vec<Route> = vec![];
    let mut catchers: Vec<Catcher> = vec![];

    routes.append(&mut routes![index]);
    routes.append(&mut routes::schedules::schedule_routes());
    routes.append(&mut routes::users::user_routes());

    catchers.append(&mut routes::catchers::catchers());

    rocket::build().mount("/", routes).register("/", catchers)
}
