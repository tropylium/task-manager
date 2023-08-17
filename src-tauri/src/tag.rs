use chrono::{DateTime, Utc, serde::ts_seconds};
use serde::{Deserialize, Serialize};
use crate::hsl_color::HslColor;
use crate::my_date_time::MyDateTime;

pub type TagId = i64;
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Represents a tag in this application.
pub struct Tag {
    pub id: TagId,
    pub name: String,
    pub color: HslColor,
    pub active: bool,
    #[serde(with = "ts_seconds")]
    pub create_time: DateTime<Utc>,
}

impl Tag {
    pub fn from_parts(editable: &EditableTagData, generated: &GeneratedTagData) -> Self {
        Tag {
            id: generated.id,
            name: editable.name.clone(),
            color: editable.color.clone(),
            active: editable.active,
            create_time: generated.create_time.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Fields of a `Tag` determined by the database when a new task is created.
pub struct GeneratedTagData {
    pub id: TagId,
    #[serde(with = "ts_seconds")]
    pub create_time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
/// Fields of a `Tag` modifiable by the client.
pub struct EditableTagData {
    pub name: String,
    pub color: HslColor,
    pub active: bool,
}
