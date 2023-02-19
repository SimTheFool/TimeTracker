use infra::db_infra::Db;

mod domain;

pub mod commands;
pub mod infra;
pub mod services;
pub mod utils;

pub fn get_time_tracker<'a, TDb: Db>(db: &TDb) {
    let (read_service, write_service) = services::get_services(db);
}
