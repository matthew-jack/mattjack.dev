#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate comrak;
extern crate rocket_contrib;
extern crate serde;

use rocket::routes;
use rocket_contrib::templates::Template;

mod controllers;
mod models;

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![controllers::home::index, controllers::home::about,],
        )
        .mount(
            "/posts",
            routes![
                controllers::posts::index,
                controllers::posts::get_post,
                controllers::posts::get_day,
                controllers::posts::get_month,
                controllers::posts::get_year,
                controllers::posts::get_misc,
            ],
        )
        .mount(
            "/public_key",
            routes![
                controllers::assets::public_key,
            ],
        )
        .attach(Template::fairing())
        .launch();
}
