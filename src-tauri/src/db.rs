use std::path::PathBuf;
use rusqlite::{Connection, OpenFlags, Params, Row};
use crate::{Tag, TagData, TagId};
use crate::db::DbError::RusqliteError;

pub enum DbError<'a> {
    RusqliteError { error: rusqlite::Error },
    TagDoesNotExistError { tag: &'a Tag },
}

impl DbError {
    pub fn error_message(&self) -> String {
        todo!()
    }
}

impl From<rusqlite::Error> for DbError {
    fn from(value: rusqlite::Error) -> Self {
        RusqliteError { error: value }
    }
}

type DbResult<'a, T> = Result<T, DbError<'a>>;

pub struct Db {
    conn: Connection, // note connection implements Drop
}

impl Db {
    pub fn new(database_file: &PathBuf) -> Self {
        Self {
            conn: Connection::open_with_flags(database_file, OpenFlags::SQLITE_OPEN_READ_WRITE).unwrap()
        }
    }

    pub fn all_tags(&self) -> DbResult<Vec<Tag>> {
        todo!()
    }

    pub fn add_new_tag(&mut self, data: TagData) -> DbResult<TagId> {
        todo!()
    }

    pub fn modify_tag(&mut self, tag: Tag) -> DbResult<()> {
        todo!()
    }

    pub fn delete_tag(&mut self, id: TagId) -> DbResult<()> {
        todo!()
    }
}