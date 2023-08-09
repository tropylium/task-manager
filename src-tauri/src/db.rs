use std::path::Path;
use rusqlite::{Connection, Params, Row};
use crate::{Tag, TagData, TagId};
use crate::db::DbError::RusqliteError;

#[derive(Debug)]
pub enum DbError<'a> {
    RusqliteError { error: rusqlite::Error },
    TagDoesNotExistError { tag: &'a Tag },
}

impl<'a> DbError<'a> {
    pub fn error_message(&self) -> String {
        todo!()
    }
}

impl<'a> From<rusqlite::Error> for DbError<'a> {
    fn from(value: rusqlite::Error) -> Self {
        RusqliteError { error: value }
    }
}

type DbResult<'a, T> = Result<T, DbError<'a>>;

pub struct Db {
    conn: Connection, // note connection implements Drop
}

impl Db {
    pub fn new<'a, P: AsRef<Path>>(database_file: P) -> DbResult<'a, Self> {
        Ok(Self {
            conn: Connection::open(database_file)?
        })
    }

    pub fn all_tags(&self) -> DbResult<Vec<Tag>> {
        todo!()
    }

    pub fn add_new_tag(&mut self, data: &TagData) -> DbResult<TagId> {
        todo!()
    }

    pub fn modify_tag(&mut self, tag: &Tag) -> DbResult<()> {
        todo!()
    }

    pub fn delete_tag(&mut self, id: TagId) -> DbResult<()> {
        todo!()
    }
}