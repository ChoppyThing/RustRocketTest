#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate mysql;
#[macro_use] extern crate chrono;
#[macro_use] extern crate serde_derive;

use rocket_contrib::templates::Template;

mod db;
mod home;
mod user;
mod admin;
mod session;

fn main() {
    rocket::ignite()
        .mount("/", routes![
            home::homepage::index,
            home::homepage::index_test,
            home::homepage::world,
            user::user::login_page,
            user::user::login,
            admin::post::new,
            admin::post::add
        ])
        .attach(Template::fairing())
        .attach(session::session::Session::default())
        .launch();
}
