mod util;
use util::*;

#[test]
fn create_database() {
    run_db_test(|_| {
        // Nothing, `run_db_test` creates a database and unwraps
    });
}