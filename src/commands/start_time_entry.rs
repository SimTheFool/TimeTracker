use crate::{
    domain::time_entry::TimeEntry, infra::db_infra::Db, services::write_service::WriteService,
};
use chrono::Local;

pub fn start_time_entry<TDb: Db>(writeService: WriteService<TDb>, name: String) -> TimeEntry {
    TimeEntry::new(name, Local::now(), None)
}
