use app::*;
use crate::DbError::TagDoesNotExistError;
mod util;
use util::*;

fn tag_data_1() -> TagData {
    TagData {
        name: String::from("new_tag"),
        color: HslColor {
            hue: 50,
            saturation: 89,
            lightness: 73,
        },
        active: true,
    }
}
fn tag_data_2() -> TagData {
    TagData {
        name: String::from("whee!"),
        color: HslColor {
            hue: 360,
            saturation: 100,
            lightness: 0,
        },
        active: false,
    }
}

#[test]
fn db_empty() {
    run_db_test(|| {
        let db = Db::new(TEST_PATH).unwrap();
        assert_eq!(db.all_tags().expect("Get all tags should not fail"), vec![]);
    });
}

#[test]
fn db_add_new_tag() {
    run_db_test(|| {
        let mut db = Db::new(TEST_PATH).unwrap();
        // note: sqlite first id is 1, not 0
        assert!(db.add_new_tag(&tag_data_1()).is_ok_and(|tag| tag == 1));
        assert!(db.add_new_tag(&tag_data_2()).is_ok_and(|tag| tag == 2));
        let mut all_tags = db.all_tags().unwrap();

        // sort just in case order isn't consistent
        all_tags.sort_by_key(|tag| tag.id);
        assert_eq!(db.all_tags().unwrap(), vec![
            Tag {
                id: 1,
                data: tag_data_1(),
            },
            Tag {
                id: 2,
                data: tag_data_2(),
            }
        ]);
    });
}

#[test]
fn db_get_by_id_success() {
    run_db_test(|| {
        let mut db = Db::new(TEST_PATH).unwrap();
        db.add_new_tag(&tag_data_1()).unwrap();
        let id2 = db.add_new_tag(&tag_data_2()).unwrap();
        assert_eq!(db.tag_by_id(id2).expect("Tag by id should not fail"), Tag {
            id: id2,
            data: tag_data_2()
        });
    });
}

#[test]
fn db_get_by_id_failure() {
    run_db_test(|| {
        let mut db = Db::new(TEST_PATH).unwrap();
        db.add_new_tag(&tag_data_1()).unwrap();
        db.add_new_tag(&tag_data_2()).unwrap();
        assert_eq!(db.tag_by_id(0), Err(TagDoesNotExistError { id: 0 }));
    });
}

#[test]
fn db_modify_tag_success() {
    run_db_test(|| {
        let mut db = Db::new(TEST_PATH).unwrap();
        let id1 = db.add_new_tag(&tag_data_1()).unwrap();
        let new_tag = Tag {
            id: id1,
            data: tag_data_2()
        };
        db.modify_tag(&new_tag).expect("Modify tag should not fail");
        assert_eq!(db.tag_by_id(id1).unwrap(), new_tag);
    });
}

#[test]
fn db_modify_tag_failure() {
    run_db_test(|| {
        let mut db = Db::new(TEST_PATH).unwrap();
        db.add_new_tag(&tag_data_1()).unwrap();
        db.add_new_tag(&tag_data_2()).unwrap();
        let new_tag = Tag {
            id: 0,
            data: tag_data_2()
        };
        assert_eq!(db.modify_tag(&new_tag), Err(TagDoesNotExistError { id: 0 }));
    });
}

#[test]
fn db_delete_tag_success() {
    run_db_test(|| {
        let mut db = Db::new(TEST_PATH).unwrap();
        let id1 = db.add_new_tag(&tag_data_1()).unwrap();
        let id2 = db.add_new_tag(&tag_data_2()).unwrap();
        db.delete_tag(id1).expect("Delete tag should not fail");
        assert_eq!(db.all_tags().unwrap(), vec![Tag {
            id: id2,
            data: tag_data_2(),
        }]);
    });
}

#[test]
fn db_delete_tag_failure() {
    run_db_test(|| {
        let mut db = Db::new(TEST_PATH).unwrap();
        assert_eq!(db.delete_tag(0), Err(TagDoesNotExistError { id: 0 }));
    });
}