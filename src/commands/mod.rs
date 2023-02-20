use self::start_time_entry::StartTimeEntryCommand;

pub mod start_time_entry;

pub struct Commands {
    pub start_time_entry: StartTimeEntryCommand,
}
