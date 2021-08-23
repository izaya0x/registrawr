use super::schema::dapps;
use chrono::NaiveDateTime;

#[derive(Queryable, Debug)]
pub struct Dapp {
    pub id: i32,
    pub name: String,
    pub version: String,
    pub install_location: String,
    pub installed_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "dapps"]
pub struct NewDapp<'a> {
    pub name: &'a str,
    pub version: &'a str,
    pub install_location: &'a str,
}
