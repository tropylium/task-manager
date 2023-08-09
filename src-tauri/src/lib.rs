mod tag;
mod task;
mod db;

pub use tag::{*};
pub use task::{*};
pub use db::{*};

#[cfg(test)]
mod tests {
    use std::panic;
    use std::sync::Mutex;
    use std::fs;
    use crate::DbError::TagDoesNotExistError;
    use super::*;

    const TEST_PATH: &str = "test-outputs/test-db.sqlite";
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

    // see
    // https://users.rust-lang.org/t/passing-test-threads-1-to-cargo-test-by-default/87225/4
    // https://stackoverflow.com/questions/58006033/how-to-run-setup-code-before-any-tests-run-in-rust
    #[derive(Copy, Clone)]
    struct DbExecutor;
    impl DbExecutor {
        fn run_db_test(&self, f: impl FnOnce() + panic::UnwindSafe) {
            // delete any database files if exists
            _ = fs::remove_file(TEST_PATH);
            f();
            // cleanup
            _ = fs::remove_file(TEST_PATH);
        }
    }

    static TESTER: Mutex<DbExecutor> = Mutex::new(DbExecutor);
    fn run_db_test(f: impl FnOnce() + panic::UnwindSafe) {
        // there is nothing to poison if a test fails; we want to run the rest anyway
        match TESTER.lock() {
            Ok(guard) => guard,
            Err(poison) => poison.into_inner()
        }.run_db_test(f);
    }

    #[test]
    fn create_database() {
        run_db_test(|| {
            let db_result = Db::new(TEST_PATH);
            assert!(db_result.is_ok())
        });
    }

    #[test]
    fn db_empty() {
        run_db_test(|| {
            let db = Db::new(TEST_PATH).unwrap();
            assert_eq!(db.all_tags().unwrap(), vec![]);
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
            assert_eq!(db.tag_by_id(id2).unwrap(), Tag {
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
            db.modify_tag(&new_tag).unwrap();
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

}