use crate::{domain::time_entry::TimeEntry, infra::db::Db, services::write_service::WriteService};
use chrono::Local;

pub type StartTimeEntryCommand = fn(String) -> TimeEntry;

pub fn get_command<TDb: Db>(writeService: WriteService<TDb>) -> StartTimeEntryCommand {
    return |name| TimeEntry::new(name, Local::now(), None);
}
