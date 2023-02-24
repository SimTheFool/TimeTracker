use crate::{domain::time_entry::TimeEntry, infra::db::Db, services::read_service::ReadService};

pub type GetDailyTimeEntriesQuery = fn() -> Vec<TimeEntry>;

pub fn get_query<TDb: Db>(readService: &ReadService<TDb>) -> GetDailyTimeEntriesQuery {
    return |_| readService.read::<Vec<TimeEntry>>("SELECT * FROM time_entries".to_string());
}
