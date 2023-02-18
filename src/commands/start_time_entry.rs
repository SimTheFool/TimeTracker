use chrono::Local;

use crate::domain::time_entry::TimeEntry;

pub fn start_time_entry(name: String) -> TimeEntry {
    TimeEntry::new(name, Local::now(), None)
}
