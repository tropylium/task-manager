use crate::TagId;

pub type TaskId = i64;

pub struct TaskData {
    pub title: String,
    pub tags: Vec<TagId>,
    pub description: String,
    pub difficulty: i32,
    pub create_time: i64,
    pub last_edit_time: i64,
    pub due_time: Option<i64>,
    pub target_time: Option<i64>,
    pub start_time: Option<i64>,
    pub done_time: Option<i64>,
    pub paused: bool,
    pub active: bool,
}

pub struct Task {
    pub id: TaskId,
    pub data: TaskData,
}
