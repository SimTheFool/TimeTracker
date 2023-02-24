use crate::{
    domain::{time_entry::TimeEntry, DomainObject},
    infra::db::Db,
    utils::app_results::AppResult,
};

pub struct WriteService<'a, TDb: Db> {
    db: &'a TDb,
}

impl<'a, TDb: Db> WriteService<'a, TDb> {
    pub fn new(db: &'a TDb) -> Self {
        WriteService { db }
    }

    pub fn write(&self, domain_object: &DomainObject) -> AppResult<()> {
        match domain_object {
            DomainObject::TimeEntry(x) => self.db.send_query("query".to_string()),
        }
    }
}

/* fn write_time_entry_sql (time_entry: &TimeEntry) {
    let sql = get_time_entry_sql(time_entry);
}; */

/* fn get_time_entry_sql(time_entry: &TimeEntry) -> String {
    "aaa".to_string()
} */
