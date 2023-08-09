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
            println!("before");
            f();
            _ = fs::remove_file(TEST_PATH);
            println!("after");
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
            let test_data = TagData {
                name: String::from("new_tag"),
                color: Color::default(),
                active: true,
            };
            assert!(db.add_new_tag(&test_data).is_ok_and(|tag| tag == 0));
            assert_eq!(db.all_tags().unwrap(), vec![Tag {
                id: 0,
                data: test_data,
            }])
        });
    }
}