use chrono::prelude::*;
use uuid::Uuid;

pub struct TimeEntryId(Uuid);

pub struct TimeEntry {
    pub id: TimeEntryId,
    pub start_time: DateTime<Local>,
    pub end_time: Option<DateTime<Local>>,
    pub name: String,
}

impl TimeEntry {
    pub fn new(
        name: String,
        start_time: DateTime<Local>,
        end_time: Option<DateTime<Local>>,
    ) -> TimeEntry {
        TimeEntry {
            id: TimeEntryId(Uuid::new_v4()),
            name,
            start_time,
            end_time,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TimeEntry;
    use chrono::prelude::*;

    #[test]
    fn create_time_entry() {
        let entry = TimeEntry::new("time entry A".to_string(), Local::now(), None);
        let now = Local::now();

        let created_since = {
            let delta = now - entry.start_time;
            delta.num_seconds()
        };

        assert_eq!(entry.name, "time entry A", "should have the same name");
        assert_eq!(entry.end_time, None, "should not have an end time");
        assert_eq!(created_since == 0, true, "should be created now");
    }
}
