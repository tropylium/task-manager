use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Default, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TagData {
    pub name: String,
    pub color: Color,
    pub active: bool,
}

pub type TagId = u16;
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Tag {
    pub id: TagId,
    pub data: TagData,
}
