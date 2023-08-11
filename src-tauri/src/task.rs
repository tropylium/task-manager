use serde::{Deserialize, Serialize};
use crate::my_date_time::MyDateTime;
use crate::TagId;

pub type TaskId = i64;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
/// Represents a task in this application.
pub struct Task {
    pub id: TaskId,
    pub title: String,
    pub tag: Option<TagId>,
    pub body: String,
    pub difficulty: i32,
    pub create_time: MyDateTime,
    pub last_edit_time: MyDateTime,
    pub due_time: Option<MyDateTime>,
    pub target_time: Option<MyDateTime>,
    pub done_time: Option<MyDateTime>,
    pub paused: bool,
}

impl Task {
    fn is_done(&self) -> bool {
        self.done_time.is_some()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
/// Fields of a `Task` modifiable by the client.
pub struct EditableTaskData {
    pub title: String,
    pub tag: Option<TagId>,
    pub body: String,
    pub difficulty: i32,
    pub due_time: Option<MyDateTime>,
    pub target_time: Option<MyDateTime>,
    pub paused: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
/// Fields of a `Task` determined by the database when a new task is created.
pub struct GeneratedTaskData {
    pub id: TaskId,
    pub create_time: MyDateTime,
    pub last_edit_time: MyDateTime,
    pub done_time: Option<MyDateTime>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
/// Fields of a `Task` determined by the database when an existing task is modified.
pub struct ModifiedTaskData {
    pub last_edit_time: MyDateTime,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
/// Fields of a `Task` determined by the database when an existing task is marked (or unmarked) as done.
pub struct FinishedTaskData {
    pub done_time: Option<MyDateTime>,
}
