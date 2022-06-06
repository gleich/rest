use std::env;

use anyhow::{Context, Result};
use chrono::NaiveDate;
use diesel::{Connection, PgConnection, RunQueryDsl};

use crate::schema::activities;

pub fn connect() -> Result<PgConnection> {
    let database_url = env::var("DATABASE_URL").context("DATABASE_URL must be set")?;

    Ok(PgConnection::establish(&database_url)
        .context(format!("Failed to connect to {}", database_url))?)
}

pub fn fetch_rides(database: &PgConnection) -> Result<Vec<Activity>> {
    Ok(activities::table.load::<Activity>(database)?)
}

#[derive(Debug, Queryable, AsChangeset)]
#[table_name(activities)]
pub struct Activity {
    pub id: i32,
    pub title: String,
    pub location: String,
    pub seconds: i32,
    pub miles: f32,
    pub date: NaiveDate,
    pub elevation: i32,
    pub average_speed: f32,
    pub average_power: i32,
}
