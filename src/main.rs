use std::env;

use rocket::build;
use rocket::fs::{relative, FileServer};

mod personal;
mod result;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    dbg!(env::var("DATABASE_URL").expect("Failed to get database URL"));
    build()
        .mount("/", routes![personal::route])
        .mount("/", FileServer::from(relative!("static")))
}
