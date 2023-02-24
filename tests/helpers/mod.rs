use time_tracker::commands::Commands;
use time_tracker::queries::Queries;

use self::test_infra::TestDb;

pub mod test_infra;

pub fn get_time_tracker() -> (Commands, Queries) {
    let testDb = TestDb::new().unwrap();
    time_tracker::get_time_tracker(&testDb)
}
