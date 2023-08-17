use serde::{Deserialize, Serialize};
use crate::TagId;
use chrono::{DateTime, Utc, serde::ts_seconds, serde::ts_seconds_option};

pub type TaskId = i64;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
/// Represents a task in this application.
pub struct Task {
    pub id: TaskId,
    pub title: String,
    pub tag: Option<TagId>,
    pub body: String,
    pub difficulty: i32,
    #[serde(with = "ts_seconds")]
    pub create_time: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub last_edit_time: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub due_time: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    pub target_time: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    pub done_time: Option<DateTime<Utc>>,
    pub paused: bool,
}

impl Task {
    pub fn from_parts(editable: &EditableTaskData, generated: &GeneratedTaskData) -> Self {
        Self {
            id: generated.id,
            title: editable.title.clone(),
            tag: editable.tag,
            body: editable.body.clone(),
            difficulty: editable.difficulty,
            create_time: generated.create_time.clone(),
            last_edit_time: generated.last_edit_time.clone(),
            due_time: editable.due_time.as_ref().map(|time| time.clone()),
            target_time: editable.target_time.as_ref().map(|time| time.clone()),
            done_time: generated.done_time.as_ref().map(|time| time.clone()),
            paused: false,
        }
    }
    pub fn is_done(&self) -> bool {
        self.done_time.is_some()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
/// Fields of a `Task` modifiable by the client.
pub struct EditableTaskData {
    pub title: String,
    pub tag: Option<TagId>,
    pub body: String,
    pub difficulty: i32,
    #[serde(with = "ts_seconds_option")]
    pub due_time: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    pub target_time: Option<DateTime<Utc>>,
    pub paused: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
/// Fields of a `Task` determined by the database when a new task is created.
pub struct GeneratedTaskData {
    pub id: TaskId,
    #[serde(with = "ts_seconds")]
    pub create_time: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub last_edit_time: DateTime<Utc>,
    #[serde(with = "ts_seconds_option")]
    pub done_time: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
/// Fields of a `Task` determined by the database when an existing task is modified.
pub struct ModifiedTaskData {
    #[serde(with = "ts_seconds")]
    pub last_edit_time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
/// Fields of a `Task` determined by the database when an existing task is marked (or unmarked) as done.
pub struct FinishedTaskData {
    #[serde(with = "ts_seconds_option")]
    pub done_time: Option<DateTime<Utc>>,
}
