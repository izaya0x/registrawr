use super::schema::dapps;

#[derive(Queryable)]
pub struct Dapp {
    pub id: i32,
    pub name: String,
    pub version: String,
}

#[derive(Insertable)]
#[table_name = "dapps"]
pub struct NewDapp<'a> {
    pub name: &'a str,
    pub version: &'a str,
}
