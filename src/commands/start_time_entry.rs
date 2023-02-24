use crate::{domain::time_entry::TimeEntry, infra::db::Db, services::write_service::WriteService};
use chrono::Local;

pub type StartTimeEntryCommand = fn(String) -> ();

pub fn get_command<TDb: Db>(writeService: &WriteService<TDb>) -> StartTimeEntryCommand {
    let command = |name| {
        let time_entry = TimeEntry::new(name, Local::now(), None);
        writeService.write(time_entry);
    };

    return command;
}
