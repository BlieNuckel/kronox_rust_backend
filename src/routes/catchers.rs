use rocket::{response::content, Catcher};

#[catch(422)]
fn unprocessable_entity() -> content::Json<&'static str> {
    content::Json("{ 'error': 'can't parse supplied body.' }")
}

pub fn catchers() -> Vec<Catcher> {
    return rocket::catchers![unprocessable_entity];
}
