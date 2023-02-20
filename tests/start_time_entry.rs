use time_tracker::{infra::db::Db, utils::app_results::AppResult};

struct TestDb {}

impl Db for TestDb {
    fn send_query(&self, query: String) -> AppResult {
        AppResult::Unit(Ok(()))
    }
}

#[test]
fn it_should_create_time_entry() {
    let commands = time_tracker::get_time_tracker(&TestDb {});
    let entry = (commands.start_time_entry)("Testa".to_string());
    assert_eq!(entry.name, "Testa");
}
