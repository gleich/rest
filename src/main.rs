use rocket::{
    build,
    fs::{relative, FileServer},
};

mod personal;
mod result;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    build()
        .mount("/", routes![personal::route])
        .mount("/", FileServer::from(relative!("static")))
}
