pub mod models;
pub mod schema;

use diesel::prelude::*;
use models::{Dapp, NewDapp};
use schema::dapps;

pub fn insert_installed_dapp<'a>(connection: &SqliteConnection, name: &'a str, version: &'a str) {
    let new_dapp = NewDapp { name, version };

    diesel::insert_into(dapps::table)
        .values(&new_dapp)
        .execute(connection)
        .expect("Error adding new dapp to local db");
}

pub fn get_installed_dapps(connection: &SqliteConnection) -> Vec<Dapp> {
    dapps::table
        .load::<Dapp>(connection)
        .expect("Error loading dapps")
}
