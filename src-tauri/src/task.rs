use serde::{Deserialize, Serialize};
use crate::my_date_time::MyDateTime;

pub type TaskId = i64;

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskData {
    pub title: String,
    pub tag: TaskId,
    pub body: String,
    pub difficulty: i32,
    pub create_time: MyDateTime,
    pub last_edit_time: MyDateTime,
    pub due_time: Option<MyDateTime>,
    pub target_time: Option<MyDateTime>,
    pub done_time: Option<MyDateTime>,
    pub paused: bool,
}

impl TaskData {
    fn is_done(&self) -> bool {
        self.done_time.is_some()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EditableTaskData {
    pub title: String,
    pub tag: TaskId,
    pub body: String,
    pub difficulty: i32,
    pub due_time: Option<MyDateTime>,
    pub target_time: Option<MyDateTime>,
    pub paused: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: TaskId,
    pub data: TaskData,
}
