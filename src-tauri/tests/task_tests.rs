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
fn db_task_add_new() {
    run_db_test(|mut db| {
        let result0 = db.add_new_task(&sample_task_data()[0])
            .expect("Adding task should not fail");
        // note: sqlite first id is 1, not 0
        assert_eq!(result0.id, 1);
        // generated timestamp is within 1 second of now
        assert!(result0.create_time.0.timestamp().abs_diff(Utc::now().timestamp()) < 2);
        assert!(result0.last_edit_time.0.timestamp().abs_diff(Utc::now().timestamp()) < 2);
        assert!(result0.done_time.is_none());

        let result1 = db.add_new_task(&sample_task_data()[1])
            .expect("Adding task should not fail");
        assert_eq!(result1.id, 2);
        assert!(result1.create_time.0.timestamp().abs_diff(Utc::now().timestamp()) < 2);
        assert!(result1.last_edit_time.0.timestamp().abs_diff(Utc::now().timestamp()) < 2);
        assert!(result1.done_time.is_none());

        let mut all_tasks = db.all_tasks().unwrap();
        // sort just in case order isn't consistent
        all_tasks.sort_by_key(|task| task.id);

        assert_eq!(db.all_tasks().unwrap(), vec![
            Task::from_parts(&sample_task_data()[0], &result0),
            Task::from_parts(&sample_task_data()[1], &result1),
        ]);
    });
}

#[test]
fn db_task_get_by_id_success() {
    run_db_test(|mut db| {
        db.add_new_task(&sample_task_data()[0]).unwrap();
        let result1 = db.add_new_task(&sample_task_data()[1]).unwrap();
        assert_eq!(db.task_by_id(result1.id).expect("Task by id should not fail"),
                   Task::from_parts(&sample_task_data()[1], &result1),
        );
    });
}

#[test]
fn db_task_get_by_id_failure() {
    run_db_test(|mut db| {
        db.add_new_task(&sample_task_data()[0]).unwrap();
        db.add_new_task(&sample_task_data()[1]).unwrap();
        assert_eq!(db.task_by_id(0), Err(TaskDoesNotExistError { id: 0 }));
    });
}

#[test]
fn db_modify_task_success() {
    run_db_test(|mut db| {
        let result0 = db.add_new_task(&sample_task_data()[0]).unwrap();
        let modify_result = db.modify_task(result0.id, &sample_task_data()[1])
            .expect("Modify task should not fail");
        assert!(modify_result.last_edit_time.0.timestamp().abs_diff(Utc::now().timestamp()) < 2);

        assert_eq!(db.task_by_id(result0.id).unwrap(),
                   Task::from_parts(&sample_task_data()[1], &result0)
        );
    });
}

#[test]
fn db_modify_task_failure() {
    run_db_test(|mut db| {
        db.add_new_task(&sample_task_data()[0]).unwrap();
        db.add_new_task(&sample_task_data()[1]).unwrap();
        assert_eq!(db.modify_task(0, &sample_task_data()[1]),
                   Err(TaskDoesNotExistError { id: 0 }));
    });
}

#[test]
fn db_delete_task_success() {
    run_db_test(|mut db| {
        let result0 = db.add_new_task(&sample_task_data()[0]).unwrap();
        let result1 = db.add_new_task(&sample_task_data()[1]).unwrap();
        db.delete_task(result0.id).expect("Delete task should not fail");
        assert_eq!(db.all_tasks().unwrap(), vec![
            Task::from_parts(&sample_task_data()[1], &result1)
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
        let result0 = db.add_new_task(&sample_task_data()[0]).unwrap();
        let finish_data = db.finish_task(result0.id).expect("finish task should not fail");
        let finish_time = finish_data.done_time.as_ref().expect("Finish data should be some,");
        assert!(finish_time.0.timestamp().abs_diff(Utc::now().timestamp()) < 2);
        let mut new_task = Task::from_parts(&sample_task_data()[0], &result0);
        new_task.done_time = finish_data.done_time;
        assert_eq!(db.task_by_id(result0.id).unwrap(),
            new_task
        );
    });
}

#[test]
fn db_task_finish_failure() {
    run_db_test(|mut db| {
        let result0 = db.add_new_task(&sample_task_data()[0]).unwrap();
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
        let result0 = db.add_new_task(&sample_task_data()[0]).unwrap();
        db.finish_task(result0.id).unwrap();
        let finish_data = db.unfinish_task(result0.id).expect("Unfinish task should not fail");
        assert!(finish_data.done_time.is_none());

        let mut new_task = Task::from_parts(&sample_task_data()[0], &result0);
        new_task.done_time = None;
        assert_eq!(db.task_by_id(result0.id).unwrap(),
                   new_task
        );
    });
}

#[test]
fn db_task_unfinish_failure() {
    run_db_test(|mut db| {
        let result0 = db.add_new_task(&sample_task_data()[0]).unwrap();
        assert_eq!(db.unfinish_task(result0.id),
            Err(TaskStatusError { id: result0.id, actual_status: false })
        )
    });
}
