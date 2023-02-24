use self::get_daily_time_entries::GetDailyTimeEntriesQuery;

pub mod get_daily_time_entries;

pub struct Queries {
    pub get_daily_time_entries: GetDailyTimeEntriesQuery,
}
