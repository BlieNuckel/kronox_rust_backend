use rocket::{response::content, Catcher};

#[catch(422)]
fn unprocessable_entity() -> content::Json<&'static str> {
    content::Json("{ 'error': 'can't parse supplied body.' }")
}

#[catch(404)]
fn not_found() -> content::Json<&'static str> {
    content::Json("{ 'error': 'couldn't find the requested schedule.' }")
}

pub fn catchers() -> Vec<Catcher> {
    return rocket::catchers![unprocessable_entity, not_found];
}
