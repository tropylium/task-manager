#[allow(dead_code, unused_mut)]

use chrono::Utc;
use app::*;
use DbError::{TaskDoesNotExistError, TaskStatusError};
mod util;
use util::*;

#[test]
fn db_task_empty() {
    run_db_test(|mut db| {
        assert_eq!(db.all_tasks().expect("Get all tasks should not fail"), vec![]);
    });
}

#[test]
fn db_task_add_new_success() {
    run_db_test(|mut db| {
        let tag_result0 = db.add_new_tag(&sample_tag_data()[0]).unwrap();
        let tag_result1 = db.add_new_tag(&sample_tag_data()[1]).unwrap();

        let mut task_data0 = sample_task_data()[0].clone();
        task_data0.tag = Some(tag_result0.id);
        let result0 = db.add_new_task(&task_data0)
            .expect("Adding task should not fail");
        // note: sqlite first id is 1, not 0
        assert_eq!(result0.id, 1);
        // generated timestamp is within 1 second of now
        assert!(result0.create_time.0.timestamp().abs_diff(Utc::now().timestamp()) < 2);
        assert!(result0.last_edit_time.0.timestamp().abs_diff(Utc::now().timestamp()) < 2);
        assert!(result0.done_time.is_none());

        let mut task_data1 = sample_task_data()[1].clone();
        task_data1.tag = Some(tag_result1.id);
        let result1 = db.add_new_task(&task_data1)
            .expect("Adding task should not fail");
        assert_eq!(result1.id, 2);
        assert!(result1.create_time.0.timestamp().abs_diff(Utc::now().timestamp()) < 2);
        assert!(result1.last_edit_time.0.timestamp().abs_diff(Utc::now().timestamp()) < 2);
        assert!(result1.done_time.is_none());

        let mut all_tasks = db.all_tasks().unwrap();
        // sort just in case order isn't consistent
        all_tasks.sort_by_key(|task| task.id);

        assert_eq!(db.all_tasks().unwrap(), vec![
            Task::from_parts(&task_data0, &result0),
            Task::from_parts(&task_data1, &result1),
        ]);
    });
}

#[test]
fn db_task_add_new_failure_no_tag() {
    run_db_test(|mut db| {
        let mut task_data_0 = sample_task_data()[0].clone();
        task_data_0.tag = Some(1);

        assert_eq!(db.add_new_task(&task_data_0),
            Err(DbError::TagDoesNotExistError {id: 1})
        );
    });
}

#[test]
fn db_task_get_by_id_success() {
    run_db_test(|mut db| {
        let tag_result0 = db.add_new_tag(&sample_tag_data()[0]).unwrap();
        let mut task_data0 = sample_task_data()[0].clone();
        task_data0.tag = Some(tag_result0.id);

        let tag_result1 = db.add_new_tag(&sample_tag_data()[1]).unwrap();
        let mut task_data1 = sample_task_data()[1].clone();
        task_data1.tag = Some(tag_result1.id);

        db.add_new_task(&task_data0).unwrap();
        let result1 = db.add_new_task(&task_data1).unwrap();
        assert_eq!(db.task_by_id(result1.id).expect("Task by id should not fail"),
                   Some(Task::from_parts(&task_data1, &result1)),
        );
    });
}

#[test]
fn db_task_get_by_id_failure() {
    run_db_test(|mut db| {
        let tag_result0 = db.add_new_tag(&sample_tag_data()[0]).unwrap();
        let mut task_data0 = sample_task_data()[0].clone();
        task_data0.tag = Some(tag_result0.id);

        let tag_result1 = db.add_new_tag(&sample_tag_data()[1]).unwrap();
        let mut task_data1 = sample_task_data()[1].clone();
        task_data1.tag = Some(tag_result1.id);

        db.add_new_task(&task_data0).unwrap();
        db.add_new_task(&task_data1).unwrap();
        assert_eq!(db.task_by_id(0).expect("Task by id should not fail"), None);
    });
}

#[test]
fn db_modify_task_success() {
    run_db_test(|mut db| {
        let tag_result0 = db.add_new_tag(&sample_tag_data()[0]).unwrap();
        let tag_result1 = db.add_new_tag(&sample_tag_data()[1]).unwrap();
        
        let mut task_data0 = sample_task_data()[0].clone();
        task_data0.tag = Some(tag_result0.id);
        let result0 = db.add_new_task(&task_data0).unwrap();

        let mut task_data1 = sample_task_data()[1].clone();
        task_data1.tag = Some(tag_result1.id);
        let result1 = db.add_new_task(&task_data1).unwrap();
        let task1 = Task::from_parts(&task_data1, &result1);

        let modify_result = db.modify_task(result0.id, &task_data1)
            .expect("Modify task should not fail");
        assert!(modify_result.last_edit_time.0.timestamp().abs_diff(Utc::now().timestamp()) < 2);

        assert_eq!(db.all_tasks().unwrap(),
            vec![
                Task::from_parts(&task_data1, &result0),
                task1,
            ]
        );
    });
}

