pub mod models;
pub mod schema;

use diesel::prelude::*;
use models::{Dapp, NewDapp};
use schema::dapps;
use std::path::Path;

pub fn insert_installed_dapp<'a>(
    connection: &SqliteConnection,
    name: &'a str,
    version: &'a str,
    install_location: &Path,
) {
    let new_dapp = NewDapp {
        name,
        version,
        install_location: install_location
            .to_str()
            .expect("Couldn't convert path to str"),
    };

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

pub fn get_installed_dapp(connection: &SqliteConnection, dapp_name: &str) -> Dapp {
    dapps::table
        .filter(dapps::name.eq(dapp_name))
        .first(connection)
        .expect(&format!("Error loading dapp with name {}", dapp_name))
}
