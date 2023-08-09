use std::path::Path;
use rusqlite::Connection;
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
    const TAG_TABLE: &'static str = "tags";

    /// Creates a database instance from either an empty/ nonexistent file
    /// or an existing database.
    pub fn new<'a, P: AsRef<Path>>(database_file: P) -> DbResult<'a, Self> {
        let connection = Connection::open(database_file)?;
        connection.execute(&format!(r#"
            create table if not exists {} (
                "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                "name" TEXT NOT NULL,
                "color" INTEGER NOT NULL,
                "active" INTEGER NOT NULL
            );
        "#, {Db::TAG_TABLE}), ())?;
        Ok(Self {
            conn: connection
        })
    }

    /// Returns all tags stored in this database in some order.
    pub fn all_tags(&self) -> DbResult<Vec<Tag>> {
        let mut stmt = self.conn.prepare(
            &format!("SELECT * FROM {}", Db::TAG_TABLE)
        ).unwrap();
        let iter = stmt.query_map([],
                                  |row| -> rusqlite::Result<Tag>{
            Ok(Tag {
                id: row.get("id")?,
                data: TagData {
                    name: row.get("name")?,
                    color: row.get("color")?,
                    active: row.get("active")?,
                },
            })
        }).unwrap();
        Ok(iter.map(|value| value.unwrap()).collect())
    }

    /// Add a new tag to the database, returning the unique id
    /// that was assigned to that tag.
    pub fn add_new_tag(&mut self, data: &TagData) -> DbResult<TagId> {
        let tx = self.conn.transaction()?;
        tx.execute(&format!("INSERT INTO {} (name, color, active) values (?1, ?2, ?3)", {Db::TAG_TABLE}),
                   (&data.name, &data.color, data.active)
        )?;
        let new_id = tx.last_insert_rowid();
        tx.commit()?;
        Ok(new_id)
    }

    /// Modifies an existing tag in the database. Returns `TagDoesNotExistError` if
    /// the tag id being modified doesn't exist in the database.
    pub fn modify_tag(&mut self, tag: &Tag) -> DbResult<()> {
        todo!()
    }

    /// Delete a tag by its id in the database.
    pub fn delete_tag(&mut self, id: TagId) -> DbResult<()> {
        todo!()
    }
}