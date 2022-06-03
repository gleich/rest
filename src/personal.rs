use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use lazy_static::lazy_static;
use rocket::serde::json::Json;
use serde::Serialize;

lazy_static! {
    static ref BIRTHDAY: DateTime<Utc> = DateTime::from_utc(
        NaiveDateTime::new(
            NaiveDate::from_ymd(2004, 4, 8),
            NaiveTime::from_hms(14, 41, 0)
        ),
        Utc
    );
}

#[derive(Serialize)]
pub struct Data {
    pub first_name: String,
    pub last_name: String,
    pub bday: DateTime<Utc>,
    pub age: f32,
    pub school: String,
}

#[get("/personal", format = "json")]
pub fn route() -> Json<Data> {
    Json(Data {
        first_name: String::from("Matt"),
        last_name: String::from("Gleich"),
        bday: *BIRTHDAY,
        age: ((Utc::now() - *BIRTHDAY).num_seconds() / 31556952) as f32,
        school: String::from("Goffstown High School"),
    })
}
