use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Pokemon {
    pub name: String,
    pub id: u32,
}