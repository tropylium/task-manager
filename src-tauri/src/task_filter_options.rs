use crate::{MyDateTime, TagId, Task, TaskId};
use crate::filters::{*};

/// Represents the possible filters from a user, for every field of a `Task` supported
/// by this application.
pub struct TaskFilterOptions {
    pub id_filter: Option<ExactlyFilter<TaskId>>,
    pub title_filter: Option<ContainsStringFilter>,
    pub tag_filter: Option<OptionalFilter<SetFilter<TagId>>>,
    pub body_filter: Option<ContainsStringFilter>,
    pub difficulty_filter: Option<SetFilter<i32>>,
    pub create_time_filter: Option<OrderedRangeFilter<MyDateTime>>,
    pub last_edit_time_filter: Option<OrderedRangeFilter<MyDateTime>>,
    pub due_time_filter: Option<OptionalFilter<OrderedRangeFilter<MyDateTime>>>,
    pub target_time_filter: Option<OptionalFilter<OrderedRangeFilter<MyDateTime>>>,
    pub done_time_filter: Option<OptionalFilter<OrderedRangeFilter<MyDateTime>>>,
    pub paused_filter: Option<ExactlyFilter<bool>>,
}

impl ApplyFilter<Task> for TaskFilterOptions {
    fn passes(&self, task: &Task) -> bool {
        none_or_filter(&self.id_filter, &task.id) &&
        none_or_filter(&self.title_filter, &task.title) &&
        none_or_filter(&self.tag_filter, &task.tag) &&
        none_or_filter(&self.body_filter, &task.body) &&
        none_or_filter(&self.difficulty_filter, &task.difficulty) &&
        none_or_filter(&self.create_time_filter, &task.create_time) &&
        none_or_filter(&self.last_edit_time_filter, &task.last_edit_time) &&
        none_or_filter(&self.due_time_filter, &task.due_time) &&
        none_or_filter(&self.target_time_filter, &task.target_time) &&
        none_or_filter(&self.done_time_filter, &task.done_time) &&
        none_or_filter(&self.paused_filter, &task.paused)
    }
}

fn none_or<T, P: FnOnce(&T) -> bool>(opt: &Option<T>, pred: P) -> bool {
    match opt {
        Some(val) => pred(val),
        None => true,
    }
}

fn none_or_filter<T, F: ApplyFilter<T>>(opt_filter: &Option<F>, value: &T) -> bool {
    none_or(&opt_filter, |filter| filter.passes(value))
}
