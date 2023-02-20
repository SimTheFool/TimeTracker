use commands::Commands;
use infra::db::Db;
use services::{read_service::ReadService, write_service::WriteService};

mod domain;

pub mod commands;
pub mod infra;
pub mod services;
pub mod utils;

pub fn get_time_tracker<'a, TDb: Db>(db: &TDb) -> Commands {
    // Services
    let write_service = WriteService::new(db);
    /* let read_service = ReadService::new(db); */

    // Commands
    let start_time_entry = commands::start_time_entry::get_command(write_service);

    let commands = Commands { start_time_entry };

    // Queries

    // Return queries and commands
    commands
}
