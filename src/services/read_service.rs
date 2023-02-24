use crate::{infra::db::Db, utils::app_results::AppResult};

pub struct ReadService<'a, TDb: Db> {
    db: &'a TDb,
}

impl<'a, TDb: Db> ReadService<'a, TDb> {
    pub fn new(db: &'a TDb) -> Self {
        ReadService { db }
    }

    pub fn read<T>(&self, query: String) -> AppResult<T> {
        self.db.send_query(query)
    }
}
