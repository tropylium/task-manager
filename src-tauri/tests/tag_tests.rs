#[allow(dead_code, unused_mut)]
use chrono::Utc;
use app::*;
use DbError::TagDoesNotExistError;
mod util;
use util::*;

#[test]
fn db_tag_empty() {
    run_db_test(|db| {
        assert_eq!(db.all_tags().expect("Get all tags should not fail"), vec![]);
    });
}

#[test]
fn db_tag_add_new() {
    run_db_test(|mut db| {
        let result0 = db.add_new_tag(&sample_tag_data()[0])
            .expect("Adding tag should not fail");
        // note: sqlite first id is 1, not 0
        assert_eq!(result0.id, 1);
        // generated timestamp is within 1 second of now
        assert!(result0.create_time.0.timestamp().abs_diff(Utc::now().timestamp()) < 2);

        let result1 = db.add_new_tag(&sample_tag_data()[1])
            .expect("Adding tag should not fail");
        assert_eq!(result1.id, 2);
        assert!(result1.create_time.0.timestamp().abs_diff(Utc::now().timestamp()) < 2);

        let mut all_tags = db.all_tags().unwrap();
        // sort just in case order isn't consistent
        all_tags.sort_by_key(|tag| tag.id);

        assert_eq!(db.all_tags().unwrap(), vec![
            Tag::from_parts(&sample_tag_data()[0], &result0),
            Tag::from_parts(&sample_tag_data()[1], &result1),
        ]);
    });
}

#[test]
fn db_tag_get_by_id_success() {
    run_db_test(|mut db| {
        db.add_new_tag(&sample_tag_data()[0]).unwrap();
        let result1 = db.add_new_tag(&sample_tag_data()[1]).unwrap();
        assert_eq!(db.tag_by_id(result1.id).expect("Tag by id should not fail"),
                   Some(Tag::from_parts(&sample_tag_data()[1], &result1)),
        );
    });
}

#[test]
fn db_tag_get_by_id_failure() {
    run_db_test(|mut db| {
        db.add_new_tag(&sample_tag_data()[0]).unwrap();
        db.add_new_tag(&sample_tag_data()[1]).unwrap();
        assert_eq!(db.tag_by_id(0), Ok(None));
    });
}

#[test]
fn db_modify_tag_success() {
    run_db_test(|mut db| {
        let result0 = db.add_new_tag(&sample_tag_data()[0]).unwrap();
        db.modify_tag(result0.id, &sample_tag_data()[1])
            .expect("Modify tag should not fail");
        assert_eq!(db.tag_by_id(result0.id).unwrap().unwrap(),
            Tag::from_parts(&sample_tag_data()[1], &result0)
        );
    });
}

#[test]
fn db_modify_tag_failure() {
    run_db_test(|mut db| {
        db.add_new_tag(&sample_tag_data()[0]).unwrap();
        db.add_new_tag(&sample_tag_data()[1]).unwrap();
        assert_eq!(db.modify_tag(0, &sample_tag_data()[1]),
                   Err(TagDoesNotExistError { id: 0 }));
    });
}

#[test]
fn db_delete_tag_success() {
    run_db_test(|mut db| {
        let result0 = db.add_new_tag(&sample_tag_data()[0]).unwrap();
        let result1 = db.add_new_tag(&sample_tag_data()[1]).unwrap();
        db.delete_tag(result0.id).expect("Delete tag should not fail");
        assert_eq!(db.all_tags().unwrap(), vec![
            Tag::from_parts(&sample_tag_data()[1], &result1)
        ]);
    });
}

#[test]
fn db_delete_tag_success_remove_from_task() {
    run_db_test(|mut db| {
        let tag_result0 = db.add_new_tag(&sample_tag_data()[0]).unwrap();
        let mut task_data0 = sample_task_data()[0].clone();
        task_data0.tag = Some(tag_result0.id);
        let task_data = db.add_new_task(&task_data0).unwrap();

        db.delete_tag(tag_result0.id).expect("Delete tag should not fail");
        task_data0.tag = None;
        assert_eq!(db.all_tasks().unwrap(), vec![
            Task::from_parts(&task_data0, &task_data)
        ]);
    });
}

#[test]
fn db_delete_tag_failure() {
    run_db_test(|mut db| {
        assert_eq!(db.delete_tag(0), Err(TagDoesNotExistError { id: 0 }));
    });
}