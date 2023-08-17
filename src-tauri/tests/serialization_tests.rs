mod util;

use std::collections::HashSet;
use chrono::{DateTime, TimeZone, Utc};
use app::{ContainsStringFilter, ExactlyFilter, GeneratedTagData, GeneratedTaskData, OptionalFilter, OrderedRangeFilter, SetFilter, Tag, Task, TaskFilterOptions};
use util::{*};

// manually inspect serialization output

#[test]
fn tag_serialization() {
    let sample_tag = Tag::from_parts(&sample_tag_data()[0],
                                     &GeneratedTagData {
        id: 0,
        create_time: Utc::now(),
    });
    let tag_json = serde_json::to_string_pretty(&sample_tag).unwrap();
    println!("{}", tag_json);
}

#[test]
fn task_serialization() {
    let sample_task = Task::from_parts(&sample_task_data()[0],
                                      &GeneratedTaskData {
                                          id: 0,
                                          create_time: Utc::now(),
                                          last_edit_time: Utc::now(),
                                          done_time: None,
                                      });
    let task_json = serde_json::to_string_pretty(&sample_task).unwrap();
    println!("{}", task_json);
}

#[test]
fn filter_serialization() {
    let sample_filter = TaskFilterOptions {
        id_filter: Some(ExactlyFilter { value: 0 }),
        title_filter: Some(ContainsStringFilter { pattern: String::from("hello") } ),
        tag_filter: Some(OptionalFilter::AcceptsSome(SetFilter { set: HashSet::from([1,2,3])})),
        body_filter: None, // same as title filter
        difficulty_filter: None, // similar to tag filter
        create_time_filter: Some(OrderedRangeFilter {
            lower_bound: Some(Utc.with_ymd_and_hms(2023, 5, 1, 15,30,0).unwrap()),
            upper_bound: Some(Utc.with_ymd_and_hms(2023, 10, 1, 15,30,0).unwrap()),
        }),
        last_edit_time_filter: None, // same to create time filter
        due_time_filter: Some(OptionalFilter::AcceptsNone),
        target_time_filter: None, // covered by above
        done_time_filter: None, // covered by above
        paused_filter: Some(ExactlyFilter { value: true }),
    };
    let filter_json = serde_json::to_string_pretty(&sample_filter).unwrap();
    println!("{}", filter_json);
}