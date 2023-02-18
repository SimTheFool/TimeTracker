use std::error::Error;

use crate::{
    domain::{time_entry::TimeEntry, DomainObject},
    infra::db_infra::Db,
};

pub fn write_domain(db: &impl Db, domain_object: &DomainObject) -> Result<(), Box<dyn Error>> {
    match domain_object {
        DomainObject::TimeEntry(x) => db.send_query(get_time_entry_sql(x)),
    }
}

fn get_time_entry_sql(time_entry: &TimeEntry) -> String {
    "aaa".to_string()
}
