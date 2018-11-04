#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate irc;

use rocket_contrib::Template;

#[get("/")]
fn index() -> String {

}

fn main() {
    rocket::ignite()
    .mount("/", routes![hello])
    .attach(Template::fairing())
    .launch();
}