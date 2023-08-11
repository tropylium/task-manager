use std::path::Path;
use chrono::Utc;
use rusqlite::{Connection, Row};
use rusqlite::Error::QueryReturnedNoRows;
use crate::{EditableTaskData, Tag, EditableTagData, GeneratedTagData, TagId, Task, TaskId, GeneratedTaskData, FinishedTaskData};
use crate::my_date_time::MyDateTime;

#[derive(Debug, PartialEq)]
/// Errors that can occur during operation of the database.
pub enum DbError {
    /// Error that occurred due to underlying sqlite library.
    RusqliteError { error: rusqlite::Error, },
    /// Error that occurred due to a command to the database with an invalid tag `id`.
    TagDoesNotExistError { id: TagId, },
    /// Error that occurred due to a command to the database with an invalid task `id`.
    TaskDoesNotExistError { id: TaskId, },
    /// Error that occurred due to a command to the database that attempted to change the done
    /// status of the task with `id` to the status it is already in.
    TaskStatusError { id: TaskId, actual_status: bool }
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

    /// Creates a database instance from either an empty/ nonexistent file or an existing database.
    pub fn new<P: AsRef<Path>>(database_file: P) -> DbResult<Self> {
        let connection = Connection::open(database_file)?;
        connection.execute(&format!(r#"
            create table if not exists {} (
                "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                "name" TEXT NOT NULL,
                "color" INTEGER NOT NULL,
                "active" INTEGER NOT NULL,
                "create_time" INTEGER NOT NULL
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

    /// Add a new tag to the database, initializing:
    /// * its unique id.
    /// * its create time to now.
    ///
    /// Returns the fields generated for this tag.
    pub fn add_new_tag(&mut self, data: &EditableTagData) -> DbResult<GeneratedTagData> {
        let now = MyDateTime::from(Utc::now());
        let tx = self.conn.transaction()?;
        tx.execute(&format!(
            "INSERT INTO {} (name, color, active, create_time) values (?1, ?2, ?3, ?4)", Db::TAG_TABLE
        ), (&data.name, &data.color, data.active, &now))?;
        let new_id = tx.last_insert_rowid();
        tx.commit()?;
        Ok(GeneratedTagData {
            id: new_id,
            create_time: now,
        })
    }

    /// Retrieve the tag with this id.
    ///
    /// # Failure
    /// Returns `DbError::TagDoesNotExistError` if the tag doesn't exist in the database.
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

    /// Modifies an existing tag in the database.
    ///
    /// # Failure
    /// Returns `DbError::TagDoesNotExistError` if the tag being modified doesn't exist in the database.
    pub fn modify_tag(&mut self, id: TagId, modify: &EditableTagData) -> DbResult<()> {
        let tx = self.conn.transaction()?;
        let rows = tx.execute(&format!(r#"
                UPDATE {} SET
                    name = ?2,
                    color = ?3,
                    active = ?4
                WHERE id = ?1;
            "#, { Db::TAG_TABLE }), (id, &modify.name, &modify.color, modify.active))?;
        tx.commit()?;
        match rows {
            0 => Err(DbError::TagDoesNotExistError { id }),
            1 => Ok(()),
            other => panic!("Modify tag changed {} rows!", other),
        }
    }

    /// Delete a tag by its id in the database (and removes it from any tasks that have this tag).
    ///
    /// # Failure
    /// Returns `DbError::TagDoesNotExistError` if the tag being deleted doesn't exist in the database.
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

    /// Add a new tag to the database, initializing:
    /// * Its id
    /// * Its create time to now
    /// * Its last edit time to now
    /// * Its done time to None
    ///
    /// Returns the fields generated for this task.
    pub fn add_new_task(&mut self, data: &EditableTaskData) -> DbResult<GeneratedTaskData> {
        todo!()
    }

    /// Retrieve the task with this id.
    ///
    /// # Failure
    /// Returns `DbError::TaskDoesNotExistError` if the task doesn't exist in the database.
    pub fn task_by_id(&self, id: TaskId) -> DbResult<Task> {
        todo!()
    }

    /// Modifies an existing task in the database.
    ///
    /// # Failure
    /// Returns `DbError::TaskDoesNotExistError` if the task being modified doesn't exist in the database.
    pub fn modify_task(&mut self, id: TaskId, data: EditableTaskData) -> DbResult<()> {
        todo!()
    }

    /// Delete a task by its id in the database.
    ///
    /// # Failure
    /// Returns `DbError::TaskDoesNotExistError` if the task being deleted doesn't exist in the database.
    pub fn delete_task(&mut self, id: TaskId) -> DbResult<()> {
        todo!()
    }

    /// Mark a task as done, updating the done time of this task. Returns the new done time.
    ///
    /// # Failure
    /// Returns `DbError::TaskDoesNotExistError` if the task doesn't exist in the database.
    /// Returns `DbError::TaskStatusError` if the task is already finished.
    pub fn finish_task(&mut self, id: TaskId) -> DbResult<FinishedTaskData> {
        todo!()
    }

    /// Mark a task as not done, updating the done time of this task. Returns the new done time.
    ///
    /// # Failure
    /// Returns `DbError::TaskDoesNotExistError` if the task doesn't exist in the database.
    /// Returns `DbError::TaskStatusError` if the task is already not finished.
    pub fn unfinish_task(&mut self, id: TaskId) -> DbResult<FinishedTaskData> {
        todo!()
    }

    fn tag_from_row(row: &Row) -> rusqlite::Result<Tag> {
        Ok(Tag {
            id: row.get("id")?,
            name: row.get("name")?,
            color: row.get("color")?,
            active: row.get("active")?,
            create_time: row.get("create_time")?,
        })
    }
}