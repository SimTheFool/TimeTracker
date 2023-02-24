use rusqlite::{Connection, Result};
use time_tracker::{infra::db::Db, utils::app_results::AppResult};

pub struct TestDb {
    connection: Connection,
}

impl TestDb {
    pub fn new() -> Result<TestDb, rusqlite::Error> {
        Connection::open_in_memory().map(|c| TestDb { connection: c })
    }
}

impl Db for TestDb {
    fn send_query(&self, query: String) -> AppResult {
        let x = self.connection.execute(&query, ());
        AppResult::Unit(Ok(()))
    }
}
