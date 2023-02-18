pub mod time_entry;

pub enum DomainObject {
    TimeEntry(time_entry::TimeEntry),
}