#[test]
fn db_modify_task_success_remove_tag() {
    run_db_test(|mut db| {
        let tag_result0 = db.add_new_tag(&sample_tag_data()[0]).unwrap();
        let mut new_modify = sample_task_data()[0].clone();
        new_modify.tag = Some(tag_result0.id);
        let result0 = db.add_new_task(&new_modify).unwrap();
        new_modify.tag = None;
        db.modify_task(result0.id, &new_modify).unwrap();

        assert_eq!(db.task_by_id(result0.id).unwrap().unwrap(),
                   Task::from_parts(&new_modify, &result0)
        );
    });
}

#[test]
fn db_modify_task_success_add_tag() {
    run_db_test(|mut db| {
        let tag_id_0 = db.add_new_tag(&sample_tag_data()[0]).unwrap().id;

        let mut new_modify = sample_task_data()[0].clone();
        new_modify.tag = None;
        let result0 = db.add_new_task(&new_modify).unwrap();
        new_modify.tag = Some(tag_id_0);
        db.modify_task(result0.id, &new_modify).unwrap();

        assert_eq!(db.task_by_id(result0.id).unwrap().unwrap(),
                   Task::from_parts(&new_modify, &result0)
        );
    });
}

#[test]
fn db_modify_task_success_replace_tag() {
    run_db_test(|mut db| {
        let tag_id_0 = db.add_new_tag(&sample_tag_data()[0]).unwrap().id;
        let tag_id_1 = db.add_new_tag(&sample_tag_data()[1]).unwrap().id;

        let mut new_modify = sample_task_data()[0].clone();
        new_modify.tag = Some(tag_id_0);
        let result0 = db.add_new_task(&new_modify).unwrap();
        new_modify.tag = Some(tag_id_1);
        db.modify_task(result0.id, &new_modify).unwrap();

        assert_eq!(db.task_by_id(result0.id).unwrap().unwrap(),
                   Task::from_parts(&new_modify, &result0)
        );
    });
}

#[test]
fn db_modify_task_success_keep_tag() {
    run_db_test(|mut db| {
        let tag_id_0 = db.add_new_tag(&sample_tag_data()[0]).unwrap().id;

        let mut new_modify = sample_task_data()[0].clone();
        new_modify.tag = Some(tag_id_0);
        let result0 = db.add_new_task(&new_modify).unwrap();
        new_modify.title = new_modify.title + " more!";
        db.modify_task(result0.id, &new_modify).unwrap();

        assert_eq!(db.task_by_id(result0.id).unwrap().unwrap(),
                   Task::from_parts(&new_modify, &result0)
        );
    });
}

#[test]
fn db_modify_task_failure_no_task() {
    run_db_test(|mut db| {
        let tag_result0 = db.add_new_tag(&sample_tag_data()[0]).unwrap();
        let mut task_data0 = sample_task_data()[0].clone();
        task_data0.tag = Some(tag_result0.id);

        let tag_result1 = db.add_new_tag(&sample_tag_data()[1]).unwrap();
        let mut task_data1 = sample_task_data()[1].clone();
        task_data1.tag = Some(tag_result1.id);

        let result0 = db.add_new_task(&task_data0).unwrap();
        let result1 = db.add_new_task(&task_data1).unwrap();
        assert_eq!(db.modify_task(0, &sample_task_data()[1]),
                   Err(TaskDoesNotExistError { id: 0 }));
        let mut all_tasks = db.all_tasks().unwrap();
        all_tasks.sort_by_key(|task| task.id);
        let mut expected_all_tasks = vec![
            Task::from_parts(&task_data0, &result0),
            Task::from_parts(&task_data1, &result1),
        ];
        expected_all_tasks.sort_by_key(|task| task.id);
        assert_eq!(all_tasks, expected_all_tasks, "should not modify on error");
    });
}

