use rocket::{serde::json::Json, Route};
use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    username: String,
    password: String,
}

#[post("/login", format = "json", data = "<user>")]
fn user_login<'r>(user: Json<User>) -> String {
    let username: &String = &user.username;
    let password: &String = &user.password;
    return format!("Username: {username}. Password: {password}.");
}

pub fn user_routes() -> Vec<Route> {
    return rocket::routes![user_login];
}
