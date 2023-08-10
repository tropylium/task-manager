use app::*;
mod util;
use util::*;

#[test]
fn create_database() {
    run_db_test(|| {
        let db_result = Db::new(TEST_PATH);
        assert!(db_result.is_ok())
    });
}