#[test]
fn db_modify_task_failure_no_tag() {
    run_db_test(|mut db| {
        let tag_result0 = db.add_new_tag(&sample_tag_data()[0]).unwrap();
        let mut task_data0 = sample_task_data()[0].clone();
        task_data0.tag = Some(tag_result0.id);

        let result0 = db.add_new_task(&task_data0).unwrap();

        let mut no_tag = sample_task_data()[0].clone();
        no_tag.tag = Some(0);

        assert_eq!(db.modify_task(result0.id, &no_tag),
            Err(DbError::TagDoesNotExistError {id: 0})
        );
        let mut all_tasks = db.all_tasks().unwrap();
        all_tasks.sort_by_key(|task| task.id);
        let mut expected_all_tasks = vec![
            Task::from_parts(&task_data0, &result0),
        ];
        expected_all_tasks.sort_by_key(|task| task.id);
        assert_eq!(all_tasks, expected_all_tasks, "should not modify on error");
    });
}

#[test]
fn db_delete_task_success() {
    run_db_test(|mut db| {
        let tag_result0 = db.add_new_tag(&sample_tag_data()[0]).unwrap();
        let mut task_data0 = sample_task_data()[0].clone();
        task_data0.tag = Some(tag_result0.id);
        let result0 = db.add_new_task(&task_data0).unwrap();

        let tag_result1 = db.add_new_tag(&sample_tag_data()[1]).unwrap();
        let mut task_data1 = sample_task_data()[1].clone();
        task_data1.tag = Some(tag_result1.id);
        let result1 = db.add_new_task(&task_data1).unwrap();
        db.delete_task(result0.id).expect("Delete task should not fail");

        assert_eq!(db.all_tasks().unwrap(), vec![
            Task::from_parts(&task_data1, &result1)
        ]);
    });
}

#[test]
fn db_delete_task_failure() {
    run_db_test(|mut db| {
        assert_eq!(db.delete_task(0), Err(TaskDoesNotExistError { id: 0 }));
    });
}

#[test]
fn db_task_finish_success() {
    run_db_test(|mut db| {
        let tag_result0 = db.add_new_tag(&sample_tag_data()[0]).unwrap();
        let mut task_data0 = sample_task_data()[0].clone();
        task_data0.tag = Some(tag_result0.id);
        let result0 = db.add_new_task(&task_data0).unwrap();

        let finish_data = db.finish_task(result0.id).expect("finish task should not fail");
        let finish_time = finish_data.done_time.as_ref().expect("Finish data should be some,");
        assert!(finish_time.0.timestamp().abs_diff(Utc::now().timestamp()) < 2);

        let mut new_task = Task::from_parts(&sample_task_data()[0], &result0);
        new_task.done_time = finish_data.done_time;
        assert_eq!(db.task_by_id(result0.id).unwrap().unwrap(),
            new_task
        );
    });
}

#[test]
fn db_task_finish_failure() {
    run_db_test(|mut db| {
        let tag_result0 = db.add_new_tag(&sample_tag_data()[0]).unwrap();
        let mut task_data0 = sample_task_data()[0].clone();
        task_data0.tag = Some(tag_result0.id);
        let result0 = db.add_new_task(&task_data0).unwrap();

        db.finish_task(result0.id).unwrap();
        db.finish_task(result0.id).unwrap();
        assert_eq!(db.unfinish_task(result0.id),
                   Err(TaskStatusError { id: result0.id, actual_status: true })
        )
    });
}

#[test]
fn db_task_unfinish_success() {
    run_db_test(|mut db| {
        let tag_result0 = db.add_new_tag(&sample_tag_data()[0]).unwrap();
        let mut task_data0 = sample_task_data()[0].clone();
        task_data0.tag = Some(tag_result0.id);
        let result0 = db.add_new_task(&task_data0).unwrap();

        db.finish_task(result0.id).unwrap();
        let finish_data = db.unfinish_task(result0.id).expect("Unfinish task should not fail");
        assert!(finish_data.done_time.is_none());

        let mut new_task = Task::from_parts(&sample_task_data()[0], &result0);
        new_task.done_time = None;
        assert_eq!(db.task_by_id(result0.id).unwrap().unwrap(),
                   new_task
        );
    });
}

#[test]
fn db_task_unfinish_failure() {
    run_db_test(|mut db| {
        let tag_result0 = db.add_new_tag(&sample_tag_data()[0]).unwrap();
        let mut task_data0 = sample_task_data()[0].clone();
        task_data0.tag = Some(tag_result0.id);
        let result0 = db.add_new_task(&task_data0).unwrap();

        assert_eq!(db.unfinish_task(result0.id),
            Err(TaskStatusError { id: result0.id, actual_status: false })
        )
    });
}
