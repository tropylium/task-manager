use std::panic;
use std::sync::Mutex;
use std::fs;
use chrono::{TimeZone, Utc};
use once_cell::sync::Lazy;
use app::{Db, EditableTagData, EditableTaskData, HslColor};

pub const TEST_PATH: &str = "test-outputs/test-db.sqlite";
// We want to run each test synchronously because they modify the same file,
// and we want to guarantee consistent starting/ ending conditions for each test.
// Reference:
// https://users.rust-lang.org/t/passing-test-threads-1-to-cargo-test-by-default/87225/4
// https://stackoverflow.com/questions/58006033/how-to-run-setup-code-before-any-tests-run-in-rust
#[derive(Copy, Clone)]
struct DbExecutor;
impl DbExecutor {
    fn run_db_test(&self, f: impl FnOnce(Db) + panic::UnwindSafe) {
        // delete any database files if exists
        _ = fs::remove_file(TEST_PATH);
        let db = Db::connect(TEST_PATH).unwrap();
        f(db);
        // cleanup
        _ = fs::remove_file(TEST_PATH);
    }
}

static TESTER: Mutex<DbExecutor> = Mutex::new(DbExecutor);
pub fn run_db_test(f: impl FnOnce(Db) + panic::UnwindSafe) {
    // there is nothing to poison if a test fails; we want to run the rest anyway
    match TESTER.lock() {
        Ok(guard) => guard,
        Err(poison) => poison.into_inner()
    }.run_db_test(f);
}

static SAMPLE_TAGS: Lazy<Vec<EditableTagData>> = Lazy::new(|| vec![
    EditableTagData {
        name: String::from("new_tag"),
        color: HslColor {
            hue: 50,
            saturation: 89,
            lightness: 73,
        },
        active: true,
    },
    EditableTagData {
        name: String::from("whee!"),
        color: HslColor {
            hue: 360,
            saturation: 100,
            lightness: 0,
        },
        active: false,
    },
]);
pub fn sample_tag_data() -> &'static [EditableTagData] {
    &SAMPLE_TAGS
}

static SAMPLE_TASKS: Lazy<Vec<EditableTaskData>> = Lazy::new(|| vec![
    EditableTaskData {
        title: String::from("Blueberries"),
        tag: Some(1),
        body: String::from("A type of berry"),
        difficulty: 3,
        due_time: Some(Utc.with_ymd_and_hms(2023, 9, 1, 0,0,0).unwrap()),
        target_time: Some(Utc.with_ymd_and_hms(2023, 8, 1, 0,0,0).unwrap()),
        paused: false,
    },
    EditableTaskData {
        title: String::from("Apples"),
        tag: None,
        body: String::from("Not a berry"),
        difficulty: 0,
        due_time: None,
        target_time: None,
        paused: false,
    },
]);
pub fn sample_task_data() -> &'static [EditableTaskData] {
    &SAMPLE_TASKS
}
