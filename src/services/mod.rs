use self::{read_service::ReadService, write_service::WriteService};
use crate::infra::db_infra::Db;

pub mod read_service;
pub mod write_service;

pub fn get_services<TDb: Db>(db: &TDb) -> (ReadService<TDb>, WriteService<TDb>) {
    let read_service = ReadService::new(db);
    let write_service = WriteService::new(db);
    (read_service, write_service)
}
