use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Serialize, Deserialize, Eq)]
pub struct TagData {
    pub name: String,
    pub color: Color,
    pub active: bool,
}

pub type TagId = u16;
#[derive(Serialize, Deserialize, Eq)]
pub struct Tag {
    pub id: TagId,
    pub data: TagData,
}
