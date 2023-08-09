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
    use super::*;

    const TEST_PATH: &str = "test-outputs/test-db.sqlite";

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
        let executor = match TESTER.lock() {
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
            let data1 = TagData {
                name: String::from("new_tag"),
                color: HslColor {
                    hue: 50,
                    saturation: 89,
                    lightness: 73,
                },
                active: true,
            };
            let data2 = TagData {
                name: String::from("whee!"),
                color: HslColor {
                    hue: 360,
                    saturation: 100,
                    lightness: 0,
                },
                active: false,
            };
            // note: sqlite first id is 1, not 0
            assert!(db.add_new_tag(&data1).is_ok_and(|tag| tag == 1));
            assert!(db.add_new_tag(&data2).is_ok_and(|tag| tag == 2));
            let mut all_tags = db.all_tags().unwrap();

            // sort just in case order isn't consistent
            all_tags.sort_by_key(|tag| tag.id);
            assert_eq!(db.all_tags().unwrap(), vec![
                Tag {
                    id: 1,
                    data: data1,
                },
                Tag {
                    id: 2,
                    data: data2,
                }
            ]);
        });
    }
}