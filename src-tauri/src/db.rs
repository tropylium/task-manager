use std::path::Path;
use rusqlite::{Connection, Row};
use rusqlite::Error::QueryReturnedNoRows;
use crate::{EditableTaskData, Tag, TagData, TagId, Task, TaskId};

#[derive(Debug, PartialEq)]
pub enum DbError {
    RusqliteError { error: rusqlite::Error },
    TagDoesNotExistError { id: TagId },
}

impl<'a> From<rusqlite::Error> for DbError {
    fn from(value: rusqlite::Error) -> Self {
        DbError::RusqliteError { error: value }
    }
}

type DbResult<T> = Result<T, DbError>;

pub struct Db {
    conn: Connection, // note connection implements Drop
}

impl Db {
    const TAG_TABLE: &'static str = "tags";

    /// Creates a database instance from either an empty/ nonexistent file
    /// or an existing database.
    pub fn new<P: AsRef<Path>>(database_file: P) -> DbResult<Self> {
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

    /// Retrieves all tags stored in this database in some order.
    pub fn all_tags(&self) -> DbResult<Vec<Tag>> {
        let mut stmt = self.conn.prepare(
            &format!("SELECT * FROM {}", Db::TAG_TABLE)
        ).unwrap();
        let iter = stmt.query_map([], Db::tag_from_row).unwrap();
        Ok(iter.map(|value| value.unwrap()).collect())
    }

    /// Add a new tag to the database, returning the unique id
    /// that was assigned to that tag.
    pub fn add_new_tag(&mut self, data: &TagData) -> DbResult<TagId> {
        let tx = self.conn.transaction()?;
        tx.execute(&format!(
            "INSERT INTO {} (name, color, active) values (?1, ?2, ?3)", Db::TAG_TABLE
        ), (&data.name, &data.color, data.active))?;
        let new_id = tx.last_insert_rowid();
        tx.commit()?;
        Ok(new_id)
    }

    /// Retrieve the tag with this id. Returns `TagDoesNotExistError` if
    /// the tag id doesn't exist in the database.
    pub fn tag_by_id(&self, id: TagId) -> DbResult<Tag> {
        let mut stmt = self.conn.prepare(&format!(
            "SELECT * FROM {} WHERE id = ?1", Db::TAG_TABLE
        ))?;
        stmt.query_row((id,), Db::tag_from_row ).map_err(|err| -> DbError {
            match err {
                QueryReturnedNoRows => DbError::TagDoesNotExistError {id},
                other => DbError::from(other),
            }
        })
    }

    /// Modifies an existing tag in the database. Returns `TagDoesNotExistError` if
    /// the tag id being modified doesn't exist in the database.
    pub fn modify_tag(&mut self, tag: &Tag) -> DbResult<()> {
        let tx = self.conn.transaction()?;
        let rows = tx.execute(&format!(r#"
                UPDATE {} SET
                    name = ?2,
                    color = ?3,
                    active = ?4
                WHERE id = ?1;
            "#, { Db::TAG_TABLE }), (tag.id, &tag.data.name, &tag.data.color, tag.data.active))?;
        tx.commit()?;
        match rows {
            0 => Err(DbError::TagDoesNotExistError { id: tag.id }),
            1 => Ok(()),
            other => panic!("Modify tag changed {} rows!", other),
        }
    }

    /// Delete a tag by its id in the database. Returns `TagDoesNotExistError` if
    /// the tag id being deleted doesn't exist in the database.
    /// Otherwise, also removes the tag being deleted from any tasks that have this tag..
    pub fn delete_tag(&mut self, id: TagId) -> DbResult<()> {
        let tx = self.conn.transaction()?;
        let rows = tx.execute(&format!(
            "DELETE FROM {} WHERE id = ?1", {Db::TAG_TABLE}
        ), (id,))?;
        tx.commit()?;
        match rows {
            0 => Err(DbError::TagDoesNotExistError {id}),
            1 => Ok(()),
            other => panic!("Delete tag changed {} rows!", other),
        }
    }

    /// Retrieves all tasks stored in this database in some order.
    pub fn all_tasks(&self) -> DbResult<Vec<Task>> {
        todo!()
    }

    pub fn add_new_task(&mut self, data: &EditableTaskData) -> DbResult<TaskId> {
        todo!()
    }

    fn tag_from_row(row: &Row) -> rusqlite::Result<Tag> {
        Ok(Tag {
            id: row.get("id")?,
            data: TagData {
                name: row.get("name")?,
                color: row.get("color")?,
                active: row.get("active")?,
            },
        })
    }
}