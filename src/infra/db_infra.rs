use std::error::Error;

pub trait Db {
    fn send_query(&self, query: String) -> Result<(), Box<dyn Error>>;
}
