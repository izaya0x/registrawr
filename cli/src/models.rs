#[derive(Queryable)]
pub struct Dapp {
    pub id: i32,
    pub name: String,
    pub version: String,
}
