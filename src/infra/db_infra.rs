use crate::utils::app_results::AppResult;

pub trait Db {
    fn send_query(&self, query: String) -> AppResult;
}
