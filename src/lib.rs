use commands::Commands;
use infra::db::Db;
use queries::Queries;
use services::{read_service::ReadService, write_service::WriteService};

pub mod commands;
pub mod domain;
pub mod infra;
pub mod queries;
pub mod services;
pub mod utils;

pub fn get_time_tracker<'a, TDb: Db>(db: &TDb) -> (Commands, Queries) {
    // Services
    let write_service = WriteService::new(db);
    let read_service = ReadService::new(db);

    // Commands
    let start_time_entry = commands::start_time_entry::get_command(&write_service);

    let commands = Commands { start_time_entry };

    // Queries
    let get_daily_time_entries = queries::get_daily_time_entries::get_query(&read_service);

    let queries = Queries {
        get_daily_time_entries,
    };

    //Initialization

    // Return queries and commands
    (commands, queries)
}
