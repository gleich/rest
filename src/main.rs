#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

use std::env;

use rocket::build;
use rocket::fs::{relative, FileServer};

mod db;
mod personal;
mod result;
mod schema;

#[launch]
fn rocket() -> _ {
    let db_conn = db::connect().expect("Failed to connect to db");
    dbg!(db::fetch_rides(&db_conn).expect("Failed to fetch rides"));

    build()
        .mount("/", routes![personal::route])
        .mount("/", FileServer::from(relative!("static")))
}
