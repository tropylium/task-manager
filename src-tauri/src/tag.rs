use serde::{Deserialize, Serialize};
use crate::hsl_color::HslColor;
use crate::my_date_time::MyDateTime;

pub type TagId = i64;
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
/// Represents a tag in this application.
pub struct Tag {
    pub id: TagId,
    pub name: String,
    pub color: HslColor,
    pub active: bool,
    pub create_time: MyDateTime,
}

impl Tag {
    pub fn from_parts(editable: &EditableTagData, generated: &GeneratedTagData) -> Self {
        Tag {
            id: generated.id,
            name: editable.name.clone(),
            color: editable.color.clone(),
            active: editable.active,
            create_time: MyDateTime::from(generated.create_time.clone()),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
/// Fields of a `Tag` determined by the database when a new task is created.
pub struct GeneratedTagData {
    pub id: TagId,
    pub create_time: MyDateTime,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
/// Fields of a `Tag` modifiable by the client.
pub struct EditableTagData {
    pub name: String,
    pub color: HslColor,
    pub active: bool,
}
